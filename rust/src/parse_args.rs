use std::collections::HashMap;

use crate::config::{Align, BgColors, CliOption, Colors, Env, Fonts, OptionType, Options, CLIOPTIONS};

fn convert_hex_to_rgb(hex: &str) -> [u8; 3] {
	let mut rgb = [0_u8, 0_u8, 0_u8];
	let clean_hex = hex.strip_prefix('#').unwrap();

	match clean_hex.len() {
		3 => {
			let r = format!("{r}{r}", r = &clean_hex[0..1]);
			rgb[0] = i64::from_str_radix(&r, 16).unwrap_or(255) as u8;

			let g = format!("{g}{g}", g = &clean_hex[1..2]);
			rgb[1] = i64::from_str_radix(&g, 16).unwrap_or(255) as u8;

			let b = format!("{b}{b}", b = &clean_hex[2..3]);
			rgb[2] = i64::from_str_radix(&b, 16).unwrap_or(255) as u8;
		}
		6 => {
			rgb[0] = i64::from_str_radix(&clean_hex[0..2], 16).unwrap_or(255) as u8;

			rgb[1] = i64::from_str_radix(&clean_hex[2..4], 16).unwrap_or(255) as u8;

			rgb[2] = i64::from_str_radix(&clean_hex[4..6], 16).unwrap_or(255) as u8;
		}
		_ => {}
	};

	rgb
}

#[test]
fn convert_hex_to_rgb_works() {
	assert_eq!(convert_hex_to_rgb("#f80"), [255, 136, 0]);
	assert_eq!(convert_hex_to_rgb("#ff8800"), [255, 136, 0]);

	assert_eq!(convert_hex_to_rgb("#fff"), [255, 255, 255]);
	assert_eq!(convert_hex_to_rgb("#ffffff"), [255, 255, 255]);

	assert_eq!(convert_hex_to_rgb("#000"), [0, 0, 0]);
	assert_eq!(convert_hex_to_rgb("#000000"), [0, 0, 0]);

	assert_eq!(convert_hex_to_rgb("#08f"), [0, 136, 255]);
	assert_eq!(convert_hex_to_rgb("#0088ff"), [0, 136, 255]);

	assert_eq!(convert_hex_to_rgb("#xxx"), [255, 255, 255]);
	assert_eq!(convert_hex_to_rgb("#xxxxxx"), [255, 255, 255]);
}

