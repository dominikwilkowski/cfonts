use std::{marker::PhantomData, num::NonZeroUsize};

use crate::{
	color::ColorOption,
	environments::{Environment, Rendered},
	fonts::Font,
	hosts::Host,
	options::{Align, BlockOptions, Options, Valign},
	render::RenderContext,
};

#[doc(hidden)]
pub struct Unset;
#[doc(hidden)]
pub struct Set;

/// A fluent cfonts composition builder
///
/// The typestate parameters record which global settings are already configured,
/// so setting one twice fails at compile time; per-block setters stay repeatable
pub struct Cfonts<
	AlignState = Unset,
	ValignState = Unset,
	SpacelessState = Unset,
	MaxLengthState = Unset,
	GlobalColorState = Unset,
> {
	options: Options,
	_state: PhantomData<(AlignState, ValignState, SpacelessState, MaxLengthState, GlobalColorState)>,
}

impl Cfonts {
	/// Starts a new cfonts composition with the first text block
	///
	/// This is the entry point for the builder API
	/// Global setters such as [`align`](Self::align) and [`valign`](Self::valign) configure the whole composition,
	/// while per-block setters such as [`font`](Self::font) configure the current text block
	///
	/// ```
	/// use cfonts::{Cfonts, Font, Options};
	///
	/// let options: Options = Cfonts::text("hello")
	///     .font(Font::Tiny)
	///     .into();
	///
	/// assert_eq!(options.blocks.len(), 1);
	/// assert_eq!(options.blocks[0].text(), "HELLO");
	/// assert_eq!(options.blocks[0].font, Font::Tiny);
	/// ```
	pub fn text(input: impl Into<String>) -> Self {
		Self {
			options: Options {
				blocks: vec![BlockOptions::new(input)],
				..Default::default()
			},
			_state: PhantomData,
		}
	}
}

