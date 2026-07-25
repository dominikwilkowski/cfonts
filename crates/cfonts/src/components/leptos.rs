use leptos::prelude::*;

use crate::{BrowserConsoleEnv, ColorLevel, Options, RenderContext, render_with};

use super::render_browser;

/// Renders cfonts HTML inside a Leptos element
///
/// Colors paint at full support because the artifact is CSS; the canvas stays
/// unlimited because browser capability discovery belongs to the application host
///
/// The seed makes candy picks reproducible across hydration
#[component]
pub fn CfontsLeptos(#[prop(into)] options: Signal<Options>, #[prop(optional)] seed: u64) -> impl IntoView {
	view! {
		<div
			inner_html=move || {
				options.with(|options| {
					render_browser(options, seed).text
				})
			}
		/>
	}
}

/// Writes the browser console artifact through the page's own console
///
/// The framework re-exports the bindings, so core carries no browser dependency;
/// style values spread as arguments exactly like the TypeScript host's say
pub fn console_say(options: &Options, seed: u64) {
	let rendered =
		render_with(options, &BrowserConsoleEnv, RenderContext::colored(ColorLevel::TrueColor).with_seed(seed));

	let arguments = leptos::web_sys::js_sys::Array::new();
	arguments.push(&leptos::wasm_bindgen::JsValue::from_str(&rendered.text));

	for style in &rendered.styles {
		arguments.push(&leptos::wasm_bindgen::JsValue::from_str(style));
	}

	leptos::web_sys::console::log(&arguments);
}
