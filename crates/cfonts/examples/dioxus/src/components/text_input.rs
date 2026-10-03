use dioxus::prelude::*;
use web_sys::{Element as DomElement, HtmlInputElement};

use crate::components::helper::{FormError, report};

#[component]
pub fn TextInput(
	label: &'static str,
	name: &'static str,
	mut value: Signal<String>,
	placeholder: &'static str,
	list: Option<&'static str>,
	enterkeyhint: Option<&'static str>,
	error: ReadSignal<Option<FormError>>,
) -> Element {
	let mut input = use_signal(|| None);
	report(input, name, error, |input: &HtmlInputElement, message| input.set_custom_validity(message));

	rsx! {
		label { class: "field",
			{label}
			input {
				r#type: "text",
				name,
				list,
				enterkeyhint,
				autocomplete: "off",
				spellcheck: false,
				placeholder,
				value: value(),
				oninput: move |event| value.set(event.value()),
				onmounted: move |event| input.set(event.downcast::<DomElement>().cloned()),
			}
		}
	}
}
