extern crate exitcode;

pub mod color;
pub mod config;
pub mod debug;
pub mod font;
pub mod helpers;
pub mod parse_args;
pub mod render;

use render::render;

pub fn say(options: config::Options) {
	let render_options = render(&options);
	println!("{:#?}", render_options.options);
}
