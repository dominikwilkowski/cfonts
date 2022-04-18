extern crate exitcode;

pub mod config;
pub mod helpers;
pub mod parse_args;

pub fn say(options: config::Options) {
	println!("{:#?}", options);
}
