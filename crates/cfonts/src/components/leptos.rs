use std::convert::Infallible;

use leptos::prelude::*;

use crate::{
	BrowserConsoleEnv, BrowserEnv, Host, Options, RenderContext, RenderOverrides, Rendered, components::render_context,
	hosts::entropy,
};

/// The Leptos host: `render` returns the HTML artifact, `say` writes through the page's console
///
/// Rendering is pure, so the host works under CSR and SSR alike; on the server
/// no page console exists, so `say` renders and writes nothing, and hydration
/// replays the call in the browser
///
/// The artifact paints in true color and never wraps unless the overrides say otherwise,
/// and candy re-rolls on every render like the hosts do, so a page rendered on the server
/// pins the seed once for the HTML and its hydration to draw the same picks
///
/// ```
/// use cfonts::{LeptosHost, RenderOverrides};
///
/// // rolled once where the server render and the hydrating page both read it
/// let host = LeptosHost::from_overrides(RenderOverrides::default().with_seed(LeptosHost::entropy()));
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct LeptosHost {
	overrides: RenderOverrides,
}

impl LeptosHost {
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
	/// use cfonts::{LeptosHost, RenderOverrides};
	///
	/// let seed = LeptosHost::entropy();
	/// let host = LeptosHost::from_overrides(RenderOverrides::default().with_seed(seed));
	///
	/// assert_ne!(seed, LeptosHost::entropy());
	/// ```
	#[must_use]
	pub fn entropy() -> u64 {
		entropy()
	}
}

impl Host for LeptosHost {
	type RenderEnvironment = BrowserEnv;
	type SayEnvironment = BrowserConsoleEnv;
	type Error = Infallible;

	fn render_environment(&self) -> &BrowserEnv {
		&BrowserEnv
	}

	fn say_environment(&self) -> &BrowserConsoleEnv {
		&BrowserConsoleEnv
	}

	fn resolve_context(&self) -> RenderContext {
		render_context(self.overrides)
	}

	/// Spreads the style values into the page console, exactly like the TypeScript host's say
	#[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
	fn write(&self, rendered: &Rendered) -> Result<(), Self::Error> {
		let arguments = leptos::web_sys::js_sys::Array::new();
		arguments.push(&leptos::wasm_bindgen::JsValue::from_str(&rendered.text));

		for style in &rendered.styles {
			arguments.push(&leptos::wasm_bindgen::JsValue::from_str(style));
		}

		leptos::web_sys::console::log(&arguments);

		Ok(())
	}

	/// The server has no page console: SSR renders and writes nothing
	#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
	fn write(&self, _rendered: &Rendered) -> Result<(), Self::Error> {
		Ok(())
	}
}

/// Renders cfonts HTML inside a Leptos element
///
/// The artifact paints in true color and never wraps unless the overrides say otherwise:
/// constrain and place it with your own page styles, or hand it a column count
///
/// Candy re-rolls on every render like the hosts do, so a page rendered on the server
/// pins the seed once for the HTML and its hydration to draw the same picks,
/// see [`LeptosHost`]
#[component]
pub fn CfontsLeptos(
	#[prop(into)] options: Signal<Options>,
	#[prop(optional)] overrides: RenderOverrides,
) -> impl IntoView {
	let host = LeptosHost::from_overrides(overrides);

	view! {
		<div inner_html=move || options.with(|options| host.render(options).text) />
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Cfonts, Color, ColorLevel, ColorOverride, Font, Valign, render_with};

	#[test]
	fn the_host_renders_the_browser_artifact() {
		let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().render(&LeptosHost::default());

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
			.background(Color::Blue)
			.render(&LeptosHost::default());

		assert!(rendered.text.contains(r#"<div style="background:#0020f5;min-height:1lh">▄▀█</div>"#));
	}

	#[test]
	fn the_host_paints_colors_by_default() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.colors(vec![Color::Red])
			.render(&LeptosHost::default());

		assert!(rendered.text.contains(r##"<span style="color:#ea3223">"##));
	}

	#[test]
	fn a_width_override_wraps_the_page() {
		// three columns hold one Tiny glyph, so the pair wraps where the unlimited page keeps it on one line
		let options: Options = Cfonts::text("AA").font(Font::Tiny).valign(Valign::Top).spaceless().into();
		let narrow = LeptosHost::from_overrides(RenderOverrides::default().with_canvas_width(3)).render(&options);

		assert_ne!(narrow.text, LeptosHost::default().render(&options).text);
		assert_eq!(
			narrow,
			render_with(
				&options,
				&BrowserEnv,
				RenderContext::with_canvas_width(3).with_color_level(Some(ColorLevel::TrueColor))
			)
		);
	}

	#[test]
	fn disabled_color_paints_nothing() {
		let banner = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().colors(vec![Color::Red]);
		let disabled = LeptosHost::from_overrides(RenderOverrides::default().with_color(ColorOverride::Disabled));

		assert!(!banner.render(&disabled).text.contains("<span"));
		assert!(banner.render(&LeptosHost::default()).text.contains("<span"));
	}

	#[test]
	fn a_pinned_seed_draws_the_same_candy_and_the_default_rolls_anew() {
		let banner = Cfonts::text("CANDY").font(Font::Tiny).colors(vec![Color::Candy]);
		let pinned = LeptosHost::from_overrides(RenderOverrides::default().with_seed(42));

		assert_eq!(banner.render(&pinned), banner.render(&pinned));
		// every default host rolls its own seed, so two of them draw different assortments
		assert_ne!(banner.render(&LeptosHost::default()), banner.render(&LeptosHost::default()));
	}

	#[test]
	fn say_cannot_fail_on_the_server() {
		// native test targets take the SSR write arm: render, write nothing
		Cfonts::text("A").say(&LeptosHost::default()).expect("the console write cannot fail");
	}
}
