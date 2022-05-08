use std::collections::HashMap;

use crate::color::{hex2rgb, rgb2hex};
use crate::config::{
	Align, BgColors, CliOption, Colors, Env, Fonts, OptionType, Options, CLIOPTIONS, GRADIENTS_AGENDER,
	GRADIENTS_AROMANTIC, GRADIENTS_ASEXUAL, GRADIENTS_BISEXUAL, GRADIENTS_GENDERFLUID, GRADIENTS_GENDERQUEER,
	GRADIENTS_INTERSEX, GRADIENTS_LESBIAN, GRADIENTS_NONBINARY, GRADIENTS_PANSEXUAL, GRADIENTS_POLYSEXUAL,
	GRADIENTS_PRIDE, GRADIENTS_TRANSGENDER,
};
use crate::debug::{d, Dt};

pub fn parse(args: Vec<String>) -> Result<Options, String> {
	let mut my_args = args;
	let mut options = Options::default();

	// create a lookup table for our CLIOPTIONS
	let mut options_lookup: HashMap<String, CliOption> = HashMap::new();

	for option in CLIOPTIONS {
		let name = option.name.to_string();
		let shortcut = option.shortcut.to_string();
		options_lookup.insert(name, option.clone());
		options_lookup.insert(shortcut, option.clone());
		if !option.fallback_shortcut.is_empty() {
			let shortcut = option.fallback_shortcut.to_string();
			options_lookup.insert(shortcut, option);
		}
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
							return Err(format!("Missing value for option: {}", this_flag.name));
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
								return Err(format!(
									"The font \"{}\" is not supported.\nAllowed options are: {:?}",
									unknown,
									Fonts::list()
								));
							}
						};
					}
					OptionType::Align => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", this_flag.name));
						}
						options.align = match my_args[i].to_lowercase().as_str() {
							"left" => Align::Left,
							"center" => Align::Center,
							"right" => Align::Right,
							"top" => Align::Top,
							"bottom" => Align::Bottom,
							unknown => {
								return Err(format!(
									"The alignment option \"{}\" is not supported.\nAllowed options are: {:?}",
									unknown,
									Align::list()
								));
							}
						};
					}
					OptionType::Colors => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", this_flag.name));
						}
						options.colors = my_args[i]
							.to_lowercase()
							.as_str()
							.split(',')
							.map(|color| match color {
								"system" => Ok(Colors::System),
								"black" => Ok(Colors::Black),
								"red" => Ok(Colors::Red),
								"green" => Ok(Colors::Green),
								"yellow" => Ok(Colors::Yellow),
								"blue" => Ok(Colors::Blue),
								"magenta" => Ok(Colors::Magenta),
								"cyan" => Ok(Colors::Cyan),
								"white" => Ok(Colors::White),
								"gray" => Ok(Colors::Gray),
								"grey" => Ok(Colors::Gray),
								"redbright" => Ok(Colors::RedBright),
								"greenbright" => Ok(Colors::GreenBright),
								"yellowbright" => Ok(Colors::YellowBright),
								"bluebright" => Ok(Colors::BlueBright),
								"magentabright" => Ok(Colors::MagentaBright),
								"cyanbright" => Ok(Colors::CyanBright),
								"whitebright" => Ok(Colors::WhiteBright),
								unknown => {
									if unknown.starts_with('#') && unknown.len() == 4 || unknown.starts_with('#') && unknown.len() == 7 {
										Ok(Colors::Rgb(hex2rgb(unknown, &options)))
									} else {
										Err(format!(
											"The color \"{}\" is not supported.\nAllowed options are: {:?}",
											unknown,
											Colors::list()
										))
									}
								}
							})
							.collect::<Result<Vec<Colors>, String>>()?;
					}
					OptionType::Color => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", this_flag.name));
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
									BgColors::Rgb(hex2rgb(unknown, &options))
								} else {
									return Err(format!(
										"The background color \"{}\" is not supported.\nAllowed options are: {:?}",
										unknown,
										BgColors::list()
									));
								}
							}
						};
					}
					OptionType::Gradient => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", this_flag.name));
						}
						let expanded_args = match my_args[i].to_lowercase().as_str() {
							"lgbt" | "lgbtq" | "lgbtqa" | "pride" => {
								options.transition_gradient = true;
								GRADIENTS_PRIDE.join(",")
							}
							"agender" => {
								options.transition_gradient = true;
								GRADIENTS_AGENDER.join(",")
							}
							"aromantic" => {
								options.transition_gradient = true;
								GRADIENTS_AROMANTIC.join(",")
							}
							"asexual" => {
								options.transition_gradient = true;
								GRADIENTS_ASEXUAL.join(",")
							}
							"bisexual" | "bi" => {
								options.transition_gradient = true;
								GRADIENTS_BISEXUAL.join(",")
							}
							"genderfluid" => {
								options.transition_gradient = true;
								GRADIENTS_GENDERFLUID.join(",")
							}
							"genderqueer" => {
								options.transition_gradient = true;
								GRADIENTS_GENDERQUEER.join(",")
							}
							"intersex" => {
								options.transition_gradient = true;
								GRADIENTS_INTERSEX.join(",")
							}
							"lesbian" => {
								options.transition_gradient = true;
								GRADIENTS_LESBIAN.join(",")
							}
							"nonbinary" => {
								options.transition_gradient = true;
								GRADIENTS_NONBINARY.join(",")
							}
							"pansexual" | "pan" => {
								options.transition_gradient = true;
								GRADIENTS_PANSEXUAL.join(",")
							}
							"polysexual" | "poly" => {
								options.transition_gradient = true;
								GRADIENTS_POLYSEXUAL.join(",")
							}
							"transgender" | "trans" => {
								options.transition_gradient = true;
								GRADIENTS_TRANSGENDER.join(",")
							}
							unknown => unknown.to_string(),
						};

						options.gradient = expanded_args
							.split(',')
							.map(|color| match color {
								"black" => Ok(String::from("#000000")),
								"red" => Ok(String::from("#ff0000")),
								"green" => Ok(String::from("#00ff00")),
								"blue" => Ok(String::from("#0000ff")),
								"magenta" => Ok(String::from("#ff00ff")),
								"cyan" => Ok(String::from("#00ffff")),
								"white" => Ok(String::from("#ffffff")),
								"gray" | "grey" => Ok(String::from("#808080")),
								unknown => {
									if unknown.starts_with('#') && unknown.len() == 4 || unknown.starts_with('#') && unknown.len() == 7 {
										// parsing hex round trip to make sure it's in a good format
										Ok(rgb2hex(&hex2rgb(unknown, &options), &options))
									} else {
										Err(format!("The gradient color \"{}\" is not supported.\nAllowed options are: black, red, green, blue, magenta, cyan, white, gray, grey", unknown))
									}
								}
							}).collect::<Result<Vec<String>,String>>()?;

						let transition_options = options_lookup.get("-t").unwrap();
						let is_transition = my_args.contains(&transition_options.name.to_string())
							|| my_args.contains(&transition_options.shortcut.to_string())
							|| options.transition_gradient;
						if is_transition && options.gradient.len() < 2 {
							return Err(format!(
								"You must specify at least two colors for transition gradients. You specified only \"{}\"",
								options.gradient.len()
							));
						}

						if !is_transition && options.gradient.len() != 2 {
							return Err(format!(
								"You must specify two colors for a gradient. You specified \"{}\"",
								options.gradient.len()
							));
						}
					}
					OptionType::Number => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", this_flag.name));
						}
						let number = match my_args[i].parse::<u16>() {
							Ok(n) => n,
							Err(_) => {
								return Err(format!(
									"Could not read argument for option: {}. Needs to be a positive number but found instead: \"{}\"",
									this_flag.name, my_args[i]
								));
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
							return Err(format!("Missing value for option: {}", this_flag.name));
						}
						options.env = match my_args[i].to_lowercase().as_str() {
							"node" | "cli" => Env::Cli,
							"browser" => Env::Browser,
							unknown => {
								return Err(format!(
									"The env option \"{}\" is not supported.\nAllowed options are: {:?}",
									unknown,
									Env::list()
								));
							}
						};
					}
				}
			}
			None => {
				/* We ignore flags we don't recognize */
				d(&format!("CLI flag \"{}\" was ignored", my_args[i]), 1, Dt::Log, &options, &mut std::io::stdout());
				// note this will only debug print flags after the encounter the debug flag
			}
		};

		// iterating to next loop count
		i += 1;
	}

	Ok(options)
}
