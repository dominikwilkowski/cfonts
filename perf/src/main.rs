//! The perf runner: `perf` prints what changed against the results in `README.md`, `save` writes them there
//! and `compare` writes the comparison with v3, each after one fairness check, the make targets run it

mod check;
mod child;
mod cli;
mod measure;
mod readme;

use std::{collections::HashMap, env, process::ExitCode, thread};

use cfonts_perf::{Scenario, Version, scenarios};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::ThreadPoolBuilder;

use crate::{
	check::{check, problems},
	child::{CHILD_FLAG, answer},
	cli::Binaries,
	readme::{README, Written, bytes, cell, count, duration, parse, significant},
};

/// The variable that picks some scenarios by name, separated by commas, for a quicker run
const SCENARIOS_VARIABLE: &str = "CFONTS_PERF_SCENARIOS";

/// The change in percent of an API median `make perf` reports, unchanged code moves it between runs by up to 4.1%,
/// so 10% stays clear of that
const API_NOISE: f64 = 10.0;

/// The change in percent of a CLI median `make perf` reports, process start follows the load of the machine,
/// unchanged code moves it between runs by up to 46% on a busy machine, so 60% stays clear of that
const CLI_NOISE: f64 = 60.0;

/// The change in percent of a peak resident set size `make perf` reports, unchanged code moves it between runs
/// by up to three pages of 16 KiB on Apple silicon, 1.9% of a 2.5 MiB process, so 4% stays clear of that
const RSS_NOISE: f64 = 4.0;

/// The paths `make perf` reports, each with its columns in the order its block lists them
const REPORT: [(&str, &[&str]); 2] = [
	("API", &["api median", "api peak heap", "api allocations", "api bytes"]),
	("CLI", &["cli median", "cli peak heap", "cli allocations", "cli bytes", "cli peak RSS"]),
];

/// The width of a label in a report block, the longest label `allocations:` and one space
const LABEL_WIDTH: usize = 13;

/// How one value of this run moved against the README
enum Movement {
	/// Within the column's noise, reported as 0%
	Still,

	/// Not measured on this path in the README nor in this run, the startup floor has no API render
	Unmeasured,

	/// Beyond the column's noise, by this many percent
	Moved(f64),

	/// No number to compare, the words say why
	Missing(String),
}

/// Every value of a run by scenario, column and version, or why it is missing
pub type Values = HashMap<(String, &'static str, Version), Result<u64, String>>;

/// One column of the results, a column per measured version in the tables
pub struct Measure {
	/// The test whose tables hold it
	pub test: &'static str,

	/// The path and what it counts, the table header after the version
	pub column: &'static str,

	/// How the tables write a value
	pub format: fn(u64) -> String,

	/// The change in percent `make perf` reports, zero reports every change
	pub noise: f64,

	/// The header of the ratio column, v4 over v3, that follows its columns in the comparison
	pub ratio: Option<&'static str>,
}

/// Every column, in the order of the tables, exact counts and peak heaps report every change
pub const MEASURES: [Measure; 9] = [
	Measure { test: "speed", column: "api median", format: duration, noise: API_NOISE, ratio: Some("api ratio") },
	Measure { test: "speed", column: "cli median", format: duration, noise: CLI_NOISE, ratio: Some("cli ratio") },
	Measure { test: "allocations", column: "api allocations", format: count, noise: 0.0, ratio: Some("ratio") },
	Measure { test: "allocations", column: "api bytes", format: bytes, noise: 0.0, ratio: None },
	Measure { test: "allocations", column: "cli allocations", format: count, noise: 0.0, ratio: None },
	Measure { test: "allocations", column: "cli bytes", format: bytes, noise: 0.0, ratio: None },
	Measure { test: "memory", column: "api peak heap", format: bytes, noise: 0.0, ratio: Some("ratio") },
	Measure { test: "memory", column: "cli peak heap", format: bytes, noise: 0.0, ratio: None },
	Measure { test: "memory", column: "cli peak RSS", format: bytes, noise: RSS_NOISE, ratio: None },
];

/// What a run does with its numbers
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
	/// Measures v4 and prints what changed against the results in the README
	Perf,

	/// Measures v4 and writes the results into the README
	Save,

	/// Measures v3 and v4 and writes the comparison into the README
	Compare,
}

