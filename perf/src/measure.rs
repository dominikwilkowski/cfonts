//! The three tests: speed one measurement after the other, allocations and memory in parallel children
//!
//! - speed: the median time of one render, timed in a child for the API path and from the runner for the CLI path
//! - allocations: the blocks and bytes of one render, counted by dhat in a child after a first render on the API path
//!   and by the interpose library for the whole process on the CLI path
//! - memory: the peak heap of a child's first render by dhat on the API path, the peak heap of the whole process
//!   by the interpose library and the peak resident set size of a second run with nothing injected on the CLI path

use std::{
	alloc::{GlobalAlloc, Layout, System},
	hint::black_box,
	sync::atomic::{AtomicBool, Ordering::Relaxed},
	time::{Duration, Instant},
};

use cfonts_perf::{Path, Scenario, Version, render_v3, render_v4};
use dhat::{Alloc, HeapStats, Profiler};
use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::{
	Values,
	check::{Outcome, Verdict},
	child::{HEAP_TASK, SPEED_TASK, TIME_LIMIT, run_limited, task},
	cli::{Binaries, peak_rss},
};

/// How long the sampler warms up, at least one run
const WARM_UP: Duration = Duration::from_millis(20);

/// The shortest sample, a faster render runs as often as fills one, so the clock's resolution stays out of the median
const SAMPLE: Duration = Duration::from_millis(1);

/// The fewest samples of one measurement, the slowest renders take only these
const MIN_SAMPLES: usize = 5;

/// The budget of one measurement in fairness check child times, capped at the longest budget
const BUDGET_FACTOR: u32 = 50;

/// The longest budget of one measurement
const MAX_BUDGET: Duration = Duration::from_secs(1);

/// How long a speed or heap child may run, one render's limit for each render a slow scenario takes there,
/// the warm-up and the fewest samples
const CHILD_LIMIT: Duration = TIME_LIMIT.saturating_mul(MIN_SAMPLES as u32 + 1);

/// The columns of the API heap child, in the order it prints them
const API_HEAP: [&str; 3] = ["api allocations", "api bytes", "api peak heap"];

/// The columns of a counted CLI run, in the order of the interpose library's counters
const CLI_HEAP: [&str; 3] = ["cli allocations", "cli bytes", "cli peak heap"];

/// The column of a plain CLI run
const CLI_RSS: [&str; 1] = ["cli peak RSS"];

/// The system allocator, or dhat's once a heap child switches to it before it measures,
/// dhat's forwards to the system allocator, so a block from either is valid for both
struct Allocator;

/// Whether this process hands its allocations to dhat
static DHAT: AtomicBool = AtomicBool::new(false);

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

unsafe impl GlobalAlloc for Allocator {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe { if DHAT.load(Relaxed) { Alloc.alloc(layout) } else { System.alloc(layout) } }
	}

	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
		unsafe { if DHAT.load(Relaxed) { Alloc.alloc_zeroed(layout) } else { System.alloc_zeroed(layout) } }
	}

	unsafe fn dealloc(&self, block: *mut u8, layout: Layout) {
		unsafe { if DHAT.load(Relaxed) { Alloc.dealloc(block, layout) } else { System.dealloc(block, layout) } }
	}

	unsafe fn realloc(&self, block: *mut u8, layout: Layout, size: usize) -> *mut u8 {
		unsafe { if DHAT.load(Relaxed) { Alloc.realloc(block, layout, size) } else { System.realloc(block, layout, size) } }
	}
}

/// Times every scenario on both paths and every measured version, one after the other
pub fn speed(binaries: &Binaries, versions: &[Version], verdicts: &[Verdict], values: &mut Values, bar: &ProgressBar) {
	bar.set_length((verdicts.len() * Path::BOTH.len() * versions.len()) as u64);
	for verdict in verdicts {
		let scenario = &verdict.scenario;
		for (path, column) in [(Path::Api, "api median"), (Path::Cli, "cli median")] {
			for &version in versions {
				bar.set_message(format!("{} {path} {version}", scenario.name));
				let budget = |time: Duration| (time * BUDGET_FACTOR).min(MAX_BUDGET);
				let median = match (verdict.outcome(path, version), path) {
					(Outcome::Passed { time, .. }, Path::Api) => {
						let mut command = task(SPEED_TASK, version, scenario);
						command.arg(budget(*time).as_nanos().to_string());
						run_limited(command, CHILD_LIMIT)
							.map_err(|crash| format!("crashed: {}", crash.reason))
							.and_then(|finished| parse(&finished.stdout))
					}
					(Outcome::Passed { time, .. }, Path::Cli) => {
						let mut command = binaries.command(version, scenario);
						let run = |()| command.output().expect("the binary ran in the fairness check, so it starts");
						Ok(median(budget(*time), || (), run).as_nanos() as u64)
					}
					(blocked, _) => Err(blocked.blocked().unwrap_or_default()),
				};
				values.insert((scenario.name.clone(), column, version), median);
				bar.inc(1);
			}
		}
	}
}

