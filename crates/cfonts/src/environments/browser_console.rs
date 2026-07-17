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
	use crate::{Cfonts, RenderContext, fonts::Font, options::Valign};

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