impl<AlignState, ValignState, SpacelessState, MaxLengthState, GlobalColorState>
	Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState, GlobalColorState>
{
	/// Returns the current block targeted by per-block setters
	///
	/// `Cfonts::text` always creates the first block, so this should only panic if a constructor bypasses that invariant
	fn current_block_mut(&mut self) -> &mut BlockOptions {
		self.options.blocks.last_mut().expect("Cfonts::text always creates one block")
	}

	/// Starts a new text block in the same composition
	///
	/// Subsequent per-block setters, such as [`font`](Self::font) and
	/// [`letter_spacing`](Self::letter_spacing), apply to this new block
	///
	/// ```
	/// use cfonts::{Cfonts, Font, Options};
	///
	/// let options: Options = Cfonts::text("hello")
	///     .font(Font::Block)
	///     .new_text("world")
	///     .font(Font::Font3D)
	///     .into();
	///
	/// assert_eq!(options.blocks.len(), 2);
	/// assert_eq!(options.blocks[0].font, Font::Block);
	/// assert_eq!(options.blocks[1].font, Font::Font3D);
	/// ```
	pub fn new_text(mut self, input: impl Into<String>) -> Self {
		self.options.blocks.push(BlockOptions::new(input));
		self
	}

	/// Sets the font for the current text block
	///
	/// ```
	/// use cfonts::{Cfonts, Font, Options};
	///
	/// let options: Options = Cfonts::text("hello")
	///     .font(Font::Tiny)
	///     .into();
	///
	/// assert_eq!(options.blocks[0].font, Font::Tiny);
	/// ```
	pub fn font(mut self, font: Font) -> Self {
		self.current_block_mut().font = font;
		self
	}

	/// Sets how many font-defined letter-space glyphs are inserted between glyphs in the current block
	///
	/// ```
	/// use cfonts::{Cfonts, Options};
	///
	/// let options: Options = Cfonts::text("hello")
	///     .letter_spacing(2)
	///     .into();
	///
	/// assert_eq!(options.blocks[0].letter_spacing, 2);
	/// ```
	pub fn letter_spacing(mut self, letter_spacing: usize) -> Self {
		self.current_block_mut().letter_spacing = letter_spacing;
		self
	}

	/// Enables word-aware wrapping for the current text block
	///
	/// ```
	/// use cfonts::{Cfonts, Options};
	///
	/// let options: Options = Cfonts::text("hello world")
	///     .word_wrap()
	///     .into();
	///
	/// assert!(options.blocks[0].word_wrap);
	/// ```
	pub fn word_wrap(mut self) -> Self {
		self.current_block_mut().word_wrap = true;
		self
	}

	/// Sets the colors for the current text block
	///
	/// Accepts a list of [`Color`](crate::Color)s, one per font color slot, a [`GradientOption`](crate::GradientOption),
	/// or a [`GradientPreset`](crate::GradientPreset)
	/// Any configured value, including an empty color list, overrides the global color for this block
	///
	/// ```
	/// use cfonts::{Cfonts, Color, ColorOption, Options};
	///
	/// let options: Options = Cfonts::text("hello")
	///     .color(vec![Color::Red, Color::System])
	///     .into();
	///
	/// assert_eq!(options.blocks[0].color, Some(ColorOption::Colors(vec![Color::Red, Color::System])));
	/// ```
	///
	/// ```
	/// use cfonts::{Cfonts, GradientOption, GradientPreset, GradientStop};
	///
	/// let _ramped = Cfonts::text("hello").color(GradientOption::TwoStop {
	///     start: GradientStop::Red,
	///     end: GradientStop::Blue,
	///     independent_gradient: false,
	/// });
	///
	/// let _preset = Cfonts::text("hello").color(GradientPreset::Pride);
	/// ```
	pub fn color(mut self, color: impl Into<ColorOption>) -> Self {
		self.current_block_mut().color = Some(color.into());
		self
	}

	/// Sets how many blank rows are inserted after each rendered line from the current block
	///
	/// ```
	/// use cfonts::{Cfonts, Options};
	///
	/// let options: Options = Cfonts::text("hello")
	///     .line_height(2)
	///     .into();
	///
	/// assert_eq!(options.blocks[0].line_height, 2);
	/// ```
	pub fn line_height(mut self, line_height: usize) -> Self {
		self.current_block_mut().line_height = line_height;
		self
	}

	/// Renders through an explicit environment and resolved context
	///
	/// This is the low-level API for consumers that need a particular artifact
	/// without host detection or output side effects
	///
	/// ```
	/// use cfonts::{
	///     BrowserEnv, Cfonts, Font, RenderContext,
	/// };
	///
	/// let rendered = Cfonts::text("A")
	///     .font(Font::Tiny)
	///     .render_with(
	///         &BrowserEnv,
	///         RenderContext::unlimited(),
	///     );
	///
	/// assert!(rendered.text.contains("▄▀█"));
	/// ```
	pub fn render_with<E: Environment + ?Sized>(&self, environment: &E, context: RenderContext) -> Rendered {
		crate::render_with(&self.options, environment, context)
	}

	/// Renders the composition through a host
	///
	/// The host resolves runtime capabilities and selects its render environment
	/// This returns a [`Rendered`] value without performing the host's output action
	///
	/// ```
	/// use cfonts::{
	///     Cfonts, Font, RenderOverrides, RustHost,
	/// };
	///
	/// let host = RustHost::from_overrides(
	///     RenderOverrides::default()
	///         .with_canvas_width(0),
	/// );
	///
	/// let rendered = Cfonts::text("A")
	///     .font(Font::Tiny)
	///     .render(&host);
	///
	/// assert!(rendered.text.contains("▄▀█"));
	/// ```
	pub fn render<H: Host + ?Sized>(&self, host: &H) -> Rendered {
		host.render(&self.options)
	}

	/// Renders the composition and performs the host's output action
	///
	/// For [`RustHost`](crate::RustHost), this writes terminal output to stdout
	/// Use [`render`](Self::render) when you need the artifact without writing it
	///
	/// ```no_run
	/// use cfonts::{Cfonts, Font, RustHost};
	///
	/// Cfonts::text("hello")
	///     .font(Font::Block)
	///     .say(&RustHost::default())
	///     .expect("stdout should be writable");
	/// ```
	pub fn say<H: Host + ?Sized>(&self, host: &H) -> Result<(), H::Error> {
		host.say(&self.options)
	}
}

// GLOBAL OPTIONS
// ALIGN

#[doc(hidden)]
#[diagnostic::on_unimplemented(
	message = "`align()` has already been set",
	label = "this global setting is already configured",
	note = "Each global setting may be set once per render."
)]
pub trait CanSetAlign {}
impl<ValignState, SpacelessState, MaxLengthState, GlobalColorState> CanSetAlign
	for Cfonts<Unset, ValignState, SpacelessState, MaxLengthState, GlobalColorState>
{
}

// VALIGN

#[doc(hidden)]
#[diagnostic::on_unimplemented(
	message = "`valign()` has already been set",
	label = "this global setting is already configured",
	note = "Each global setting may be set once per render."
)]
pub trait CanSetValign {}
impl<AlignState, SpacelessState, MaxLengthState, GlobalColorState> CanSetValign
	for Cfonts<AlignState, Unset, SpacelessState, MaxLengthState, GlobalColorState>
{
}

