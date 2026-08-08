//! Hosts resolve runtime capabilities into a context and perform the output action

use crate::{
	environments::{Environment, Rendered},
	options::Options,
	render::{ColorLevel, ColorOverride, RenderContext, render_with},
};

#[cfg(not(target_arch = "wasm32"))]
mod rust;
#[cfg(not(target_arch = "wasm32"))]
pub use rust::RustHost;

/// The color decision before capability detection: either the chain resolved,
/// or nothing claims the level and the host must detect
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorDecision {
	/// The chain resolved without consulting the terminal; `None` paints nothing
	Resolved(Option<ColorLevel>),

	/// No environment variable or override claims the level:
	/// detect capabilities and finish with [`decide_detected`]
	Detect,
}

/// The one home of the color precedence: FORCE_COLOR, then NO_COLOR, then the API override
///
/// Every present FORCE_COLOR value resolves the chain the way the wider ecosystem
/// reads the variable, so a set variable never falls through to detection
///
/// That totality is what keeps detection pure: detection libraries interpret
/// FORCE_COLOR and NO_COLOR themselves, and [`ColorDecision::Detect`] is only
/// returned when neither variable is present for them to reinterpret
/// Anything beyond those two variables is deliberately delegated to detection
pub fn decide_color(forced: Option<&str>, no_color: bool, override_color: ColorOverride) -> ColorDecision {
	if let Some(forced) = forced {
		return ColorDecision::Resolved(forced_color_level(forced));
	}

	if no_color {
		return ColorDecision::Resolved(None);
	}

	match override_color {
		ColorOverride::Auto => ColorDecision::Detect,
		ColorOverride::Disabled => ColorDecision::Resolved(None),
		ColorOverride::Level(level) => ColorDecision::Resolved(Some(level)),
	}
}

/// Finishes a [`ColorDecision::Detect`]: terminals that cannot be detected still get full color
pub fn decide_detected(detected: Option<ColorLevel>) -> Option<ColorLevel> {
	Some(detected.unwrap_or(ColorLevel::TrueColor))
}

/// The level a present FORCE_COLOR value forces, total over every possible value
///
/// `true`, the empty string and `1` force basic; `false` and `0` force no color;
/// `2` and `3` force their levels; numbers above three clamp to full color the
/// way the detection libraries read them; anything else forces basic
fn forced_color_level(forced: &str) -> Option<ColorLevel> {
	match forced {
		"false" => None,
		"true" | "" => Some(ColorLevel::Basic),
		value => match value.parse::<u64>() {
			Ok(0) => None,
			Ok(1) => Some(ColorLevel::Basic),
			Ok(2) => Some(ColorLevel::Ansi256),
			Ok(_) => Some(ColorLevel::TrueColor),
			Err(_) => Some(ColorLevel::Basic),
		},
	}
}

/// Resolves runtime capabilities and performs host-specific output
///
/// A host may use different environments for returned and emitted artifacts
/// Browser hosts use this distinction to return HTML from `render` while
/// emitting browser-console output from `say`
pub trait Host {
	/// Environment used when returning an artifact
	type RenderEnvironment: Environment + ?Sized;

	/// Environment used when performing the host's output action
	type SayEnvironment: Environment + ?Sized;

	/// Error returned by the host's output action
	type Error;

	/// Returns the environment used by [`render`](Self::render)
	fn render_environment(&self) -> &Self::RenderEnvironment;

	/// Returns the environment used by [`say`](Self::say)
	fn say_environment(&self) -> &Self::SayEnvironment;

	/// Resolves host capabilities once for one render operation
	fn resolve_context(&self) -> RenderContext;

	/// Performs the host-specific output action
	fn write(&self, rendered: &Rendered) -> Result<(), Self::Error>;

	/// Resolves context once and returns one rendered artifact
	#[must_use]
	fn render(&self, options: &Options) -> Rendered {
		let context = self.resolve_context();

		render_with(options, self.render_environment(), context)
	}

	/// Resolves context once, renders through the say environment and writes once
	fn say(&self, options: &Options) -> Result<(), Self::Error> {
		let context = self.resolve_context();
		let rendered = render_with(options, self.say_environment(), context);

		self.write(&rendered)
	}
}

#[cfg(test)]
mod tests {
	use std::{
		cell::{Cell, RefCell},
		convert::Infallible,
	};

	use super::{ColorDecision, Host, decide_color, decide_detected};
	use crate::{ColorLevel, ColorOverride, Environment, Options, RenderContext, Rendered};

	struct SpyEnvironment {
		marker: &'static str,
		render_calls: Cell<usize>,
	}

	impl SpyEnvironment {
		fn new(marker: &'static str) -> Self {
			Self {
				marker,
				render_calls: Cell::new(0),
			}
		}
	}

