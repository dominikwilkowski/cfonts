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

use debug::{d, Dt};
use render::render;

pub fn say(options: &mut config::Options) {
	d("say()", 1, Dt::Head, options, &mut std::io::stdout());
	d(&format!("say() Options:\n{:#?}", options), 3, Dt::Log, options, &mut std::io::stdout());

	// For API later:
	// extern crate cfonts;
	//
	// use cfonts::{ Options, say }
	//
	// fn main() {
	// 	say(Options {
	// 		font: String::from("notblock"),
	// 		..Options::default()
	// 	});
	// }

	let render_options = render(options);
	println!("{}", render_options.text);
}
