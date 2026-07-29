#[cfg(feature = "dioxus")]
mod dioxus;
#[cfg(feature = "dioxus")]
pub use dioxus::CfontsDioxus;

#[cfg(feature = "leptos")]
mod leptos;
#[cfg(feature = "leptos")]
pub use leptos::{CfontsLeptos, LeptosHost};

#[cfg(feature = "ratatui")]
mod ratatui;
#[cfg(feature = "ratatui")]
pub use ratatui::CfontsWidget;

#[cfg(any(feature = "leptos", feature = "dioxus"))]
use crate::{ColorLevel, RenderContext};

#[cfg(feature = "dioxus")]
use crate::{BrowserEnv, Options, Rendered, render_with};

/// The one home of the component render policy: colors paint at full support
/// because the artifact is CSS, and the canvas stays unlimited because browser
/// capability discovery belongs to the application host
/// The seed makes candy picks reproducible across renders
#[cfg(any(feature = "leptos", feature = "dioxus"))]
pub(crate) fn render_context(seed: u64) -> RenderContext {
	RenderContext::colored(ColorLevel::TrueColor).with_seed(seed)
}

/// Adapts cfonts options to the HTML artifact the Dioxus component consumes
///
/// Dioxus is multi-renderer and re-exports no browser bindings, so it gets the
/// render adapter without a say action
#[cfg(feature = "dioxus")]
pub(crate) fn render_browser(options: &Options, seed: u64) -> Rendered {
	render_with(options, &BrowserEnv, render_context(seed))
}

#[cfg(all(test, any(feature = "leptos", feature = "dioxus")))]
mod tests {
	use super::*;

	#[test]
	fn the_component_context_paints_at_full_support_without_a_canvas() {
		let context = render_context(42);

		assert_eq!(context.color_level(), Some(ColorLevel::TrueColor));
		assert_eq!(context.canvas_width(), None);
		assert_eq!(context.seed(), 42);
	}
}

#[cfg(all(test, feature = "dioxus"))]
mod dioxus_tests {
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
