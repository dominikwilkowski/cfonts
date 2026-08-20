use cfonts::{Align, Cfonts, CfontsLeptos, Font, GradientPreset, LeptosHost, Options};
use leptos::prelude::*;

/// Shows the feature-gated cfonts Leptos adapter
#[component]
fn App() -> impl IntoView {
	let options: Options =
		Cfonts::text("hello").font(Font::Block).align(Align::Center).global_colors(GradientPreset::Pride).into();

	// the console artifact logs with its styles, straight from Rust
	Cfonts::text("hello world")
		.font(Font::Block)
		.global_colors(GradientPreset::Transgender)
		.say(&LeptosHost::default())
		.expect("the page console cannot fail");

	view! {
		<main>
			<CfontsLeptos options=options />
		</main>
	}
}

fn main() {
	leptos::mount::mount_to_body(App);
}
