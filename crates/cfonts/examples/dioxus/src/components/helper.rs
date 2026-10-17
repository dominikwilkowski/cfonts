use cfonts::{BrowserHost, RenderOverrides};
use dioxus::prelude::*;
use web_sys::{Element as DomElement, wasm_bindgen::JsCast};

/// The page is an eighty column terminal, so long text wraps the way it would there
///
/// The host is built here because this is the one place that knows the width,
/// the canvas and the console print share the width
pub fn host() -> BrowserHost {
	BrowserHost::from_overrides(RenderOverrides::default().with_canvas_width(80))
}

/// The option the command line would refuse
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormError {
	/// The name attribute of the control that holds the refused value
	pub name: &'static str,

	pub message: String,
}

/// Mirrors the error compose reports against one field into its control's custom validity,
/// so :user-invalid styling and validity.valid behave as they do in the browser example
///
/// The setter takes the control because inputs and selects share no trait for it,
/// the control arrives through onmounted as the element the page handed over
pub fn report<E: JsCast + 'static>(
	control: Signal<Option<DomElement>>,
	name: &'static str,
	error: ReadSignal<Option<FormError>>,
	set: fn(&E, &str),
) {
	let message =
		use_memo(move || error.read().as_ref().filter(|error| error.name == name).map(|error| error.message.clone()));

	use_effect(move || {
		if let Some(control) = control.read().as_ref().and_then(|control| control.dyn_ref::<E>()) {
			set(control, message.read().as_deref().unwrap_or_default());
		}
	});
}
