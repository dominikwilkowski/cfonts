//! The two hosts of the npm package behind the boundary: the decisions run here, TypeScript gathers the facts
//! a page or a Node process alone can read and keeps the one write to its own stream or console

use std::{cell::OnceCell, convert::Infallible, num::NonZeroUsize};

use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use cfonts::{
	BrowserHost as CoreBrowserHost, ColorLevel, Environment, Host, RenderOverrides, Rendered as CoreRendered,
	hosts::{
		terminal_canvas_width::TerminalCanvasWidth,
		terminal_color_support::{TerminalColorSupport, WindowsConsole},
	},
};

use crate::{Cfonts, EnvironmentKind, Rendered, input, types::with_environment};

/// The page host of the core behind the boundary
///
/// TypeScript forwards, the decisions and the console write live here:
/// a page has no terminal to measure, so only a column count wraps, the artifact is CSS,
/// so automatic color paints at full support, candy rolls a fresh seed per render unless
/// one is pinned, and `say` spreads the styles into the page console
#[wasm_bindgen]
pub struct BrowserHost {
	inner: CoreBrowserHost,
}

#[wasm_bindgen]
impl BrowserHost {
	/// Creates the page host from the overrides JavaScript spells, or one that decides everything itself
	/// when the object is left out
	///
	/// No width leaves the decision to the host and zero means unlimited, a disabled color paints
	/// nothing and a level pins the palette, a seed makes candy reproducible
	#[wasm_bindgen(js_name = fromOverrides)]
	pub fn from_overrides(
		#[wasm_bindgen(unchecked_optional_param_type = "RenderOverrides")] overrides: JsValue,
	) -> Result<BrowserHost, JsValue> {
		let overrides = input::overrides(&overrides, "fromOverrides")?;

		Ok(Self { inner: CoreBrowserHost::from_overrides(overrides) })
	}

	/// Renders the composition into the environment's format with the host's three answers pinned
	///
	/// The artifact crosses through [`Ts`] so a serialization failure surfaces as a JavaScript error instead of a leak
	pub fn render(
		&self,
		composition: &Cfonts,
		environment: EnvironmentKind,
		raw_mode: bool,
	) -> Result<Ts<Rendered>, JsError> {
		let rendered: Rendered =
			with_environment!(environment, raw_mode, |env| self.inner.render(&env, composition.options())).into();

		Ok(rendered.into_ts()?)
	}

	/// Renders once and writes once into the page console
	pub fn say(&self, composition: &Cfonts, environment: EnvironmentKind, raw_mode: bool) {
		with_environment!(environment, raw_mode, |env| self.inner.say(&env, composition.options()))
			.expect("the page console cannot fail");
	}
}

/// One lookup of an environment variable by its name, absent where the variable is unset
///
/// JavaScript hands the resolution a function it calls per name, so a variable is read through Node's own
/// property read on `process.env` and carries the runtime's rule on every platform, case insensitive on a
/// Windows main thread and exact elsewhere, while the names the resolution asks for live in the core alone,
/// the native tests hand a map, and an exception the function throws is the `Err`
pub(crate) type Lookup = Box<dyn Fn(&str) -> Result<Option<String>, JsValue>>;

/// The terminal facts a Node process alone can read, gathered by TypeScript per render and crossed as one object
///
/// The column counts are the streams' own, absent where a stream is redirected, the environment crosses as a
/// lookup the resolution calls per name, so `FORCE_SIZE`, `FORCE_COLOR`, `NO_COLOR` and the detection variables
/// are read here, each through the runtime's own property read at the moment of the render
#[derive(Tsify)]
#[serde(rename_all = "camelCase")]
pub struct Terminal {
	#[tsify(optional)]
	pub stdout_columns: Option<u32>,
	#[tsify(optional)]
	pub stderr_columns: Option<u32>,
	pub attached: bool,
	pub platform: String,
	pub release: String,
	#[tsify(type = "(name: string) => string | undefined")]
	pub environment: Lookup,
}

impl Terminal {
	/// The measured width of the stream still attached to a terminal, stdout first, then stderr
	///
	/// A zero width stream measures nothing, matching the native probe
	fn measured(&self) -> Option<NonZeroUsize> {
		[self.stdout_columns, self.stderr_columns]
			.into_iter()
			.flatten()
			.find_map(|columns| NonZeroUsize::new(columns as usize))
	}

