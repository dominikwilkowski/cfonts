mod builder;
mod color_support;
mod types;

pub use builder::Cfonts;
pub use color_support::{ColorDecision, decide_color, decide_detected};
pub use types::{Align, Color, ColorLevel, Font, GradientPreset, Rendered, Valign, hex_to_rgb};
