use crate::config::Options;
use crate::debug::{d, Dt};

pub fn version(options: &Options) -> String {
	d("cli::version()", 1, Dt::Head, options, &mut std::io::stdout());

	let version = env!("CARGO_PKG_VERSION");
	d(&format!("cli::version() version: {}", version), 3, Dt::Log, options, &mut std::io::stdout());

	format!("v{}", version)
}
