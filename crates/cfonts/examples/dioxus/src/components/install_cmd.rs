use dioxus::prelude::*;

use crate::components::prompt::Prompt;

#[component]
pub fn InstallCmd(platform: &'static str, cmd: &'static str) -> Element {
	rsx! {
		li {
			span { class: "key", {platform} }
			code {
				Prompt {}
				{cmd}
			}
		}
	}
}
