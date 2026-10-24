use dioxus::prelude::*;

#[component]
pub fn Radio(name: &'static str, value: &'static str, label: &'static str, mut group: Signal<String>) -> Element {
	rsx! {
		label { class: "radio",
			input {
				r#type: "radio",
				name,
				value,
				checked: group() == value,
				onchange: move |event| group.set(event.value()),
			}
			{label}
		}
	}
}

#[component]
pub fn RadioGroup(
	legend: &'static str,
	name: &'static str,
	names: &'static [&'static str],
	group: Signal<String>,
) -> Element {
	rsx! {
		fieldset { class: "choice",
			legend { {legend} }
			for value in names {
				Radio { name, value: *value, label: *value, group }
			}
		}
	}
}
