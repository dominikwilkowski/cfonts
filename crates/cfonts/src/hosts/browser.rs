use std::convert::Infallible;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use web_sys::wasm_bindgen::JsValue;

use crate::{ColorLevel, Host, RenderOverrides, Rendered, hosts::entropy};

/// The browser host: nothing is detected, output goes to the page's console
///
/// Rendering is pure, so the host works under CSR and SSR alike, on the server
/// no page console exists, so `say` renders and writes nothing, and hydration
/// replays the call in the browser
///
/// The artifact paints in true color and never wraps unless the overrides say otherwise,
/// and candy re-rolls on every render like the hosts do, so a page rendered on the server
/// pins the seed once for the HTML and its hydration to draw the same picks
///
/// ```
/// use cfonts::{BrowserHost, RenderOverrides};
///
/// // rolled once where the server render and the hydrating page both read it
/// let host = BrowserHost::from_overrides(RenderOverrides::default().with_seed(BrowserHost::entropy()));
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct BrowserHost {
	overrides: RenderOverrides,
}

impl BrowserHost {
	/// Creates a page host with explicit overrides
	#[must_use]
	pub const fn from_overrides(overrides: RenderOverrides) -> Self {
		Self { overrides }
	}

	/// A fresh seed for candy colors, the one a render rolls when no seed override is given
	///
	/// Pages that hold a seed of their own take it from here, so the candy picks differ
	/// between page loads and hold for as long as the seed is kept
	///
	/// ```
	/// use cfonts::{BrowserHost, RenderOverrides};
	///
	/// let seed = BrowserHost::entropy();
	/// let host = BrowserHost::from_overrides(RenderOverrides::default().with_seed(seed));
	///
	/// assert_ne!(seed, BrowserHost::entropy());
	/// ```
	#[must_use]
	pub fn entropy() -> u64 {
		entropy()
	}
}

impl Host for BrowserHost {
	type Error = Infallible;

	/// A page has no terminal to measure, so only a column count wraps
	fn canvas_width(&self) -> Option<usize> {
		self.overrides.canvas_width().columns_or(None)
	}

	/// The artifact is CSS, so automatic color paints at full support
	fn color_level(&self) -> Option<ColorLevel> {
		self.overrides.color().level_or(Some(ColorLevel::TrueColor))
	}

	fn seed(&self) -> u64 {
		self.overrides.seed().unwrap_or_else(Self::entropy)
	}

	/// Spreads the style values into the page console, the one write of the npm browser host as well
	#[cfg(all(target_family = "wasm", target_os = "unknown"))]
	fn write(&self, rendered: &Rendered, _line_end: &str) -> Result<(), Self::Error> {
		let arguments = js_sys::Array::new();
		arguments.push(&JsValue::from_str(&rendered.text));

		for style in &rendered.styles {
			arguments.push(&JsValue::from_str(style));
		}

		web_sys::console::log(&arguments);

		Ok(())
	}

	/// The server has no page console: SSR renders and writes nothing
	#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
	fn write(&self, _rendered: &Rendered, _line_end: &str) -> Result<(), Self::Error> {
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{BrowserConsoleEnv, BrowserEnv, Cfonts, Color, ColorOverride, Font, Options, Valign, render_with};

	#[test]
	fn the_host_renders_the_browser_artifact() {
		let rendered =
			Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().render(&BrowserHost::default(), &BrowserEnv);

		assert_eq!(
			rendered.text,
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll">▄▀█<br>█▀█</div>"#,
		);
	}

	#[test]
	fn the_host_carries_a_background() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.background(Color::BLUE)
			.render(&BrowserHost::default(), &BrowserEnv);

		assert!(rendered.text.contains(r#"<div style="background:#0020f5;min-height:1lh">▄▀█</div>"#));
	}

	#[test]
	fn the_host_paints_colors_by_default() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.colors(vec![Color::RED])
			.render(&BrowserHost::default(), &BrowserEnv);

		assert!(rendered.text.contains(r##"<span style="color:#ea3223">"##));
	}

	#[test]
	fn only_a_column_count_limits_the_canvas() {
		// three columns hold one Tiny glyph, so the pair wraps where the unlimited page keeps it on one line
		let options: Options = Cfonts::text("AA").font(Font::Tiny).valign(Valign::Top).spaceless().into();
		let narrow =
			BrowserHost::from_overrides(RenderOverrides::default().with_canvas_width(3)).render(&BrowserEnv, &options);
		let unlimited =
			BrowserHost::from_overrides(RenderOverrides::default().with_canvas_width(0)).render(&BrowserEnv, &options);

		assert_ne!(narrow.text, unlimited.text);
		assert_eq!(unlimited.text, BrowserHost::default().render(&BrowserEnv, &options).text);
		assert_eq!(
			narrow,
			render_with(
				&options,
				&BrowserEnv,
				RenderOverrides::default().with_canvas_width(3).with_color(ColorOverride::Level(ColorLevel::TrueColor))
			)
		);
	}

	#[test]
	fn the_color_override_resolves_to_its_level_or_to_nothing() {
		let banner = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().colors(vec![Color::RED]);
		let disabled = BrowserHost::from_overrides(RenderOverrides::default().with_color(ColorOverride::Disabled));
		let basic =
			BrowserHost::from_overrides(RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::Basic)));

		assert_eq!(disabled.color_level(), None);
		assert_eq!(basic.color_level(), Some(ColorLevel::Basic));
		assert!(!banner.render(&disabled, &BrowserEnv).text.contains("<span"));
		assert!(banner.render(&BrowserHost::default(), &BrowserEnv).text.contains("<span"));
	}

	#[test]
	fn a_pinned_seed_draws_the_same_candy_and_the_default_rolls_anew() {
		let banner = Cfonts::text("CANDY").font(Font::Tiny).colors(vec![Color::CANDY]);
		let pinned = BrowserHost::from_overrides(RenderOverrides::default().with_seed(42));

		assert_eq!(pinned.seed(), 42);
		assert_eq!(banner.render(&pinned, &BrowserEnv), banner.render(&pinned, &BrowserEnv));
		// every default host rolls its own seed, so two of them draw different assortments
		assert_ne!(
			banner.render(&BrowserHost::default(), &BrowserEnv),
			banner.render(&BrowserHost::default(), &BrowserEnv)
		);
	}

	#[test]
	fn say_cannot_fail_on_the_server() {
		// native test targets take the SSR write arm: render, write nothing
		Cfonts::text("A").say(&BrowserHost::default(), &BrowserConsoleEnv).expect("the console write cannot fail");
	}
}
