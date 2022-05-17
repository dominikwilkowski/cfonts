use crate::color::{color, get_background_color};
use crate::config::{BgColors, Colors, Options};

pub enum Dt {
	Head,
	Log,
	Error,
}

pub fn d(text: &str, level: u16, debug_type: Dt, options: &Options, stdout: &mut dyn std::io::Write) {
	if !options.debug || level > options.debug_level {
		// we discard everything if debug is disabled or if the set level is not deep enough
	} else {
		match debug_type {
			Dt::Head => {
				let (bg_start, bg_end) = get_background_color(&BgColors::Yellow);
				writeln!(stdout, "{}\n  {} {}", bg_start, color(text, Colors::Black, options), bg_end).unwrap_or(());
			}
			Dt::Log => {
				writeln!(stdout, "  {}", color(text, Colors::Yellow, options)).unwrap_or(());
			}
			Dt::Error => {
				let (bg_start, bg_end) = get_background_color(&BgColors::Red);
				writeln!(
					stdout,
					"{}{}{} {}",
					bg_start,
					color(" ERROR ", Colors::White, options),
					bg_end,
					color(text, Colors::Red, options)
				)
				.unwrap_or(());
			}
		}
	}
}
