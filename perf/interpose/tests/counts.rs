//! Proves the counters: the fixture runs under the library with no rounds and with a hundred on each of its threads,
//! the difference between the two runs must match what the rounds allocate, to the byte

use std::{
	collections::HashMap,
	env, fs,
	path::PathBuf,
	process::{self, Command},
};

/// The rounds of the measured run
const ROUNDS: i64 = 100;

/// The threads of the fixture, each runs every round
const THREADS: i64 = 4;

/// The size the fixture's realloc grows its block to
const GROWN: i64 = 1 << 20;

/// The library as cargo builds it for the tests, next to the test binary
const LIBRARY: &str =
	if cfg!(target_os = "macos") { "libcfonts_perf_interpose.dylib" } else { "libcfonts_perf_interpose.so" };

/// The variable the loader injects a library from
const INJECT: &str = if cfg!(target_os = "macos") { "DYLD_INSERT_LIBRARIES" } else { "LD_PRELOAD" };

/// The variable that names the file for the counters
const OUT: &str = "CFONTS_PERF_INTERPOSE_OUT";

/// The library next to the test binary, or a panic that says how to get it
fn library() -> PathBuf {
	let library = env::current_exe().expect("the test binary knows its own path").with_file_name(LIBRARY);
	assert!(
		library.is_file(),
		"{} is missing, cargo builds it with the tests, run `cargo test` again",
		library.display()
	);

	library
}

/// The fixture for some rounds under the library
fn fixture(rounds: i64) -> Command {
	let mut command = Command::new(env!("CARGO_BIN_EXE_allocate"));
	// zero padded so both runs pass an argument of the same length, its string is an allocation too
	command.arg(format!("{rounds:03}")).env(INJECT, library()).env_remove(OUT);
	command
}

/// Runs the fixture for some rounds under the library and reads back its counters
fn counters(rounds: i64) -> HashMap<String, i64> {
	let out = env::temp_dir().join(format!("cfonts-perf-interpose-test-{}-{rounds}.counters", process::id()));
	let status = fixture(rounds).env(OUT, &out).status().expect("the fixture should start");
	assert!(status.success(), "the fixture failed with {status}");

	let text = fs::read_to_string(&out).unwrap_or_else(|error| {
		panic!("the library wrote no counters to {} ({error}), the loader refused the injection", out.display())
	});
	fs::remove_file(&out).expect("the counters file should be removable");
	assert_eq!(text.lines().count(), 1, "the library writes one line, once, at exit: {text:?}");

	text
		.split_whitespace()
		.map(|pair| {
			let (name, value) = pair.split_once('=').expect("every counter is a name=value pair");
			(name.to_string(), value.parse().expect("every counter value is a number"))
		})
		.collect()
}

#[test]
fn counts_every_call_block_and_byte_of_the_fixture() {
	let baseline = counters(0);
	let measured = counters(ROUNDS);
	let delta = |name: &str| measured[name] - baseline[name];
	let calls = ROUNDS * THREADS;

	assert_eq!(delta("mallocs"), 2 * calls, "a plain malloc and the one realloc grows from");
	assert_eq!(delta("callocs"), calls, "calloc never counts again as the malloc underneath it");
	assert_eq!(delta("reallocs"), calls);
	assert_eq!(delta("aligned"), 2 * calls, "posix_memalign and aligned_alloc");
	assert_eq!(delta("frees"), 5 * calls, "the free of a null pointer counts nothing");
	assert_eq!(delta("allocations"), 6 * calls, "every call above makes one block, the growing realloc included");
	assert_eq!(delta("bytes"), calls * (100 + 10 * 10 + 16 + GROWN + 100 + 128));
	assert_eq!(delta("live"), 0, "every block is freed again");
	assert!(
		measured["peak"] >= GROWN,
		"the grown block alone peaks at {GROWN} bytes, the counter says {}",
		measured["peak"]
	);
}

#[test]
fn runs_cleanly_without_a_file_to_write_to() {
	let status = fixture(ROUNDS).status().expect("the fixture should start");

	assert!(status.success(), "the fixture failed with {status} under the library without {OUT}");
}
