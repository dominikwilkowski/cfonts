use cfonts::{BrowserEnv, render_with};
use leptos::prelude::*;

use crate::components::{
	command::{Command, Composition},
	helper::context,
	prompt::Prompt,
};

/// The message as HTML text, it repeats what the user typed
fn escaped(text: &str) -> String {
	text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[component]
pub fn Console(text: RwSignal<String>, composition: Signal<Composition>) -> impl IntoView {
	let label = move || {
		composition.with(|composition| match &composition.options {
			Ok(_) => text.get(),
			Err(error) => error.message.clone(),
		})
	};
	let canvas = move || {
		composition.with(|composition| match &composition.options {
			Ok(options) => render_with(options, &BrowserEnv, context()).text,
			Err(error) => format!("<span class=\"error\">ERROR</span> {}", escaped(&error.message)),
		})
	};

	view! {
		<section class="terminal frame" aria-labelledby="terminal_title">
			<h3 class="visually-hidden" id="terminal_title">Terminal</h3>
			<p class="titlebar" aria-hidden="true">
				<span class="dot"></span><span class="dot"></span><span class="dot"></span>
				<span class="title">console</span>
			</p>
			<pre><Prompt /><Command composition=composition /></pre>
			<samp id="canvas" role="img" aria-label=label inner_html=canvas></samp>
		</section>
	}
}
