//! Your favorite rust -> wasm workflow tool!

#![deny(missing_docs)]

extern crate cargo_metadata;
extern crate console;
extern crate strsim;
#[macro_use]
extern crate failure;
extern crate glob;
extern crate parking_lot;
extern crate semver;
extern crate serde;
extern crate which;
#[macro_use]
extern crate serde_derive;
extern crate serde_ignored;
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate binary_install;
extern crate chrono;
extern crate curl;
extern crate dialoguer;
extern crate log;
extern crate toml;
extern crate walkdir;

pub mod bindgen;
pub mod build;
pub mod cache;
pub mod child;
pub mod command;
pub mod emoji;
pub mod generate;
pub mod install;
pub mod license;
pub mod lockfile;
pub mod manifest;
pub mod npm;
pub mod progressbar;
pub mod readme;
pub mod stamps;
pub mod target;
pub mod test;
pub mod wasm_opt;

use progressbar::ProgressOutput;

/// The global progress bar and user-facing message output.
pub static PBAR: ProgressOutput = ProgressOutput;

/// 📦 ✨  pack and publish your wasm!
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// The subcommand to run.
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub cmd: command::Command,

    /// Log verbosity is based off the number of v used
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    pub verbosity: u8,
}
