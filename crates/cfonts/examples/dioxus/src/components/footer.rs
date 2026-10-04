use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
	rsx! {
		footer { class: "statusline",
			span { class: "name", "cfonts" }
			span { "GPL-3.0-or-later" }
			span { class: "spacer" }
			a { href: "https://github.com/dominikwilkowski/cfonts", target: "_blank", "github" }
		}
	}
}
