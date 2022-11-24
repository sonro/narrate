//! Using `narrate::Result` when returning from `main` will use
//! `narrate::Error`'s `Debug` formatting. It looks similar to
//! `narrate::report::err_full` output, except it doesn't use any colors.
//!
//! This example will print:
//! ```console
//! Error: outer error
//! Cause: inner error
//!
//! help message
//! ```

use std::fmt;

use narrate::{ErrorWrap, Result};

fn main() -> Result<()> {
    run().wrap("outer error").add_help("help message")?;
    Ok(())
}

fn run() -> Result<(), Error> {
    Err(Error)
}

#[derive(Debug)]
struct Error;

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "inner error".fmt(f)
    }
}
