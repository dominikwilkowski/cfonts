//! All gradient step calculation: colors between stops, one per output column
//!
//! Two stop gradients travel through hue space (radial hue, linear saturation and value)
//! so every color in between gets visited; transitions travel in straight lines through RGB

use std::f64::consts::{PI, TAU};

use crate::color::{GradientOption, Rgb};

use cfonts_macros::All;

/// Hue in degrees, saturation and value in percent
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Hsv(pub f64, pub f64, pub f64);

/// Hue in radians, saturation and value in percent
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Rsv(pub f64, pub f64, pub f64);

impl Rgb {
	/// This color in HSV space
	pub(crate) fn to_hsv(self) -> Hsv {
		let red = self.red as f64 / 255.0;
		let green = self.green as f64 / 255.0;
		let blue = self.blue as f64 / 255.0;

		let max = red.max(green).max(blue);
		let min = red.min(green).min(blue);
		let diff = max - min;

		let v = max * 100.0;
		let s = if max == 0.0 { 0.0 } else { (diff / max) * 100.0 };

		let h = if (max - min).abs() < f64::EPSILON {
			0.0
		} else if (max - red).abs() < f64::EPSILON && green >= blue {
			60.0 * ((green - blue) / diff)
		} else if (max - red).abs() < f64::EPSILON && green < blue {
			60.0 * ((green - blue) / diff) + 360.0
		} else if (max - green).abs() < f64::EPSILON {
			60.0 * ((blue - red) / diff) + 120.0
		} else {
			60.0 * ((red - green) / diff) + 240.0
		};

		Hsv(h, s, v)
	}
}

impl Hsv {
	/// This color in RGB space
	pub(crate) fn to_rgb(self) -> Rgb {
		let Hsv(h_input, s_input, v_input) = self;
		let hue = h_input / 60.0;
		let saturation = s_input / 100.0;
		let mut val = v_input / 100.0;

		let f = hue - hue.floor();
		let p = 255.0 * val * (1.0 - saturation);
		let q = 255.0 * val * (1.0 - (saturation * f));
		let t = 255.0 * val * (1.0 - (saturation * (1.0 - f)));
		val *= 255.0;

		match (hue.floor() % 6.0) as u8 {
			0 => Rgb {
				red: val as u8,
				green: t as u8,
				blue: p as u8,
			},
			1 => Rgb {
				red: q as u8,
				green: val as u8,
				blue: p as u8,
			},
			2 => Rgb {
				red: p as u8,
				green: val as u8,
				blue: t as u8,
			},
			3 => Rgb {
				red: p as u8,
				green: q as u8,
				blue: val as u8,
			},
			4 => Rgb {
				red: t as u8,
				green: p as u8,
				blue: val as u8,
			},
			// due to the modulo operation we can never get anything above 5
			_ => Rgb {
				red: val as u8,
				green: p as u8,
				blue: q as u8,
			},
		}
	}

	/// This color with its hue in radians
	pub(crate) fn to_rsv(self) -> Rsv {
		let Hsv(h, s, v) = self;

		Rsv((h * PI) / 180.0, s, v)
	}
}

impl Rsv {
	/// This color with its hue in degrees, rounded to twelve decimals
	pub(crate) fn to_hsv(self) -> Hsv {
		let Rsv(r, s, v) = self;
		let precision = 1_000_000_000_000.0;
		let h = (((r * 180.0) / PI) * precision).round() / precision;

		Hsv(h, s, v)
	}
}

/// Linear interpolation between two points at a certain step of `steps`
fn linear(from: f64, to: f64, this_step: usize, steps: usize) -> f64 {
	if steps == 0 {
		return to;
	}

	from + this_step as f64 * ((to - from) / steps as f64)
}

/// Radial interpolation between two angles at a certain step of `steps`, always the long way around
fn radial(from: f64, to: f64, this_step: usize, steps: usize) -> f64 {
	if steps == 0 {
		return to;
	}

	let long_distance = if from > to {
		if from - to < PI { TAU - (from - to) } else { to - from }
	} else if to - from < PI {
		(to - from) - TAU
	} else {
		-(from - to)
	};

	let mut result = from + (this_step as f64 * (long_distance / steps as f64));

	if result < 0.0 {
		result += TAU;
	}

	if result > TAU {
		result -= TAU;
	}

	result
}

