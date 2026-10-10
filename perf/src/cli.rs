//! The shipped binaries: where the make targets built them, how a scenario runs them, counted runs under the interpose
//! library and plain runs for the peak resident set size

use std::{
	env, fs,
	io::{self, Read},
	mem,
	os::unix::process::ExitStatusExt,
	path::PathBuf,
	process::{self, Child, Command, ExitStatus},
	sync::atomic::{AtomicU64, Ordering::Relaxed},
	thread,
};

use cfonts_perf::{COLUMNS, Scenario, Version};
use libc::{pid_t, rusage, wait4};

use crate::child::{Crash, under_fixed_conditions};

/// The variable `make compare` names the v3 binary in
const V3_VARIABLE: &str = "CFONTS_PERF_V3";

/// The variable every make target names the v4 binary in
const V4_VARIABLE: &str = "CFONTS_PERF_V4";

/// The variable every make target names the interpose library in
const INTERPOSE_VARIABLE: &str = "CFONTS_PERF_INTERPOSE";

/// The variable the interpose library reads the file for its counters from
const INTERPOSE_OUT_VARIABLE: &str = "CFONTS_PERF_INTERPOSE_OUT";

/// The variable the loader injects a library from
const INJECT_VARIABLE: &str = if cfg!(target_os = "macos") { "DYLD_INSERT_LIBRARIES" } else { "LD_PRELOAD" };

/// Numbers the counter files of one run, so no two counted runs share one
static COUNTED_RUNS: AtomicU64 = AtomicU64::new(0);

/// The files the runner needs, handed over by the make targets
pub struct Binaries {
	/// The v3 binary, installed by `cargo install cfonts --version =1.3.0`, `None` when v3 is not measured
	v3: Option<PathBuf>,

	/// The v4 binary, the release build of this repository
	v4: PathBuf,

	/// The interpose library that counts allocations
	interpose: PathBuf,
}

impl Binaries {
	/// Reads the paths the measured versions need, or explains how to get them
	pub fn from_env(versions: &[Version]) -> Result<Self, String> {
		Ok(Self {
			v3: if versions.contains(&Version::V3) { Some(existing_file(V3_VARIABLE)?) } else { None },
			v4: existing_file(V4_VARIABLE)?,
			interpose: existing_file(INTERPOSE_VARIABLE)?,
		})
	}

	/// The shipped binary of one version, set up for one scenario under the child conditions
	pub fn command(&self, version: Version, scenario: &Scenario) -> Command {
		let binary = match version {
			Version::V3 => self.v3.as_ref().expect("v3 runs only when it is measured, and then from_env found its binary"),
			Version::V4 => &self.v4,
		};
		let mut command = Command::new(binary);
		under_fixed_conditions(command.args(scenario.args(version)));

		// v4 takes its width from FORCE_SIZE, v3 has no such variable and falls back to 80 columns without a terminal
		if version == Version::V4 {
			command.env("FORCE_SIZE", COLUMNS.to_string());
		}

		command
	}

	/// Runs one command with the interpose library injected, its blocks, bytes and peak heap
	pub fn run_counted(&self, mut command: Command) -> Result<Vec<u64>, String> {
		let out =
			env::temp_dir().join(format!("cfonts-perf-{}-{}.counters", process::id(), COUNTED_RUNS.fetch_add(1, Relaxed)));
		command.env(INJECT_VARIABLE, &self.interpose).env(INTERPOSE_OUT_VARIABLE, &out);
		run_to_end(command)?;

		let line = fs::read_to_string(&out).map_err(|error| {
			format!(
				"the interpose library wrote no counters to {} ({error}), the loader refused to inject it, \
				 a binary signed with the hardened runtime needs `codesign --remove-signature` first",
				out.display()
			)
		})?;
		fs::remove_file(&out).ok();

		let counter = |name: &str| {
			line
				.split_whitespace()
				.find_map(|pair| pair.strip_prefix(name)?.strip_prefix('=')?.parse::<u64>().ok())
				.ok_or_else(|| format!("the counters {line:?} have no {name}"))
		};

		["allocations", "bytes", "peak"].into_iter().map(counter).collect()
	}
}

/// Runs one command as it ships, nothing injected, its peak resident set size in bytes
pub fn peak_rss(command: Command) -> Result<u64, String> {
	run_to_end(command)
}

/// Runs one command to its end with its output read and discarded, its peak resident set size in bytes
fn run_to_end(mut command: Command) -> Result<u64, String> {
	let mut child = command.spawn().map_err(|error| format!("{:?} did not start ({error})", command.get_program()))?;
	let mut stdout = child.stdout.take().expect("stdout is piped");
	let mut stderr = child.stderr.take().expect("stderr is piped");
	let mut errors = Vec::new();
	thread::scope(|scope| {
		scope.spawn(|| stderr.read_to_end(&mut errors));
		io::copy(&mut stdout, &mut io::sink())
	})
	.map_err(|error| format!("reading the output failed ({error})"))?;

	// a table cell holds one line, so a crash gives its reason alone, as the fairness check's cells do
	let (status, peak_rss) = wait_with_rusage(&child)?;
	if status.success() {
		Ok(peak_rss)
	} else {
		Err(format!("crashed: {}", Crash::from_status(status, &String::from_utf8_lossy(&errors)).reason))
	}
}

/// One path from the environment that must name an existing file
fn existing_file(variable: &str) -> Result<PathBuf, String> {
	let Some(path) = env::var_os(variable).map(PathBuf::from) else {
		return Err(format!(
			"{variable} is not set, run `make perf`, `make perf-save` or `make compare` from the repository root, \
			 they build what the runner measures and pass the paths"
		));
	};

	if path.is_file() {
		Ok(path)
	} else {
		Err(format!("{variable} names {}, which does not exist, the make targets build it", path.display()))
	}
}

/// Reaps one child with wait4, its exit status and its peak resident set size in bytes
fn wait_with_rusage(child: &Child) -> Result<(ExitStatus, u64), String> {
	let mut status = 0;
	// rusage is plain integers, all zero is a valid value
	let mut usage: rusage = unsafe { mem::zeroed() };

	// the pid is this process's own child, not reaped yet, both pointers outlive the call
	if unsafe { wait4(child.id() as pid_t, &mut status, 0, &mut usage) } < 0 {
		return Err(format!("waiting for the child failed ({})", io::Error::last_os_error()));
	}

	// macOS reports ru_maxrss in bytes, Linux in kilobytes
	let unit = if cfg!(target_os = "macos") { 1 } else { 1024 };
	Ok((ExitStatus::from_raw(status), usage.ru_maxrss as u64 * unit))
}
