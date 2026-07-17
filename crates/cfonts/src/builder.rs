use std::{marker::PhantomData, num::NonZeroUsize};

use crate::{
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

pub struct Cfonts<AlignState = Unset, ValignState = Unset, SpacelessState = Unset, MaxLengthState = Unset> {
	options: Options,
	_state: PhantomData<(AlignState, ValignState, SpacelessState, MaxLengthState)>,
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
}

impl<AlignState, ValignState, SpacelessState, MaxLengthState>
	Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState>
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
impl<ValignState, SpacelessState, MaxLengthState> CanSetAlign
	for Cfonts<Unset, ValignState, SpacelessState, MaxLengthState>
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
impl<AlignState, SpacelessState, MaxLengthState> CanSetValign
	for Cfonts<AlignState, Unset, SpacelessState, MaxLengthState>
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
impl<AlignState, ValignState, MaxLengthState> CanSetSpaceless
	for Cfonts<AlignState, ValignState, Unset, MaxLengthState>
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
impl<AlignState, ValignState, SpacelessState> CanSetMaxLength
	for Cfonts<AlignState, ValignState, SpacelessState, Unset>
{
}

impl<AlignState, ValignState, SpacelessState, MaxLengthState>
	Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState>
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
	pub fn align(self, align: Align) -> Cfonts<Set, ValignState, SpacelessState, MaxLengthState>
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
	pub fn valign(self, valign: Valign) -> Cfonts<AlignState, Set, SpacelessState, MaxLengthState>
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
	pub fn spaceless(self) -> Cfonts<AlignState, ValignState, Set, MaxLengthState>
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
	pub fn max_length(self, max_length: usize) -> Cfonts<AlignState, ValignState, SpacelessState, Set>
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
/// This is useful when you want the ergonomic builder API for setup while retaining access to the underlying options
///
/// Pass the resulting options to [`render_with`](crate::render_with) or to a custom [`Host`](crate::Host)
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
impl<AlignState, ValignState, SpacelessState, MaxLengthState>
	From<Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState>> for Options
{
	fn from(builder: Cfonts<AlignState, ValignState, SpacelessState, MaxLengthState>) -> Self {
		builder.options
	}
}