/// The colors of one gradient, one color per output column
///
/// Owning the buffer lets repeated fills reuse its capacity
#[derive(Debug, Default)]
pub(crate) struct GradientColors {
	colors: Vec<Rgb>,
}

impl GradientColors {
	/// An empty gradient buffer, filled per domain by [`fill`](Self::fill)
	pub(crate) fn new() -> Self {
		Self { colors: Vec::new() }
	}

	/// All colors, one per column
	pub(crate) fn colors(&self) -> &[Rgb] {
		&self.colors
	}

	/// Fills the buffer with exactly `steps` colors for the given stops
	///
	/// Two stops with `transition: false` travel through hue space; transitions travel linearly
	/// through RGB, distributing the steps over the gaps between the stops
	pub(crate) fn fill(&mut self, stops: &[Rgb], transition: bool, steps: usize) {
		self.colors.clear();

		if stops.is_empty() || steps == 0 {
			return;
		}

		if !transition {
			debug_assert!(stops.len() == 2, "Error: a two stop gradient must hold exactly two stops");
			self.fill_two_stop(stops[0], stops[1], steps);
			return;
		}

		if steps <= 1 {
			self.colors.push(stops[stops.len() - 1]);
			return;
		}

		let mut gaps = Self::transition_steps(stops.len(), steps);

		for (i, stop) in stops.iter().enumerate() {
			if i > 0 {
				let step = gaps.next().expect("one gap sits between every pair of stops");
				self.fill_transition(stops[i - 1], *stop, step);

				if step != -1 {
					self.colors.push(*stop);
				}
			} else {
				self.colors.push(*stop);
			}
		}
	}

	/// Appends `steps` colors between two colors by going through the colors in between
	fn fill_two_stop(&mut self, from: Rgb, to: Rgb, steps: usize) {
		let Rsv(from_r, from_s, from_v) = from.to_hsv().to_rsv();
		let Rsv(to_r, to_s, to_v) = to.to_hsv().to_rsv();

		for n in 0..steps {
			let r = radial(from_r, to_r, n, steps - 1);
			let s = linear(from_s, to_s, n, steps - 1);
			let v = linear(from_v, to_v, n, steps - 1);

			self.colors.push(Rsv(r, s, v).to_hsv().to_rgb());
		}
	}

	/// Appends `steps` colors between two colors by going straight from one to the other
	///
	/// A negative step count appends nothing
	fn fill_transition(&mut self, from: Rgb, to: Rgb, steps: i64) {
		for n in 1..=steps {
			let r = linear(from.red.into(), to.red.into(), n as usize, (steps + 1) as usize);
			let g = linear(from.green.into(), to.green.into(), n as usize, (steps + 1) as usize);
			let b = linear(from.blue.into(), to.blue.into(), n as usize, (steps + 1) as usize);

			self.colors.push(Rgb {
				red: r as u8,
				green: g as u8,
				blue: b as u8,
			});
		}
	}

	/// The steps of each gap between transition stops: what to skip and what to paint
	///
	/// A gap of -1 signals that its right hand stop does not fit into the output;
	/// the leftover steps are distributed onto the gaps at the end
	fn transition_steps(color_count: usize, steps: usize) -> impl Iterator<Item = i64> {
		// steps per color transition
		let base = ((steps as f64 - color_count as f64) / (color_count as f64 - 1.0)).floor() as i64;
		// steps left over to be distributed from the end
		let rest = (steps as i64 - (color_count as i64 + base * (color_count as i64 - 1))) as usize;
		// one gap between each pair of neighboring stops
		let gaps = color_count - 1;

		(0..gaps).map(move |gap| if gap >= gaps - rest { base + 1 } else { base })
	}
}

/// The bundled transition gradient presets
///
/// ![The gradient option and its output with cfonts](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/transition-gradient.png)
#[derive(Debug, Clone, Copy, PartialEq, Eq, All)]
pub enum GradientPreset {
	Pride,
	Agender,
	Aromantic,
	Asexual,
	Bisexual,
	Genderfluid,
	Genderqueer,
	Intersex,
	Lesbian,
	Nonbinary,
	Pansexual,
	Polysexual,
	Transgender,
}

