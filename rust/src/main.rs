extern crate exitcode;

use std::env::args;

pub mod config;
pub mod helpers;
pub mod parse_args;
pub mod render;

use parse_args::parse_args;
use render::render;

fn main() {
	// For API later:
	//
	// let defaults = Options::default();
	// let defaults = Options {
	// 	font: String::from("notblock"),
	// 	..Options::default()
	// };

	let options = parse_args(args().collect::<Vec<String>>());
	let render_options = render(options);
	println!("{:#?}", render_options.options);

	// println!("start \x1b[38;2;0;0;255mcolored bit\x1b[0;0;m end");
	// println!("Hello, world!");
}
