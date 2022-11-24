//! If your application already uses anyhow, there are several ways to use this
//! library.
//!
//! 1. Replace every instance of any `anyhow` import with `narrate` (This could
//!    be tedious if you have a lot of contexts).
//!
//! 2. Only use `narrate::Error` in your outermost functions to use the help
//!    message feature.
//!
//! 3. Just use the `report` module to "pretty print" the anyhow errors.
//!
//! This is an example of the second method. It prints:
//!
//! ```console
//! error: error message
//!
//! help message
//! ```

use narrate::{report, Error, ErrorWrap, Result};

fn main() {
    if let Err(ref err) = outer_fn() {
        report::err(err);
    }
}

fn outer_fn() -> Result<()> {
    inner_fn()
        // here is where we convert from `anyhow::Error` to `narrate::Error`
        .map_err(Error::from_anyhow)
        // we can now safely add a help message
        .add_help("help message")
}

fn inner_fn() -> anyhow::Result<()> {
    anyhow::bail!("error message")
}
