use dioxus::prelude::*;

#[component]
pub fn Checkbox(label: &'static str, name: &'static str, mut value: Signal<bool>) -> Element {
	rsx! {
		label { class: "check",
			input {
				r#type: "checkbox",
				name,
				checked: value(),
				onchange: move |event| value.set(event.checked()),
			}
			{label}
		}
	}
}
