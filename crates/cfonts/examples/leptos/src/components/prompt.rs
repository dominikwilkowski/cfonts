use leptos::prelude::*;

#[component]
pub fn Prompt() -> impl IntoView {
	view! {
		<span class="prompt" aria-hidden="true">"$ "</span>
	}
}
