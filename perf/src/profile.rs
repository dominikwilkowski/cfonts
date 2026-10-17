//! A flamegraph and a heap profile of the API path per scenario, to find where a render spends its time and memory

use std::{
	fs::File,
	hint::black_box,
	path::Path,
	time::{Duration, Instant},
};

use cfonts_perf::{Scenario, render_v4};
use dhat::Profiler;
use pprof::ProfilerGuardBuilder;

use crate::measure::hand_allocations_to_dhat;

/// How long one scenario renders under the sampling profiler, a few thousand samples
pub const FLAME_TIME: Duration = Duration::from_secs(3);

/// How often the sampling profiler reads the stack, per second
const FREQUENCY: i32 = 1000;

/// Answers the profile task in a child, both files land in `directory`:
/// - `<scenario>.svg`, the flamegraph of the scenario rendered over and over under the sampling profiler
/// - `<scenario>.dhat.json`, the heap profile of one render under dhat, every allocation with where it came from
pub fn api_profiles(scenario: &Scenario, directory: &Path) {
	let options = scenario.v4_options();

	let guard = ProfilerGuardBuilder::default().frequency(FREQUENCY).build().expect("the sampling profiler starts");
	let started = Instant::now();
	while started.elapsed() < FLAME_TIME {
		black_box(render_v4(&options));
	}
	let report = guard.report().build().expect("the sampling profiler reports");
	let graph = File::create(directory.join(format!("{}.svg", scenario.name))).expect("the flamegraph file opens");
	report.flamegraph(graph).expect("the flamegraph is written");

	hand_allocations_to_dhat();
	let _profiler = Profiler::builder().file_name(directory.join(format!("{}.dhat.json", scenario.name))).build();
	drop(render_v4(&options));
}
