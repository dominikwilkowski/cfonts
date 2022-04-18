extern crate exitcode;

use std::env::args;

pub mod config;
pub mod helpers;
pub mod parse_args;

use parse_args::parse_args;

fn main() {
	// For API later:
	//
	// let defaults = Options::default();
	// let defaults = Options {
	// 	font: String::from("notblock"),
	// 	..Options::default()
	// };

	let options = parse_args(args().collect::<Vec<String>>());
	println!("{:#?}", options);

	println!("Hello, world!");
}
