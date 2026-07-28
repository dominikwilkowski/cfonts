#[cfg(feature = "dioxus")]
mod dioxus;
#[cfg(feature = "dioxus")]
pub use dioxus::CfontsDioxus;

#[cfg(feature = "leptos")]
mod leptos;
#[cfg(feature = "leptos")]
pub use leptos::{CfontsLeptos, console_say};

#[cfg(feature = "ratatui")]
mod ratatui;
#[cfg(feature = "ratatui")]
pub use ratatui::CfontsWidget;

#[cfg(any(feature = "leptos", feature = "dioxus"))]
use crate::{BrowserEnv, ColorLevel, Options, RenderContext, Rendered, render_with};

/// Adapts cfonts options to the HTML artifact the framework components consume
///
/// The one home of the component render policy: colors paint at full support
/// because the artifact is CSS, and the canvas stays unlimited because browser
/// capability discovery belongs to the application host
/// The seed makes candy picks reproducible across renders
#[cfg(any(feature = "leptos", feature = "dioxus"))]
pub(crate) fn render_browser(options: &Options, seed: u64) -> Rendered {
	render_with(options, &BrowserEnv, RenderContext::colored(ColorLevel::TrueColor).with_seed(seed))
}

#[cfg(all(test, any(feature = "leptos", feature = "dioxus")))]
mod tests {
	use super::*;
	use crate::{Cfonts, Color, Font, Valign};

	#[test]
	fn the_adapter_renders_through_the_browser_environment() {
		let options: Options = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().into();

		let rendered = render_browser(&options, 0);

		assert_eq!(
			rendered.text,
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">▄▀█<br>█▀█</div>"#,
		);
	}

	#[test]
	fn the_adapter_paints_colors_by_default() {
		let options: Options =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().color(vec![Color::Red]).into();

		let rendered = render_browser(&options, 0);

		assert!(rendered.text.contains(r##"<span style="color:#ea3223">"##));
	}
}