	/// The Windows console facts, absent off Windows
	///
	/// Node switches escape processing on at startup, so the build, the third part of the release,
	/// is the only console fact the classifier needs
	fn windows_console(&self) -> Option<WindowsConsole> {
		(self.platform == "win32").then(|| WindowsConsole {
			ansi_enabled: true,
			build: self.release.split('.').nth(2).and_then(|build| build.parse().ok()).unwrap_or(0),
		})
	}
}

/// The Node host behind the boundary: the overrides are its own, the terminal facts arrive with every render
///
/// The write is a no op here, TypeScript owns the one write to `process.stdout`
#[wasm_bindgen]
pub struct NodeHost {
	overrides: RenderOverrides,
}

#[wasm_bindgen]
impl NodeHost {
	/// Creates the Node host from the overrides JavaScript spells, or one that detects everything from the
	/// terminal facts when the object is left out
	///
	/// `FORCE_SIZE`, `FORCE_COLOR` and `NO_COLOR` still take precedence over these values
	#[wasm_bindgen(js_name = fromOverrides)]
	pub fn from_overrides(
		#[wasm_bindgen(unchecked_optional_param_type = "RenderOverrides")] overrides: JsValue,
	) -> Result<NodeHost, JsValue> {
		Ok(Self { overrides: input::overrides(&overrides, "fromOverrides")? })
	}

	/// Renders the composition into the environment's format, the three answers resolved from the terminal facts
	///
	/// The artifact crosses through [`Ts`] so a serialization failure surfaces as a JavaScript error instead of a leak
	pub fn render(
		&self,
		composition: &Cfonts,
		environment: EnvironmentKind,
		raw_mode: bool,
		#[wasm_bindgen(unchecked_param_type = "Terminal")] terminal: JsValue,
	) -> Result<Ts<Rendered>, JsValue> {
		let terminal = input::terminal(&terminal, "render")?;
		let host = TerminalHost::new(self.overrides, &terminal);
		let rendered: Rendered =
			with_environment!(environment, raw_mode, |env| host.render(&env, composition.options())).into();

		// the artifact of a render whose lookup threw rests on facts the consumer never answered,
		// so the consumer's own exception comes back in its place
		if let Some(exception) = host.thrown.into_inner() {
			return Err(exception);
		}

		Ok(rendered.into_ts().map_err(JsError::from)?)
	}
}

/// One render of the Node host: its overrides resolved against the terminal facts of this call
struct TerminalHost<'a> {
	overrides: RenderOverrides,
	terminal: &'a Terminal,
	/// The first exception the lookup threw in this render, which the render returns in place of its artifact
	thrown: OnceCell<JsValue>,
}

impl<'a> TerminalHost<'a> {
	fn new(overrides: RenderOverrides, terminal: &'a Terminal) -> Self {
		Self { overrides, terminal, thrown: OnceCell::new() }
	}

	/// The environment the resolution reads, the lookup of the terminal facts called per name
	///
	/// An exception the lookup throws is the consumer's own: the first one is kept for the render to return,
	/// and every later name reads as absent without a call, so no consumer code runs after its own failure
	fn environment(&self) -> impl Fn(&str) -> Option<String> + '_ {
		move |name: &str| {
			if self.thrown.get().is_some() {
				return None;
			}

			(self.terminal.environment)(name).unwrap_or_else(|exception| {
				self.thrown.get_or_init(|| exception);

				None
			})
		}
	}
}

