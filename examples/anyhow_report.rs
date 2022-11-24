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
//! This is an example of the third method. It prints:
//!
//! ```console
//! error: operating system error
//! cause: inner error
//! ```

use anyhow::Context;
use narrate::{report, CliError, ExitCode};

fn main() {
    let res = inner_fn().context(CliError::OsErr);

    if let Err(ref err) = res {
        report::anyhow_err_full(err);
        // As the error contains a `CliError`, this code will match its
        // error_code. In this case `71`. If there was no underlying `CliError`,
        // the code will default to `70`.
        std::process::exit(err.exit_code());
    }
}

fn inner_fn() -> anyhow::Result<()> {
    anyhow::bail!("inner error")
}
