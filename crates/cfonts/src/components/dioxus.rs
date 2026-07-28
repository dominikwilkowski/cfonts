use dioxus::prelude::*;

use crate::Options;

use super::render_browser;

/// Renders cfonts HTML inside a Dioxus element
///
/// The artifact always paints in full color and never wraps:
/// constrain and place it with your own page styles
///
/// The seed makes candy picks reproducible across renders
#[component]
pub fn CfontsDioxus(options: ReadSignal<Options>, #[props(default)] seed: u64) -> Element {
	let options = options.read();
	let rendered = render_browser(&options, seed);

	rsx! {
		div {
			dangerous_inner_html: rendered.text
		}
	}
}
