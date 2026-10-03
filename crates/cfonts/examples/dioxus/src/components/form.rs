use cfonts::{Align, BrowserConsoleEnv, Color, Font, GradientPreset, Host, Valign};
use dioxus::prelude::*;
use web_sys::{
	Event, HtmlInputElement, console,
	wasm_bindgen::{JsCast, JsValue},
};

use crate::components::{
	checkbox::Checkbox,
	command::Composition,
	configurator::FormState,
	datalist::Datalist,
	helper::host,
	number_input::NumberInput,
	radio::{Radio, RadioGroup},
	select::Select,
	text_input::TextInput,
};

/// The colors a text field suggests, the four shapes of the grammar first
const COLOR_EXAMPLES: [(&str, &str); 4] = [
	("red-blue", "a gradient"),
	("red:yellow:green", "a transition"),
	("red,blue", "one color per font slot"),
	("#ff8800", "a hex value"),
];

/// The backgrounds a text field suggests, the three shapes of the grammar and the one that paints nothing first
const BACKGROUND_EXAMPLES: [(&str, &str); 4] = [
	("system", "paints nothing"),
	("red-blue", "a gradient from the top down"),
	("red:yellow:green", "a transition"),
	("#222222", "a hex value"),
];

/// Prints the banner into the devtools console, or the error where the banner would go
///
/// It composes from the form itself, a print is an action and reads nothing the page tracks
fn print(form_state: &FormState) {
	match Composition::compose(form_state).options {
		Ok(options) => host().say(&BrowserConsoleEnv, &options).expect("the page console cannot fail"),
		Err(error) => console::error_1(&JsValue::from_str(&format!("ERROR {}", error.message))),
	}
}

/// Whether the event comes from a control whose value waits for enter
fn typed(event: &FormEvent) -> bool {
	event
		.downcast::<Event>()
		.and_then(|event| event.target())
		.and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
		.is_some_and(|input| matches!(input.type_().as_str(), "text" | "number"))
}

#[component]
pub fn Form(form_state: FormState, composition: ReadSignal<Composition>) -> Element {
	let error = use_memo(move || composition.read().options.as_ref().err().cloned());

	// the console prints on every pick and a typed value prints on enter, the page follows on its own
	let print_to_console = move || {
		if form_state.env.read().as_str() == "console" {
			print(&form_state);
		}
	};

	rsx! {
		form {
			id: "configurator",
			class: "frame",
			aria_labelledby: "configurator_title",
			onchange: move |event| {
				if !typed(&event) {
					print_to_console();
				}
			},
			onsubmit: move |event| {
				event.prevent_default();
				print_to_console();
			},
			h2 { class: "frame-title", id: "configurator_title", "Configure" }
			p { class: "intro",
				"Every option of the command line. The terminal below runs the command as you pick."
			}
			fieldset { class: "group",
				legend { class: "visually-hidden", "output" }
				Radio { name: "env", value: "browser", label: "browser", group: form_state.env }
				Radio { name: "env", value: "console", label: "browser console", group: form_state.env }
				button { r#type: "submit", class: "run", "run" }
			}
			fieldset { class: "group",
				legend { class: "visually-hidden", "text" }
				TextInput {
					label: "text",
					name: "text",
					value: form_state.text,
					placeholder: "hello world",
					enterkeyhint: "go",
					error,
				}
				Select { label: "font", name: "font", value: form_state.font, names: &Font::NAMES, error }
				NumberInput {
					label: "letter spacing",
					name: "letter-spacing",
					value: form_state.letter_spacing,
					min: "0",
					max: "20",
					placeholder: "1",
					error,
				}
				NumberInput {
					label: "line height",
					name: "line-height",
					value: form_state.line_height,
					min: "0",
					max: "20",
					placeholder: "1",
					error,
				}
				Checkbox { label: "word wrap", name: "word-wrap", value: form_state.word_wrap }
			}
			fieldset { class: "group",
				legend { class: "visually-hidden", "colors" }
				TextInput {
					label: "colors",
					name: "colors",
					value: form_state.colors,
					placeholder: "red-blue",
					list: "color_grammar",
					error,
				}
				TextInput {
					label: "background",
					name: "background",
					value: form_state.background,
					placeholder: "blue",
					list: "background_grammar",
					error,
				}
				Checkbox { label: "independent gradient", name: "independent-gradient", value: form_state.independent_gradient }
			}
			fieldset { class: "group",
				legend { class: "visually-hidden", "layout" }
				RadioGroup { legend: "align", name: "align", names: &Align::NAMES, group: form_state.align }
				NumberInput { label: "max length", name: "max-length", value: form_state.max_length, min: "0", error }
				Checkbox { label: "spaceless", name: "spaceless", value: form_state.spaceless }
			}
			fieldset { class: "group",
				legend { class: "visually-hidden", "next text block" }
				TextInput {
					label: "next",
					name: "next",
					value: form_state.next,
					placeholder: "next text block",
					enterkeyhint: "go",
					error,
				}
				Select { label: "font", name: "next-font", value: form_state.next_font, names: &Font::NAMES, error }
			}
			RadioGroup { legend: "valign", name: "valign", names: &Valign::NAMES, group: form_state.valign }
		}
		Datalist {
			id: "color_grammar",
			examples: &COLOR_EXAMPLES,
			names: Color::NAMES.iter().chain(GradientPreset::NAMES.iter()).copied().collect(),
		}
		Datalist {
			id: "background_grammar",
			examples: &BACKGROUND_EXAMPLES,
			names: Color::NAMES
				.iter()
				.filter(|name| !matches!(**name, "candy" | "system"))
				.chain(GradientPreset::NAMES.iter())
				.copied()
				.collect(),
		}
	}
}
