use cfonts::{Cfonts, CfontsLeptos, GradientOption, GradientStop, Rgb};
use leptos::prelude::*;
use std::time::Duration;

const GRADIENT_COLORS: [Rgb; 57] = [
	Rgb { red: 255, green: 0, blue: 0 },
	Rgb { red: 255, green: 0, blue: 26 },
	Rgb { red: 255, green: 0, blue: 53 },
	Rgb { red: 255, green: 0, blue: 80 },
	Rgb { red: 255, green: 0, blue: 107 },
	Rgb { red: 255, green: 0, blue: 134 },
	Rgb { red: 255, green: 0, blue: 161 },
	Rgb { red: 255, green: 0, blue: 187 },
	Rgb { red: 255, green: 0, blue: 214 },
	Rgb { red: 255, green: 0, blue: 241 },
	Rgb { red: 241, green: 0, blue: 255 },
	Rgb { red: 214, green: 0, blue: 255 },
	Rgb { red: 187, green: 0, blue: 255 },
	Rgb { red: 161, green: 0, blue: 255 },
	Rgb { red: 134, green: 0, blue: 255 },
	Rgb { red: 107, green: 0, blue: 255 },
	Rgb { red: 80, green: 0, blue: 255 },
	Rgb { red: 53, green: 0, blue: 255 },
	Rgb { red: 26, green: 0, blue: 255 },
	Rgb { red: 0, green: 0, blue: 255 },
	Rgb { red: 0, green: 26, blue: 255 },
	Rgb { red: 0, green: 53, blue: 255 },
	Rgb { red: 0, green: 80, blue: 255 },
	Rgb { red: 0, green: 107, blue: 255 },
	Rgb { red: 0, green: 134, blue: 255 },
	Rgb { red: 0, green: 161, blue: 255 },
	Rgb { red: 0, green: 187, blue: 255 },
	Rgb { red: 0, green: 214, blue: 255 },
	Rgb { red: 0, green: 241, blue: 255 },
	Rgb { red: 0, green: 255, blue: 241 },
	Rgb { red: 0, green: 255, blue: 214 },
	Rgb { red: 0, green: 255, blue: 187 },
	Rgb { red: 0, green: 255, blue: 161 },
	Rgb { red: 0, green: 255, blue: 134 },
	Rgb { red: 0, green: 255, blue: 107 },
	Rgb { red: 0, green: 255, blue: 80 },
	Rgb { red: 0, green: 255, blue: 53 },
	Rgb { red: 0, green: 255, blue: 26 },
	Rgb { red: 0, green: 255, blue: 0 },
	Rgb { red: 26, green: 255, blue: 0 },
	Rgb { red: 53, green: 255, blue: 0 },
	Rgb { red: 80, green: 255, blue: 0 },
	Rgb { red: 107, green: 255, blue: 0 },
	Rgb { red: 134, green: 255, blue: 0 },
	Rgb { red: 161, green: 255, blue: 0 },
	Rgb { red: 187, green: 255, blue: 0 },
	Rgb { red: 214, green: 255, blue: 0 },
	Rgb { red: 241, green: 255, blue: 0 },
	Rgb { red: 255, green: 241, blue: 0 },
	Rgb { red: 255, green: 214, blue: 0 },
	Rgb { red: 255, green: 187, blue: 0 },
	Rgb { red: 255, green: 161, blue: 0 },
	Rgb { red: 255, green: 134, blue: 0 },
	Rgb { red: 255, green: 107, blue: 0 },
	Rgb { red: 255, green: 80, blue: 0 },
	Rgb { red: 255, green: 53, blue: 0 },
	Rgb { red: 255, green: 26, blue: 0 },
];

#[component]
pub fn Logo() -> impl IntoView {
	let offset = RwSignal::new(0);
	let options = Signal::derive(move || {
		Cfonts::text("cfonts")
			.spaceless()
			.global_colors(GradientOption::TwoStop {
				start: GradientStop::Rgb(GRADIENT_COLORS[(offset.get() + GRADIENT_COLORS.len() - 1) % GRADIENT_COLORS.len()]),
				end: GradientStop::Rgb(GRADIENT_COLORS[offset.get()]),
			})
			.into()
	});

	// wrapping it in an effect means the interval is SSR save
	Effect::new(move |_| {
		let reduced_motion =
			window().match_media("(prefers-reduced-motion: reduce)").ok().flatten().is_some_and(|query| query.matches());

		if !reduced_motion {
			let handle = set_interval_with_handle(
				move || offset.update(|o| *o = (*o + 1) % GRADIENT_COLORS.len()),
				Duration::from_millis(100),
			)
			.expect("the page can always schedule an interval");
			on_cleanup(move || handle.clear());
		}
	});

	view! {
		<h1 class="visually-hidden">cfonts</h1>
		<div id="logo" role="img" aria-label="cfonts, drawn in the block font with a red to blue gradient">
			<CfontsLeptos options=options />
		</div>
	}
}
