//! Renders a cfonts banner inside a ratatui application.
//!
//! Run with:
//! ```sh
//! cargo run --example ratatui --features ratatui
//! ```
//! Press any key to exit.

use std::io::stdout;

use ratatui::{
	Terminal,
	backend::CrosstermBackend,
	crossterm::{
		event, execute,
		terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
	},
};

use cfonts::{
	environments::CfontsWidget,
	fonts::Font,
	options::{BlockOptions, Options},
};

fn main() -> std::io::Result<()> {
	let options = Options {
		blocks: vec![BlockOptions {
			text: String::from("HELLO"),
			font: Font::Block,
			word_wrap: true,
			..Default::default()
		}],
		..Default::default()
	};

	enable_raw_mode()?;
	execute!(stdout(), EnterAlternateScreen)?;
	let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

	terminal.draw(|frame| {
		frame.render_widget(&CfontsWidget { options: &options }, frame.area());
	})?;

	event::read()?;

	disable_raw_mode()?;
	execute!(stdout(), LeaveAlternateScreen)?;
	Ok(())
}
