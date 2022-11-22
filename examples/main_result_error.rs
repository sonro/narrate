use std::fmt;

use narrate::{ErrorWrap, Result};

fn main() -> Result<()> {
    run().wrap("outer error")?;
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
        "inner error...".fmt(f)
    }
}
