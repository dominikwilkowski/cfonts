//! The command line parser behind the cfonts binary
//!
//! Public so cfonts compatible command lines can be built and tested against
//! rendering itself stays with the hosts and targets

mod args;
pub use args::Args;
mod docs;
mod process;
pub use process::Process;
mod cli_parser;
pub(crate) use cli_parser::{CliBlockOptions, ParseState};
pub use cli_parser::{ParseError, ParseFailure, ParsedArgs, StdinError, StdinProvider, parse_args};
mod help;
pub use help::cli_help;
mod demo;
pub use demo::cli_demo;

/// The version of this crate, spelled the way the binary prints it
pub const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

/// The shell prompt the error messages and the demo open an example line with, its dollar in bold
///
/// The help compiles the same prompt in `cfonts-macros`, a test in `cli/docs.rs` holds the two together
pub(crate) const PROMPT_COLORED: &str = "  \x1B[1m$\x1B[0m";

/// The same prompt without any style code
pub(crate) const PROMPT_PLAIN: &str = "  $";
