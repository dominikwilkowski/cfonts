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
//! ```sh
//! $ cfonts "Hello world" --font
//!
//!
//!  ╦ ╦ ╔═╗ ╦   ╦   ╔═╗      ╦ ╦ ╔═╗ ╦═╗ ╦   ╔╦╗
//!  ╠═╣ ║╣  ║   ║   ║ ║      ║║║ ║ ║ ╠╦╝ ║    ║║
//!  ╩ ╩ ╚═╝ ╩═╝ ╩═╝ ╚═╝      ╚╩╝ ╚═╝ ╩╚═ ╩═╝ ═╩╝
//!
//!
//! ```
//!
//! **Learn more by reading the [README](https://github.com/dominikwilkowski/cfonts)**
extern crate exitcode;

use std::env::args;

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

use debug::{d, Dt};
use render::render;

fn main() {
	let options = match args::parse(args().collect::<Vec<String>>()) {
		Ok(options) => options,
		Err(msg) => {
			println!("{}", msg);
			std::process::exit(exitcode::USAGE);
		}
	};

	d("main()", 1, Dt::Head, &options, &mut std::io::stdout());
	d(
		&format!("main() CLI args parsed from\n{:#?}\nto:\n{:#?}", args().collect::<Vec<String>>(), options),
		3,
		Dt::Log,
		&options,
		&mut std::io::stdout(),
	);

	if options.version {
		println!("{}", cli::version(&options));
		return;
	}

	if options.help {
		println!("{}", cli::help(&options));
		return;
	}

	let render_options = render(options);
	println!("{}", render_options.text);
}