impl GradientPreset {
	/// Looks up a preset by its name or its aliases, case insensitively
	pub fn from_name(name: &str) -> Option<Self> {
		match name.to_ascii_lowercase().as_str() {
			"pride" | "lgbt" | "lgbtq" | "lgbtqa" => Some(Self::Pride),
			"agender" => Some(Self::Agender),
			"aromantic" => Some(Self::Aromantic),
			"asexual" => Some(Self::Asexual),
			"bisexual" | "bi" => Some(Self::Bisexual),
			"genderfluid" => Some(Self::Genderfluid),
			"genderqueer" => Some(Self::Genderqueer),
			"intersex" => Some(Self::Intersex),
			"lesbian" => Some(Self::Lesbian),
			"nonbinary" => Some(Self::Nonbinary),
			"pansexual" | "pan" => Some(Self::Pansexual),
			"polysexual" | "poly" => Some(Self::Polysexual),
			"transgender" | "trans" => Some(Self::Transgender),
			_ => None,
		}
	}

	/// The stops of this preset
	pub const fn stops(self) -> &'static [Rgb] {
		match self {
			Self::Pride => &[
				Rgb {
					red: 117,
					green: 7,
					blue: 135,
				},
				Rgb {
					red: 0,
					green: 77,
					blue: 255,
				},
				Rgb {
					red: 0,
					green: 128,
					blue: 38,
				},
				Rgb {
					red: 255,
					green: 237,
					blue: 0,
				},
				Rgb {
					red: 255,
					green: 140,
					blue: 0,
				},
				Rgb {
					red: 228,
					green: 3,
					blue: 3,
				},
			],
			Self::Agender => &[
				Rgb {
					red: 0,
					green: 0,
					blue: 0,
				},
				Rgb {
					red: 185,
					green: 185,
					blue: 185,
				},
				Rgb {
					red: 255,
					green: 255,
					blue: 255,
				},
				Rgb {
					red: 184,
					green: 244,
					blue: 131,
				},
				Rgb {
					red: 255,
					green: 255,
					blue: 255,
				},
				Rgb {
					red: 185,
					green: 185,
					blue: 185,
				},
				Rgb {
					red: 0,
					green: 0,
					blue: 0,
				},
			],
			Self::Aromantic => &[
				Rgb {
					red: 61,
					green: 165,
					blue: 66,
				},
				Rgb {
					red: 167,
					green: 211,
					blue: 121,
				},
				Rgb {
					red: 255,
					green: 255,
					blue: 255,
				},
				Rgb {
					red: 169,
					green: 169,
					blue: 169,
				},
				Rgb {
					red: 0,
					green: 0,
					blue: 0,
				},
			],
			Self::Asexual => &[
				Rgb {
					red: 0,
					green: 0,
					blue: 0,
				},
				Rgb {
					red: 163,
					green: 163,
					blue: 163,
				},
				Rgb {
					red: 255,
					green: 255,
					blue: 255,
				},
				Rgb {
					red: 128,
					green: 0,
					blue: 128,
				},
			],
			Self::Bisexual => &[
				Rgb {
					red: 214,
					green: 2,
					blue: 112,
				},
				Rgb {
					red: 214,
					green: 2,
					blue: 112,
				},
				Rgb {
					red: 155,
					green: 79,
					blue: 150,
				},
				Rgb {
					red: 0,
					green: 56,
					blue: 168,
				},
				Rgb {
					red: 0,
					green: 56,
					blue: 168,
				},
			],
			Self::Genderfluid => &[
				Rgb {
					red: 255,
					green: 117,
					blue: 162,
				},
				Rgb {
					red: 255,
					green: 255,
					blue: 255,
				},
				Rgb {
					red: 190,
					green: 24,
					blue: 214,
				},
				Rgb {
					red: 0,
					green: 0,
					blue: 0,
				},
				Rgb {
					red: 51,
					green: 62,
					blue: 189,
				},
			],
			Self::Genderqueer => &[
				Rgb {
					red: 181,
					green: 126,
					blue: 220,
				},
				Rgb {
					red: 255,
					green: 255,
					blue: 255,
				},
				Rgb {
					red: 74,
					green: 129,
					blue: 35,
				},
			],
			Self::Intersex => &[
				Rgb {
					red: 255,
					green: 216,
					blue: 0,
				},
				Rgb {
					red: 255,
					green: 216,
					blue: 0,
				},
				Rgb {
					red: 121,
					green: 2,
					blue: 170,
				},
				Rgb {
					red: 255,
					green: 216,
					blue: 0,
				},
				Rgb {
					red: 255,
					green: 216,
					blue: 0,
				},
			],
			Self::Lesbian => &[
				Rgb {
					red: 213,
					green: 45,
					blue: 0,
				},
				Rgb {
					red: 255,
					green: 154,
					blue: 86,
				},
				Rgb {
					red: 255,
					green: 255,
					blue: 255,
				},
				Rgb {
					red: 211,
					green: 98,
					blue: 164,
				},
				Rgb {
					red: 163,
					green: 2,
					blue: 98,
				},
			],
			Self::Nonbinary => &[
				Rgb {
					red: 252,
					green: 244,
					blue: 52,
				},
				Rgb {
					red: 255,
					green: 255,
					blue: 255,
				},
				Rgb {
					red: 156,
					green: 92,
					blue: 212,
				},
				Rgb {
					red: 44,
					green: 44,
					blue: 44,
				},
			],
			Self::Pansexual => &[
				Rgb {
					red: 255,
					green: 33,
					blue: 140,
				},
				Rgb {
					red: 255,
					green: 216,
					blue: 0,
				},
				Rgb {
					red: 33,
					green: 177,
					blue: 255,
				},
			],
			Self::Polysexual => &[
				Rgb {
					red: 246,
					green: 28,
					blue: 185,
				},
				Rgb {
					red: 7,
					green: 213,
					blue: 105,
				},
				Rgb {
					red: 28,
					green: 146,
					blue: 246,
				},
			],
			Self::Transgender => &[
				Rgb {
					red: 91,
					green: 206,
					blue: 250,
				},
				Rgb {
					red: 245,
					green: 169,
					blue: 184,
				},
				Rgb {
					red: 255,
					green: 255,
					blue: 255,
				},
				Rgb {
					red: 245,
					green: 169,
					blue: 184,
				},
				Rgb {
					red: 91,
					green: 206,
					blue: 250,
				},
			],
		}
	}

	/// This preset as a gradient
	pub fn to_gradient(self, independent_gradient: bool) -> GradientOption {
		GradientOption::Preset {
			preset: self,
			independent_gradient,
		}
	}
}

