pub mod environments;
pub mod fonts;
pub mod layout;
pub mod options;

#[cfg(test)]
mod tests;

pub use environments::{Environment, Rendered};

use crate::{layout::Layout, options::Options};

/// Renders the given options into the output of its environment
pub fn render(options: &Options) -> Rendered {
	let env = options.env.get_env();
	let layout = Layout::build(options, env.canvas_width());

	env.render(&layout.output, options)
}

/// Renders the given options and performs the environment's output action
/// (printing to stdout for Cli and Browser environments)
pub fn say(options: &Options) {
	options.env.get_env().say(&render(options));
}
