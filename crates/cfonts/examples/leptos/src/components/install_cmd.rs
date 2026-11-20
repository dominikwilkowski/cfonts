use leptos::prelude::*;

use crate::components::prompt::Prompt;

#[component]
pub fn InstallCmd(platform: &'static str, cmd: &'static str) -> impl IntoView {
	view! {
		<li><span class="key">{platform}</span><code><Prompt />{cmd}</code></li>
	}
}