// SPACELESS

#[doc(hidden)]
#[diagnostic::on_unimplemented(
	message = "`spaceless()` has already been set",
	label = "this global setting is already configured",
	note = "Each global setting may be set once per render."
)]
pub trait CanSetSpaceless {}
impl<AlignState, ValignState, MaxLengthState, GlobalColorState> CanSetSpaceless
	for Cfonts<AlignState, ValignState, Unset, MaxLengthState, GlobalColorState>
{
}

// MAX_LENGTH

#[doc(hidden)]
#[diagnostic::on_unimplemented(
	message = "`max_length()` has already been set",
	label = "this global setting is already configured",
	note = "Each global setting may be set once per render."
)]
pub trait CanSetMaxLength {}
impl<AlignState, ValignState, SpacelessState, GlobalColorState> CanSetMaxLength
	for Cfonts<AlignState, ValignState, SpacelessState, Unset, GlobalColorState>
{
}

// GLOBAL_COLOR

#[doc(hidden)]
#[diagnostic::on_unimplemented(
	message = "`global_color()` has already been set",
	label = "this global setting is already configured",
	note = "Each global setting may be set once per render."
)]
pub trait CanSetGlobalColor {}
impl<AlignState, ValignState, SpacelessState, MaxLengthState> CanSetGlobalColor
	for Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState, Unset>
{
}

impl<AlignState, ValignState, SpacelessState, MaxLengthState, GlobalColorState>
	Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState, GlobalColorState>
{
	/// Sets the horizontal alignment for the whole rendered composition
	/// *This is a global setting and may only be configured once*
	///
	/// ```
	/// use cfonts::{Align, Cfonts};
	///
	/// let _banner = Cfonts::text("hello")
	///     .align(Align::Center);
	/// ```
	///
	/// ```compile_fail
	/// use cfonts::{Align, Cfonts};
	///
	/// let _banner = Cfonts::text("hello")
	///     .align(Align::Left)
	///     .align(Align::Right); // compiler error
	/// ```
	pub fn align(self, align: Align) -> Cfonts<Set, ValignState, SpacelessState, MaxLengthState, GlobalColorState>
	where
		Self: CanSetAlign,
	{
		let mut options = self.options;
		options.align = align;

		Cfonts {
			options,
			_state: PhantomData,
		}
	}

	/// Sets the vertical alignment used when blocks with different font heights share a line
	/// *This is a global setting and may only be configured once*
	///
	/// ```
	/// use cfonts::{Cfonts, Valign};
	///
	/// let _banner = Cfonts::text("hello")
	///     .valign(Valign::Bottom);
	/// ```
	///
	/// ```compile_fail
	/// use cfonts::{Cfonts, Valign};
	///
	/// let _banner = Cfonts::text("hello")
	///     .valign(Valign::Top)
	///     .valign(Valign::Middle); // compiler error
	/// ```
	pub fn valign(self, valign: Valign) -> Cfonts<AlignState, Set, SpacelessState, MaxLengthState, GlobalColorState>
	where
		Self: CanSetValign,
	{
		let mut options = self.options;
		options.valign = valign;

		Cfonts {
			options,
			_state: PhantomData,
		}
	}

	/// Controls whether the environment adds its usual top and bottom padding
	/// *This is a global setting and may only be configured once*
	///
	/// ```
	/// use cfonts::Cfonts;
	///
	/// let _banner = Cfonts::text("hello")
	///     .spaceless();
	/// ```
	///
	/// ```compile_fail
	/// use cfonts::Cfonts;
	///
	/// let _banner = Cfonts::text("hello")
	///     .spaceless()
	///     .spaceless(); // compiler error
	/// ```
	pub fn spaceless(self) -> Cfonts<AlignState, ValignState, Set, MaxLengthState, GlobalColorState>
	where
		Self: CanSetSpaceless,
	{
		let mut options = self.options;
		options.spaceless = true;

		Cfonts {
			options,
			_state: PhantomData,
		}
	}

	/// Sets the maximum number of printable glyphs per rendered line
	/// Passing `0` disables the limit
	/// *This is a global setting and may only be configured once*
	///
	/// ```
	/// use cfonts::Cfonts;
	///
	/// let _banner = Cfonts::text("hello world")
	///     .max_length(10);
	/// ```
	///
	/// ```compile_fail
	/// use cfonts::Cfonts;
	///
	/// let _banner = Cfonts::text("hello world")
	///     .max_length(10)
	///     .max_length(20); // compiler error
	/// ```
	pub fn max_length(self, max_length: usize) -> Cfonts<AlignState, ValignState, SpacelessState, Set, GlobalColorState>
	where
		Self: CanSetMaxLength,
	{
		let mut options = self.options;
		options.max_length = NonZeroUsize::new(max_length);

		Cfonts {
			options,
			_state: PhantomData,
		}
	}

	/// Sets the colors or a gradient across the whole composition
	/// Blocks with their own [`color`](Self::color) override it for their columns
	/// *This is a global setting and may only be configured once*
	///
	/// ```
	/// use cfonts::{Cfonts, Color, GradientOption, GradientPreset, GradientStop};
	///
	/// let _banner = Cfonts::text("hello")
	///     .global_color(vec![Color::Red]);
	///
	/// let _ramped = Cfonts::text("hello")
	///     .global_color(GradientOption::TwoStop {
	///         start: GradientStop::Red,
	///         end: GradientStop::Blue,
	///         independent_gradient: false,
	///     });
	///
	/// let _preset = Cfonts::text("hello")
	///     .global_color(GradientPreset::Pride);
	/// ```
	///
	/// ```compile_fail
	/// use cfonts::{Cfonts, Color, GradientPreset};
	///
	/// let _banner = Cfonts::text("hello")
	///     .global_color(vec![Color::Red])
	///     .global_color(GradientPreset::Agender); // compiler error
	/// ```
	pub fn global_color(
		self,
		color: impl Into<ColorOption>,
	) -> Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState, Set>
	where
		Self: CanSetGlobalColor,
	{
		let mut options = self.options;
		options.global_color = Some(color.into());

		Cfonts {
			options,
			_state: PhantomData,
		}
	}
}

