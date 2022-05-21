//! # Cfonts - Sexy fonts for the console
//!
//! ```sh
//! $ cfonts "hi there"
//!
//! ██╗  ██╗ ██╗     ████████╗ ██╗  ██╗ ███████╗ ██████╗  ███████╗
//! ██║  ██║ ██║     ╚══██╔══╝ ██║  ██║ ██╔════╝ ██╔══██╗ ██╔════╝
//! ███████║ ██║        ██║    ███████║ █████╗   ██████╔╝ █████╗
//! ██╔══██║ ██║        ██║    ██╔══██║ ██╔══╝   ██╔══██╗ ██╔══╝
//! ██║  ██║ ██║        ██║    ██║  ██║ ███████╗ ██║  ██║ ███████╗
//! ╚═╝  ╚═╝ ╚═╝        ╚═╝    ╚═╝  ╚═╝ ╚══════╝ ╚═╝  ╚═╝ ╚══════╝
//! ```
//!
//! 💡  This is a silly little command line tool for sexy fonts in the console. **Give your cli some love.**
//!
//! ---
//! This library supports:
//! - different fonts
//! - multiple colors per font
//! - color gradient
//! - text alignment
//! - letter spacing and line height options
//!
//! ## Usage
//!
//! Print directly to your console via the [`println!()`](https://doc.rust-lang.org/std/macro.println.html) macro:
//!
//! ```rust
//! extern crate cfonts;
//!
//! use cfonts::{ say, Options };
//!
//! fn main() {
//!     say(Options {
//!         text: String::from("hello"),
//!         ..Options::default()
//!     });
//! }
//! ```
//!
//! Or use the output struct however you like:
//!
//! ```rust
//! extern crate cfonts;
//!
//! use cfonts::{ render, Options, Fonts };
//!
//! fn main() {
//!     let output = render(Options {
//!         text: String::from("hello"),
//!         font: Fonts::FontTiny,
//!         ..Options::default()
//!     });
//!
//!     assert_eq!(
//!         output.text,
//!         format!("{}{}{}",
//!             "\n\n",
//!             " █ █ █▀▀ █   █   █▀█\n",
//!             " █▀█ ██▄ █▄▄ █▄▄ █▄█\n\n"
//!         )
//!     );
//!
//!     assert_eq!(output.vec, vec![
//!         String::from("\n\n █ █ █▀▀ █   █   █▀█"),
//!         String::from(    " █▀█ ██▄ █▄▄ █▄▄ █▄█"),
//!         String::from("\n"),
//!     ]);
//!
//!     assert_eq!(output.lines, 1);
//!
//!     assert_eq!(output.options, Options {
//!         text: String::from("hello"),
//!         font: Fonts::FontTiny,
//!         ..Options::default()
//!     });
//! }
//! ```
//!

extern crate exitcode;

pub mod args;
pub mod chars;
pub mod cli;
pub mod color;
pub mod config;
pub mod debug;
pub mod font;
pub mod gradient;
pub mod helpers;
pub mod render;

pub use color::Rgb;
pub use config::{Align, BgColors, Colors, Env, Fonts, Options};
use debug::{d, Dt};
pub use render::render;

/// The `say` function will print your cfonts output to `stdout`.
///
/// The way you pass it Options is the same as for [`render()`]
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{ say, Options, Align, BgColors, Colors, Env, Fonts, Rgb };
///
/// fn main() {
///     say(Options {
///         text: String::from("hello"),
///         font: Fonts::FontSlick,
///         colors: vec![Colors::Red, Colors::Rgb(Rgb::Val(20, 216, 79))],
///         background: BgColors::BlueBright,
///         align: Align::Center,
///         letter_spacing: 1,
///         line_height: 2,
///         spaceless: true,
///         max_length: 15,
///         gradient: vec![String::from("#ff8800"), String::from("#88ff00")],
///         independent_gradient: false,
///         transition_gradient: false,
///         env: Env::Browser,
///         ..Options::default()
///     });
/// }
/// ```
pub fn say(options: Options) {
	d("say()", 1, Dt::Head, &options, &mut std::io::stdout());
	d(&format!("say() Options:\n{:#?}", options), 3, Dt::Log, &options, &mut std::io::stdout());

	let render_options = render(options);
	println!("{}", render_options.text);
}
