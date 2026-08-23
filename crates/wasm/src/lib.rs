mod builder;
mod terminal_color_support;
mod types;

pub use builder::Cfonts;
pub use terminal_color_support::detect_color_support;
pub use types::{Align, Color, ColorLevel, EnvironmentKind, Font, GradientPreset, Rendered, Valign, hex_to_rgb};
