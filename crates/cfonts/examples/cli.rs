use cfonts::{Align, Cfonts, CliEnv, Font};

fn main() {
	// say() prints to stdout at the detected terminal width
	// (the FORCE_SIZE environment variable overrides the detection)
	Cfonts::text("hello").font(Font::Block).new_text("world").font(Font::Tiny).say(&CliEnv::default());

	// render() returns the artifact instead, so the host decides what happens with it
	// the environment is a value: a host that knows its width passes it along
	let rendered =
		Cfonts::text("hello world").font(Font::Tiny).align(Align::Center).render(&CliEnv { canvas_width: Some(40) });

	println!("{}", rendered.text);
}
