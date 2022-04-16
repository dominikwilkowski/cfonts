extern crate exitcode;

use std::collections::HashMap;
use std::env::args;

pub mod config;
pub mod helpers;

use config::{CliOption, Fonts, OptionType, Options, CLIOPTIONS};

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
	for mut i in 2..args.len() {
		match options_lookup.get(&args[i]) {
			Some(this_flag) => {
				println!("found {:?}", this_flag.kind);
				match this_flag.kind {
					OptionType::Text => { /* Only Text type is on argvs[1] */ }
					OptionType::Font => {
						i += 1;
						options.font = match args[i].to_lowercase().as_str() {
							"console" => Fonts::FontConsole,
							"block" => Fonts::FontBlock,
							"simpleblock" => Fonts::FontSimpleBlock,
							"simple" => Fonts::FontSimple,
							"3d" => Fonts::Font3d,
							"simple3d" => Fonts::FontSimple3d,
							"chrome" => Fonts::FontChrome,
							"huge" => Fonts::FontHuge,
							"shade" => Fonts::FontShade,
							"slick" => Fonts::FontSlick,
							"grid" => Fonts::FontGrid,
							"pallet" => Fonts::FontPallet,
							"tiny" => Fonts::FontTiny,
							unknown => {
								println!("This font is not supported: {:?}\nAllowed options are: {:?}", unknown, Fonts::list());
								std::process::exit(exitcode::USAGE);
							}
						};
					}
					OptionType::Align => {}
					OptionType::Colors => {}
					OptionType::Color => {}
					OptionType::Number => {}
					OptionType::Bool => {}
					OptionType::Env => {}
				}
				// if this_flag.getter.len() > 0 {
				// 	println!("getter {:?}", this_flag);
				// 	// i += 1;
				// 	// let value = this_flag.get(&args[i]);
				// 	// println!("value {:?}", value);
				// }
			}
			None => { /* (TODO: debug message) We ignore flags we don't recognize */ }
		};
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
