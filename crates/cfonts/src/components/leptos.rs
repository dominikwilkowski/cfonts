use leptos::prelude::*;

use crate::{BrowserHost, BrowserTarget, Host, Options, RenderOverrides};

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
		<div inner_html=move || options.with(|options| host.render(&BrowserTarget, options).text) />
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Cfonts, Color, Font, Valign};

	/// A Tiny banner without padding rows
	fn banner(text: &str) -> Options {
		Cfonts::text(text).font(Font::Tiny).valign(Valign::Top).spaceless().into()
	}

	/// A banner rolling a fresh candy pick per painted segment
	fn candy() -> Options {
		Cfonts::text("CANDY").font(Font::Tiny).colors(vec![Color::CANDY]).into()
	}

	/// A red hex line wider than eighty columns, so a column count would wrap it and disabled color would drop its paint
	fn wide_red_line() -> Options {
		Cfonts::text("WIDER THAN EIGHTY COLUMNS").colors(vec![Color::rgb(255, 0, 0)]).into()
	}

	/// The host's artifact inside the one div the component adds
	fn in_a_div(host: BrowserHost, options: &Options) -> String {
		format!("<div>{}</div>", host.render(&BrowserTarget, options).text)
	}

	#[test]
	fn the_component_renders_the_browser_artifact_inside_one_div() {
		let html = view! { <CfontsLeptos options=banner("A") /> }.to_html();

		assert_eq!(html, in_a_div(BrowserHost::default(), &banner("A")));
	}

	#[test]
	fn an_omitted_overrides_prop_renders_like_the_default_overrides() {
		let html = view! { <CfontsLeptos options=wide_red_line() /> }.to_html();

		assert_eq!(html, in_a_div(BrowserHost::from_overrides(RenderOverrides::default()), &wide_red_line()));
	}

	#[test]
	fn a_column_override_wraps_the_artifact() {
		// three columns hold one Tiny glyph, so the pair wraps where the default page keeps it on one line
		let narrow = RenderOverrides::default().with_canvas_width(3);
		let wrapped = view! { <CfontsLeptos options=banner("AA") overrides=narrow /> }.to_html();
		let unlimited = view! { <CfontsLeptos options=banner("AA") /> }.to_html();

		assert_eq!(wrapped, in_a_div(BrowserHost::from_overrides(narrow), &banner("AA")));
		assert_ne!(wrapped, unlimited);
	}

	#[test]
	fn a_pinned_seed_draws_the_candy_of_the_core() {
		let pinned = RenderOverrides::default().with_seed(42);
		let html = view! { <CfontsLeptos options=candy() overrides=pinned /> }.to_html();

		assert_eq!(html, in_a_div(BrowserHost::from_overrides(pinned), &candy()));
	}

	#[test]
	fn the_component_renders_the_latest_value_of_its_options_signal() {
		// the view is built before the signal changes and renders after it, so only a reactive read shows the new text
		let options = RwSignal::new(banner("A"));
		let view = view! { <CfontsLeptos options=options /> };
		options.set(banner("B"));

		assert_eq!(view.to_html(), in_a_div(BrowserHost::default(), &banner("B")));
	}
}
