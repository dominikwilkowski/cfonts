//! The command line parser behind the cfonts binary
//!
//! Public so cfonts compatible command lines can be built and tested against;
//! rendering itself stays with the hosts and environments

mod args;
pub use args::Args;
mod cli_parser;
pub(crate) use cli_parser::{CliBlockOptions, GradientInput, ParseState};
pub use cli_parser::{ParseError, ParseFailure, ParsedArgs, StdinProvider, parse_args};
mod help;
pub use help::cli_help;
pub(crate) mod helper;

/// The version of this crate, spelled the way the binary prints it
pub const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
