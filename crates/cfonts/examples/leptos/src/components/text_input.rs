use leptos::{html::Input, prelude::*};

use crate::components::helper::{FormError, report};

#[component]
pub fn TextInput(
	label: &'static str,
	name: &'static str,
	value: RwSignal<String>,
	placeholder: &'static str,
	#[prop(optional)] list: Option<&'static str>,
	#[prop(optional)] enterkeyhint: Option<&'static str>,
	error: Signal<Option<FormError>>,
) -> impl IntoView {
	let input = NodeRef::<Input>::new();
	report(input, name, error, |input, message| input.set_custom_validity(message));

	view! {
		<label class="field">
			{label}
			<input
				type="text"
				name=name
				list=list
				enterkeyhint=enterkeyhint
				autocomplete="off"
				spellcheck="false"
				placeholder=placeholder
				bind:value=value
				node_ref=input
			/>
		</label>
	}
}
