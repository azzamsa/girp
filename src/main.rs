#![deny(unsafe_code)]
use std::io::{self, Write};
use std::process;

use clap::Parser;
use miette::Result;
use owo_colors::OwoColorize;

use girp::cli::Opts;
use girp::exit_codes::ExitCode;

fn main() {
    let result = run();
    match result {
        Ok(exit_code) => {
            process::exit(exit_code.into());
        }
        Err(err) => {
            writeln!(io::stderr(), "Error: {:?}", err).ok();
            process::exit(ExitCode::GeneralError.into());
        }
    }
}

fn run() -> Result<ExitCode> {
    let _opts = Opts::parse();
    println!("{}", "Hello, world!".blue());
    Ok(ExitCode::Success)
}
