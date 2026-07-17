use cfonts::{Align, Cfonts, CfontsLeptos, Font, Options};
use leptos::prelude::*;

/// Shows the feature-gated cfonts Leptos adapter
#[component]
fn App() -> impl IntoView {
	let options: Options = Cfonts::text("hello").font(Font::Block).align(Align::Center).into();

	view! {
		<main>
			<CfontsLeptos options=options />
		</main>
	}
}

fn main() {
	leptos::mount::mount_to_body(App);
}
