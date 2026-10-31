//! The process boundary: the three standard streams and the exit status of a write that failed

use std::{
	fmt::Display,
	io::{self, IsTerminal, Read, Write, stdin},
	process::ExitCode,
};

use crate::cli::StdinProvider;

/// The process boundary of the binary, the one place that reads stdin, writes the two output streams
/// and knows the exit statuses the shell reads
///
/// The streams belong to the process, so nothing here carries state, the struct names the concern,
/// a parse that failed decides its own exit status through [`ParseError::exit_code`](crate::cli::ParseError::exit_code)
pub struct Process;

impl Process {
	/// The exit status of a command line that could not be used, the `sysexits` `EX_USAGE`
	pub(crate) fn usage_error() -> ExitCode {
		ExitCode::from(64)
	}

	/// The exit status of a stream that could not be read or written, the `sysexits` `EX_IOERR`
	pub(crate) fn io_error() -> ExitCode {
		ExitCode::from(74)
	}

	/// The parser's way to stdin, whether it is a terminal and how to read it
	#[must_use]
	pub fn stdin() -> StdinProvider {
		StdinProvider { interactive: stdin().is_terminal(), read: Self::read_stdin }
	}

	/// Reads all of stdin, telling a person at a terminal how to end the input
	fn read_stdin() -> io::Result<String> {
		if stdin().is_terminal() {
			let eof_key = if cfg!(windows) { "Ctrl-Z then Enter" } else { "Ctrl-D" };
			Self::report(format_args!("Start typing, end with {eof_key} on an empty line…"));
		}
		let mut buffer = String::new();
		stdin().read_to_string(&mut buffer)?;
		Ok(buffer)
	}

	/// Prints one line to stderr, best effort:
	/// a broken error stream cannot be reported to itself and never changes the outcome
	pub fn report(message: impl Display) {
		let _ = writeln!(io::stderr(), "{message}");
	}

	/// Prints one screen to stdout, the fallible way
	pub fn print(text: impl Display) -> io::Result<()> {
		writeln!(io::stdout(), "{text}")
	}

	/// Judges the stdout outcome at the process boundary:
	/// a closed pipe is a reader that has seen enough, any other write failure is an io error
	///
	/// ```
	/// use std::{io, process::ExitCode};
	/// use cfonts::cli::Process;
	///
	/// assert_eq!(Process::exit_after_writing(Ok(())), ExitCode::SUCCESS);
	/// assert_eq!(Process::exit_after_writing(Err(io::ErrorKind::BrokenPipe.into())), ExitCode::SUCCESS);
	/// ```
	#[must_use]
	pub fn exit_after_writing(written: io::Result<()>) -> ExitCode {
		match written {
			Ok(()) => ExitCode::SUCCESS,
			Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
			Err(error) => {
				Self::report(format_args!(" ERROR  Writing the output failed ({error})"));
				Self::io_error()
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_exit_statuses_are_the_sysexits_codes() {
		assert_eq!(Process::usage_error(), ExitCode::from(64));
		assert_eq!(Process::io_error(), ExitCode::from(74));
	}

	#[test]
	fn a_written_screen_exits_successfully() {
		assert_eq!(Process::exit_after_writing(Ok(())), ExitCode::SUCCESS);
	}

	#[test]
	fn a_closed_pipe_is_a_reader_that_has_seen_enough() {
		// the reader closed its end, nothing went wrong on ours, the OS detail is irrelevant
		let closed = io::Error::new(io::ErrorKind::BrokenPipe, "the reader went away");

		assert_eq!(Process::exit_after_writing(Err(closed)), ExitCode::SUCCESS);
	}

	#[test]
	fn the_stdin_provider_answers_for_the_real_stream() {
		// the provider carries whether stdin is a terminal, the one fact the parser asks before reading
		assert_eq!(Process::stdin().interactive, stdin().is_terminal());
	}
}
