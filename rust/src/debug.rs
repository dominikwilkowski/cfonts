//! The contents of this module is all about debugging
//!
//! We use the `debug_level` of [`Options`] to toggle visibility of debug messages
//!
//! The debug message _may_ later go into a file you can send to in for analysis.
use crate::color::{color, get_background_color};
use crate::config::{BgColors, Colors, Options};

/// The `Dt` enum includes all debug types
pub enum Dt {
	/// A headline
	Head,
	/// A log
	Log,
	/// An error
	Error,
}

/// The `d` function is for all debug calls
///
/// Calling `d` with a `level` will allow the system to hide of show debug messages based on the [`Options`].debug_level
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{ Options };
/// use cfonts::debug::{d, Dt};
///
/// let options = Options::default();
/// d("My debug message for this header", 1, Dt::Head, &options, &mut std::io::stdout());
///
/// // only shows up on debug_level > 1
/// d("My debug message", 2, Dt::Log, &options, &mut std::io::stdout());
///
/// // only shows up on debug_level > 3
/// d("My error debug message", 4, Dt::Error, &options, &mut std::io::stdout());
/// ```
pub fn d(text: &str, level: u16, debug_type: Dt, options: &Options, stdout: &mut dyn std::io::Write) {
	if !options.debug || level > options.debug_level {
		// we discard everything if debug is disabled or if the set level is not deep enough
	} else {
		match debug_type {
			Dt::Head => {
				let (bg_start, bg_end) = get_background_color(&BgColors::Yellow);
				writeln!(stdout, "{}\n  {} {}", bg_start, color(text, Colors::Black), bg_end).unwrap_or(());
			}
			Dt::Log => {
				writeln!(stdout, "  {}", color(text, Colors::Yellow)).unwrap_or(());
			}
			Dt::Error => {
				let (bg_start, bg_end) = get_background_color(&BgColors::Red);
				writeln!(stdout, "{}{}{} {}", bg_start, color(" ERROR ", Colors::White), bg_end, color(text, Colors::Red))
					.unwrap_or(());
			}
		}
	}
}
