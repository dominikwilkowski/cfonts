//! # Parsing cli arguments
//!
//! The contents of this module is all about parsing cli arguments
use std::collections::HashMap;

use crate::color::{color, get_foreground_color, hex2rgb, rgb2hex};
use crate::config::{
	Align, BgColors, CliOption, Colors, Env, Fonts, OptionType, Options, CLIOPTIONS, GRADIENTS_AGENDER,
	GRADIENTS_AROMANTIC, GRADIENTS_ASEXUAL, GRADIENTS_BISEXUAL, GRADIENTS_GENDERFLUID, GRADIENTS_GENDERQUEER,
	GRADIENTS_INTERSEX, GRADIENTS_LESBIAN, GRADIENTS_NONBINARY, GRADIENTS_PANSEXUAL, GRADIENTS_POLYSEXUAL,
	GRADIENTS_PRIDE, GRADIENTS_TRANSGENDER,
};
use crate::debug::{d, Dt};

/// This function converts command line arguments into an [`Options`] struct
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::{ Options, Align, Colors, BgColors, Fonts, Env };
/// use cfonts::args::parse;
///
/// let mut options = Options::default();
/// options.text = "long text|with new line".to_string();
/// options.font = Fonts::FontSimple3d;
/// options.align = Align::Center;
/// options.colors = vec![Colors::Blue, Colors::White];
/// options.background = BgColors::CyanBright;
/// options.letter_spacing = 9;
/// options.line_height = 2;
/// options.spaceless = true;
/// options.max_length = 100;
/// options.gradient = vec!["#ff0000".to_string(), "#0000ff".to_string()];
/// options.independent_gradient = true;
/// options.transition_gradient = true;
/// options.env = Env::Browser;
/// options.help = true;
/// options.version = true;
/// options.debug = true;
/// options.debug_level = 3;
///
/// // All shortcut flags
/// assert_eq!(
///     parse(vec![
///         "path/to/bin".to_string(),
///         "long text|with new line".to_string(),
///         "-f".to_string(),
///         "simple3d".to_string(),
///         "-a".to_string(),
///         "center".to_string(),
///         "-c".to_string(),
///         "blue,white".to_string(),
///         "-b".to_string(),
///         "cyanBright".to_string(),
///         "-l".to_string(),
///         "9".to_string(),
///         "-z".to_string(),
///         "2".to_string(),
///         "-s".to_string(),
///         "-m".to_string(),
///         "100".to_string(),
///         "-g".to_string(),
///         "red,blue".to_string(),
///         "-i".to_string(),
///         "-t".to_string(),
///         "-e".to_string(),
///         "browser".to_string(),
///         "-h".to_string(),
///         "-v".to_string(),
///         "-d".to_string(),
///         "-x".to_string(),
///         "3".to_string(),
///     ])
///     .unwrap(),
///     options
/// );
///
/// // All shortcut flags but all boolean flags are stacked
/// assert_eq!(
///     parse(vec![
///         "path/to/bin".to_string(),
///         "long text|with new line".to_string(),
///         "-f".to_string(),
///         "simple3d".to_string(),
///         "-a".to_string(),
///         "center".to_string(),
///         "-c".to_string(),
///         "blue,white".to_string(),
///         "-b".to_string(),
///         "cyanBright".to_string(),
///         "-l".to_string(),
///         "9".to_string(),
///         "-z".to_string(),
///         "2".to_string(),
///         "-sithvd".to_string(), // <-- stacked boolean flags
///         "-m".to_string(),
///         "100".to_string(),
///         "-g".to_string(),
///         "red,blue".to_string(),
///         "-e".to_string(),
///         "browser".to_string(),
///         "-x".to_string(),
///         "3".to_string(),
///     ])
///     .unwrap(),
///     options
/// );
///
/// // All long-form flags
/// assert_eq!(
///     parse(vec![
///         "path/to/bin".to_string(),
///         "long text|with new line".to_string(),
///         "--font".to_string(),
///         "simple3d".to_string(),
///         "--align".to_string(),
///         "center".to_string(),
///         "--colors".to_string(),
///         "blue,white".to_string(),
///         "--background".to_string(),
///         "cyanBright".to_string(),
///         "--letter-spacing".to_string(),
///         "9".to_string(),
///         "--line-height".to_string(),
///         "2".to_string(),
///         "-sithvd".to_string(),
///         "--max-length".to_string(),
///         "100".to_string(),
///         "--gradient".to_string(),
///         "red,blue".to_string(),
///         "--env".to_string(),
///         "browser".to_string(),
///         "--debug-level".to_string(),
///         "3".to_string(),
///     ])
///     .unwrap(),
///     options
/// );
/// ```
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

	if my_args.len() < 2 {
		let (start, end) = get_foreground_color(&Colors::Green);
		return Err(format!(
			"Please provide text to convert with: {start}cfonts \"Text\"{end}\nRun {start}cfonts --help{end} for more infos",
			start = start,
			end = end
		));
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
							return Err(format!("Missing value for option: {}", color(this_flag.name, Colors::Green, &options)));
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
									"The font \"{}\" is not supported.\nAllowed options are: {}",
									color(unknown, Colors::Green, &options),
									color(&Fonts::list(), Colors::Green, &options)
								));
							}
						};
					}
					OptionType::Align => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", color(this_flag.name, Colors::Green, &options)));
						}
						options.align = match my_args[i].to_lowercase().as_str() {
							"left" => Align::Left,
							"center" => Align::Center,
							"right" => Align::Right,
							"top" => Align::Top,
							"bottom" => Align::Bottom,
							unknown => {
								return Err(format!(
									"The alignment option \"{}\" is not supported.\nAllowed options are: {}",
									color(unknown, Colors::Green, &options),
									color(&Align::list(), Colors::Green, &options)
								));
							}
						};
					}
					OptionType::Colors => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", color(this_flag.name, Colors::Green, &options)));
						}
						options.colors = my_args[i]
							.to_lowercase()
							.as_str()
							.split(',')
							.map(|this_color| match this_color {
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
								"candy" => Ok(Colors::Candy),
								unknown => {
									if unknown.starts_with('#') && unknown.len() > 2 {
										Ok(Colors::Rgb(hex2rgb(unknown, &options)))
									} else {
										Err(format!(
											"The color \"{}\" is not supported.\nAllowed options are: {}",
											color(unknown, Colors::Green, &options),
											color(&Colors::list(), Colors::Green, &options)
										))
									}
								}
							})
							.collect::<Result<Vec<Colors>, String>>()?;
					}
					OptionType::BgColor => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", color(this_flag.name, Colors::Green, &options)));
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
								if unknown.starts_with('#') && unknown.len() > 2 {
									BgColors::Rgb(hex2rgb(unknown, &options))
								} else {
									return Err(format!(
										"The background color \"{}\" is not supported.\nAllowed options are: {}",
										color(unknown, Colors::Green, &options),
										color(&BgColors::list(), Colors::Green, &options)
									));
								}
							}
						};
					}
					OptionType::Gradient => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", color(this_flag.name, Colors::Green, &options)));
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
							.map(|this_color| match this_color {
								"black" => Ok(String::from("#000000")),
								"red" => Ok(String::from("#ff0000")),
								"green" => Ok(String::from("#00ff00")),
								"blue" => Ok(String::from("#0000ff")),
								"magenta" => Ok(String::from("#ff00ff")),
								"cyan" => Ok(String::from("#00ffff")),
								"white" => Ok(String::from("#ffffff")),
								"gray" | "grey" => Ok(String::from("#808080")),
								unknown => {
									if unknown.starts_with('#') && unknown.len() > 2 {
										// parsing hex round trip to make sure it's in a good format
										Ok(rgb2hex(&hex2rgb(unknown, &options), &options))
									} else {
										Err(format!("The gradient color \"{}\" is not supported.\nAllowed options are: black, red, green, blue, magenta, cyan, white, gray, grey", color(unknown, Colors::Green, &options)))
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
								color(&format!("{}", options.gradient.len()), Colors::Green, &options)
							));
						}

						if !is_transition && options.gradient.len() != 2 {
							return Err(format!(
								"You must specify two colors for a gradient. You specified \"{}\"",
								color(&format!("{}", options.gradient.len()), Colors::Green, &options)
							));
						}
					}
					OptionType::Number => {
						i += 1;
						if i >= args_length {
							return Err(format!("Missing value for option: {}", color(this_flag.name, Colors::Green, &options)));
						}
						let number = match my_args[i].parse::<u16>() {
							Ok(n) => n,
							Err(_) => {
								return Err(format!(
									"Could not read argument for option: {}. Needs to be a positive number but found instead: \"{}\"",
									color(this_flag.name, Colors::Green, &options),
									color(&my_args[i], Colors::Green, &options)
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
							return Err(format!("Missing value for option: {}", color(this_flag.name, Colors::Green, &options)));
						}
						options.env = match my_args[i].to_lowercase().as_str() {
							"node" | "cli" => Env::Cli,
							"browser" => Env::Browser,
							unknown => {
								return Err(format!(
									"The env option \"{}\" is not supported.\nAllowed options are: {}",
									color(unknown, Colors::Green, &options),
									color(&Env::list(), Colors::Green, &options)
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
