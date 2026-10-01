use dioxus::prelude::*;

use crate::{Options, RenderOverrides, components::render_browser};

/// Renders cfonts HTML inside a Dioxus element
///
/// The artifact paints in true color and never wraps unless the overrides say otherwise:
/// constrain and place it with your own page styles, or hand it a column count
///
/// Candy re-rolls on every render like the hosts do, so a page rendered on the server
/// pins the seed once for the HTML and its hydration to draw the same picks
/// A page built for the browser alone has no entropy source of its own in cfonts, every render rolls afresh there
///
/// ```
/// use cfonts::{RenderOverrides, RustHost};
///
/// // rolled on the server, where a Rust host exists, and carried to the page through the props the hydration reads
/// let overrides = RenderOverrides::default().with_seed(RustHost::entropy());
/// ```
#[component]
pub fn CfontsDioxus(options: ReadSignal<Options>, #[props(default)] overrides: RenderOverrides) -> Element {
	let options = options.read();
	let rendered = render_browser(&options, overrides);

	rsx! {
		div {
			dangerous_inner_html: rendered.text
		}
	}
}
