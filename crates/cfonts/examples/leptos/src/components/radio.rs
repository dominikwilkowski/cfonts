use leptos::prelude::*;

#[component]
pub fn Radio(name: &'static str, value: &'static str, label: &'static str, group: RwSignal<String>) -> impl IntoView {
	// bind:group matches the value attribute, so the attribute has to be on the element before the binding
	view! {
		<label class="radio">
			<input type="radio" name=name value=value bind:group=group />
			{label}
		</label>
	}
}

#[component]
pub fn RadioGroup(
	legend: &'static str,
	name: &'static str,
	names: &'static [&'static str],
	group: RwSignal<String>,
) -> impl IntoView {
	view! {
		<fieldset class="choice">
			<legend>{legend}</legend>
			{names.iter().map(|value| view! { <Radio name=name value=*value label=*value group=group /> }).collect_view()}
		</fieldset>
	}
}
