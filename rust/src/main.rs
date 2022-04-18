extern crate exitcode;

use std::collections::HashMap;
use std::env::args;

pub mod config;
pub mod helpers;

use config::{Align, BgColors, CliOption, Colors, Env, Fonts, OptionType, Options, CLIOPTIONS};

fn parse_args(args: Vec<String>) -> Options {
	let mut my_args = args.clone();
	let mut options = Options::default();

	// create a lookup table for our CLIOPTIONS
	let mut options_lookup: HashMap<String, CliOption> = HashMap::new();

	for option in CLIOPTIONS {
		options_lookup.insert(option.name.clone().to_string(), option.clone());
		options_lookup.insert(option.shortcut.clone().to_string(), option);
	}

	// our text to be converted
	options.text = my_args[1].clone();

	let mut args_length = my_args.len();
	let mut i = 2;
	// we iterate over all arguments and match them with our lookup table
	while i < args_length {
		// before we see if this flag exists in our lookup we see if boolean flags have been stacked here
		if my_args[i].starts_with("-") && !my_args[i].starts_with("--") && my_args[i].len() > 2 {
			// we know that this is a stack of boolean flags
			let mut this_flag = my_args[i].clone();
			let mut middle_flags = Vec::new();

			// unwrap is guarded by if clause it's contained in
			for flag in this_flag.strip_prefix("-").unwrap().chars() {
				let flag_name = format!("-{}", flag);
				match options_lookup.get(&flag_name) {
					Some(_) => {
						middle_flags.push(flag_name);
					}
					None => {}
				}
			}

			my_args.splice(i..i + 1, middle_flags.iter().cloned());
			args_length = my_args.len();
		}

		match options_lookup.get(&my_args[i]) {
			Some(this_flag) => {
				println!("found {:?}", this_flag.kind);
				match this_flag.kind {
					OptionType::Text => { /* Only Text type is on argvs[1] */ }
					OptionType::Font => {
						i += 1;
						if i >= args_length {
							println!("Missing value for option: {}", this_flag.name);
							std::process::exit(exitcode::USAGE);
						}
						options.font = match my_args[i].to_lowercase().as_str() {
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
					OptionType::Align => {
						i += 1;
						if i >= args_length {
							println!("Missing value for option: {}", this_flag.name);
							std::process::exit(exitcode::USAGE);
						}
						options.align = match my_args[i].to_lowercase().as_str() {
							"left" => Align::Left,
							"center" => Align::Center,
							"right" => Align::Right,
							"top" => Align::Top,
							"bottom" => Align::Bottom,
							unknown => {
								println!(
									"This alignment option is not supported: {:?}\nAllowed options are: {:?}",
									unknown,
									Align::list()
								);
								std::process::exit(exitcode::USAGE);
							}
						};
					}
					OptionType::Colors => {}
					OptionType::Color => {}
					OptionType::Number => {
						i += 1;
						if i >= args_length {
							println!("Missing value for option: {}", this_flag.name);
							std::process::exit(exitcode::USAGE);
						}
						let number = match my_args[i].parse::<u16>() {
							Ok(n) => n,
							Err(_) => {
								println!(
									"Could not read argument for option: {}. Needs to be a positive number but found instead: \"{}\"",
									this_flag.name, my_args[i]
								);
								std::process::exit(exitcode::USAGE);
							}
						};

						match this_flag.key {
							"letter_spacing" => {
								options.letter_spacing = number;
							}
							"line_height" => {
								options.line_height = number;
							}
							"max_length" => {
								options.max_length = number;
							}
							"debug_level" => {
								options.debug_level = number;
							}
							_ => {}
						}
					}
					OptionType::Bool => match this_flag.key {
						"spaceless" => {
							options.spaceless = true;
						}
						"independent_gradient" => {
							options.independent_gradient = true;
						}
						"transition_gradient" => {
							options.transition_gradient = true;
						}
						"version" => {
							options.version = true;
						}
						"debug" => {
							options.debug = true;
						}
						_ => {}
					},
					OptionType::Env => {}
				}
			}
			None => { /* (TODO: debug message) We ignore flags we don't recognize */ }
		};

		// iterating to next loop count
		i += 1;
	}

	options
}

fn main() {
	// For API later:
	//
	// let defaults = Options::default();
	// let defaults = Options {
	// 	font: String::from("notblock"),
	// 	..Options::default()
	// };

	let options = parse_args(args().collect());
	println!("{:#?}", options);

	println!("Hello, world!");
}