pub fn parse_args(args: Vec<String>) -> Options {
	let mut my_args = args;
	let mut options = Options::default();

	// create a lookup table for our CLIOPTIONS
	let mut options_lookup: HashMap<String, CliOption> = HashMap::new();

	for option in CLIOPTIONS {
		let name = option.name.to_string();
		let shortcut = option.shortcut.to_string();
		options_lookup.insert(name, option.clone());
		options_lookup.insert(shortcut, option);
	}

	// our text to be converted
	options.text = my_args[1].clone();

	let mut args_length = my_args.len();
	let mut i = 2; // we skip the first two arguments as the first is path to binary and the second we already take care of above
							 // we iterate over all arguments and match them with our lookup table
	while i < args_length {
		// before we see if this flag exists in our lookup we see if boolean flags have been stacked here
		if my_args[i].starts_with('-') && !my_args[i].starts_with("--") && my_args[i].len() > 2 {
			// we know that this is a stack of boolean flags
			let this_flag = my_args[i].clone();
			let mut middle_flags = Vec::new();

			// unwrap is guarded by if clause it's contained in
			for flag in this_flag.strip_prefix('-').unwrap().chars() {
				let flag_name = format!("-{}", flag);
				if options_lookup.get(&flag_name).is_some() {
					middle_flags.push(flag_name);
				}
			}

			my_args.splice(i..i + 1, middle_flags.iter().cloned());
			args_length = my_args.len();
		}

		match options_lookup.get(&my_args[i]) {
			Some(this_flag) => {
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
								println!("The font \"{}\" is not supported.\nAllowed options are: {:?}", unknown, Fonts::list());
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
									"The alignment option \"{}\" is not supported.\nAllowed options are: {:?}",
									unknown,
									Align::list()
								);
								std::process::exit(exitcode::USAGE);
							}
						};
					}
					OptionType::Colors => {
						i += 1;
						if i >= args_length {
							println!("Missing value for option: {}", this_flag.name);
							std::process::exit(exitcode::USAGE);
						}
						let colors = my_args[i]
							.to_lowercase()
							.as_str()
							.split(',')
							.map(|color| match color {
								"system" => Colors::System,
								"black" => Colors::Black,
								"red" => Colors::Red,
								"green" => Colors::Green,
								"yellow" => Colors::Yellow,
								"blue" => Colors::Blue,
								"magenta" => Colors::Magenta,
								"cyan" => Colors::Cyan,
								"white" => Colors::White,
								"gray" => Colors::Gray,
								"grey" => Colors::Gray,
								"redbright" => Colors::RedBright,
								"greenbright" => Colors::GreenBright,
								"yellowbright" => Colors::YellowBright,
								"bluebright" => Colors::BlueBright,
								"magentabright" => Colors::MagentaBright,
								"cyanbright" => Colors::CyanBright,
								"whitebright" => Colors::WhiteBright,
								unknown => {
									if unknown.starts_with('#') && unknown.len() == 4 || unknown.starts_with('#') && unknown.len() == 7 {
										Colors::Rgb(convert_hex_to_rgb(unknown))
									} else {
										println!("The color \"{}\" is not supported.\nAllowed options are: {:?}", unknown, Colors::list());
										std::process::exit(exitcode::USAGE);
									}
								}
							})
							.collect::<Vec<Colors>>();

						match this_flag.key {
							"colors" => {
								options.colors = colors;
							}
							"gradient" => {
								options.gradient = colors;
							}
							_ => {}
						}
					}
					OptionType::Color => {
						i += 1;
						if i >= args_length {
							println!("Missing value for option: {}", this_flag.name);
							std::process::exit(exitcode::USAGE);
						}
						options.background = match my_args[i].to_lowercase().as_str() {
							"transparent" => BgColors::Transparent,
							"black" => BgColors::Black,
							"red" => BgColors::Red,
							"green" => BgColors::Green,
							"yellow" => BgColors::Yellow,
							"blue" => BgColors::Blue,
							"magenta" => BgColors::Magenta,
							"cyan" => BgColors::Cyan,
							"white" => BgColors::White,
							"gray" => BgColors::Gray,
							"grey" => BgColors::Gray,
							"redbright" => BgColors::RedBright,
							"greenbright" => BgColors::GreenBright,
							"yellowbright" => BgColors::YellowBright,
							"bluebright" => BgColors::BlueBright,
							"magentabright" => BgColors::MagentaBright,
							"cyanbright" => BgColors::CyanBright,
							"whitebright" => BgColors::WhiteBright,
							unknown => {
								if unknown.starts_with('#') && unknown.len() == 4 || unknown.starts_with('#') && unknown.len() == 7 {
									BgColors::Rgb(convert_hex_to_rgb(unknown))
								} else {
									println!(
										"The background color \"{}\" is not supported.\nAllowed options are: {:?}",
										unknown,
										BgColors::list()
									);
									std::process::exit(exitcode::USAGE);
								}
							}
						};
					}
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
						"version" => {
							options.version = true;
						}
						"help" => {
							options.help = true;
						}
						"spaceless" => {
							options.spaceless = true;
						}
						"independent_gradient" => {
							options.independent_gradient = true;
						}
						"transition_gradient" => {
							options.transition_gradient = true;
						}
						"debug" => {
							options.debug = true;
						}
						_ => {}
					},
					OptionType::Env => {
						i += 1;
						if i >= args_length {
							println!("Missing value for option: {}", this_flag.name);
							std::process::exit(exitcode::USAGE);
						}
						options.env = match my_args[i].to_lowercase().as_str() {
							"node" => Env::Node,
							"browser" => Env::Browser,
							unknown => {
								println!("The env option \"{}\" is not supported.\nAllowed options are: {:?}", unknown, Env::list());
								std::process::exit(exitcode::USAGE);
							}
						};
					}
				}
			}
			None => {
				println!("TODO debug message");
				/* (TODO: debug message) We ignore flags we don't recognize */
			}
		};

		// iterating to next loop count
		i += 1;
	}

	options
}
