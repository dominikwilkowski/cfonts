use dioxus::prelude::*;

/// The suggestions of one text field: a few explained examples of the grammar, then every name
#[component]
pub fn Datalist(
	id: &'static str,
	examples: &'static [(&'static str, &'static str)],
	names: Vec<&'static str>,
) -> Element {
	rsx! {
		datalist { id,
			for (value, meaning) in examples {
				option { value: *value, {*meaning} }
			}
			for name in names {
				option { value: name }
			}
		}
	}
}
