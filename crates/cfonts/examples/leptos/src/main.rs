use leptos::prelude::*;

mod components;
use components::{configurator::Configurator, footer::Footer, header::Header};

#[component]
fn App() -> impl IntoView {
	view! {
		<div class="page">
			<Header />
			<Configurator />
			<Footer />
		</div>
	}
}

fn main() {
	leptos::mount::mount_to_body(App);
}
