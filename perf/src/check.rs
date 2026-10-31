//! The fairness check: the probe and every scenario on both paths and every measured version, each in a child process
//! under the time limit, before anything is measured
//!
//! A render passes when its output
//! - contains every escape code family the scenario paints with
//! - has no row wider than 80 visible columns once the escape codes are gone
//!
//! and the check counts the rows that carry glyphs, so the run reports where v3 and v4 differ when both run
//!
//! The probe must also wrap at exactly 80 columns, a path and version whose probe does not is unfair in every scenario,
//! a narrower canvas than 80 passes every other test

use std::time::Duration;

use cfonts_perf::{COLUMNS, Path, Scenario, Version};
use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::{
	child::{Crash, RENDER_TASK, TIME_LIMIT, run_limited, task},
	cli::Binaries,
};

/// What the check found for one scenario on one path and version
#[derive(Clone, Debug)]
pub enum Outcome {
	/// Rendered under the fixed conditions, with the rows that carry glyphs, the widest row and the child's time
	Passed { glyph_rows: usize, widest: usize, time: Duration },

	/// Crashed, was killed or ran out of time
	Crashed(Crash),

	/// Rendered but broke a fixed condition
	Unfair(String),

	/// The scenario has no such path, the startup floor renders nothing in process
	Absent,
}

impl Outcome {
	/// The table cell that stands in for a measurement, `None` when this one may be measured
	pub fn blocked(&self) -> Option<String> {
		match self {
			Self::Passed { .. } => None,
			Self::Crashed(crash) => Some(format!("crashed: {}", crash.reason)),
			Self::Unfair(problem) => Some(format!("unfair: {problem}")),
			Self::Absent => Some(String::from("n/a")),
		}
	}
}

/// Everything the check found for one scenario
pub struct Verdict {
	/// The scenario checked
	pub scenario: Scenario,

	/// The outcomes by path, then by version
	outcomes: [[Outcome; 2]; 2],
}

impl Verdict {
	/// The outcome of one path and version
	pub fn outcome(&self, path: Path, version: Version) -> &Outcome {
		&self.outcomes[path as usize][version as usize]
	}
}

/// Checks the probe and every scenario on both paths and every measured version, in parallel children
pub fn check(
	binaries: &Binaries,
	versions: &[Version],
	mut scenarios: Vec<Scenario>,
	bar: &ProgressBar,
) -> Vec<Verdict> {
	scenarios.push(Scenario::probe());
	let jobs: Vec<(usize, Path, Version)> = (0..scenarios.len())
		.flat_map(|index| Path::BOTH.map(|path| versions.iter().map(move |&version| (index, path, version))))
		.flatten()
		.collect();
	bar.set_length(jobs.len() as u64);

	let outcomes: Vec<Outcome> = jobs
		.par_iter()
		.map(|&(index, path, version)| {
			let scenario = &scenarios[index];
			bar.set_message(format!("{} {path} {version}", scenario.name));
			let outcome = check_one(binaries, scenario, path, version);
			bar.inc(1);
			outcome
		})
		.collect();

	let mut verdicts: Vec<Verdict> = scenarios
		.into_iter()
		.map(|scenario| Verdict {
			scenario,
			outcomes: [[Outcome::Absent, Outcome::Absent], [Outcome::Absent, Outcome::Absent]],
		})
		.collect();
	for (&(index, path, version), outcome) in jobs.iter().zip(outcomes) {
		verdicts[index].outcomes[path as usize][version as usize] = outcome;
	}

	let probe = verdicts.pop().expect("the probe is the last verdict");
	for path in Path::BOTH {
		for &version in versions {
			let Some(problem) = probe_problem(probe.outcome(path, version)) else {
				continue;
			};
			for verdict in &mut verdicts {
				let outcome = &mut verdict.outcomes[path as usize][version as usize];
				if matches!(outcome, Outcome::Passed { .. }) {
					*outcome = Outcome::Unfair(problem.clone());
				}
			}
		}
	}

	verdicts
}

