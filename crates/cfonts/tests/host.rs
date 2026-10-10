use cfonts::{
	CanvasWidth, Cfonts, CliEnv, Color, ColorLevel, ColorOverride, Font, Options, RenderOverrides, render_with,
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
fn explicit_overrides_control_wrapping_without_detection() {
	let options: Options = Cfonts::text("AA").font(Font::Tiny).line_height(0).spaceless().into();

	let narrow = render_with(&options, &CliEnv::default(), RenderOverrides::default().with_canvas_width(3));
	let unlimited = render_with(&options, &CliEnv::default(), RenderOverrides::default());

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
fn without_a_host_auto_means_off() {
	// nothing detects here: no color paints and candy rolls from the zero seed, so two renders agree
	let candy = Cfonts::text("CANDY").font(Font::Tiny).colors(vec![Color::CANDY]);
	let plain = candy.render_with(&CliEnv::default(), RenderOverrides::default());
	assert!(!plain.text.contains('\u{1b}'));

	let leveled = RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor));
	assert!(candy.render_with(&CliEnv::default(), leveled).text.contains('\u{1b}'));
	assert_eq!(candy.render_with(&CliEnv::default(), leveled), candy.render_with(&CliEnv::default(), leveled));
	assert_ne!(
		candy.render_with(&CliEnv::default(), leveled),
		candy.render_with(&CliEnv::default(), leveled.with_seed(42))
	);
}

#[test]
fn a_color_level_without_color_options_paints_nothing() {
	// capabilities alone paint nothing: only configured colors consume the level
	let banner = Cfonts::text("HI").font(Font::Tiny);
	let plain = banner.render_with(&CliEnv::default(), RenderOverrides::default());
	let leveled = banner.render_with(
		&CliEnv::default(),
		RenderOverrides::default().with_color(ColorOverride::Level(ColorLevel::TrueColor)).with_seed(42),
	);

	assert_eq!(plain.text, leveled.text);
}
