use miette::Diagnostic;
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("{0}")]
    Internal(String),
}