/// One line for every crash and every broken condition, and with v3 in the run one for every scenario
/// whose glyph rows differ between the versions
pub fn problems(versions: &[Version], verdicts: &[Verdict]) -> Vec<String> {
	let mut lines = Vec::new();
	for verdict in verdicts {
		let name = &verdict.scenario.name;
		for path in Path::BOTH {
			for &version in versions {
				match verdict.outcome(path, version) {
					Outcome::Crashed(crash) => lines.push(format!("{name} {path} {version} crashed: {crash}")),
					Outcome::Unfair(problem) => lines.push(format!("{name} {path} {version} unfair: {problem}")),
					Outcome::Passed { .. } | Outcome::Absent => {}
				}
			}

			let rows = Version::BOTH.map(|version| match verdict.outcome(path, version) {
				Outcome::Passed { glyph_rows, .. } if versions.contains(&version) => Some(*glyph_rows),
				_ => None,
			});
			if let [Some(v3), Some(v4)] = rows
				&& v3 != v4
			{
				lines.push(format!("{name} {path} glyph rows differ: v3 {v3}, v4 {v4}"));
			}
		}
	}

	lines
}

/// What the probe found wrong on one path and version, `None` when it wrapped at exactly 80 columns in true color
fn probe_problem(outcome: &Outcome) -> Option<String> {
	match outcome {
		Outcome::Passed { widest, .. } if *widest == COLUMNS => None,
		Outcome::Passed { widest, .. } => Some(format!("the probe wraps at {widest} columns, not {COLUMNS}")),
		Outcome::Unfair(problem) => Some(format!("the probe found {problem}")),
		Outcome::Crashed(crash) => Some(format!("the probe crashed, {crash}")),
		Outcome::Absent => Some(String::from("the probe did not run")),
	}
}

/// Renders one scenario on one path and version in a child and holds the output against the fixed conditions
fn check_one(binaries: &Binaries, scenario: &Scenario, path: Path, version: Version) -> Outcome {
	let command = match path {
		Path::Api if !scenario.renders() => return Outcome::Absent,
		Path::Api => task(RENDER_TASK, version, scenario),
		Path::Cli => binaries.command(version, scenario),
	};

	match run_limited(command, TIME_LIMIT) {
		Ok(finished) => inspect(scenario, &finished.stdout, finished.time),
		Err(crash) => Outcome::Crashed(crash),
	}
}

/// Holds one output against the fixed conditions
fn inspect(scenario: &Scenario, output: &str, time: Duration) -> Outcome {
	if !scenario.renders() {
		return if output.trim().is_empty() {
			Outcome::Unfair(String::from("printed nothing"))
		} else {
			Outcome::Passed { glyph_rows: 0, widest: 0, time }
		};
	}

	if let Some(paint) = scenario.paint.iter().find(|paint| !paint.found_in(output)) {
		return Outcome::Unfair(format!("no {paint}"));
	}

	let rows: Vec<String> = output.lines().map(visible).collect();
	let widest = rows.iter().map(|row| row.chars().count()).max().unwrap_or(0);
	if widest > COLUMNS {
		return Outcome::Unfair(format!("a row {widest} columns wide"));
	}

	Outcome::Passed { glyph_rows: rows.iter().filter(|row| !row.trim().is_empty()).count(), widest, time }
}

/// One row as a terminal shows it, every escape sequence removed
fn visible(row: &str) -> String {
	let mut text = String::with_capacity(row.len());
	let mut characters = row.chars();

	while let Some(character) = characters.next() {
		if character != '\x1b' {
			text.push(character);
		} else if characters.next() == Some('[') {
			// a control sequence ends with its final byte, any other escape is two characters long
			characters.by_ref().find(|byte| ('@'..='~').contains(byte));
		}
	}

	text
}
