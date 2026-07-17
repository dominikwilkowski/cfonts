use dioxus::prelude::*;

use crate::{BrowserEnv, Options, RenderContext, Rendered, render_with};

/// Renders cfonts HTML inside a Dioxus element
///
/// Rendering is pure and uses an unlimited canvas because browser capability
/// discovery belongs to the application host
#[component]
pub fn CfontsDioxus(options: ReadSignal<Options>) -> Element {
	let options = options.read();
	let rendered = render_browser(&options);

	rsx! {
		div {
			dangerous_inner_html: rendered.text
		}
	}
}

/// Adapts cfonts options to the HTML artifact consumed by Dioxus
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
