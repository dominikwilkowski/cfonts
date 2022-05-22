//! The contents of this module is all about cli specific functionality
use crate::color::color;
use crate::config::{Align, BgColors, Colors, Env, Fonts, OptionType, Options, CLIOPTIONS};
use crate::debug::{d, Dt};
use crate::render::render;

/// Return the string to display when the `version` option is passed in via the [`Options`] struct
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::cli::version;
///
/// let options = Options::default();
/// assert_eq!(version(&options), format!("v{}", env!("CARGO_PKG_VERSION")));
/// ```
pub fn version(options: &Options) -> String {
	d("cli::version()", 5, Dt::Head, options, &mut std::io::stdout());

	let version = env!("CARGO_PKG_VERSION");
	d(&format!("cli::version() version: {}", version), 5, Dt::Log, options, &mut std::io::stdout());

	format!("v{}", version)
}

/// Return the string to display when the `help` option is passed in via the [`Options`] struct
///
/// ```rust
/// extern crate cfonts;
///
/// use cfonts::Options;
/// use cfonts::cli::help;
///
/// let options = Options::default();
/// assert!(help(&options).contains("Usage:"));
/// ```
pub fn help(options: &Options) -> String {
	d("cli::help()", 5, Dt::Head, options, &mut std::io::stdout());

	let mut output = String::new();
	let render_options = render(Options {
		text: String::from("cfonts"),
		align: Align::Left,
		gradient: vec![String::from("#ff0000"), String::from("#00ff00")],
		spaceless: true,
		..Options::default()
	});

	output += "\n\n";
	output += &render_options.text;
	output += "\n\n";
	output += "This is a tool for sexy fonts in the console. Give your cli some love.\n";
	output += "\n";
	output += "Usage: cfonts \"<value>\" [option1] <input1> [option2] <input1>,<input2> [option3]\n";
	output += "Example: \x1b[1m$ cfonts \"sexy font\" -f chrome -a center -c red,green,gray\x1b[22m\n";
	output += "\n";
	output += "Options:\n";

	for option in CLIOPTIONS {
		output += &format!("\n\x1b[1m{}, {}", option.name, option.shortcut);
		if !option.fallback_shortcut.is_empty() {
			output += &format!(", {}", option.fallback_shortcut);
		}
		output += "\x1b[22m\n";
		output += &format!("{}\n", option.description);
		output += &format!("\x1b[1m$\x1b[22m cfonts {}", option.example);
		match option.kind {
			OptionType::Font => {
				output += &color(&format!(" [ {} ]", Fonts::list()), Colors::Green, options).to_string();
			}
			OptionType::Colors => {
				output += &color(&format!(" [ {} ]", Colors::list()), Colors::Green, options).to_string();
			}
			OptionType::BgColor => {
				output += &color(&format!(" [ {} ]", BgColors::list()), Colors::Green, options).to_string();
			}
			OptionType::Align => {
				output += &color(&format!(" [ {} ]", Align::list()), Colors::Green, options).to_string();
			}
			OptionType::Env => {
				output += &color(&format!(" [ {} ]", Env::list()), Colors::Green, options).to_string();
			}
			_ => {}
		}
		output += "\n";
	}

	output
}
