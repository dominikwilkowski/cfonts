use std::num::NonZeroUsize;

use crate::environments::Environment;
#[cfg(target_arch = "wasm32")]
use crate::environments::Rendered;

/// The browser console environment renders the banner for a browser's `console.log`
/// TODO: add colors (`%c` segments with style arguments; user `%` must become `%%` from then on)
#[derive(Debug, Default)]
pub struct BrowserConsoleEnv {
	/// The canvas width; zero means unlimited, mirroring FORCE_SIZE
	/// (there is no console width detection, so None means unlimited too)
	pub canvas_width: Option<usize>,
}

impl Environment for BrowserConsoleEnv {
	fn get_canvas_width(&self) -> Option<usize> {
		self.canvas_width.and_then(NonZeroUsize::new).map(NonZeroUsize::get)
	}

	/// Calls the host's `console.log` with the banner
	#[cfg(target_arch = "wasm32")]
	fn say(&self, rendered: &Rendered) {
		web_sys::console::log_1(&rendered.text.as_str().into());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Cfonts, fonts::Font, options::Valign};

	// get_canvas_width

	#[test]
	fn the_canvas_width_defaults_to_unlimited() {
		assert_eq!(BrowserConsoleEnv::default().get_canvas_width(), None);
	}

	#[test]
	fn the_user_defines_the_canvas_width() {
		assert_eq!(BrowserConsoleEnv { canvas_width: Some(42) }.get_canvas_width(), Some(42));
	}

	#[test]
	fn a_zero_canvas_width_means_unlimited() {
		// mirrors FORCE_SIZE and max-length: zero disables the limit
		assert_eq!(BrowserConsoleEnv { canvas_width: Some(0) }.get_canvas_width(), None);
	}

	// render

	#[test]
	fn render_produces_the_plain_banner() {
		temp_env::with_var("FORCE_SIZE", None::<&str>, || {
			let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).render(&BrowserConsoleEnv::default());

			assert_eq!(rendered.text, "▄▀█\n█▀█");
		});
	}
}
