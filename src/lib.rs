#![deny(unsafe_code)]

pub mod cli;
pub mod error;
pub mod exit_codes;

// Aliases
pub use error::Error;