	impl Environment for SpyEnvironment {
		fn wrapper_start(&self, _options: &Options, out: &mut Rendered) {
			self.render_calls.set(self.render_calls.get() + 1);

			out.text.push_str(self.marker);
		}
	}

	struct SpyHost {
		render_environment: SpyEnvironment,
		say_environment: SpyEnvironment,
		context_resolutions: Cell<usize>,
		write_calls: Cell<usize>,
		written: RefCell<String>,
	}

	impl Default for SpyHost {
		fn default() -> Self {
			Self {
				render_environment: SpyEnvironment::new("render"),
				say_environment: SpyEnvironment::new("say"),
				context_resolutions: Cell::new(0),
				write_calls: Cell::new(0),
				written: RefCell::new(String::new()),
			}
		}
	}

	impl Host for SpyHost {
		type RenderEnvironment = SpyEnvironment;
		type SayEnvironment = SpyEnvironment;
		type Error = Infallible;

		fn render_environment(&self) -> &Self::RenderEnvironment {
			&self.render_environment
		}

		fn say_environment(&self) -> &Self::SayEnvironment {
			&self.say_environment
		}

		fn resolve_context(&self) -> RenderContext {
			self.context_resolutions.set(self.context_resolutions.get() + 1);

			RenderContext::unlimited()
		}

		fn write(&self, rendered: &Rendered) -> Result<(), Self::Error> {
			self.write_calls.set(self.write_calls.get() + 1);
			self.written.replace(rendered.text.clone());

			Ok(())
		}
	}

	#[test]
	fn render_resolves_once_and_uses_only_the_render_environment() {
		let host = SpyHost::default();

		let rendered = Host::render(&host, &Options::default());

		assert_eq!(rendered.text, "render");
		assert_eq!(host.context_resolutions.get(), 1);
		assert_eq!(host.render_environment.render_calls.get(), 1,);
		assert_eq!(host.say_environment.render_calls.get(), 0,);
		assert_eq!(host.write_calls.get(), 0);
		assert!(host.written.borrow().is_empty());
	}

	#[test]
	fn say_resolves_once_renders_once_and_writes_once() {
		let host = SpyHost::default();

		Host::say(&host, &Options::default()).expect("the spy host cannot fail");

		assert_eq!(host.context_resolutions.get(), 1);
		assert_eq!(host.render_environment.render_calls.get(), 0,);
		assert_eq!(host.say_environment.render_calls.get(), 1,);
		assert_eq!(host.write_calls.get(), 1);
		assert_eq!(host.written.borrow().as_str(), "say",);
	}

	// decide_color

	#[test]
	fn every_present_force_color_value_resolves_without_detection() {
		for (value, resolved) in [
			("0", None),
			("false", None),
			("1", Some(ColorLevel::Basic)),
			("true", Some(ColorLevel::Basic)),
			("", Some(ColorLevel::Basic)),
			("2", Some(ColorLevel::Ansi256)),
			("3", Some(ColorLevel::TrueColor)),
			("4", Some(ColorLevel::TrueColor)),
			("04", Some(ColorLevel::TrueColor)),
			("+5", Some(ColorLevel::TrueColor)),
			("junk", Some(ColorLevel::Basic)),
			("-1", Some(ColorLevel::Basic)),
			("TRUE", Some(ColorLevel::Basic)),
		] {
			assert_eq!(decide_color(Some(value), false, ColorOverride::Auto), ColorDecision::Resolved(resolved), "{value:?}");
			// a present FORCE_COLOR also beats NO_COLOR and any override
			assert_eq!(
				decide_color(Some(value), true, ColorOverride::Disabled),
				ColorDecision::Resolved(resolved),
				"{value:?} with NO_COLOR and a disabled override"
			);
		}
	}

	#[test]
	fn no_color_beats_the_override_and_only_auto_detects() {
		assert_eq!(decide_color(None, true, ColorOverride::Level(ColorLevel::TrueColor)), ColorDecision::Resolved(None));
		assert_eq!(decide_color(None, false, ColorOverride::Disabled), ColorDecision::Resolved(None));
		assert_eq!(
			decide_color(None, false, ColorOverride::Level(ColorLevel::Ansi256)),
			ColorDecision::Resolved(Some(ColorLevel::Ansi256))
		);
		assert_eq!(decide_color(None, false, ColorOverride::Auto), ColorDecision::Detect);
	}

	#[test]
	fn undetectable_terminals_get_full_color() {
		assert_eq!(decide_detected(None), Some(ColorLevel::TrueColor));
		assert_eq!(decide_detected(Some(ColorLevel::Basic)), Some(ColorLevel::Basic));
	}
}
