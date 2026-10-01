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

#[cfg(any(feature = "leptos", feature = "dioxus", feature = "ratatui"))]
use crate::{CanvasWidth, ColorLevel, ColorOverride, RenderContext, RenderOverrides, hosts::entropy};

#[cfg(feature = "dioxus")]
use crate::{BrowserEnv, Options, Rendered, render_with};

/// The one home of the component render policy: how overrides resolve where no terminal answers
///
/// A page has no terminal to measure, so an automatic width and an unlimited one both leave the
/// canvas unlimited, only a column count wraps
/// The artifact is CSS or a terminal buffer the application already owns, so automatic color
/// paints at full support, a level paints that level and disabled paints nothing
/// The seed is the override or a fresh roll, so candy re-rolls on every render like the hosts do
#[cfg(any(feature = "leptos", feature = "dioxus", feature = "ratatui"))]
pub(crate) fn render_context(overrides: RenderOverrides) -> RenderContext {
	let canvas_width = match overrides.canvas_width() {
		CanvasWidth::Auto | CanvasWidth::Unlimited => 0,
		CanvasWidth::Columns(columns) => columns.get(),
	};
	let color_level = match overrides.color() {
		ColorOverride::Auto => Some(ColorLevel::TrueColor),
		ColorOverride::Disabled => None,
		ColorOverride::Level(level) => Some(level),
	};

	RenderContext::with_canvas_width(canvas_width)
		.with_color_level(color_level)
		.with_seed(overrides.seed().unwrap_or_else(entropy))
}

/// Adapts cfonts options to the HTML artifact the Dioxus component consumes
///
/// Dioxus is multi-renderer and re-exports no browser bindings, so it gets the
/// render adapter without a say action
#[cfg(feature = "dioxus")]
pub(crate) fn render_browser(options: &Options, overrides: RenderOverrides) -> Rendered {
	render_with(options, &BrowserEnv, render_context(overrides))
}

#[cfg(all(test, any(feature = "leptos", feature = "dioxus", feature = "ratatui")))]
mod tests {
	use super::*;

	#[test]
	fn the_default_overrides_paint_at_full_support_without_a_canvas() {
		let context = render_context(RenderOverrides::default().with_seed(42));

		assert_eq!(context.color_level(), Some(ColorLevel::TrueColor));
		assert_eq!(context.canvas_width(), None);
		assert_eq!(context.seed(), 42);
	}

	#[test]
	fn only_a_column_count_limits_the_canvas() {
		assert_eq!(render_context(RenderOverrides::default().with_canvas_width(0)).canvas_width(), None);
		assert_eq!(render_context(RenderOverrides::default().with_canvas_width(7)).canvas_width(), Some(7));
	}

	#[test]
	fn the_color_override_resolves_to_its_level_or_to_nothing() {
		let disabled = RenderOverrides::default().with_color(ColorOverride::Disabled);
		let basic = RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::Basic));

		assert_eq!(render_context(disabled).color_level(), None);
		assert_eq!(render_context(basic).color_level(), Some(ColorLevel::Basic));
	}

	#[test]
	fn the_context_seeds_itself_without_an_override() {
		let one = render_context(RenderOverrides::default()).seed();
		let two = render_context(RenderOverrides::default()).seed();

		assert_ne!(one, two);
	}
}

#[cfg(all(test, feature = "dioxus"))]
mod dioxus_tests {
	use super::*;
	use crate::{Cfonts, Color, Font, Valign};

	#[test]
	fn the_adapter_renders_through_the_browser_environment() {
		let options: Options = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().into();

		let rendered = render_browser(&options, RenderOverrides::default());

		assert_eq!(
			rendered.text,
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll">▄▀█<br>█▀█</div>"#,
		);
	}

	#[test]
	fn the_adapter_carries_a_background() {
		let options: Options =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().background(Color::Blue).into();

		let rendered = render_browser(&options, RenderOverrides::default());

		assert!(rendered.text.contains(r#"<div style="background:#0020f5;min-height:1lh">▄▀█</div>"#));
	}

	#[test]
	fn the_adapter_paints_colors_by_default() {
		let options: Options =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().colors(vec![Color::Red]).into();

		let rendered = render_browser(&options, RenderOverrides::default());

		assert!(rendered.text.contains(r##"<span style="color:#ea3223">"##));
	}

	#[test]
	fn the_adapter_honours_the_overrides() {
		let options: Options = Cfonts::text("AA").font(Font::Tiny).valign(Valign::Top).spaceless().into();

		// three columns hold one Tiny glyph, so the pair wraps where the unlimited page keeps it on one line
		let narrow = render_browser(&options, RenderOverrides::default().with_canvas_width(3));
		let wide = render_browser(&options, RenderOverrides::default());
		assert_ne!(narrow.text, wide.text);
		assert_eq!(
			narrow,
			render_with(
				&options,
				&BrowserEnv,
				RenderContext::with_canvas_width(3).with_color_level(Some(ColorLevel::TrueColor))
			)
		);

		let colored: Options =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().colors(vec![Color::Red]).into();
		let disabled = render_browser(&colored, RenderOverrides::default().with_color(ColorOverride::Disabled));
		assert!(!disabled.text.contains("<span"));
	}
}
