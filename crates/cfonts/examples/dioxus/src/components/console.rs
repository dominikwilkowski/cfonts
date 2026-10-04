use cfonts::{BrowserEnv, Host};
use dioxus::prelude::*;

use crate::components::{
	command::{Command, Composition},
	helper::host,
	prompt::Prompt,
};

/// The message as HTML text, it repeats what the user typed
fn escaped(text: &str) -> String {
	text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[component]
pub fn Console(text: Signal<String>, composition: ReadSignal<Composition>) -> Element {
	let spelled = composition.read();
	let label = match &spelled.options {
		Ok(_) => text(),
		Err(error) => error.message.clone(),
	};
	let canvas = match &spelled.options {
		Ok(options) => host().render(&BrowserEnv, options).text,
		Err(error) => format!("<span class=\"error\">ERROR</span> {}", escaped(&error.message)),
	};

	rsx! {
		section { class: "terminal frame", aria_labelledby: "terminal_title",
			h3 { class: "visually-hidden", id: "terminal_title", "Terminal" }
			p { class: "titlebar", aria_hidden: "true",
				span { class: "dot" }
				span { class: "dot" }
				span { class: "dot" }
				span { class: "title", "console" }
			}
			pre {
				Prompt {}
				Command { composition }
			}
			samp { id: "canvas", role: "img", aria_label: label, dangerous_inner_html: canvas }
		}
	}
}
