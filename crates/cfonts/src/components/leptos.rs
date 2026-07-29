use std::convert::Infallible;

use leptos::prelude::*;

use crate::{BrowserConsoleEnv, BrowserEnv, Host, Options, RenderContext, Rendered};

use super::render_context;

/// The Leptos host: `render` returns the HTML artifact, `say` writes through the page's console
///
/// Rendering is pure, so the host works under CSR and SSR alike; on the server
/// no page console exists, so `say` renders and writes nothing, and hydration
/// replays the call in the browser
///
/// The fixed default seed keeps candy colors identical between the server-rendered
/// HTML and its hydration, so the two never mismatch
#[derive(Debug, Clone, Copy, Default)]
pub struct LeptosHost {
	/// Entropy for candy picks; the same seed draws the same assortment
	pub seed: u64,
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
		render_context(self.seed)
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
/// The artifact always paints in full color and never wraps:
/// constrain and place it with your own page styles
///
/// The seed makes candy picks reproducible across hydration
#[component]
pub fn CfontsLeptos(#[prop(into)] options: Signal<Options>, #[prop(optional)] seed: u64) -> impl IntoView {
	let host = LeptosHost { seed };

	view! {
		<div inner_html=move || options.with(|options| host.render(options).text) />
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Cfonts, Color, Font, Valign};

	#[test]
	fn the_host_renders_the_browser_artifact() {
		let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().render(&LeptosHost::default());

		assert_eq!(
			rendered.text,
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">▄▀█<br>█▀█</div>"#,
		);
	}

	#[test]
	fn the_host_paints_colors_by_default() {
		let rendered = Cfonts::text("A")
			.font(Font::Tiny)
			.valign(Valign::Top)
			.spaceless()
			.color(vec![Color::Red])
			.render(&LeptosHost::default());

		assert!(rendered.text.contains(r##"<span style="color:#ea3223">"##));
	}

	#[test]
	fn say_cannot_fail_on_the_server() {
		// native test targets take the SSR write arm: render, write nothing
		Cfonts::text("A").say(&LeptosHost::default()).expect("the console write cannot fail");
	}
}
