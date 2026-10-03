use leptos::prelude::*;

use crate::{BrowserEnv, BrowserHost, Host, Options, RenderOverrides};

/// Renders cfonts HTML inside a Leptos element
///
/// The artifact paints in true color and never wraps unless the overrides say otherwise:
/// constrain and place it with your own page styles, or hand it a column count
///
/// Candy re-rolls on every render like the hosts do, so a page rendered on the server
/// pins the seed once for the HTML and its hydration to draw the same picks,
/// see [`BrowserHost`]
#[component]
pub fn CfontsLeptos(
	#[prop(into)] options: Signal<Options>,
	#[prop(optional)] overrides: RenderOverrides,
) -> impl IntoView {
	let host = BrowserHost::from_overrides(overrides);

	view! {
		<div inner_html=move || options.with(|options| host.render(&BrowserEnv, options).text) />
	}
}
