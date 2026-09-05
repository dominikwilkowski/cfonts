//! End to end test for width measurement on a real Windows console
//!
//! The suite runs piped, so this file allocates its own console and compares
//! the probe against the width that console's window really has; a single
//! test in its own process, because a console is process-global state

#![cfg(windows)]

use std::num::NonZeroUsize;

use cfonts::{CanvasWidth, hosts::terminal_canvas_width::TerminalCanvasWidth};

#[test]
fn a_console_reports_its_window_width() {
	use windows_sys::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE};
	use windows_sys::Win32::Storage::FileSystem::{CreateFileW, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING};
	use windows_sys::Win32::System::Console::{
		AllocConsole, CONSOLE_SCREEN_BUFFER_INFO, COORD, FreeConsole, GetConsoleScreenBufferInfo, GetStdHandle, SMALL_RECT,
		STD_ERROR_HANDLE, STD_OUTPUT_HANDLE, SetConsoleWindowInfo, SetStdHandle,
	};

	// SAFETY: every call touches only this process' console state, and this
	// file holds one test, so no parallel test shares that state
	let expected = unsafe {
		let piped_stderr = GetStdHandle(STD_ERROR_HANDLE);

		// the harness hands the test an inherited console; only a fresh one
		// has a window this process may configure, so detaching may not fail
		// but allocating must succeed
		FreeConsole();
		assert_ne!(AllocConsole(), 0, "a console must open once the inherited one is gone");

		// the console device names the active screen buffer however the
		// standard handles are wired
		let name: Vec<u16> = "CONOUT$\0".encode_utf16().collect();
		let console = CreateFileW(
			name.as_ptr(),
			GENERIC_READ | GENERIC_WRITE,
			FILE_SHARE_READ | FILE_SHARE_WRITE,
			std::ptr::null(),
			OPEN_EXISTING,
			0,
			std::ptr::null_mut(),
		);
		assert_ne!(console, INVALID_HANDLE_VALUE, "the fresh console must open");

		// the probe reads the process stdout handle, the way a terminal wires
		// it; panics keep reporting through the harness pipe on stderr
		SetStdHandle(STD_OUTPUT_HANDLE, console);
		SetStdHandle(STD_ERROR_HANDLE, piped_stderr);

		// shrinking the window is best effort: a host may refuse, and the
		// oracle below asks for the width the window really has
		let window = SMALL_RECT { Left: 0, Top: 0, Right: 65, Bottom: 23 };
		SetConsoleWindowInfo(console, 1, &window);

		let corner = COORD { X: 0, Y: 0 };
		let mut info = CONSOLE_SCREEN_BUFFER_INFO {
			dwSize: corner,
			dwCursorPosition: corner,
			wAttributes: 0,
			srWindow: SMALL_RECT { Left: 0, Top: 0, Right: 0, Bottom: 0 },
			dwMaximumWindowSize: corner,
		};
		assert_ne!(GetConsoleScreenBufferInfo(console, &raw mut info), 0, "the fresh console must answer");

		// the environment must not decide before the measurement can
		std::env::remove_var("FORCE_SIZE");

		usize::try_from(i32::from(info.srWindow.Right) - i32::from(info.srWindow.Left) + 1).expect("a window has width")
	};

	assert!(expected > 0, "the console window must have columns");
	assert_eq!(TerminalCanvasWidth::detect(CanvasWidth::Auto), NonZeroUsize::new(expected));
}
