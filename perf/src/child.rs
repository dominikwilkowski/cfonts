//! Child processes: the conditions every child runs under, the runner running itself for one task,
//! the time limit and the reasons a child did not exit cleanly

use std::{
	env,
	fmt::{self, Display},
	io::Read,
	process::{Command, ExitCode, ExitStatus, Stdio},
	thread,
	time::{Duration, Instant},
};

use cfonts_perf::{Scenario, Version, render_text};
use wait_timeout::ChildExt;

use crate::measure::{api_heap, api_speed};

/// The flag the runner runs itself with for one task: `cfonts-perf --perf-child <task> <version> <scenario> [budget]`
pub const CHILD_FLAG: &str = "--perf-child";

/// The task of the fairness check: render one scenario once and print the text
pub const RENDER_TASK: &str = "render";

/// The task of the speed test: time one render within the budget in nanoseconds and print the median in nanoseconds
pub const SPEED_TASK: &str = "speed";

/// The task of the allocation and memory tests: print the blocks, bytes and peak heap of one render
pub const HEAP_TASK: &str = "heap";

/// How long one render may take before its child is killed, the fairness check's limit
pub const TIME_LIMIT: Duration = Duration::from_secs(30);

/// Sets the conditions every child runs under: no inherited variable, true color forced, stdin on /dev/null,
/// output piped
pub fn under_fixed_conditions(command: &mut Command) -> &mut Command {
	// the shell's PATH, HOME and TMPDIR move the bytes a process allocates at startup, so the counts follow the code alone
	command.env_clear().env("FORCE_COLOR", "3").stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped())
}

/// The command that runs one task in a fresh child of the runner, under the fixed conditions
pub fn task(name: &str, version: Version, scenario: &Scenario) -> Command {
	let mut command = Command::new(env::current_exe().expect("a running binary knows its own path"));
	under_fixed_conditions(command.args([CHILD_FLAG, name, &version.to_string(), &scenario.name]));
	command
}

/// Answers one task in a child, a panic here is the crash the parent records
pub fn answer(task: &str, version: &str, scenario: &str, budget: Option<&str>) -> ExitCode {
	let version = Version::from_name(version).unwrap_or_else(|| panic!("{version} is no version, pass v3 or v4"));
	let scenario =
		Scenario::find(scenario).unwrap_or_else(|| panic!("{scenario} is no scenario, perf/src/lib.rs lists them"));

	match (task, budget.map(str::parse)) {
		(RENDER_TASK, _) => print!("{}", render_text(version, &scenario)),
		(SPEED_TASK, Some(Ok(budget))) => println!("{}", api_speed(version, &scenario, Duration::from_nanos(budget))),
		(HEAP_TASK, _) => println!("{}", api_heap(version, &scenario).map(|value| value.to_string()).join(" ")),
		_ => panic!("{task} is no task, pass {RENDER_TASK}, {SPEED_TASK} with a budget in nanoseconds or {HEAP_TASK}"),
	}

	ExitCode::SUCCESS
}

/// What a child printed and how long it ran
pub struct Finished {
	/// Everything it wrote to stdout
	pub stdout: String,

	/// From spawn to exit
	pub time: Duration,
}

/// Why a child did not exit cleanly
#[derive(Clone, Debug)]
pub struct Crash {
	/// What the tables show: the panic location, the signal, the exit status or the timeout
	pub reason: String,

	/// The panic message, or else the last line the child wrote to stderr, empty when it wrote none
	pub detail: String,
}

impl Crash {
	/// Reads the reason out of a finished child's status and stderr
	pub fn from_status(status: ExitStatus, stderr: &str) -> Self {
		if let Some((location, message)) = panic_report(stderr) {
			return Self { reason: format!("panic at {location}"), detail: message };
		}

		let detail = stderr.lines().rev().map(str::trim).find(|line| !line.is_empty()).unwrap_or_default();
		Self { reason: status.to_string(), detail: detail.to_string() }
	}
}

impl Display for Crash {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.detail.is_empty() {
			formatter.write_str(&self.reason)
		} else {
			write!(formatter, "{}, {}", self.reason, self.detail)
		}
	}
}

/// Runs a child to its end under a time limit, what it printed and how long it ran, or why it did not exit cleanly
pub fn run_limited(mut command: Command, limit: Duration) -> Result<Finished, Crash> {
	let started = Instant::now();
	let mut child =
		command.spawn().map_err(|error| Crash { reason: String::from("did not start"), detail: error.to_string() })?;
	let stdout = child.stdout.take().expect("stdout is piped");
	let stderr = child.stderr.take().expect("stderr is piped");

	// both pipes are read while the child runs, a full pipe would stall it
	let (status, stdout, stderr) = thread::scope(|scope| {
		let stdout = scope.spawn(|| read_all(stdout));
		let stderr = scope.spawn(|| read_all(stderr));
		let status = child.wait_timeout(limit).ok().flatten();
		if status.is_none() {
			child.kill().ok();
			child.wait().ok();
		}
		(status, stdout.join().unwrap_or_default(), stderr.join().unwrap_or_default())
	});

	match status {
		Some(status) if status.success() => Ok(Finished { stdout, time: started.elapsed() }),
		Some(status) => Err(Crash::from_status(status, &stderr)),
		None => Err(Crash { reason: format!("timeout after {} s", limit.as_secs()), detail: String::new() }),
	}
}

/// Everything one pipe delivers until it closes
fn read_all(mut pipe: impl Read) -> String {
	let mut bytes = Vec::new();
	pipe.read_to_end(&mut bytes).ok();
	String::from_utf8_lossy(&bytes).into_owned()
}

/// The file and line of a Rust panic with its message, `gradient.rs:138` out of `panicked at src/gradient.rs:138:5:`
fn panic_report(stderr: &str) -> Option<(String, String)> {
	let mut lines = stderr.lines().skip_while(|line| !line.contains("panicked at "));
	let place = lines.next()?.split("panicked at ").nth(1)?.trim_end_matches(':');
	let mut parts = place.rsplitn(3, ':');
	let (_column, line, path) = (parts.next()?, parts.next()?, parts.next()?);
	let file = path.rsplit('/').next()?;
	let message = lines.next().unwrap_or_default().trim().to_string();

	Some((format!("{file}:{line}"), message))
}
