//! Hosts resolve runtime capabilities into a context and perform the output action

use crate::{
	environments::{Environment, Rendered},
	options::Options,
	render::{RenderContext, render_with},
};

#[cfg(not(target_arch = "wasm32"))]
mod rust;
#[cfg(not(target_arch = "wasm32"))]
pub use rust::RustHost;

/// Resolves runtime capabilities and performs host-specific output
///
/// A host may use different environments for returned and emitted artifacts
/// Browser hosts use this distinction to return HTML from `render` while
/// emitting browser-console output from `say`
pub trait Host {
	/// Environment used when returning an artifact
	type RenderEnvironment: Environment + ?Sized;

	/// Environment used when performing the host's output action
	type SayEnvironment: Environment + ?Sized;

	/// Error returned by the host's output action
	type Error;

	/// Returns the environment used by [`render`](Self::render)
	fn render_environment(&self) -> &Self::RenderEnvironment;

	/// Returns the environment used by [`say`](Self::say)
	fn say_environment(&self) -> &Self::SayEnvironment;

	/// Resolves host capabilities once for one render operation
	fn resolve_context(&self) -> RenderContext;

	/// Performs the host-specific output action
	fn write(&self, rendered: &Rendered) -> Result<(), Self::Error>;

	/// Resolves context once and returns one rendered artifact
	#[must_use]
	fn render(&self, options: &Options) -> Rendered {
		let context = self.resolve_context();

		render_with(options, self.render_environment(), context)
	}

	/// Resolves context once, renders through the say environment and writes once
	fn say(&self, options: &Options) -> Result<(), Self::Error> {
		let context = self.resolve_context();
		let rendered = render_with(options, self.say_environment(), context);

		self.write(&rendered)
	}
}

#[cfg(test)]
mod tests {
	use std::{
		cell::{Cell, RefCell},
		convert::Infallible,
	};

	use super::Host;
	use crate::{Environment, Options, RenderContext, Rendered};

	struct SpyEnvironment {
		marker: &'static str,
		render_calls: Cell<usize>,
	}

	impl SpyEnvironment {
		fn new(marker: &'static str) -> Self {
			Self {
				marker,
				render_calls: Cell::new(0),
			}
		}
	}

	impl Environment for SpyEnvironment {
		fn wrapper_start(&self, _options: &Options, out: &mut Rendered) {
			self.render_calls.set(self.render_calls.get() + 1);

			out.text.push_str(self.marker);
		}
	}

	struct SpyHost {
		render_environment: SpyEnvironment,
		say_environment: SpyEnvironment,
		context_resolutions: Cell<usize>,
		write_calls: Cell<usize>,
		written: RefCell<String>,
	}

	impl Default for SpyHost {
		fn default() -> Self {
			Self {
				render_environment: SpyEnvironment::new("render"),
				say_environment: SpyEnvironment::new("say"),
				context_resolutions: Cell::new(0),
				write_calls: Cell::new(0),
				written: RefCell::new(String::new()),
			}
		}
	}

	impl Host for SpyHost {
		type RenderEnvironment = SpyEnvironment;
		type SayEnvironment = SpyEnvironment;
		type Error = Infallible;

		fn render_environment(&self) -> &Self::RenderEnvironment {
			&self.render_environment
		}

		fn say_environment(&self) -> &Self::SayEnvironment {
			&self.say_environment
		}

		fn resolve_context(&self) -> RenderContext {
			self.context_resolutions.set(self.context_resolutions.get() + 1);

			RenderContext::unlimited()
		}

		fn write(&self, rendered: &Rendered) -> Result<(), Self::Error> {
			self.write_calls.set(self.write_calls.get() + 1);
			self.written.replace(rendered.text.clone());

			Ok(())
		}
	}

	#[test]
	fn render_resolves_once_and_uses_only_the_render_environment() {
		let host = SpyHost::default();

		let rendered = Host::render(&host, &Options::default());

		assert_eq!(rendered.text, "render");
		assert_eq!(host.context_resolutions.get(), 1);
		assert_eq!(host.render_environment.render_calls.get(), 1,);
		assert_eq!(host.say_environment.render_calls.get(), 0,);
		assert_eq!(host.write_calls.get(), 0);
		assert!(host.written.borrow().is_empty());
	}

	#[test]
	fn say_resolves_once_renders_once_and_writes_once() {
		let host = SpyHost::default();

		Host::say(&host, &Options::default()).expect("the spy host cannot fail");

		assert_eq!(host.context_resolutions.get(), 1);
		assert_eq!(host.render_environment.render_calls.get(), 0,);
		assert_eq!(host.say_environment.render_calls.get(), 1,);
		assert_eq!(host.write_calls.get(), 1);
		assert_eq!(host.written.borrow().as_str(), "say",);
	}
}
