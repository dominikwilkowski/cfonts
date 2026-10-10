//! The fixture of the counter test: allocates exactly the blocks its argument asks for, through every counted call
//!
//! Four threads run the rounds at the same time, each round makes and frees
//! - one malloc of 100 bytes
//! - one calloc of 10 times 10 bytes
//! - one malloc of 16 bytes grown by realloc to one mebibyte
//! - one posix_memalign of 100 bytes at a 64 byte alignment
//! - one aligned_alloc of 128 bytes at a 64 byte alignment
//!
//! and frees a null pointer, which counts nothing

use std::{env, ffi::c_void, hint::black_box, process::ExitCode, ptr, thread};

use libc::{aligned_alloc, calloc, free, malloc, posix_memalign, realloc};

/// The threads that run the rounds at the same time, the counter test multiplies by the same number
const THREADS: usize = 4;

/// The size the realloc of every round grows its block to
const GROWN: usize = 1 << 20;

fn main() -> ExitCode {
	let Some(rounds) = env::args().nth(1).and_then(|rounds| rounds.parse::<u64>().ok()) else {
		eprintln!("pass the number of rounds, for example: allocate 100");
		return ExitCode::FAILURE;
	};

	// joined one by one, a scope returns before its threads finish exiting, and their last frees would race the counters
	let threads: Vec<_> = (0..THREADS).map(|_| thread::spawn(move || allocate(rounds))).collect();
	for thread in threads {
		thread.join().expect("a fixture thread only allocates and frees");
	}

	ExitCode::SUCCESS
}

/// The rounds of one thread
fn allocate(rounds: u64) {
	for _ in 0..rounds {
		// black_box keeps the optimiser from pairing up and removing each malloc and free
		unsafe {
			free(black_box(malloc(100)));
			free(black_box(calloc(10, 10)));
			free(black_box(realloc(black_box(malloc(16)), GROWN)));

			let mut aligned: *mut c_void = ptr::null_mut();
			posix_memalign(&mut aligned, 64, 100);
			free(black_box(aligned));

			free(black_box(aligned_alloc(64, 128)));
			free(black_box(ptr::null_mut()));
		}
	}
}
