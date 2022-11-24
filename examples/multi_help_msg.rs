//! Help messages can be stacked. The innermost help message will appear first
//! when reporting or debugging.
//!
//! This example will print:
//!
//! ```console
//! error: outer error
//! cause: inner error
//!
//! inner help
//! outer help
//! ```

use narrate::{bail, ErrorWrap, Result};

fn main() -> Result<()> {
    outer_fn().wrap("outer error").add_help("outer help")
}

fn outer_fn() -> Result<()> {
    inner_fn().add_help("inner help")
}

fn inner_fn() -> Result<()> {
    bail!("inner error")
}
