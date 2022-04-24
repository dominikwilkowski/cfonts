extern crate exitcode;

use std::env::args;

pub mod color;
pub mod config;
pub mod debug;
pub mod helpers;
pub mod parse_args;
pub mod render;

use debug::{d, Dt};
use parse_args::parse_args;
use render::render;

// extern crate cfonts;

// use cfonts::{ Options, say }

// fn main() {
// 	say(Options {
// 		font: String::from("notblock"),
// 		..Options::default()
// 	});
// }

fn main() {
	let options = parse_args(args().collect::<Vec<String>>());
	d("main()", 1, Dt::Head, &options, &mut std::io::stdout());
	d(
		&format!("CLI args parsed from\n{:#?}\nto:\n{:#?}", args().collect::<Vec<String>>(), options),
		3,
		Dt::Log,
		&options,
		&mut std::io::stdout(),
	);

	// For API later:
	//
	// let defaults = Options::default();
	// let defaults = Options {
	// 	font: String::from("notblock"),
	// 	..Options::default()
	// };

	let render_options = render(&options);
	println!("{:#?}", render_options.options);

	// println!("Thing not colored {} end", color::color("color me", config::Colors::Rgb([255, 200, 0])));

	// println!("start \x1b[38;2;0;0;255mcolored bit\x1b[0;0;m end");
	// println!("Hello, world!");
}