/// Converts a [`Cfonts`] builder into the underlying [`Options`]
///
/// This is useful when you want the ergonomic builder API for setup while retaining access to the underlying options
///
/// Pass the resulting options to [`render_with`](crate::render_with) or to a custom [`Host`]
///
/// ```
/// use cfonts::{Cfonts, Font, Options};
///
/// let options: Options = Cfonts::text("hello")
///     .font(Font::Block)
///     .new_text("world")
///     .font(Font::Font3D)
///     .into();
///
/// assert_eq!(options.blocks.len(), 2);
/// assert_eq!(options.blocks[0].text(), "HELLO");
/// assert_eq!(options.blocks[0].font, Font::Block);
/// assert_eq!(options.blocks[1].text(), "WORLD");
/// assert_eq!(options.blocks[1].font, Font::Font3D);
/// ```
impl<AlignState, ValignState, SpacelessState, MaxLengthState, GlobalColorState>
	From<Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState, GlobalColorState>> for Options
{
	fn from(builder: Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState, GlobalColorState>) -> Self {
		builder.options
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CliEnv, Color, GradientOption, GradientPreset, GradientStop};

	// Double-setting a global is a compile error, not a runtime panic:
	// that guarantee lives in the `compile_fail` doctests on each global setter

	// global setters

	#[test]
	fn global_setters_land_in_the_options() {
		let options: Options = Cfonts::text("hello")
			.align(Align::Center)
			.valign(Valign::Top)
			.spaceless()
			.max_length(20)
			.global_color(GradientPreset::Pride)
			.into();

		assert_eq!(options.align, Align::Center);
		assert_eq!(options.valign, Valign::Top);
		assert!(options.spaceless);
		assert_eq!(options.max_length, NonZeroUsize::new(20));
		assert_eq!(options.global_color, Some(ColorOption::from(GradientPreset::Pride)));
	}

	#[test]
	fn the_global_color_accepts_colors_gradients_and_presets() {
		let options: Options = Cfonts::text("hello").global_color(vec![Color::Red, Color::System]).into();
		assert_eq!(options.global_color, Some(ColorOption::Colors(vec![Color::Red, Color::System])));

		let two_stop = GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: true,
		};
		let options: Options = Cfonts::text("hello").global_color(two_stop.clone()).into();
		assert_eq!(options.global_color, Some(ColorOption::Gradient(two_stop)));

		let options: Options = Cfonts::text("hello").global_color(GradientPreset::Pride).into();
		assert_eq!(options.global_color, Some(ColorOption::from(GradientPreset::Pride)));
	}

	#[test]
	fn a_zero_max_length_means_unlimited() {
		let options: Options = Cfonts::text("hello").max_length(0).into();

		assert_eq!(options.max_length, None);
	}

	#[test]
	fn global_setters_are_order_independent() {
		let one: Options = Cfonts::text("A").align(Align::Right).valign(Valign::Top).into();
		let two: Options = Cfonts::text("A").valign(Valign::Top).align(Align::Right).into();

		assert_eq!(one.align, two.align);
		assert_eq!(two.valign, Valign::Top);
	}

	// local setters

	#[test]
	fn per_block_setters_target_the_current_block() {
		let options: Options =
			Cfonts::text("one").font(Font::Tiny).letter_spacing(2).new_text("two").font(Font::Block).into();

		assert_eq!(options.blocks[0].font, Font::Tiny);
		assert_eq!(options.blocks[0].letter_spacing, 2);
		assert_eq!(options.blocks[1].font, Font::Block);
		assert_eq!(options.blocks[1].letter_spacing, 1);
	}

	#[test]
	fn per_block_setters_are_repeatable() {
		let options: Options = Cfonts::text("one").font(Font::Tiny).font(Font::Block).into();

		assert_eq!(options.blocks[0].font, Font::Block);
	}

	#[test]
	fn color_targets_the_current_block() {
		let options: Options = Cfonts::text("one").color(vec![Color::Red]).new_text("two").into();

		assert_eq!(options.blocks[0].color, Some(ColorOption::Colors(vec![Color::Red])));
		assert_eq!(options.blocks[1].color, None);
	}

	#[test]
	fn an_empty_color_list_is_still_a_configured_color() {
		let options: Options = Cfonts::text("one").color(Vec::<Color>::new()).into();

		assert_eq!(options.blocks[0].color, Some(ColorOption::Colors(vec![])));
	}

	#[test]
	fn color_is_repeatable_and_takes_gradients() {
		let options: Options = Cfonts::text("one").color(vec![Color::Red]).color(GradientPreset::Pride).into();

		assert_eq!(options.blocks[0].color, Some(ColorOption::Gradient(GradientPreset::Pride.to_gradient(false))));
	}

	// test hosts

	/// A host that captures its write instead of touching stdout
	#[derive(Default)]
	struct CaptureHost {
		written: std::cell::RefCell<Vec<String>>,
	}

	impl Host for CaptureHost {
		type RenderEnvironment = CliEnv;
		type SayEnvironment = CliEnv;
		type Error = std::convert::Infallible;

		fn render_environment(&self) -> &CliEnv {
			&CliEnv
		}

		fn say_environment(&self) -> &CliEnv {
			&CliEnv
		}

		fn resolve_context(&self) -> RenderContext {
			RenderContext::unlimited()
		}

		fn write(&self, rendered: &Rendered) -> Result<(), Self::Error> {
			self.written.borrow_mut().push(rendered.text.clone());
			Ok(())
		}
	}

	/// A host whose output action always fails
	struct FailingHost;

	impl Host for FailingHost {
		type RenderEnvironment = CliEnv;
		type SayEnvironment = CliEnv;
		type Error = &'static str;

		fn render_environment(&self) -> &CliEnv {
			&CliEnv
		}

		fn say_environment(&self) -> &CliEnv {
			&CliEnv
		}

		fn resolve_context(&self) -> RenderContext {
			RenderContext::unlimited()
		}

		fn write(&self, _rendered: &Rendered) -> Result<(), Self::Error> {
			Err("the writer is broken")
		}
	}

	// render

	#[test]
	fn render_returns_the_artifact_without_writing() {
		let host = CaptureHost::default();

		let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().render(&host);

		assert_eq!(rendered.text, "▄▀█\n█▀█");
		assert!(host.written.borrow().is_empty());
	}

	#[test]
	fn render_succeeds_even_when_the_hosts_writer_is_broken() {
		// render never touches the output action, so a broken writer cannot matter
		let rendered = Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().render(&FailingHost);

		assert_eq!(rendered.text, "▄▀█\n█▀█");
	}

	// say

	#[test]
	fn say_writes_the_composition_through_the_host() {
		let host = CaptureHost::default();

		Cfonts::text("A").font(Font::Tiny).valign(Valign::Top).spaceless().say(&host).expect("CaptureHost cannot fail");

		assert_eq!(host.written.borrow().as_slice(), ["▄▀█\n█▀█"]);
	}

	#[test]
	fn say_returns_the_hosts_write_error() {
		assert_eq!(Cfonts::text("A").say(&FailingHost), Err("the writer is broken"));
	}
}
