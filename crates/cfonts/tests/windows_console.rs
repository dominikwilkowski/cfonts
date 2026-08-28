//! End to end test for width measurement on a real Windows console
//!
//! The suite runs piped, so this file allocates its own console with a known
//! window size and measures it; a single test in its own process, because a
//! console is process-global state

#![cfg(windows)]

use std::num::NonZeroUsize;

use cfonts::{CanvasWidth, hosts::terminal_canvas_width::TerminalCanvasWidth};

#[test]
fn a_console_reports_its_configured_width() {
	use windows_sys::Win32::System::Console::{
		AllocConsole, GetStdHandle, SMALL_RECT, STD_ERROR_HANDLE, STD_OUTPUT_HANDLE, SetConsoleWindowInfo, SetStdHandle,
	};

	// SAFETY: every call touches only this process' console state, and this
	// file holds one test, so no parallel test shares that state
	unsafe {
		let piped_stderr = GetStdHandle(STD_ERROR_HANDLE);

		assert_ne!(AllocConsole(), 0, "the test process must start without a console");

		// a sixty-six column window; the buffer opens larger and stays so
		let window = SMALL_RECT { Left: 0, Top: 0, Right: 65, Bottom: 23 };
		assert_ne!(SetConsoleWindowInfo(GetStdHandle(STD_OUTPUT_HANDLE), 1, &window), 0);

		// panics report through the harness pipe, while stdout keeps the console
		SetStdHandle(STD_ERROR_HANDLE, piped_stderr);

		// the environment must not decide before the measurement can
		std::env::remove_var("FORCE_SIZE");
	}

	assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Auto), NonZeroUsize::new(66));
}
