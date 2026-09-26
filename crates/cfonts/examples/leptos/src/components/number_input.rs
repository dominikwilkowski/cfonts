use leptos::{html::Input, prelude::*};

use crate::components::helper::{FormError, report};

#[component]
pub fn NumberInput(
	label: &'static str,
	name: &'static str,
	value: RwSignal<String>,
	min: &'static str,
	#[prop(optional)] max: Option<&'static str>,
	#[prop(optional)] placeholder: Option<&'static str>,
	error: Signal<Option<FormError>>,
) -> impl IntoView {
	let input = NodeRef::<Input>::new();
	report(input, name, error, |input, message| input.set_custom_validity(message));

	view! {
		<label class="field">
			{label}
			<input type="number" name=name min=min max=max placeholder=placeholder bind:value=value node_ref=input />
		</label>
	}
}
