//! `perf/README.md` as the baseline: `save` and `compare` write its tables between their marker comments,
//! `perf` reads the results back, the text around the markers stays as it is

use std::{
	collections::HashMap,
	env::consts::{ARCH, OS},
	fs,
	process::Command,
};

use cfonts_perf::{Version, scenarios};
use indicatif::HumanCount;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use tabled::{builder::Builder, settings::Style};

use crate::{MEASURES, Values};

/// The README the tables go into
pub const README: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/README.md");

/// The tests, each with a block in the results and one in the comparison
const TESTS: [&str; 3] = ["speed", "allocations", "memory"];

/// The block of the scenario list, `save` and `compare` write it
const SCENARIOS: &str = "scenarios";

/// The v4 results the README holds, as written, by scenario and column
pub type Written = HashMap<(String, &'static str), String>;

/// Makes sure every block of a section and the scenario list have both their markers, before anything is measured
pub fn check_blocks(section: &str) -> Result<(), String> {
	let readme = read()?;
	TESTS.iter().map(|test| format!("{section}:{test}")).chain([String::from(SCENARIOS)]).try_for_each(|block| {
		split(&readme, &block)?;
		Ok(())
	})
}

/// Writes the tables of one section, `results` or `compare`, and the scenario list, each under a line naming
/// when and where they ran
pub fn write(section: &str, versions: &[Version], names: &[String], values: &Values) -> Result<(), String> {
	let mut readme = read()?;
	for test in TESTS {
		readme = replace(&readme, &format!("{section}:{test}"), &results(test, versions, names, values))?;
	}
	let list: Vec<Vec<String>> = scenarios()
		.into_iter()
		.map(|scenario| vec![format!("`{}`", scenario.name), String::from(scenario.reason)])
		.collect();
	readme = replace(&readme, SCENARIOS, &table(vec![String::from("scenario"), String::from("why")], list))?;

	fs::write(README, readme).map_err(|error| format!("{README} could not be written ({error})"))
}

/// Reads the v4 results back, every cell of the results tables
pub fn written() -> Result<Written, String> {
	let readme = read()?;
	let mut found = Written::new();

	for test in TESTS {
		let (_, block, _) = split(&readme, &format!("results:{test}"))?;
		let mut rows = block.lines().filter(|line| line.starts_with('|')).map(|line| {
			line.trim_matches('|').split('|').map(|cell| cell.trim().trim_matches('`').to_string()).collect::<Vec<_>>()
		});
		let header = rows.next().unwrap_or_default();
		for row in rows.filter(|row| !row[0].starts_with('-')) {
			for (heading, cell) in header.iter().zip(&row).skip(1) {
				let measure = MEASURES.iter().find(|measure| *heading == format!("v4 {}", measure.column));
				if let Some(measure) = measure {
					found.insert((row[0].clone(), measure.column), cell.clone());
				}
			}
		}
	}

	if found.is_empty() {
		return Err(format!("{README} holds no results yet, run `make perf-save` first, it writes them"));
	}
	Ok(found)
}

/// A value as the tables write it, in nanoseconds, bytes or a count, `None` for a reason in its place
pub fn parse(cell: &str) -> Option<u64> {
	let (number, unit) = cell.split_once(' ').unwrap_or((cell, ""));
	let scale = match unit {
		"" | "B" | "ns" => 1.0,
		"µs" => 1e3,
		"ms" => 1e6,
		"s" => 1e9,
		_ => return None,
	};
	let number: f64 = number.replace(',', "").parse().ok()?;

	Some((number * scale).round() as u64)
}

/// A measurement's cell, or the reason it is missing
pub fn cell(value: &Result<u64, String>, format: fn(u64) -> String) -> String {
	value.as_ref().map_or_else(Clone::clone, |value| format(*value))
}

/// A count with thousands separators
pub fn count(value: u64) -> String {
	HumanCount(value).to_string()
}

/// Bytes, exact, with thousands separators
pub fn bytes(value: u64) -> String {
	format!("{} B", HumanCount(value))
}

/// A time in nanoseconds, to three significant digits in the unit that keeps it short
pub fn duration(nanoseconds: u64) -> String {
	const UNITS: [(&str, f64); 3] = [("s", 1e9), ("ms", 1e6), ("µs", 1e3)];

	let time = nanoseconds as f64;
	match UNITS.iter().find(|(_, size)| time >= *size) {
		Some((unit, size)) => format!("{} {unit}", significant(time / size)),
		None => format!("{nanoseconds} ns"),
	}
}

/// A positive number to three significant digits
pub fn significant(value: f64) -> String {
	let decimals = (2 - value.log10().floor() as i32).max(0) as usize;
	format!("{value:.decimals$}")
}

/// The table of one test: the scenario, then per measure a column per measured version and its ratio,
/// v4 over v3, when both versions run
fn results(test: &str, versions: &[Version], names: &[String], values: &Values) -> String {
	let compare = versions.contains(&Version::V3);
	let measures: Vec<_> = MEASURES.iter().filter(|measure| measure.test == test).collect();

	let mut header = vec![String::from("scenario")];
	for measure in &measures {
		header.extend(versions.iter().map(|version| format!("{version} {}", measure.column)));
		header.extend(measure.ratio.filter(|_| compare).map(String::from));
	}

	let rows = names.iter().map(|name| {
		let mut row = vec![format!("`{name}`")];
		for measure in &measures {
			let value = |version| &values[&(name.clone(), measure.column, version)];
			row.extend(versions.iter().map(|&version| cell(value(version), measure.format)));
			if compare && measure.ratio.is_some() {
				row.push(ratio(value(Version::V3), value(Version::V4)));
			}
		}
		row
	});

	table(header, rows.collect())
}

/// v4 over v3 to three significant digits, `0.25x` means v4 needs a quarter of what v3 needs,
/// `n/a` when either is missing
fn ratio(v3: &Result<u64, String>, v4: &Result<u64, String>) -> String {
	match (v3, v4) {
		(Ok(v3), Ok(0)) if *v3 > 0 => String::from("0x"),
		(Ok(v3), Ok(v4)) if *v3 > 0 => format!("{}x", significant(*v4 as f64 / *v3 as f64)),
		_ => String::from("n/a"),
	}
}

/// One markdown table, every column as wide as its widest cell
fn table(header: Vec<String>, rows: Vec<Vec<String>>) -> String {
	let mut builder = Builder::from(rows);
	builder.insert_record(0, header);
	builder.build().with(Style::markdown()).to_string()
}

/// The whole README
fn read() -> Result<String, String> {
	fs::read_to_string(README).map_err(|error| format!("{README} could not be read ({error})"))
}

/// Replaces what one block holds with a line naming when and where it ran and the content
fn replace(readme: &str, block: &str, content: &str) -> Result<String, String> {
	let (head, _, tail) = split(readme, block)?;
	let (start, end) = markers(block);

	Ok(format!("{head}{start}\n{}\n\n{content}\n\n{end}{tail}", stamp()))
}

/// The start and the end marker of one block
fn markers(block: &str) -> (String, String) {
	(format!("<!-- perf:{block}:start -->"), format!("<!-- perf:{block}:end -->"))
}

/// The text before a block's start marker, between its markers and after its end marker, or which marker to add
fn split<'a>(readme: &'a str, block: &str) -> Result<(&'a str, &'a str, &'a str), String> {
	let (start, end) = markers(block);
	let missing = |marker: &str| {
		format!("{README} has no {marker}, add {start} and {end} on lines of their own where the {block} block goes")
	};

	let (head, rest) = readme.split_once(&start).ok_or_else(|| missing(&start))?;
	let (content, tail) = rest.split_once(&end).ok_or_else(|| missing(&end))?;

	Ok((head, content, tail))
}

/// When and where a block ran: the date, the operating system and architecture, the CPU and the rustc that built v4
fn stamp() -> String {
	let system = System::new_with_specifics(RefreshKind::nothing().with_cpu(CpuRefreshKind::nothing()));
	let cpu = system.cpus().first().map_or("unknown CPU", |cpu| cpu.brand());

	format!("Ran {} on {OS} {ARCH}, {cpu}, {}", first_line("date", &["+%Y-%m-%d"]), first_line("rustc", &["--version"]))
}

/// The first line a command prints, `unknown` and the command when it prints nothing
fn first_line(program: &str, args: &[&str]) -> String {
	let output = Command::new(program).args(args).output().map(|output| output.stdout).unwrap_or_default();
	let text = String::from_utf8_lossy(&output);

	text.lines().next().map_or_else(|| format!("unknown {program}"), str::to_string)
}