impl Host for TerminalHost<'_> {
	type Error = Infallible;

	/// `FORCE_SIZE`, then the API override, then the measured width, then the eighty column fallback
	fn canvas_width(&self) -> Option<usize> {
		let environment = self.environment();

		TerminalCanvasWidth {
			measured: self.terminal.measured(),
			environment: &environment,
			override_width: self.overrides.canvas_width(),
		}
		.resolve()
		.map(NonZeroUsize::get)
	}

	/// `FORCE_COLOR`, then `NO_COLOR`, then the API override, then the cascade of an attached terminal
	///
	/// An undetectable attached terminal falls back to full color, matching the native render stream
	fn color_level(&self) -> Option<ColorLevel> {
		let environment = self.environment();

		TerminalColorSupport {
			attached: self.terminal.attached,
			environment: &environment,
			windows_console: self.terminal.windows_console(),
			override_color: self.overrides.color(),
			fallback: Some(ColorLevel::TrueColor),
		}
		.resolve()
	}

	fn seed(&self) -> u64 {
		self.overrides.seed().unwrap_or_else(CoreBrowserHost::entropy)
	}

	fn write(&self, _rendered: &CoreRendered, _line_end: &str) -> Result<(), Self::Error> {
		Ok(())
	}
}

/// What a host writes after the artifact to end it, the environment's own answer
///
/// Only the terminal ends its artifact, a page or a console ends its own output,
/// and the Node host carries this value to its write instead of deciding it
#[wasm_bindgen(js_name = lineEnd)]
pub fn line_end(environment: EnvironmentKind, raw_mode: bool) -> String {
	with_environment!(environment, raw_mode, |env| env.line_end().to_owned())
}

/// A fresh seed for candy colors, the one the hosts roll when no seed override is given
///
/// The boundary carries seeds as u32, so the core's roll is cut to that width,
/// which loses nothing a seed needs: every call still rolls a value of its own
#[wasm_bindgen]
pub fn entropy() -> u32 {
	CoreBrowserHost::entropy() as u32
}

#[cfg(test)]
mod tests {
	use std::{cell::Cell, collections::HashMap, rc::Rc};

	use super::*;
	use cfonts::ColorOverride;

	/// A darwin terminal with the given columns and variables, the variables behind a map backed lookup
	fn terminal(stdout: Option<u32>, stderr: Option<u32>, variables: &[(&str, &str)]) -> Terminal {
		let variables: HashMap<String, String> =
			variables.iter().map(|(name, value)| (String::from(*name), String::from(*value))).collect();

		Terminal {
			stdout_columns: stdout,
			stderr_columns: stderr,
			attached: true,
			platform: String::from("darwin"),
			release: String::from("25.6.0"),
			environment: Box::new(move |name| Ok(variables.get(name).cloned())),
		}
	}

	#[test]
	fn the_resolution_reads_through_the_lookup_and_an_unset_name_reads_as_absent() {
		let terminal = terminal(None, None, &[("FORCE_SIZE", "12"), ("TERM", "xterm")]);
		let host = TerminalHost::new(RenderOverrides::default(), &terminal);
		let environment = host.environment();

		assert_eq!(environment("FORCE_SIZE"), Some(String::from("12")));
		assert_eq!(environment("TERM"), Some(String::from("xterm")));
		assert_eq!(environment("NO_COLOR"), None);
		assert!(host.thrown.get().is_none());
	}

	#[test]
	fn the_first_exception_of_the_lookup_is_kept_and_no_name_is_asked_after_it() {
		let asked = Rc::new(Cell::new(0));
		let counting = Rc::clone(&asked);
		let mut terminal = terminal(Some(120), None, &[]);
		terminal.environment = Box::new(move |_| {
			counting.set(counting.get() + 1);

			Err(JsValue::NULL)
		});
		let host = TerminalHost::new(RenderOverrides::default(), &terminal);

		// the width asks for FORCE_SIZE and the color would ask for FORCE_COLOR: one call, every later name
		// reads as absent, and the render carries the exception out in place of the facts resolved this way
		assert_eq!(host.canvas_width(), Some(120));
		assert_eq!(host.color_level(), Some(ColorLevel::TrueColor));
		assert_eq!(asked.get(), 1);
		assert!(host.thrown.into_inner().is_some());
	}

	#[test]
	fn stdout_answers_before_stderr_and_a_zero_width_measures_nothing() {
		let host = |stdout, stderr| {
			let terminal = terminal(stdout, stderr, &[]);
			TerminalHost::new(RenderOverrides::default(), &terminal).canvas_width()
		};

		assert_eq!(host(Some(120), Some(13)), Some(120));
		assert_eq!(host(Some(0), Some(13)), Some(13));
		assert_eq!(host(None, Some(13)), Some(13));
		assert_eq!(host(None, None), Some(80));
	}

