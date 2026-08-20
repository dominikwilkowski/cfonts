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

use cfonts::{Align, Cfonts, CfontsWidget, Color, Font, GradientOption, GradientStop, Options};

fn run(options: &Options) -> std::io::Result<()> {
	let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

	terminal.draw(|frame| {
		frame.render_widget(&CfontsWidget { options, seed: 0 }, frame.area());
	})?;

	event::read()?;

	Ok(())
}

fn main() -> std::io::Result<()> {
	// the builder is the primary API; the widget consumes the underlying options
	// and re-wraps and re-aligns on every terminal resize
	// gradients are block colors too: the widget ramps them one cell per column
	let options: Options = Cfonts::text("hello")
		.font(Font::Block)
		.word_wrap()
		.align(Align::Center)
		.colors(vec![Color::Yellow, Color::Blue])
		.new_text(" there")
		.font(Font::Tiny)
		.colors(GradientOption::TwoStop {
			start: GradientStop::Red,
			end: GradientStop::Blue,
			independent_gradient: false,
		})
		.into();

	enable_raw_mode()?;

	if let Err(error) = execute!(stdout(), EnterAlternateScreen) {
		disable_raw_mode()?;
		return Err(error);
	}

	// hold the result so the terminal is always restored, even when the app errors
	let result = run(&options);

	disable_raw_mode()?;
	execute!(stdout(), LeaveAlternateScreen)?;

	result
}
