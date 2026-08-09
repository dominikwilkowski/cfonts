//! Sexy fonts for the console: cfonts renders text as banner art
//!
//! [`Cfonts`] is the builder, a [`Host`] resolves runtime capabilities and
//! performs output, and pure environments format the artifact for their target

mod args;
mod builder;
pub mod color;
mod components;
pub mod environments;
pub mod fonts;
pub mod hosts;
pub mod layout;
pub mod options;
mod render;
pub use builder::Cfonts;
pub use color::{Color, ColorError, ColorOption, GradientOption, GradientPreset, GradientStop, Rgb, TransitionStops};
pub use environments::{BrowserConsoleEnv, BrowserEnv, CliEnv, ColorTokens, Environment, Rendered};
pub use fonts::Font;
pub use hosts::Host;
pub use options::{Align, Options, Valign};
pub use render::{CanvasWidth, ColorLevel, ColorOverride, RenderContext, RenderOverrides, render_with};

#[cfg(not(target_arch = "wasm32"))]
pub use hosts::RustHost;

#[cfg(feature = "dioxus")]
pub use components::CfontsDioxus;
#[cfg(feature = "ratatui")]
pub use components::CfontsWidget;
#[cfg(feature = "leptos")]
pub use components::{CfontsLeptos, LeptosHost};

#[cfg(test)]
mod tests;
