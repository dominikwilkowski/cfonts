use dioxus::prelude::*;

use crate::{BrowserEnv, BrowserHost, Host, Options, RenderOverrides};

/// Renders cfonts HTML inside a Dioxus element
///
/// The artifact paints in true color and never wraps unless the overrides say otherwise:
/// constrain and place it with your own page styles, or hand it a column count
///
/// Candy re-rolls on every render like the hosts do, so a page rendered on the server
/// pins the seed once for the HTML and its hydration to draw the same picks,
/// see [`BrowserHost`]
///
/// ```
/// use cfonts::{BrowserHost, RenderOverrides};
///
/// // rolled on the server and carried to the page through the props the hydration reads
/// let overrides = RenderOverrides::default().with_seed(BrowserHost::entropy());
/// ```
#[component]
pub fn CfontsDioxus(options: ReadSignal<Options>, #[props(default)] overrides: RenderOverrides) -> Element {
	let options = options.read();
	let rendered = BrowserHost::from_overrides(overrides).render(&BrowserEnv, &options);

	rsx! {
		div {
			dangerous_inner_html: rendered.text
		}
	}
}
