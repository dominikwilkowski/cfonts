use leptos::prelude::*;

/// The suggestions of one text field: a few explained examples of the grammar, then every name
#[component]
pub fn Datalist(
	id: &'static str,
	examples: &'static [(&'static str, &'static str)],
	names: Vec<&'static str>,
) -> impl IntoView {
	view! {
		<datalist id=id>
			{examples.iter().map(|(value, meaning)| view! { <option value=*value>{*meaning}</option> }).collect_view()}
			{names.into_iter().map(|name| view! { <option value=name></option> }).collect_view()}
		</datalist>
	}
}
