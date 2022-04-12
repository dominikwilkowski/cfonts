use std::collections::HashMap;
use std::env::args;

pub mod config;

use config::{Options, CliOption, CLIOPTIONS};

fn parse_args(args: &Vec<String>) -> Options {
	let mut options = Options::default();

	// create a lookup table for our CLIOPTIONS
	let mut options_lookup: HashMap<String, CliOption> = HashMap::new();

	for option in CLIOPTIONS {
		options_lookup.insert(option.name.clone().to_string(), option.clone());
		options_lookup.insert(option.shortcut.clone().to_string(), option);
	}

	println!("{:#?}", options_lookup);

	options
}

fn print_type_of<T>(_: &T) {
	println!("{}", std::any::type_name::<T>())
}

fn main() {
	// For API later:
	//
	// let defaults = Options::default();
	// let defaults = Options {
	// 	font: String::from("notblock"),
	// 	..Options::default()
	// };

	let cli_args: Vec<String> = args().collect();

	parse_args(&cli_args);

	println!("Hello, world!");
}
