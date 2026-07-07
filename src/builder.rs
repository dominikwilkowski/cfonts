use std::{marker::PhantomData, num::NonZeroUsize};

use crate::{
	environments::{Env, Rendered},
	fonts::Font,
	options::{Align, BlockOptions, Options, Valign},
};

#[doc(hidden)]
pub struct Unset;
#[doc(hidden)]
pub struct Set;

pub struct Cfonts<
	EnvState = Unset,
	AlignState = Unset,
	ValignState = Unset,
	SpacelessState = Unset,
	MaxLengthState = Unset,
> {
	options: Options,
	_state: PhantomData<(EnvState, AlignState, ValignState, SpacelessState, MaxLengthState)>,
}

impl Cfonts {
	/// Builds one text block and normalizes text to the supported uppercase glyph set
	fn new_block(input: impl Into<String>) -> BlockOptions {
		let mut text = input.into();
		text.make_ascii_uppercase();

		BlockOptions {
			text,
			..Default::default()
		}
	}

	/// Starts a new cfonts composition with the first text block
	///
	/// This is the entry point for the builder API
	/// Global setters such as [`align`](Self::align) and [`env`](Self::env) configure the whole composition,
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
	/// assert_eq!(options.blocks[0].text, "HELLO");
	/// assert_eq!(options.blocks[0].font, Font::Tiny);
	/// ```
	pub fn text(input: impl Into<String>) -> Self {
		Self {
			options: Options {
				blocks: vec![Self::new_block(input)],
				..Default::default()
			},
			_state: PhantomData,
		}
	}

	/// Renders hand-built or tweaked [`Options`] without going through the builder
	///
	/// ```
	/// use cfonts::{Cfonts, Env, Font, Options};
	///
	/// let mut options: Options = Cfonts::text("A").font(Font::Tiny).env(Env::Browser).into();
	/// options.spaceless = true;
	///
	/// let rendered = Cfonts::render_from(&options);
	/// assert!(rendered.text.contains("▄▀█"));
	/// ```
	pub fn render_from(options: &Options) -> Rendered {
		options.env.get_env().render_from(options)
	}

	/// Renders hand-built or tweaked [`Options`] and performs the environment's output action
	///
	/// ```no_run
	/// use cfonts::{Cfonts, Options};
	///
	/// let options = Options::default();
	/// Cfonts::say_from(&options);
	/// ```
	pub fn say_from(options: &Options) {
		options.env.get_env().say_from(options);
	}
}