/// A preset used directly is a gradient over its stops
impl From<GradientPreset> for GradientOption {
	fn from(preset: GradientPreset) -> Self {
		preset.to_gradient(false)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// helpers

	// The hex forms of a fill, so the documented values read as they were written
	fn fill_as_hex(stops: &[&str], transition: bool, steps: usize) -> Vec<String> {
		let stops: Vec<Rgb> = stops.iter().map(|hex| Rgb::from_hex(hex).expect("test stops are valid hex")).collect();
		let mut gradient = GradientColors::new();
		gradient.fill(&stops, transition, steps);

		gradient.colors().iter().map(|color| color.to_hex()).collect()
	}

	// Rgb::to_hsv

	#[test]
	fn to_hsv_converts_correctly() {
		assert_eq!(
			Rgb {
				red: 0,
				green: 0,
				blue: 0
			}
			.to_hsv(),
			Hsv(0.0, 0.0, 0.0)
		);
		assert_eq!(
			Rgb {
				red: 166,
				green: 20,
				blue: 100
			}
			.to_hsv(),
			Hsv(327.1232876712329, 87.95180722891565, 65.09803921568627)
		);
	}

	// Hsv::to_rgb

	#[test]
	fn to_rgb_converts_correctly() {
		assert_eq!(
			Hsv(0.0, 0.0, 0.0).to_rgb(),
			Rgb {
				red: 0,
				green: 0,
				blue: 0
			}
		);
		assert_eq!(
			Hsv(30.0, 20.0, 20.0).to_rgb(),
			Rgb {
				red: 51,
				green: 45,
				blue: 40
			}
		);
		assert_eq!(
			Hsv(80.0, 20.0, 20.0).to_rgb(),
			Rgb {
				red: 47,
				green: 51,
				blue: 40
			}
		);
		assert_eq!(
			Hsv(120.0, 20.0, 20.0).to_rgb(),
			Rgb {
				red: 40,
				green: 51,
				blue: 40
			}
		);
	}

	// Hsv::to_rsv / Rsv::to_hsv

	#[test]
	fn to_rsv_converts_the_hue_to_radians() {
		assert_eq!(Hsv(0.0, 0.0, 0.0).to_rsv(), Rsv(0.0, 0.0, 0.0));
		assert_eq!(Hsv(360.0, 0.0, 0.0).to_rsv(), Rsv(TAU, 0.0, 0.0));
		assert_eq!(Hsv(180.0, 0.0, 0.0).to_rsv(), Rsv(PI, 0.0, 0.0));
	}

	#[test]
	fn to_hsv_converts_the_hue_to_degrees() {
		assert_eq!(Rsv(0.0, 0.0, 0.0).to_hsv(), Hsv(0.0, 0.0, 0.0));
		assert_eq!(Rsv(TAU, 0.0, 0.0).to_hsv(), Hsv(360.0, 0.0, 0.0));
		assert_eq!(Rsv(PI, 0.0, 0.0).to_hsv(), Hsv(180.0, 0.0, 0.0));
		assert_eq!(Rsv(5.235987755982989, 0.0, 0.0).to_hsv(), Hsv(300.0, 0.0, 0.0));
	}

	// linear

	#[test]
	fn linear_walks_evenly() {
		assert!((linear(0.0, 5.0, 0, 5) - 0.0).abs() < f64::EPSILON);
		assert!((linear(0.0, 5.0, 1, 5) - 1.0).abs() < f64::EPSILON);
		assert!((linear(0.0, 5.0, 4, 5) - 4.0).abs() < f64::EPSILON);
		assert!((linear(0.0, 5.0, 5, 5) - 5.0).abs() < f64::EPSILON);
		assert!((linear(0.0, 5.0, 3, 0) - 5.0).abs() < f64::EPSILON);
	}

	// radial

	#[test]
	fn radial_walks_the_long_way_around() {
		assert!((radial(3.0, 2.0, 0, 3) - 3.0).abs() < f64::EPSILON);
		assert!((radial(3.0, 2.0, 1, 3) - 4.761061769059862).abs() < f64::EPSILON);
		assert!((radial(3.0, 2.0, 2, 3) - 0.23893823094013733).abs() < f64::EPSILON);
		assert!((radial(3.0, 2.0, 3, 3) - 2.0).abs() < f64::EPSILON);
	}

	// GradientColors::fill: two stop

	#[test]
	fn two_stop_fill_matches_the_documented_gradient() {
		assert_eq!(
			fill_as_hex(&["#ff8800", "#8899dd"], false, 10),
			vec![
				"#ff8800", "#fbe211", "#c0f721", "#7bf331", "#44ef41", "#50ec86", "#5fe8c0", "#6ddbe4", "#7ab4e0", "#8799dd",
			]
		);
	}

	// GradientColors::fill: transition

	#[test]
	fn transition_fill_matches_the_documented_values() {
		assert_eq!(fill_as_hex(&["#ff0000", "#0000ff"], true, 1), vec!["#0000ff"]);
		assert_eq!(fill_as_hex(&["#ff0000", "#0000ff"], true, 2), vec!["#ff0000", "#0000ff"]);
		assert_eq!(fill_as_hex(&["#ff0000", "#0000ff"], true, 3), vec!["#ff0000", "#7f007f", "#0000ff"]);
		assert_eq!(fill_as_hex(&["#ff0000", "#0000ff"], true, 4), vec!["#ff0000", "#aa0055", "#5500aa", "#0000ff"]);
		assert_eq!(
			fill_as_hex(&["#ff0000", "#0000ff"], true, 7),
			vec![
				"#ff0000", "#d4002a", "#aa0055", "#7f007f", "#5500aa", "#2a00d4", "#0000ff"
			]
		);
	}

	// GradientColors::transition_steps

	// The gaps of a transition, collected for comparison
	fn gaps(color_count: usize, steps: usize) -> Vec<i64> {
		GradientColors::transition_steps(color_count, steps).collect()
	}

	#[test]
	fn transition_steps_distribute_evenly() {
		assert_eq!(gaps(2, 1), vec![-1]);
		assert_eq!(gaps(2, 1), vec![-1]);
		assert_eq!(gaps(2, 2), vec![0]);
		assert_eq!(gaps(2, 3), vec![1]);
		assert_eq!(gaps(2, 4), vec![2]);
		assert_eq!(gaps(2, 5), vec![3]);
	}

	#[test]
	fn transition_steps_distribute_the_rest_from_the_end() {
		assert_eq!(gaps(3, 7), vec![2, 2]);
		assert_eq!(gaps(3, 8), vec![2, 3]);
	}

	// GradientColors::fill: buffer reuse and exact step counts

	#[test]
	fn fill_clears_the_buffer_and_hits_the_step_count() {
		let stops = [
			Rgb {
				red: 255,
				green: 0,
				blue: 0,
			},
			Rgb {
				red: 0,
				green: 0,
				blue: 255,
			},
		];
		let mut gradient = GradientColors::new();

		for steps in [1_usize, 2, 5, 40] {
			gradient.fill(&stops, false, steps);
			assert_eq!(gradient.colors().len(), steps);

			gradient.fill(&stops, true, steps);
			assert_eq!(gradient.colors().len(), steps);
		}
	}

	// GradientColors::fill: edge cases

	#[test]
	fn a_single_column_gets_the_end_color_in_both_modes() {
		let stops = [
			Rgb {
				red: 255,
				green: 0,
				blue: 0,
			},
			Rgb {
				red: 0,
				green: 0,
				blue: 255,
			},
		];
		let mut gradient = GradientColors::new();

		gradient.fill(&stops, false, 1);
		assert_eq!(
			gradient.colors(),
			&[Rgb {
				red: 0,
				green: 0,
				blue: 255
			}]
		);

		gradient.fill(&stops, true, 1);
		assert_eq!(
			gradient.colors(),
			&[Rgb {
				red: 0,
				green: 0,
				blue: 255
			}]
		);
	}

	#[test]
	fn equal_two_stop_endpoints_walk_the_full_hue_circle() {
		// the radial interpolation always takes the long way, so the same color
		// twice sweeps every hue and returns: red at both ends, cyan in the middle
		let red = Rgb {
			red: 255,
			green: 0,
			blue: 0,
		};
		let mut gradient = GradientColors::new();
		gradient.fill(&[red, red], false, 5);

		assert_eq!(gradient.colors()[0], red);
		assert_eq!(gradient.colors()[2].to_hex(), "#00ffff");
		assert_eq!(gradient.colors()[4], red);
	}

	#[test]
	fn a_preset_fill_starts_and_ends_on_its_stops() {
		let stops = GradientPreset::Transgender.stops();
		let mut gradient = GradientColors::new();
		gradient.fill(stops, true, 40);

		assert_eq!(gradient.colors().len(), 40);
		assert_eq!(gradient.colors()[0], stops[0]);
		assert_eq!(gradient.colors()[39], stops[stops.len() - 1]);
	}

	// GradientPreset

	#[test]
	fn presets_carry_the_documented_stop_lists() {
		let pride: Vec<String> = GradientPreset::Pride.stops().iter().map(|rgb| rgb.to_hex()).collect();
		assert_eq!(pride, vec!["#750787", "#004dff", "#008026", "#ffed00", "#ff8c00", "#e40303"]);

		let trans: Vec<String> = GradientPreset::Transgender.stops().iter().map(|rgb| rgb.to_hex()).collect();
		assert_eq!(trans, vec!["#5bcefa", "#f5a9b8", "#ffffff", "#f5a9b8", "#5bcefa"]);

		assert_eq!(GradientPreset::Genderqueer.stops().len(), 3);
		assert_eq!(GradientPreset::Agender.stops().len(), 7);
	}

	#[test]
	fn presets_convert_into_preset_gradients() {
		assert_eq!(
			GradientPreset::Transgender.to_gradient(true),
			GradientOption::Preset {
				preset: GradientPreset::Transgender,
				independent_gradient: true,
			}
		);
		assert_eq!(
			GradientOption::from(GradientPreset::Pride),
			GradientOption::Preset {
				preset: GradientPreset::Pride,
				independent_gradient: false,
			}
		);
	}

	#[test]
	fn every_preset_has_a_parseable_name() {
		for preset in GradientPreset::ALL {
			let name = format!("{preset:?}").to_lowercase();
			assert!(GradientPreset::from_name(&name).is_some(), "{name} does not parse back to {preset:?}");
		}
	}
}