fn main() -> ExitCode {
	let args: Vec<String> = env::args().skip(1).collect();
	let args: Vec<&str> = args.iter().map(String::as_str).collect();
	let mode = match args[..] {
		[CHILD_FLAG, task, version, scenario, ..] => return answer(task, version, scenario, args.get(4).copied()),
		["perf"] => Mode::Perf,
		["save"] => Mode::Save,
		["compare"] => Mode::Compare,
		_ => {
			eprintln!("cfonts-perf takes perf, save or compare, run `make perf`, `make perf-save` or `make compare`");
			return ExitCode::FAILURE;
		}
	};

	match run(mode) {
		Ok(lines) => {
			println!("{}", lines.join("\n"));
			ExitCode::SUCCESS
		}
		Err(message) => {
			eprintln!("{message}");
			ExitCode::FAILURE
		}
	}
}

/// Checks and measures every scenario, then reports, writes or compares, the lines to print at the end
fn run(mode: Mode) -> Result<Vec<String>, String> {
	let versions: &[Version] = if mode == Mode::Compare { &Version::BOTH } else { &[Version::V4] };
	let binaries = Binaries::from_env(versions)?;
	let scenarios = picked()?;
	let written = match mode {
		Mode::Perf => Some(readme::written()?),
		Mode::Save => readme::check_blocks("results").map(|()| None)?,
		Mode::Compare => readme::check_blocks("compare").map(|()| None)?,
	};

	// one core stays free for the runner and the system
	let workers = thread::available_parallelism().map_or(1, |cores| cores.get().saturating_sub(1).max(1));
	ThreadPoolBuilder::new().num_threads(workers).build_global().map_err(|error| error.to_string())?;

	let verdicts = with_bar("fairness check", |bar| check(&binaries, versions, scenarios, bar));
	let mut lines = problems(versions, &verdicts);
	let mut values = Values::new();
	with_bar("speed", |bar| measure::speed(&binaries, versions, &verdicts, &mut values, bar));
	with_bar("allocations, memory", |bar| measure::heap(&binaries, versions, &verdicts, &mut values, bar));

	let names: Vec<String> = verdicts.into_iter().map(|verdict| verdict.scenario.name).collect();
	match written {
		Some(written) => lines.extend(changes(&names, &values, &written)),
		None if mode == Mode::Save => {
			readme::write("results", versions, &names, &values)?;
			lines.push(format!("Wrote the results of {} scenarios into {README}", names.len()));
		}
		None => {
			readme::write("compare", versions, &names, &values)?;
			lines.extend(comparison(&names, &values));
			lines.push(format!("Wrote the comparison of {} scenarios into {README}", names.len()));
		}
	}

	Ok(lines)
}

/// Every scenario, or the ones `CFONTS_PERF_SCENARIOS` names
fn picked() -> Result<Vec<Scenario>, String> {
	let Ok(names) = env::var(SCENARIOS_VARIABLE) else {
		return Ok(scenarios());
	};

	let picked: Vec<Scenario> =
		scenarios().into_iter().filter(|scenario| names.split(',').any(|name| name == scenario.name)).collect();
	if picked.len() == names.split(',').count() {
		Ok(picked)
	} else {
		Err(format!("{SCENARIOS_VARIABLE} is {names:?}, name scenarios perf/src/lib.rs lists, separated by commas"))
	}
}

/// Runs one test under a progress bar that names it and the scenario it measures, the bar is gone when it ends
fn with_bar<T>(test: &str, work: impl FnOnce(&ProgressBar) -> T) -> T {
	let style = ProgressStyle::with_template("{prefix:>19} [{bar:40}] {pos}/{len} {msg}").expect("the template parses");
	let bar = ProgressBar::no_length().with_style(style.progress_chars("=> ")).with_prefix(test.to_string());
	let result = work(&bar);
	bar.finish_and_clear();

	result
}

/// One block per scenario with every column of both paths in it, then one line that counts the scenarios that moved beyond noise
fn changes(names: &[String], values: &Values, written: &Written) -> Vec<String> {
	let mut lines = Vec::new();
	let mut changed = 0;
	for name in names {
		let (block, moved) = block(name, |measure| against_readme(name, measure, values, written));
		changed += usize::from(moved);
		lines.extend(block);
		lines.push(String::new());
	}

	let checked = names.len();
	lines.push(match changed {
		0 => format!("No change beyond noise, {checked} scenarios checked"),
		changed => format!("{changed} of {checked} scenarios changed beyond noise"),
	});
	lines
}

/// One block per scenario with every column of both paths in it, v4 against v3
fn comparison(names: &[String], values: &Values) -> Vec<String> {
	let mut lines = Vec::new();
	for name in names {
		let (block, _) = block(name, |measure| against_v3(name, measure, values));
		lines.extend(block);
		lines.push(String::new());
	}
	lines
}

