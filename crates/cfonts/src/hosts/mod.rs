//! Hosts answer what their runtime can show and perform the output action

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use std::{
	collections::hash_map::RandomState,
	hash::{BuildHasher, Hasher},
};

use crate::{
	color::ColorLevel,
	options::Options,
	render::RenderContext,
	targets::{Rendered, Target},
};

#[cfg(feature = "web")]
mod browser;
#[cfg(not(target_arch = "wasm32"))]
mod rust;
pub mod terminal_canvas_width;
pub mod terminal_color_support;
#[cfg(feature = "web")]
pub use browser::BrowserHost;
#[cfg(not(target_arch = "wasm32"))]
pub use rust::RustHost;

/// A fresh seed for candy colors, the one every host rolls when no seed override is given
///
/// The standard library seeds its hasher keys randomly per thread and steps them per instance,
/// so every hash of nothing is a new value
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) fn entropy() -> u64 {
	RandomState::new().build_hasher().finish()
}

/// A fresh seed for candy colors, the one every host rolls when no seed override is given
///
/// The browser keys the standard library's hasher from memory addresses, identical on every
/// page load, so the seed asks the page instead: two draws of `Math.random`, one per half
///
/// The browser host, the widget and the wasm package behind the npm hosts all roll here
#[cfg(all(target_family = "wasm", target_os = "unknown", any(feature = "web", feature = "ratatui")))]
pub(crate) fn entropy() -> u64 {
	let half = || (js_sys::Math::random() * f64::from(u32::MAX)) as u64;

	(half() << 32) | half()
}

/// Answers where a render runs and how its output leaves the program
///
/// A host answers three questions, the canvas width, the color level and the candy seed,
/// and owns the one write, the target answers the format, the caller combines them:
/// every pair of host and target is legal
pub trait Host {
	/// Error returned by the host's output action
	type Error;

	/// The width in columns the render wraps at, None for no limit
	fn canvas_width(&self) -> Option<usize>;

	/// The color support the render paints with, None paints nothing
	fn color_level(&self) -> Option<ColorLevel>;

	/// The seed that makes candy colors reproducible
	fn seed(&self) -> u64;

	/// Performs the host-specific output action, `line_end` is what the target ends the artifact with
	fn write(&self, rendered: &Rendered, line_end: &str) -> Result<(), Self::Error>;

	/// Answers the three questions once and returns one rendered artifact
	#[must_use]
	fn render<T: Target + ?Sized>(&self, target: &T, options: &Options) -> Rendered {
		RenderContext::resolved(self.canvas_width(), self.color_level(), self.seed()).render(options, target)
	}

	/// Renders once and writes once
	fn say<T: Target + ?Sized>(&self, target: &T, options: &Options) -> Result<(), Self::Error> {
		let rendered = self.render(target, options);

		self.write(&rendered, target.line_end())
	}
}

#[cfg(test)]
mod tests {
	use std::{
		cell::{Cell, RefCell},
		convert::Infallible,
	};

	use super::Host;
	use crate::{Cfonts, ColorLevel, Font, Options, Rendered, Target};

	struct SpyTarget {
		marker: &'static str,
		render_calls: Cell<usize>,
	}

	impl Target for SpyTarget {
		fn wrapper_start(&self, _options: &Options, _banded: bool, out: &mut Rendered) {
			self.render_calls.set(self.render_calls.get() + 1);

			out.text.push_str(self.marker);
		}

		fn line_end(&self) -> &'static str {
			"|end"
		}
	}

	#[derive(Default)]
	struct SpyHost {
		answers: Cell<usize>,
		write_calls: Cell<usize>,
		written: RefCell<String>,
	}

	impl Host for SpyHost {
		type Error = Infallible;

		fn canvas_width(&self) -> Option<usize> {
			self.answers.set(self.answers.get() + 1);

			Some(3)
		}

		fn color_level(&self) -> Option<ColorLevel> {
			self.answers.set(self.answers.get() + 1);

			Some(ColorLevel::TrueColor)
		}

		fn seed(&self) -> u64 {
			self.answers.set(self.answers.get() + 1);

			42
		}

		fn write(&self, rendered: &Rendered, line_end: &str) -> Result<(), Self::Error> {
			self.write_calls.set(self.write_calls.get() + 1);
			self.written.replace(format!("{}{line_end}", rendered.text));

			Ok(())
		}
	}

	#[test]
	fn render_answers_each_question_once_through_the_given_target() {
		let host = SpyHost::default();
		let target = SpyTarget { marker: "render", render_calls: Cell::new(0) };

		let rendered = host.render(&target, &Options::default());

		assert_eq!(rendered.text, "render");
		assert_eq!(host.answers.get(), 3);
		assert_eq!(target.render_calls.get(), 1);
		assert_eq!(host.write_calls.get(), 0);
		assert!(host.written.borrow().is_empty());
	}

	#[test]
	fn say_renders_once_and_writes_once_with_the_targets_line_end() {
		let host = SpyHost::default();
		let target = SpyTarget { marker: "say", render_calls: Cell::new(0) };

		host.say(&target, &Options::default()).expect("the spy host cannot fail");

		assert_eq!(host.answers.get(), 3);
		assert_eq!(target.render_calls.get(), 1);
		assert_eq!(host.write_calls.get(), 1);
		assert_eq!(host.written.borrow().as_str(), "say|end");
	}

	#[test]
	fn the_answers_reach_the_render() {
		// three columns hold one Tiny glyph, so the pair wraps where an unlimited canvas keeps it on one line
		let host = SpyHost::default();
		let options: Options = Cfonts::text("AA").font(Font::Tiny).line_height(0).spaceless().into();
		let target = SpyTarget { marker: "", render_calls: Cell::new(0) };

		let rendered = host.render(&target, &options);

		assert_eq!(rendered.text.lines().count(), 4);
	}
}
