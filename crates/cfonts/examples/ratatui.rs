//! A cfonts banner with a small color picker
//!
//! Run with:
//! ```sh
//! cargo run --example ratatui --features ratatui
//! ```

use std::io;

use ratatui::{
	DefaultTerminal,
	buffer::Buffer,
	crossterm::event::{self, KeyCode, KeyModifiers},
	layout::{Constraint, Layout, Rect},
	text::Line,
	widgets::{Block, Tabs, Widget},
};

use cfonts::{
	Align, Cfonts, CfontsWidget, Color, ColorOption, Font, GradientOption, GradientPreset, GradientStop, Options,
	RustHost,
};

/// One entry of the picker, the label with its configuration, so adding a choice needs one entry
struct Choice {
	label: &'static str,
	colors: ColorOption,
	/// Candy picks hold across redraws by default, a rolling choice takes a fresh seed on every event
	rolling: bool,
}

/// The picker's choices with their selection, and the composition they color
struct App {
	choices: Vec<Choice>,
	selected: usize,
	options: Options,
	/// Rolled once per run and again on every event while a rolling choice is selected
	seed: u64,
	exit: bool,
}

impl App {
	fn new() -> Self {
		let choices = vec![
			Choice { label: "Red", colors: vec![Color::Red].into(), rolling: false },
			Choice { label: "Candy Fixed", colors: vec![Color::Candy].into(), rolling: false },
			Choice { label: "Candy Random", colors: vec![Color::Candy].into(), rolling: true },
			Choice { label: "Cyan,Magenta", colors: vec![Color::Cyan, Color::Magenta].into(), rolling: false },
			Choice {
				label: "Red-blue",
				colors: GradientOption::TwoStop { start: GradientStop::Red, end: GradientStop::Blue }.into(),
				rolling: false,
			},
			Choice {
				label: "Green-magenta",
				colors: GradientOption::TwoStop { start: GradientStop::Green, end: GradientStop::Magenta }.into(),
				rolling: false,
			},
			Choice { label: "Nonbinary", colors: GradientPreset::Nonbinary.into(), rolling: false },
		];
		let options: Options = Cfonts::text("Hello Ratatui")
			.font(Font::Neat)
			.word_wrap()
			.align(Align::Center)
			.next("||Sexy fonts in the console")
			.word_wrap()
			.into();

		let mut app = Self { choices, selected: 0, options, seed: RustHost::entropy(), exit: false };
		app.select(0);

		app
	}

	/// Moves the selection and colors the composition with it, the one place the options change
	fn select(&mut self, selected: usize) {
		self.selected = selected;
		self.options.global_colors = Some(self.choices[selected].colors.clone());
	}

	/// Draws the state and waits for the next event until a key asks to exit
	fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
		while !self.exit {
			terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
			self.handle_events()?;
		}

		Ok(())
	}

	/// Resize events reach the next draw on their own, only key presses change the state
	fn handle_events(&mut self) -> io::Result<()> {
		if let Some(key) = event::read()?.as_key_press_event() {
			match key.code {
				KeyCode::Left => self.select((self.selected + self.choices.len() - 1) % self.choices.len()),
				KeyCode::Right => self.select((self.selected + 1) % self.choices.len()),
				KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
				KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => self.exit = true,
				_ => {}
			}
		}

		// every event, a key or a resize, ends in a redraw, so a rolling choice picks anew each time
		if self.choices[self.selected].rolling {
			self.seed = RustHost::entropy();
		}

		Ok(())
	}
}

impl Widget for &App {
	fn render(self, area: Rect, buffer: &mut Buffer) {
		let [picker, preview] = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);

		Tabs::new(self.choices.iter().map(|Choice { label, .. }| *label)).select(self.selected).render(picker, buffer);

		let block = Block::bordered()
			.title(" Ratatui Example ")
			.title_bottom(Line::from(" ←/→ color · q quit · resize to wrap · any other key to re-roll ").centered());
		// The panel's inner width controls the banner's wrapping and alignment
		let inner = block.inner(preview);
		block.render(preview, buffer);
		CfontsWidget { options: &self.options, seed: self.seed }.render(inner, buffer);
	}
}

fn main() -> io::Result<()> {
	// Ratatui handles raw mode, the alternate screen and restoring the terminal
	ratatui::run(|terminal| App::new().run(terminal))
}