/// The report block of one scenario and whether anything in it moved, `shown` gives a column's text
/// and whether that column moved
fn block(name: &str, shown: impl Fn(&Measure) -> (String, bool)) -> (Vec<String>, bool) {
	let mut lines = vec![style(format!("== {name} ==")).yellow().to_string()];
	let mut moved = false;

	for (path, columns) in REPORT {
		lines.push(path.to_string());
		for column in columns {
			let measure =
				MEASURES.iter().find(|measure| measure.column == *column).expect("every reported column is a measure");
			let label = format!("{}:", column.split_once(' ').map_or(*column, |(_, label)| label));
			let (text, column_moved) = shown(measure);
			moved |= column_moved;
			lines.push(format!("  {label:<LABEL_WIDTH$}{text}"));
		}
	}

	(lines, moved)
}

/// One column of this run against the README and whether it moved beyond noise,
/// a drop is green and a rise red, lower is better for every column, a value within its noise reads 0%
fn against_readme(name: &str, measure: &Measure, values: &Values, written: &Written) -> (String, bool) {
	let now = &values[&(name.to_string(), measure.column, Version::V4)];
	let before = written.get(&(name.to_string(), measure.column)).map_or("not in the README", String::as_str);

	match movement(measure, before, now) {
		Movement::Still => (String::from("0%"), false),
		Movement::Unmeasured => (String::from("n/a"), false),
		Movement::Moved(percent) => {
			let places = decimals(percent);
			(painted(format!("{percent:+.places$}%"), percent < 0.0), true)
		}
		Movement::Missing(reason) => (style(reason).yellow().to_string(), true),
	}
}

/// One column of v4 against v3 as a factor and whether the two differ,
/// green when v4 is ahead and red when it is behind, lower is better for every column
fn against_v3(name: &str, measure: &Measure, values: &Values) -> (String, bool) {
	let [v3, v4] = Version::BOTH.map(|version| &values[&(name.to_string(), measure.column, version)]);
	let (better, worse) = if measure.test == "speed" { ("faster", "slower") } else { ("less", "more") };

	match (v3, v4) {
		(Ok(v3), Ok(v4)) if v3 == v4 => (String::from("same"), false),
		(Ok(v3), Ok(v4)) if *v3 > 0 && *v4 > 0 => {
			let ahead = v4 < v3;
			let (factor, word) = if ahead { (*v3 as f64 / *v4 as f64, better) } else { (*v4 as f64 / *v3 as f64, worse) };
			(painted(format!("{}x {word}", significant(factor)), ahead), true)
		}
		_ if cell(v3, measure.format) == cell(v4, measure.format) => (cell(v4, measure.format), false),
		_ => {
			let text = format!("{} against {} in v3", cell(v4, measure.format), cell(v3, measure.format));
			(style(text).yellow().to_string(), true)
		}
	}
}

/// `text` in green when it is good news and in red when it is not
fn painted(text: String, good: bool) -> String {
	if good { style(text).green().to_string() } else { style(text).red().to_string() }
}

/// The decimals a change shows, a few allocations out of thousands is a real change, so small moves keep three
fn decimals(percent: f64) -> usize {
	if percent.abs() < 0.1 { 3 } else { 1 }
}

/// Whether a change still reads as one at the decimals it shows, a change that rounds to `-0.000%` reads as none
fn shows(percent: f64) -> bool {
	let scale = 10_f64.powi(decimals(percent) as i32);
	(percent * scale).round() != 0.0
}

/// How one column of this run moved against the README, judged against the column's noise
fn movement(measure: &Measure, before: &str, now: &Result<u64, String>) -> Movement {
	match (parse(before), now) {
		(Some(before), Ok(now)) if before == *now => Movement::Still,
		(Some(0), Ok(_)) => Movement::Missing(format!("{} now, 0 in the README", cell(now, measure.format))),
		(Some(before), Ok(now)) => {
			let percent = (*now as f64 / before as f64 - 1.0) * 100.0;
			let beyond_noise = measure.noise == 0.0 || percent.abs() > measure.noise;
			if beyond_noise && shows(percent) { Movement::Moved(percent) } else { Movement::Still }
		}
		(None, _) if before == cell(now, measure.format) => Movement::Unmeasured,
		_ => Movement::Missing(format!("{} now, {before} in the README", cell(now, measure.format))),
	}
}