/// Counts the allocations and peaks of every scenario on both paths and every measured version, in parallel children
pub fn heap(binaries: &Binaries, versions: &[Version], verdicts: &[Verdict], values: &mut Values, bar: &ProgressBar) {
	let columns: [&[&'static str]; 3] = [&API_HEAP, &CLI_HEAP, &CLI_RSS];
	let jobs: Vec<(&Verdict, Version, &[&str])> = verdicts
		.iter()
		.flat_map(|verdict| versions.iter().flat_map(move |&version| columns.map(|job| (verdict, version, job))))
		.collect();
	bar.set_length(jobs.len() as u64);

	let measured: Vec<Result<Vec<u64>, String>> = jobs
		.par_iter()
		.map(|&(verdict, version, job)| {
			let scenario = &verdict.scenario;
			bar.set_message(format!("{} {version}", scenario.name));
			let path = if job == API_HEAP { Path::Api } else { Path::Cli };
			let measured = match verdict.outcome(path, version).blocked() {
				Some(cell) => Err(cell),
				None if job == API_HEAP => run_limited(task(HEAP_TASK, version, scenario), CHILD_LIMIT)
					.map_err(|crash| format!("crashed: {}", crash.reason))
					.and_then(|finished| finished.stdout.split_whitespace().map(parse).collect()),
				None if job == CLI_HEAP => binaries.run_counted(binaries.command(version, scenario)),
				None => peak_rss(binaries.command(version, scenario)).map(|rss| vec![rss]),
			};
			bar.inc(1);
			measured
		})
		.collect();

	for ((verdict, version, job), measured) in jobs.into_iter().zip(measured) {
		for (index, &column) in job.iter().enumerate() {
			let number = |numbers: Vec<u64>| numbers.get(index).copied().ok_or_else(|| String::from("too few numbers"));
			let value = measured.clone().and_then(number);
			values.insert((verdict.scenario.name.clone(), column, version), value);
		}
	}
}

/// Answers the speed task in a child: the median nanoseconds of one in process render within the budget,
/// the options are built outside the timing
pub fn api_speed(version: Version, scenario: &Scenario, budget: Duration) -> u128 {
	match version {
		Version::V3 => median(budget, || scenario.v3_options(), render_v3),
		Version::V4 => {
			let options = scenario.v4_options();
			median(budget, || &options, render_v4)
		}
	}
	.as_nanos()
}

/// Answers the heap task in a child: the blocks and bytes of a render after a first one and the peak heap of that first,
/// under dhat started after the options are built
pub fn api_heap(version: Version, scenario: &Scenario) -> [u64; 3] {
	DHAT.store(true, Relaxed);
	let mut v3 = [scenario.v3_options(), scenario.v3_options()].into_iter();
	let v4 = scenario.v4_options();
	let mut render = || match version {
		Version::V3 => drop(render_v3(v3.next().expect("one set of options per render"))),
		Version::V4 => drop(render_v4(&v4)),
	};
	let _profiler = Profiler::builder().testing().build();

	render();
	let first = HeapStats::get();
	render();
	let second = HeapStats::get();

	[second.total_blocks - first.total_blocks, second.total_bytes - first.total_bytes, first.max_bytes as u64]
}

/// The median time of one run of `routine` on an input from `setup`, after a warm-up,
/// in samples until the budget is spent and at least the fewest samples are taken
fn median<I, O>(budget: Duration, mut setup: impl FnMut() -> I, mut routine: impl FnMut(I) -> O) -> Duration {
	let mut sample = |runs: u32| {
		let inputs: Vec<I> = (0..runs).map(|_| setup()).collect();
		let mut outputs = Vec::with_capacity(inputs.len());
		let started = Instant::now();
		outputs.extend(inputs.into_iter().map(|input| routine(black_box(input))));
		let time = started.elapsed() / runs;
		black_box(outputs);
		time
	};

	let warming = Instant::now();
	let mut run = sample(1);
	while warming.elapsed() < WARM_UP {
		run = sample(1);
	}

	let runs = (SAMPLE.as_nanos() / run.as_nanos().max(1)).max(1) as u32;
	let started = Instant::now();
	let mut samples = Vec::new();
	while samples.len() < MIN_SAMPLES || started.elapsed() < budget {
		samples.push(sample(runs));
	}

	samples.sort();
	samples[samples.len() / 2]
}

/// One number a child printed
fn parse(printed: &str) -> Result<u64, String> {
	printed.trim().parse().map_err(|_| format!("the child printed {printed:?} instead of a number"))
}