	#[test]
	fn the_width_resolves_force_size_then_the_override_then_the_measurement() {
		let forced = terminal(Some(120), None, &[("FORCE_SIZE", "12")]);
		let measured = terminal(Some(120), None, &[]);

		assert_eq!(TerminalHost::new(RenderOverrides::default(), &forced).canvas_width(), Some(12));
		assert_eq!(TerminalHost::new(RenderOverrides::default().with_canvas_width(42), &forced).canvas_width(), Some(12));
		assert_eq!(TerminalHost::new(RenderOverrides::default().with_canvas_width(42), &measured).canvas_width(), Some(42));
		assert_eq!(TerminalHost::new(RenderOverrides::default().with_canvas_width(0), &measured).canvas_width(), None);
	}

	#[test]
	fn the_color_resolves_the_chain_then_the_override_then_the_cascade() {
		let level = |variables: &[(&str, &str)], override_color| {
			let terminal = terminal(Some(80), None, variables);
			TerminalHost::new(RenderOverrides::default().with_color(override_color), &terminal).color_level()
		};

		// FORCE_COLOR wins over the cascade and over a disabled override, NO_COLOR silences the terminal
		assert_eq!(
			level(&[("TERM", "xterm-256color"), ("FORCE_COLOR", "2")], ColorOverride::Disabled),
			Some(ColorLevel::Ansi256)
		);
		assert_eq!(level(&[("TERM", "xterm-256color"), ("NO_COLOR", "1")], ColorOverride::Auto), None);
		assert_eq!(level(&[], ColorOverride::Disabled), None);
		assert_eq!(level(&[], ColorOverride::Level(ColorLevel::Basic)), Some(ColorLevel::Basic));

		// the cascade answers an attached terminal, the fallback covers the rest
		assert_eq!(level(&[("TERM", "ansi")], ColorOverride::Auto), Some(ColorLevel::Basic));
		assert_eq!(level(&[("TERM", "xterm-256color")], ColorOverride::Auto), Some(ColorLevel::Ansi256));
		assert_eq!(level(&[], ColorOverride::Auto), Some(ColorLevel::TrueColor));

		// a terminal that refuses escape codes stays plain, the fallback paints only an undetected one
		assert_eq!(level(&[("TERM", "dumb")], ColorOverride::Auto), None);
	}

	#[test]
	fn a_detached_stream_takes_the_fallback() {
		let mut terminal = terminal(None, None, &[("TERM", "xterm-256color")]);
		terminal.attached = false;

		assert_eq!(TerminalHost::new(RenderOverrides::default(), &terminal).color_level(), Some(ColorLevel::TrueColor));
	}

	#[test]
	fn the_windows_console_answers_by_the_build_of_the_release() {
		let windows = |release: &str| {
			let mut terminal = terminal(Some(80), None, &[]);
			terminal.platform = String::from("win32");
			terminal.release = String::from(release);
			TerminalHost::new(RenderOverrides::default(), &terminal).color_level()
		};

		assert_eq!(windows("10.0.22631"), Some(ColorLevel::TrueColor));
		assert_eq!(windows("10.0.10586"), Some(ColorLevel::Ansi256));
		assert_eq!(windows("6.3.9600"), Some(ColorLevel::Basic));
		// a release without a build dates nothing and gets the oldest palette
		assert_eq!(windows("10"), Some(ColorLevel::Basic));
		assert!(terminal(Some(80), None, &[]).windows_console().is_none());
	}

	#[test]
	fn the_seed_is_the_override_or_a_fresh_roll() {
		let terminal = terminal(None, None, &[]);
		let rolling = || TerminalHost::new(RenderOverrides::default(), &terminal);

		assert_eq!(TerminalHost::new(RenderOverrides::default().with_seed(42), &terminal).seed(), 42);
		// eight u64 rolls agree only at 2^-448, a bound no run meets, so the roll is guarded
		// without a seam into the entropy source
		let rolls: Vec<u64> = (0..8).map(|_| rolling().seed()).collect();
		assert!(rolls.iter().any(|roll| *roll != rolls[0]), "all rolls agree");
	}
}