impl<EnvState, AlignState, ValignState, SpacelessState, MaxLengthState>
	Cfonts<EnvState, AlignState, ValignState, SpacelessState, MaxLengthState>
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
		self.options.blocks.push(Cfonts::new_block(input));
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

	// 	/// Sets the color palette for the current text block
	// 	///
	// 	/// ```
	// 	/// use cfonts::{Cfonts, Colors, Options};
	// 	///
	// 	/// let options: Options = Cfonts::text("hello")
	// 	///     .colors(vec![Colors::Red])
	// 	///     .into();
	// 	///
	// 	/// assert_eq!(options.blocks[0].colors, vec![Colors::Red]);
	// 	/// ```
	// 	// TODO: add color setter
	//
	// 	/// Sets the background color for the current text block
	// 	///
	// 	/// ```
	// 	/// use cfonts::{Cfonts, Colors, Options};
	// 	///
	// 	/// let options: Options = Cfonts::text("hello")
	// 	///     .background(Colors::Blue)
	// 	///     .into();
	// 	///
	// 	/// assert_eq!(options.blocks[0].background, Some(Colors::Blue));
	// 	/// ```
	// 	// TODO: add background setter
	//
	// 	/// Enables gradient coloring for the current text block
	// 	///
	// 	/// ```
	// 	/// use cfonts::{Cfonts, Colors, Options};
	// 	///
	// 	/// let options: Options = Cfonts::text("hello")
	// 	///     .gradient(vec![Colors::Red, Colors::Blue])
	// 	///     .into();
	// 	///
	// 	/// assert_eq!(options.blocks[0].gradient, vec![Colors::Red, Colors::Blue]);
	// 	/// ```
	// 	// TODO: add gradient setter
	//
	// 	/// Controls whether the current block's gradient is calculated independently
	// 	///
	// 	/// ```
	// 	/// use cfonts::{Cfonts, Options};
	// 	///
	// 	/// let options: Options = Cfonts::text("hello")
	// 	///     .independent_gradient()
	// 	///     .into();
	// 	///
	// 	/// assert!(options.blocks[0].independent_gradient);
	// 	/// ```
	// 	// TODO: add independent_gradient setter
	//
	// 	/// Controls whether the current block transitions its gradient into the next block
	// 	///
	// 	/// ```
	// 	/// use cfonts::{Cfonts, Options};
	// 	///
	// 	/// let options: Options = Cfonts::text("hello")
	// 	///     .transition_gradient()
	// 	///     .into();
	// 	///
	// 	/// assert!(options.blocks[0].transition_gradient);
	// 	/// ```
	// 	// TODO: add transition_gradient setter
	//
	// 	/// Sets how many blank rows are inserted after each rendered line from the current block
	// 	///
	// 	/// ```
	// 	/// use cfonts::{Cfonts, Options};
	// 	///
	// 	/// let options: Options = Cfonts::text("hello")
	// 	///     .line_height(2)
	// 	///     .into();
	// 	///
	// 	/// assert_eq!(options.blocks[0].line_height, 2);
	// 	/// ```
	// 	// TODO: add line_height setter

	/// Renders the composition into the selected environment's output format
	///
	/// This returns a [`Rendered`] value and does not print anything
	/// Use [`say`](Self::say) when you want to perform the environment's output action
	///
	/// ```
	/// use cfonts::{Cfonts, Env, Font};
	///
	/// let rendered = Cfonts::text("A")
	///     .font(Font::Tiny)
	///     .env(Env::Browser)
	///     .render();
	///
	/// assert!(rendered.text.contains("▄▀█"));
	/// ```
	pub fn render(&self) -> Rendered {
		self.options.env.get_env().render_from(&self.options)
	}

	/// Renders the composition and performs the selected environment's output action
	///
	/// For the CLI environment this prints the rendered output to stdout
	/// Use [`render`](Self::render) when you want to receive the rendered value instead
	///
	/// ```no_run
	/// use cfonts::{Cfonts, Font};
	///
	/// Cfonts::text("hello")
	///     .font(Font::Block)
	///     .say();
	/// ```
	pub fn say(&self) {
		self.options.env.get_env().say_from(&self.options);
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
impl<EnvState, ValignState, SpacelessState, MaxLengthState> CanSetAlign
	for Cfonts<EnvState, Unset, ValignState, SpacelessState, MaxLengthState>
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
impl<EnvState, AlignState, SpacelessState, MaxLengthState> CanSetValign
	for Cfonts<EnvState, AlignState, Unset, SpacelessState, MaxLengthState>
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
impl<EnvState, AlignState, ValignState, MaxLengthState> CanSetSpaceless
	for Cfonts<EnvState, AlignState, ValignState, Unset, MaxLengthState>
{
}

// ENV

#[doc(hidden)]
#[diagnostic::on_unimplemented(
	message = "`env()` has already been set",
	label = "this global setting is already configured",
	note = "Each global setting may be set once per render."
)]
pub trait CanSetEnv {}
impl<AlignState, ValignState, SpacelessState, MaxLengthState> CanSetEnv
	for Cfonts<Unset, AlignState, ValignState, SpacelessState, MaxLengthState>
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
impl<EnvState, AlignState, ValignState, SpacelessState> CanSetMaxLength
	for Cfonts<EnvState, AlignState, ValignState, SpacelessState, Unset>
{
}

impl<EnvState, AlignState, ValignState, SpacelessState, MaxLengthState>
	Cfonts<EnvState, AlignState, ValignState, SpacelessState, MaxLengthState>
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
	pub fn align(self, align: Align) -> Cfonts<EnvState, Set, ValignState, SpacelessState, MaxLengthState>
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
	pub fn valign(self, valign: Valign) -> Cfonts<EnvState, AlignState, Set, SpacelessState, MaxLengthState>
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
	pub fn spaceless(self) -> Cfonts<EnvState, AlignState, ValignState, Set, MaxLengthState>
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

	/// Sets the output environment used for rendering and output
	/// *This is a global setting and may only be configured once*
	///
	/// ```
	/// use cfonts::{Cfonts, Env};
	///
	/// let _banner = Cfonts::text("hello")
	///     .env(Env::Browser);
	/// ```
	///
	/// ```compile_fail
	/// use cfonts::{Cfonts, Env};
	///
	/// let _banner = Cfonts::text("hello")
	///     .env(Env::Cli)
	///     .env(Env::Browser); // compiler error
	/// ```
	pub fn env(self, env: Env) -> Cfonts<Set, AlignState, ValignState, SpacelessState, MaxLengthState>
	where
		Self: CanSetEnv,
	{
		let mut options = self.options;
		options.env = env;

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
	pub fn max_length(self, max_length: usize) -> Cfonts<EnvState, AlignState, ValignState, SpacelessState, Set>
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
}

/// Converts a [`Cfonts`] builder into the underlying [`Options`]
///
/// This is useful when you want the ergonomic builder API for setup,
/// but still want to inspect or tweak the final options object yourself;
/// render it afterwards with [`Cfonts::render_from`] or [`Cfonts::say_from`]
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
/// assert_eq!(options.blocks[0].text, "HELLO");
/// assert_eq!(options.blocks[0].font, Font::Block);
/// assert_eq!(options.blocks[1].text, "WORLD");
/// assert_eq!(options.blocks[1].font, Font::Font3D);
/// ```
impl<EnvState, AlignState, ValignState, SpacelessState, MaxLengthState>
	From<Cfonts<EnvState, AlignState, ValignState, SpacelessState, MaxLengthState>> for Options
{
	fn from(builder: Cfonts<EnvState, AlignState, ValignState, SpacelessState, MaxLengthState>) -> Self {
		builder.options
	}
}
