use leptos::{html::Select as LeptosSelect, prelude::*};

use crate::components::helper::{FormError, report};

#[component]
pub fn Select(
	label: &'static str,
	name: &'static str,
	value: RwSignal<String>,
	names: &'static [&'static str],
	error: Signal<Option<FormError>>,
) -> impl IntoView {
	let select = NodeRef::<LeptosSelect>::new();
	report(select, name, error, |select, message| select.set_custom_validity(message));

	// bind sets the select's value before its options exist, so the chosen option marks itself
	let chosen = value.get_untracked();

	view! {
		<label class="field select">
			{label}
			<select name=name bind:value=value node_ref=select>
				{names
					.iter()
					.map(|name| view! { <option value=*name selected=*name == chosen>{*name}</option> })
					.collect_view()}
			</select>
		</label>
	}
}
