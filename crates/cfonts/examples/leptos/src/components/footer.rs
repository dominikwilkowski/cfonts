use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
	view! {
		<footer class="statusline">
			<span class="name">"cfonts"</span>
			<span>"GPL-3.0-or-later"</span>
			<span class="spacer"></span>
			<a href="https://github.com/dominikwilkowski/cfonts" target="_blank">github</a>
		</footer>
	}
}
