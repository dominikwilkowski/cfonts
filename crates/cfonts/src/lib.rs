//! Sexy fonts for the console: cfonts renders text as banner art
//!
//! [`Cfonts`] is the builder, a [`Host`] answers what its runtime can show and
//! performs output, and pure environments format the artifact for their target

mod builder;
#[cfg(not(target_arch = "wasm32"))]
pub mod cli;
pub mod color;
mod components;
pub mod environments;
pub mod fonts;
pub mod hosts;
pub mod layout;
pub mod options;
mod render;
pub use builder::Cfonts;
pub use color::{
	Background, BackgroundOption, Color, ColorError, ColorLevel, ColorOption, Gradient, GradientOption, GradientPreset,
	Kind, Rgb, TakesCandy, TakesSystem, Text, TransitionStops,
};
pub use environments::{BrowserConsoleEnv, BrowserEnv, CliEnv, ColorTokens, Environment, Rendered};
pub use fonts::Font;
pub use hosts::Host;
pub use options::{Align, BlockOptions, Options, Valign};
pub use render::{CanvasWidth, ColorOverride, RenderContext, RenderOverrides, render_with};

use cfonts_macros::supported_chars;

#[cfg(feature = "web")]
pub use hosts::BrowserHost;
#[cfg(not(target_arch = "wasm32"))]
pub use hosts::RustHost;

#[cfg(feature = "dioxus")]
pub use components::CfontsDioxus;
#[cfg(feature = "leptos")]
pub use components::CfontsLeptos;
#[cfg(feature = "ratatui")]
pub use components::CfontsWidget;

#[cfg(test)]
mod tests;

// GLOBAL CONSTS

/// The char from your input that will cause a new line in the cfonts output
pub const NEW_LINE_CHAR: char = '|';

/// All supported characters of cfonts
pub const SUPPORTED_CHARS: &[char] = &supported_chars!();
