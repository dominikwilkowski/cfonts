use tsify::{Ts, Tsify};
use wasm_bindgen::prelude::*;

use cfonts::{BrowserHost as CoreBrowserHost, ColorOverride, Environment, Host, RenderOverrides};

use crate::{Cfonts, ColorLevel, EnvironmentKind, Rendered, types::with_environment};

/// The page host of the core behind the boundary
///
/// TypeScript validates and forwards, the decisions and the console write live here:
/// a page has no terminal to measure, so only a column count wraps, the artifact is CSS,
/// so automatic color paints at full support, candy rolls a fresh seed per render unless
/// one is pinned, and `say` spreads the styles into the page console
#[wasm_bindgen]
pub struct BrowserHost {
	inner: CoreBrowserHost,
}

#[wasm_bindgen]
impl BrowserHost {
	/// Creates the page host from the overrides TypeScript validated
	///
	/// No width leaves the decision to the host and zero means unlimited, a disabled color paints
	/// nothing and a level pins the palette, a seed makes candy reproducible
	#[wasm_bindgen(js_name = fromOverrides)]
	pub fn from_overrides(
		canvas_width: Option<usize>,
		color_disabled: bool,
		color_level: Option<ColorLevel>,
		seed: Option<u32>,
	) -> Self {
		let color = if color_disabled {
			ColorOverride::Disabled
		} else {
			color_level.map_or(ColorOverride::Auto, |level| ColorOverride::Level(level.into()))
		};
		let overrides = RenderOverrides::default().with_color(color);
		let overrides = canvas_width.map_or(overrides, |columns| overrides.with_canvas_width(columns));
		let overrides = seed.map_or(overrides, |seed| overrides.with_seed(u64::from(seed)));

		Self { inner: CoreBrowserHost::from_overrides(overrides) }
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

/// What a host writes after the artifact to end it, the environment's own answer
///
/// Only the terminal ends its artifact, a page or a console ends its own output,
/// and the Node host carries this value to its write instead of deciding it
#[wasm_bindgen(js_name = lineEnd)]
pub fn line_end(environment: EnvironmentKind, raw_mode: bool) -> String {
	with_environment!(environment, raw_mode, |env| env.line_end().to_owned())
}

/// A fresh seed for candy colors, the one the page host rolls when no seed override is given
///
/// The boundary carries seeds as u32, so the core's roll is cut to that width,
/// which loses nothing a seed needs: every call still rolls a value of its own
#[wasm_bindgen]
pub fn entropy() -> u32 {
	CoreBrowserHost::entropy() as u32
}
