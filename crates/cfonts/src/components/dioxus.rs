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

#[cfg(test)]
mod tests {
	use super::*;
	use dioxus::dioxus_core::NoOpMutations;

	use crate::{Cfonts, Color, Font, Valign};

	/// A red Tiny banner, its artifact full of the quotes and angle brackets an escaping pass would change
	fn banner(text: &str) -> Options {
		Cfonts::text(text).font(Font::Tiny).valign(Valign::Top).spaceless().colors(vec![Color::RED]).into()
	}

	/// The core artifact for `options` inside the one div the component renders
	fn wrapped(options: &Options, overrides: RenderOverrides) -> String {
		format!("<div>{}</div>", BrowserHost::from_overrides(overrides).render(&BrowserEnv, options).text)
	}

	/// A page holding the component with the overrides it receives and the options signal provided at the root
	fn page(overrides: RenderOverrides) -> Element {
		rsx! {
			CfontsDioxus { options: consume_context::<Signal<Options>>(), overrides }
		}
	}

	/// The page leaving the overrides prop out
	fn page_without_overrides() -> Element {
		rsx! {
			CfontsDioxus { options: consume_context::<Signal<Options>>() }
		}
	}

	/// Provides `options` as the signal the page reads and renders the dom once, handing the signal back for writes
	fn rebuilt(dom: &mut VirtualDom, options: Options) -> Signal<Options> {
		let options = dom.in_scope(ScopeId::ROOT, || Signal::new(options));
		dom.provide_root_context(options);
		dom.rebuild_in_place();
		options
	}

	/// The HTML a server sends for the page with `options` and `overrides`
	fn served(options: Options, overrides: RenderOverrides) -> String {
		let mut dom = VirtualDom::new_with_props(page, overrides);
		rebuilt(&mut dom, options);
		dioxus_ssr::render(&dom)
	}

	#[test]
	fn the_component_renders_the_browser_artifact_inside_one_div() {
		// the overrides prop is left out, so the component renders what the default overrides render
		let mut dom = VirtualDom::new(page_without_overrides);
		rebuilt(&mut dom, banner("A"));

		assert_eq!(dioxus_ssr::render(&dom), wrapped(&banner("A"), RenderOverrides::default()));
	}

	#[test]
	fn a_column_override_wraps_the_artifact_like_the_core() {
		// three columns hold one Tiny glyph, so the pair wraps where the unlimited page keeps it on one line
		let overrides = RenderOverrides::default().with_canvas_width(3);
		let html = served(banner("AA"), overrides);

		assert_eq!(html, wrapped(&banner("AA"), overrides));
		assert_ne!(html, wrapped(&banner("AA"), RenderOverrides::default()));
	}

	#[test]
	fn a_pinned_seed_draws_the_candy_of_the_core() {
		let candy = || -> Options { Cfonts::text("CANDY").font(Font::Tiny).colors(vec![Color::CANDY]).into() };
		let overrides = RenderOverrides::default().with_seed(42);

		assert_eq!(served(candy(), overrides), wrapped(&candy(), overrides));
	}

	#[test]
	fn a_new_options_value_renders_the_new_artifact() {
		let mut dom = VirtualDom::new(page_without_overrides);
		let mut options = rebuilt(&mut dom, banner("A"));

		options.set(banner("B"));
		dom.render_immediate(&mut NoOpMutations);

		assert_eq!(dioxus_ssr::render(&dom), wrapped(&banner("B"), RenderOverrides::default()));
	}
}
