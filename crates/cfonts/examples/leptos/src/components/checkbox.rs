use leptos::prelude::*;

#[component]
pub fn Checkbox(label: &'static str, name: &'static str, value: RwSignal<bool>) -> impl IntoView {
	view! {
		<label class="check">
			<input type="checkbox" name=name bind:checked=value />
			{label}
		</label>
	}
}
