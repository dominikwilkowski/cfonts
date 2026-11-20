use dioxus::prelude::*;
use web_sys::{Element as DomElement, HtmlSelectElement};

use crate::components::helper::{FormError, report};

#[component]
pub fn Select(
	label: &'static str,
	name: &'static str,
	mut value: Signal<String>,
	names: &'static [&'static str],
	error: ReadSignal<Option<FormError>>,
) -> Element {
	let mut select = use_signal(|| None);
	report(select, name, error, |select: &HtmlSelectElement, message| select.set_custom_validity(message));

	// a select takes no value before its options exist, so the chosen option marks itself
	let chosen = value();

	rsx! {
		label { class: "field select",
			{label}
			select {
				name,
				onchange: move |event| value.set(event.value()),
				onmounted: move |event| select.set(event.downcast::<DomElement>().cloned()),
				for name in names {
					option { value: *name, selected: *name == chosen, {*name} }
				}
			}
		}
	}
}
