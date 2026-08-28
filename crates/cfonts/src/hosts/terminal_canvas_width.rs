//! Terminal canvas width: one home for the whole decision
//!
//! FORCE_SIZE, then the API override, then the measured terminal width, then
//! the eighty-column fallback — shared by every host: the native host measures
//! real streams, the npm host ships its facts across the wasm boundary
//!
//! COLUMNS is deliberately not read: FORCE_SIZE is the one environment knob

use std::num::NonZeroUsize;

use crate::CanvasWidth;

/// The width a render wraps to when nothing can be measured
const FALLBACK_WIDTH: usize = 80;

/// Everything one canvas-width resolution reads, gathered before any decision
///
/// [`detect`](Self::detect) gathers the real facts of the process in one shot;
/// tests and boundary hosts fill the struct literally and call
/// [`resolve`](Self::resolve)
pub struct TerminalCanvasWidth<'a> {
	/// The measured width of the terminal behind stdout or stderr, `None` when
	/// both are redirected
	pub measured: Option<NonZeroUsize>,

	/// The environment FORCE_SIZE is read from
	pub environment: &'a dyn Fn(&str) -> Option<String>,

	/// The API override, applied after FORCE_SIZE
	pub override_width: CanvasWidth,
}

impl TerminalCanvasWidth<'_> {
	/// Resolves the canvas width of the real process in one shot
	///
	/// # Examples
	///
	/// ```
	/// use std::num::NonZeroUsize;
	///
	/// use cfonts::{CanvasWidth, hosts::terminal_canvas_width::TerminalCanvasWidth};
	///
	/// temp_env::with_var("FORCE_SIZE", Some("12"), || {
	///     assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Auto), NonZeroUsize::new(12));
	/// });
	///
	/// temp_env::with_var("FORCE_SIZE", Some("0"), || {
	///     assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Auto), None); // no width limit
	/// });
	/// ```
	#[cfg(not(target_arch = "wasm32"))]
	#[must_use]
	pub fn detect(override_width: CanvasWidth) -> Option<NonZeroUsize> {
		use crate::hosts::terminal_color_support::TerminalColorSupport;

		TerminalCanvasWidth {
			measured: Self::measure(),
			environment: &TerminalColorSupport::process_environment,
			override_width,
		}
		.resolve()
	}

	/// Resolves the gathered facts: FORCE_SIZE, then the API override, then the
	/// measured width, then the eighty-column fallback
	///
	/// `None` means no width limit
	///
	/// # Examples
	///
	/// ```
	/// use std::num::NonZeroUsize;
	///
	/// use cfonts::{CanvasWidth, hosts::terminal_canvas_width::TerminalCanvasWidth};
	///
	/// let environment = |name: &str| (name == "FORCE_SIZE").then(|| String::from("12"));
	///
	/// let resolved = TerminalCanvasWidth {
	///     measured: NonZeroUsize::new(200),
	///     environment: &environment,
	///     override_width: CanvasWidth::Auto,
	/// }
	/// .resolve();
	///
	/// assert_eq!(resolved, NonZeroUsize::new(12));
	/// ```
	#[must_use]
	pub fn resolve(&self) -> Option<NonZeroUsize> {
		// The outer Option distinguishes invalid input from valid FORCE_SIZE=0
		// Digits only and at most u32::MAX: the same rule the npm host applies
		if let Some(forced_width) = (self.environment)("FORCE_SIZE")
			.filter(|value| value.bytes().all(|byte| byte.is_ascii_digit()))
			.and_then(|value| value.parse::<u32>().ok())
			.map(|value| NonZeroUsize::new(value as usize))
		{
			return forced_width;
		}

		match self.override_width {
			CanvasWidth::Auto => self.measured.or_else(|| NonZeroUsize::new(FALLBACK_WIDTH)),
			CanvasWidth::Unlimited => None,
			CanvasWidth::Columns(width) => Some(width),
		}
	}

	/// The measured width of the stream still attached to a terminal: stdout,
	/// else stderr
	///
	/// Captured output is never wrapped to an invisible window, so a fully
	/// redirected process falls back instead of probing stdin
	#[cfg(any(unix, windows))]
	fn measure() -> Option<NonZeroUsize> {
		Self::width_of(&std::io::stdout()).or_else(|| Self::width_of(&std::io::stderr()))
	}

	/// A target without terminals measures nothing
	#[cfg(not(any(unix, windows, target_arch = "wasm32")))]
	fn measure() -> Option<NonZeroUsize> {
		None
	}

	/// The width of the terminal behind one stream
	///
	/// Only the column count matters, so a zero-row terminal still measures —
	/// the same reading the npm host takes from `stream.columns`
	/// A stream without a terminal fails the ioctl and measures nothing
	#[cfg(unix)]
	fn width_of(stream: &impl std::os::fd::AsRawFd) -> Option<NonZeroUsize> {
		let mut size = libc::winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };

		// SAFETY: TIOCGWINSZ writes only the winsize struct behind the valid
		// pointer and fails on a stream without a terminal
		if unsafe { libc::ioctl(stream.as_raw_fd(), libc::TIOCGWINSZ, &raw mut size) } != 0 {
			return None;
		}

		NonZeroUsize::new(size.ws_col as usize)
	}

	/// The width of the console window behind one stream
	#[cfg(windows)]
	fn width_of(stream: &impl std::os::windows::io::AsRawHandle) -> Option<NonZeroUsize> {
		Self::console_width(stream.as_raw_handle().cast())
	}

	/// The width of the console window behind one handle
	///
	/// Only the column count matters; a handle without a console fails the
	/// call and measures nothing
	#[cfg(windows)]
	fn console_width(handle: windows_sys::Win32::Foundation::HANDLE) -> Option<NonZeroUsize> {
		use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
		use windows_sys::Win32::System::Console::{
			CONSOLE_SCREEN_BUFFER_INFO, COORD, GetConsoleScreenBufferInfo, SMALL_RECT,
		};

		if handle.is_null() || handle == INVALID_HANDLE_VALUE {
			return None;
		}

		let corner = COORD { X: 0, Y: 0 };
		let mut info = CONSOLE_SCREEN_BUFFER_INFO {
			dwSize: corner,
			dwCursorPosition: corner,
			wAttributes: 0,
			srWindow: SMALL_RECT { Left: 0, Top: 0, Right: 0, Bottom: 0 },
			dwMaximumWindowSize: corner,
		};

		// SAFETY: the call writes only the buffer info struct behind the valid
		// pointer and fails on a handle without a console
		if unsafe { GetConsoleScreenBufferInfo(handle, &raw mut info) } == 0 {
			return None;
		}

		let width = i32::from(info.srWindow.Right) - i32::from(info.srWindow.Left) + 1;
		usize::try_from(width).ok().and_then(NonZeroUsize::new)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn width(value: usize) -> NonZeroUsize {
		NonZeroUsize::new(value).expect("test widths must be non-zero")
	}

	fn resolve_with(forced: Option<&str>, measured: Option<usize>, override_width: CanvasWidth) -> Option<NonZeroUsize> {
		let environment = |name: &str| if name == "FORCE_SIZE" { forced.map(String::from) } else { None };

		TerminalCanvasWidth { measured: measured.and_then(NonZeroUsize::new), environment: &environment, override_width }
			.resolve()
	}

	#[test]
	fn force_size_overrides_the_measurement() {
		assert_eq!(resolve_with(Some("120"), Some(200), CanvasWidth::Auto), Some(width(120)));
	}

	#[test]
	fn force_size_overrides_explicit_columns() {
		assert_eq!(resolve_with(Some("120"), Some(200), CanvasWidth::Columns(width(42))), Some(width(120)));
	}

	#[test]
	fn force_size_overrides_explicit_unlimited() {
		assert_eq!(resolve_with(Some("120"), None, CanvasWidth::Unlimited), Some(width(120)));
	}

	#[test]
	fn force_size_zero_means_unlimited() {
		assert_eq!(resolve_with(Some("0"), Some(200), CanvasWidth::Columns(width(42))), None);
	}

	#[test]
	fn invalid_force_size_falls_through_to_the_api_override() {
		// "+5" and 2^32 parse as numbers in Rust but not under the npm host's rule
		for forced in ["", "abc", "-1", "12.5", "+5", "4294967296"] {
			assert_eq!(resolve_with(Some(forced), Some(200), CanvasWidth::Columns(width(42))), Some(width(42)), "{forced:?}");
		}
	}

	#[test]
	fn invalid_force_size_falls_through_to_the_measurement() {
		for forced in ["", "abc", "-1", "12.5", "+5", "4294967296"] {
			assert_eq!(resolve_with(Some(forced), Some(200), CanvasWidth::Auto), Some(width(200)), "{forced:?}");
		}
	}

	#[test]
	fn explicit_columns_beat_the_measurement() {
		assert_eq!(resolve_with(None, Some(200), CanvasWidth::Columns(width(42))), Some(width(42)));
	}

	#[test]
	fn explicit_unlimited_beats_the_measurement() {
		assert_eq!(resolve_with(None, Some(200), CanvasWidth::Unlimited), None);
	}

	#[test]
	fn auto_uses_the_measured_width() {
		assert_eq!(resolve_with(None, Some(120), CanvasWidth::Auto), Some(width(120)));
	}

	#[test]
	fn auto_falls_back_to_eighty_columns() {
		assert_eq!(resolve_with(None, None, CanvasWidth::Auto), Some(width(FALLBACK_WIDTH)));
	}

	#[test]
	fn the_columns_variable_is_deliberately_ignored() {
		let environment = |name: &str| (name == "COLUMNS").then(|| String::from("13"));

		let resolved =
			TerminalCanvasWidth { measured: None, environment: &environment, override_width: CanvasWidth::Auto }.resolve();

		assert_eq!(resolved, NonZeroUsize::new(FALLBACK_WIDTH));
	}

	#[test]
	fn detect_reads_the_real_environment() {
		temp_env::with_var("FORCE_SIZE", Some("7"), || {
			assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Auto), Some(width(7)));
		});

		temp_env::with_var("FORCE_SIZE", Some("0"), || {
			assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Columns(width(42))), None);
		});
	}

	#[cfg(unix)]
	mod unix {
		use std::os::fd::{FromRawFd, OwnedFd};

		use super::*;

		/// A real terminal pair with the given geometry
		fn pty(columns: u16, rows: u16) -> (OwnedFd, OwnedFd) {
			let mut controller = 0;
			let mut follower = 0;
			let mut size = libc::winsize { ws_row: rows, ws_col: columns, ws_xpixel: 0, ws_ypixel: 0 };

			// SAFETY: openpty writes the two file descriptors and reads the size
			let result = unsafe {
				libc::openpty(&raw mut controller, &raw mut follower, std::ptr::null_mut(), std::ptr::null_mut(), &raw mut size)
			};
			assert_eq!(result, 0, "openpty must succeed");

			// SAFETY: both descriptors were just opened and are owned here
			unsafe { (OwnedFd::from_raw_fd(controller), OwnedFd::from_raw_fd(follower)) }
		}

		#[test]
		fn a_pty_reports_its_configured_width() {
			let (controller, follower) = pty(137, 43);

			assert_eq!(TerminalCanvasWidth::width_of(&controller), NonZeroUsize::new(137));
			assert_eq!(TerminalCanvasWidth::width_of(&follower), NonZeroUsize::new(137));
		}

		#[test]
		fn a_zero_width_pty_measures_nothing() {
			let (controller, _follower) = pty(0, 43);

			assert_eq!(TerminalCanvasWidth::width_of(&controller), None);
		}

		#[test]
		fn a_zero_row_pty_still_measures() {
			let (controller, _follower) = pty(120, 0);

			assert_eq!(TerminalCanvasWidth::width_of(&controller), NonZeroUsize::new(120));
		}

		#[test]
		fn a_pipe_measures_nothing() {
			let (reader, writer) = std::io::pipe().expect("a pipe always opens");

			assert_eq!(TerminalCanvasWidth::width_of(&reader), None);
			assert_eq!(TerminalCanvasWidth::width_of(&writer), None);
		}

		#[test]
		fn a_resize_reaches_the_next_measurement() {
			let (controller, _follower) = pty(80, 24);
			let size = libc::winsize { ws_row: 24, ws_col: 66, ws_xpixel: 0, ws_ypixel: 0 };

			// SAFETY: TIOCSWINSZ reads only the winsize struct behind the valid pointer
			let result =
				unsafe { libc::ioctl(std::os::fd::AsRawFd::as_raw_fd(&controller), libc::TIOCSWINSZ, &raw const size) };
			assert_eq!(result, 0);

			assert_eq!(TerminalCanvasWidth::width_of(&controller), NonZeroUsize::new(66));
		}

		/// The oracle comparison ported from the terminal_size crate
		#[test]
		fn the_measurement_matches_stty() {
			use std::process::{Command, Stdio};

			// cargo test pipes stdout, so stderr is the stream a local run still
			// attaches; without a terminal there is nothing to compare against
			if TerminalCanvasWidth::width_of(&std::io::stderr()).is_none() {
				return;
			}

			let output = if cfg!(target_os = "linux") {
				Command::new("stty").arg("size").arg("-F").arg("/dev/stderr").stderr(Stdio::inherit()).output()
			} else {
				Command::new("stty").arg("-f").arg("/dev/stderr").arg("size").stderr(Stdio::inherit()).output()
			}
			.expect("stty must run");
			assert!(output.status.success());

			// stty answers "rows columns"
			let answer = String::from_utf8(output.stdout).expect("stty answers text");
			let columns = answer.split_whitespace().nth(1).expect("stty answers rows then columns");

			assert_eq!(
				TerminalCanvasWidth::width_of(&std::io::stderr()),
				NonZeroUsize::new(columns.parse().expect("stty answers numbers"))
			);
		}
	}

	#[cfg(windows)]
	mod windows {
		use super::*;

		#[test]
		fn an_invalid_handle_measures_nothing() {
			use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;

			assert_eq!(TerminalCanvasWidth::console_width(INVALID_HANDLE_VALUE), None);
			assert_eq!(TerminalCanvasWidth::console_width(std::ptr::null_mut()), None);
		}
	}
}
