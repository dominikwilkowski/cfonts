use cfonts::{CanvasWidth, Cfonts, CliEnv, Font, Options, RenderContext, RenderOverrides, render_with};

#[test]
fn render_overrides_distinguish_auto_unlimited_and_columns() {
	assert_eq!(RenderOverrides::default().canvas_width(), CanvasWidth::Auto);

	let unlimited = RenderOverrides::default().with_canvas_width(0);
	assert_eq!(unlimited.canvas_width(), CanvasWidth::Unlimited);

	let columns = RenderOverrides::default().with_canvas_width(42);
	assert!(matches!(columns.canvas_width(), CanvasWidth::Columns(width) if width.get() == 42));
}

#[test]
fn explicit_context_controls_wrapping_without_detection() {
	let options: Options = Cfonts::text("AA").font(Font::Tiny).line_height(0).spaceless().into();

	let narrow = render_with(&options, &CliEnv, RenderContext::with_canvas_width(3));
	let unlimited = render_with(&options, &CliEnv, RenderContext::unlimited());

	assert_ne!(narrow.text, unlimited.text);
}
