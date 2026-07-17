use leptos::prelude::*;

use crate::{BrowserEnv, Options, RenderContext, Rendered, render_with};

/// Renders cfonts HTML inside a Leptos element
///
/// Rendering is pure and uses an unlimited canvas because browser capability
/// discovery belongs to the application host
#[component]
pub fn CfontsLeptos(#[prop(into)] options: Signal<Options>) -> impl IntoView {
	view! {
		<div
			inner_html=move || {
				options.with(|options| {
					render_browser(options).text
				})
			}
		/>
	}
}

/// Adapts cfonts options to the HTML artifact consumed by Leptos
fn render_browser(options: &Options) -> Rendered {
	render_with(options, &BrowserEnv, RenderContext::unlimited())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Cfonts, Font, Valign};

	#[test]
	fn adapter_renders_through_the_browser_environment() {
		let options: Options = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().into();

		let rendered = render_browser(&options);

		assert_eq!(
			rendered.text,
			r#"<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">▄▀█<br>█▀█</div>"#,
		);
	}
}
