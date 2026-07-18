use crate::environments::Environment;

/// The browser-console artifact formatter
///
/// TODO: add `%c` format markers and corresponding style values
#[derive(Debug, Clone, Copy, Default)]
pub struct BrowserConsoleEnv;

impl Environment for BrowserConsoleEnv {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		Cfonts, RenderContext,
		fonts::Font,
		options::{Align, Valign},
	};

	#[test]
	fn render_aligns_inside_a_user_defined_canvas() {
		// the console env has no alignment syntax of its own, so the default physical padding applies
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.align(Align::Right)
			.valign(Valign::Top)
			.spaceless()
			.render_with(&BrowserConsoleEnv, RenderContext::with_canvas_width(10));

		assert_eq!(rendered.text, "       ▄▀█\n       █▀█");
	}

	#[test]
	fn render_produces_the_plain_banner() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.render_with(&BrowserConsoleEnv, RenderContext::unlimited());

		assert_eq!(rendered.text, "▄▀█\n█▀█");
	}
}
