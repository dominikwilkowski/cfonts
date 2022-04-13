use std::collections::HashMap;
use std::env::args;

pub mod config;

use config::{CliOption, Options, CLIOPTIONS};

fn parse_args(args: &Vec<String>) -> Options {
	let mut options = Options::default();

	// create a lookup table for our CLIOPTIONS
	let mut options_lookup: HashMap<String, CliOption> = HashMap::new();

	for option in CLIOPTIONS {
		options_lookup.insert(option.name.clone().to_string(), option.clone());
		options_lookup.insert(option.shortcut.clone().to_string(), option);
	}

	// our text to be converted
	options.text = args[1].clone();

	// we iterate over all arguments and match them with our lookup table
	for i in 2..args.len() {
		match options_lookup.get(&args[i]) {
			Some(this_flag) => {
				println!("!!!{:?}", this_flag);
			}
			None => { /* (TODO: debug message) We ignore flags we don't recognize */ }
		};

		println!("{:?}", args[i]);
	}

	// println!("{:#?}", options_lookup);

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

	let options = parse_args(&args().collect());
	println!("{:#?}", options);

	println!("Hello, world!");
}
