use cfonts::{
	CanvasWidth, Cfonts, CliEnv, ColorLevel, ColorOverride, Font, Options, RenderContext, RenderOverrides, render_with,
};

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

	let narrow = render_with(&options, &CliEnv::default(), RenderContext::with_canvas_width(3));
	let unlimited = render_with(&options, &CliEnv::default(), RenderContext::unlimited());

	assert_ne!(narrow.text, unlimited.text);
}

#[test]
fn overrides_carry_color_and_seed() {
	let overrides = RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::Basic)).with_seed(7);

	assert_eq!(overrides.color(), ColorOverride::Level(ColorLevel::Basic));
	assert_eq!(overrides.seed(), Some(7));
	assert_eq!(RenderOverrides::default().color(), ColorOverride::Auto);
	assert_eq!(RenderOverrides::default().seed(), None);
}

#[test]
fn contexts_default_to_colorless_and_carry_what_they_are_given() {
	let plain = RenderContext::unlimited();
	assert_eq!(plain.color_level(), None);
	assert_eq!(plain.seed(), 0);

	let colorful = RenderContext::colored(ColorLevel::Ansi256).with_seed(42);
	assert_eq!(colorful.color_level(), Some(ColorLevel::Ansi256));
	assert_eq!(colorful.seed(), 42);
}

#[test]
fn a_color_level_without_color_options_paints_nothing() {
	// capabilities alone paint nothing: only configured colors consume the level
	let banner = Cfonts::text("HI").font(Font::Tiny);
	let plain = banner.render_with(&CliEnv::default(), RenderContext::unlimited());
	let leveled = banner.render_with(&CliEnv::default(), RenderContext::colored(ColorLevel::TrueColor).with_seed(42));

	assert_eq!(plain.text, leveled.text);
}
