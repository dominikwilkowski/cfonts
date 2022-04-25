use crate::color::color;
use crate::config::{Align, BgColors, Colors, Env, Fonts, OptionType, Options, CLIOPTIONS};
use crate::debug::{d, Dt};

pub fn version(options: &Options) -> String {
	d("cli::version()", 1, Dt::Head, options, &mut std::io::stdout());

	let version = env!("CARGO_PKG_VERSION");
	d(&format!("cli::version() version: {}", version), 3, Dt::Log, options, &mut std::io::stdout());

	format!("v{}", version)
}

pub fn help(options: &Options) -> String {
	d("cli::help()", 1, Dt::Head, options, &mut std::io::stdout());

	let mut output = String::new();
	// TODO cfonts banner with gradient

	output += "\n";
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
				output += &color(&format!(" [ {} ]", Fonts::list()), Colors::Green).to_string();
			}
			OptionType::Colors => {
				output += &color(&format!(" [ {} ]", Colors::list()), Colors::Green).to_string();
			}
			OptionType::Color => {
				output += &color(&format!(" [ {} ]", BgColors::list()), Colors::Green).to_string();
			}
			OptionType::Align => {
				output += &color(&format!(" [ {} ]", Align::list()), Colors::Green).to_string();
			}
			OptionType::Env => {
				output += &color(&format!(" [ {} ]", Env::list()), Colors::Green).to_string();
			}
			_ => {}
		}
		output += "\n";
	}

	output
}
