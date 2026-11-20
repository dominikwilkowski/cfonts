use dioxus::prelude::*;

#[component]
pub fn Prompt() -> Element {
	rsx! {
		span { class: "prompt", aria_hidden: "true", "$ " }
	}
}
