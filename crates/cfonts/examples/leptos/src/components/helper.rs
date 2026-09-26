use cfonts::{ColorLevel, RenderContext};
use leptos::{html::ElementType, prelude::*, wasm_bindgen::JsCast};

/// The page is an eighty column terminal, so long text wraps the way it would there
///
/// The one place that knows the width, the canvas and the console print share it
pub fn context() -> RenderContext {
	RenderContext::with_canvas_width(80).with_color_level(Some(ColorLevel::TrueColor))
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
/// The setter takes the control because inputs and selects share no trait for it
pub fn report<E>(control: NodeRef<E>, name: &'static str, error: Signal<Option<FormError>>, set: fn(&E::Output, &str))
where
	E: ElementType + 'static,
	E::Output: JsCast + Clone + 'static,
{
	let message = Memo::new(move |_| {
		error.with(|error| error.as_ref().filter(|error| error.name == name).map(|error| error.message.clone()))
	});

	Effect::new(move |_| {
		if let Some(control) = control.get() {
			set(&control, message.get().as_deref().unwrap_or_default());
		}
	});
}
