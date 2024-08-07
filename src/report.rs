//! Functions for printing status and error messages to stderr.
//!
//! ## Report status
//!
//! Similar to [Cargo](https://github.com/rust-lang/cargo/) output, a [`status`]
//! title is justified, colored and made bold. Coloring is provided by the
//! [`Color`] enum.
//!
//! ## Report errors
//!
//! Use [`err`] or [`anyhow_err`] to print error information from either a
//! [`narrate::Error`](Error) or an [`anyhow::Error`] respectively. Include
//! error chains/causes in your output by using [`err_full`] or
//! [`anyhow_err_full`].
//!
//! ## Features
//!
//! If you have no desire to use any of narrate's other features, you can use
//! just this module's functionality by disabling the default features and just
//! using "report".
//!
//! ```toml
//! [dependencies]
//! narrate = { version = "0.4.2", default-features = false, features = ["report"] }
//! ```
//!
//! This will still allow you to report [anyhow errors](anyhow), but not [narrate
//! errors](Error).

use std::io::{self, stderr, Write};

use colored::{Color, Colorize};

#[cfg(feature = "error")]
use crate::Error;

const STDERR: &str = "writing to stderr";

/// Report a status to stderr.
///
/// ```txt
///     <title>: <msg>
/// ```
///
/// The title will be justified in the style of Cargo's statuses. If stderr
/// is directed to a TTY (as is normal for a CLI app), it will have it's color
/// set.
pub fn status<T, M>(title: T, msg: M, color: Color)
where
    T: AsRef<str>,
    M: AsRef<str>,
{
    let color = match atty::is(atty::Stream::Stderr) {
        true => Some(color),
        false => None,
    };
    let mut f = stderr().lock();
    format_status(title, msg, color, &mut f).expect(STDERR);
}

/// Report an [`Error`] to stderr.
///
/// The message will consist of a red `error:` title, followed by the
/// [`Display`](std::fmt::Display) impl for the underlying error.
///
/// If the [`Error`] contains a help message, that will be printed 2 lines
/// below.
///
/// ## Examples
///
/// Standard error.
///
/// ```
/// # use narrate::error_from;
/// # use narrate::report;
/// let error = error_from!("invalid configuration");
/// # /*
/// report::err(&error);
/// # */
/// // error: invalid configuration
/// ```
///
/// Error with a help message.
///
/// ```
/// # use narrate::CliError;
/// # use narrate::Error;
/// # use narrate::report;
/// let mut error = Error::new(CliError::Config);
/// error.add_help("try something else");
/// # /*
/// report::err(&error);
/// # */
/// // error: invalid configuration
/// //
/// // try something else
/// ```
#[cfg(feature = "error")]
pub fn err(err: &Error) {
    let color = atty::is(atty::Stream::Stderr);
    let mut f = stderr().lock();
    format_error_title(err.to_string(), color, &mut f).expect(STDERR);
    format_error_help(err, &mut f).expect(STDERR);
}

/// Report an [`Error`] to stderr, printing a list of causes
///
/// The message will consist of a red `error:` title, followed by the
/// [`Display`](std::fmt::Display) impl for the underlying error.
/// Each subsequent wrapped error will have a plain `cause:` title.
///
/// ## Examples
///
/// Wrapped error.
///
/// ```
/// # fn parse_config_file(path: &str) -> narrate::Result<()> {
/// #   Ok(())
/// # }
/// use narrate::{report, ErrorWrap, Result};
///
/// fn setup_config() -> Result<()> {
/// # /*
///     ...
/// # */
/// # let path = "";
///     let user_config = parse_config_file(&path)
///         .wrap_with(|| format!("invalid config file: `{}`", &path))?;
/// # /*
///     ...
/// # */
/// # Ok(())
/// }
///
/// fn main() {
/// # /*
///     ...
/// # */
///     let res = setup_config().wrap("invalid configuration");
///     if let Err(ref err) = res {
/// # /*
///         report::err_full(err);
/// # */
///         // error: invalid configuration
///         // cause: invalid config file: `config.toml`
///         // cause: missing key: `author`
///     }
/// # /*
///     ...
/// # */
/// }
/// ```
///
/// Wrapped error with a help message.
///
/// ```
/// use std::{fs::File, path::PathBuf};
///
/// use narrate::{report, CliError, ErrorWrap, Result};
///
/// fn run() -> Result<()> {
///     let path = PathBuf::from("/nopermission/file.txt");
///     File::create(&path)
///         .wrap_with(|| CliError::CreateFile(path))
///         .add_help("try using a valid file name")?;
///     Ok(())
/// }
///
///
/// fn main() {
///     if let Err(err) = run() {
/// # /*
///         report::err_full(&err);
/// # */
///         // error: cannot create file: "/nopermission.txt"
///         // cause: permission denied
///         //
///         // try using a valid file name
///     }
/// }
/// ```
#[cfg(feature = "error")]
pub fn err_full(err: &Error) {
    let color = atty::is(atty::Stream::Stderr);
    let mut f = stderr().lock();
    format_error_title(err.to_string(), color, &mut f).expect(STDERR);
    format_error_causes(&err.inner, color, &mut f).expect(STDERR);
    format_error_help_all(err, &mut f).expect(STDERR);
}

/// Report an [`anyhow::Error`] to stderr
///
/// The message will consist of a red `error:` title, followed by the
/// [`Display`](std::fmt::Display) impl for the underlying error.
///
/// ## Example
///
/// ```
/// # use anyhow::anyhow;
/// # use narrate::report;
/// let error = anyhow!("invalid configuration");
/// # /*
/// report::anyhow_err(&error);
/// # */
/// // error: invalid configuration
/// ```
pub fn anyhow_err(err: &anyhow::Error) {
    let color = atty::is(atty::Stream::Stderr);
    let mut f = stderr().lock();
    format_error_title(err.to_string(), color, &mut f).expect(STDERR);
}

/// Report an [`anyhow::Error`] to stderr, printing a list of causes
///
/// The message will consist of a red `error:` title, followed by the
/// [`Display`](std::fmt::Display) impl for the underlying error.
/// Each subsequent wrapped error will have a plain `cause:` title.
///
/// ## Example
///
/// Context wrapped error.
///
/// ```
/// # fn parse_config_file(path: &str) -> anyhow::Result<()> {
/// #   Ok(())
/// # }
/// use narrate::report;
/// use anyhow::{Context, Result};
///
/// fn setup_config() -> Result<()> {
/// # /*
///     ...
/// # */
/// # let path = "";
///     let user_config = parse_config_file(&path)
///         .with_context(|| format!("invalid config file: `{}`", &path))?;
/// # /*
///     ...
/// # */
/// # Ok(())
/// }
///
/// fn main() {
/// # /*
///     ...
/// # */
///     let res = setup_config().context("invalid configuration");
///     if let Err(ref err) = res {
/// # /*
///         report::anyhow_err_full(err);
/// # */
///         // error: invalid configuration
///         // cause: invalid config file: `config.toml`
///         // cause: missing key: `author`
///     }
/// # /*
///     ...
/// # */
/// }
/// ```
pub fn anyhow_err_full(err: &anyhow::Error) {
    let color = atty::is(atty::Stream::Stderr);
    let mut f = stderr().lock();
    format_error_title(err.to_string(), color, &mut f).expect(STDERR);
    format_error_causes(err, color, &mut f).expect(STDERR);
}

#[inline]
fn format_error_title(msg: String, color: bool, f: &mut io::StderrLock) -> io::Result<()> {
    let color = match color {
        true => Some(Color::Red),
        false => None,
    };
    format_line("error", msg, color, true, f)
}

#[inline]
fn format_error_causes(
    anyhow_err: &anyhow::Error,
    color: bool,
    f: &mut io::StderrLock,
) -> io::Result<()> {
    let color = match color {
        true => Some(Color::Red),
        false => None,
    };
    for cause in anyhow_err.chain().skip(1) {
        format_line("cause", cause.to_string(), color, false, f)?;
    }
    Ok(())
}

#[inline]
#[cfg(feature = "error")]
fn format_error_help_all(err: &Error, f: &mut io::StderrLock) -> io::Result<()> {
    if let Some(help) = err.help() {
        writeln!(f, "\n{}", help)?;
    }
    Ok(())
}

#[inline]
#[cfg(feature = "error")]
fn format_error_help(err: &Error, f: &mut io::StderrLock) -> io::Result<()> {
    if let Some(help) = err.help() {
        let help = help
            .lines()
            .last()
            .expect("there will be at least one line of help");
        writeln!(f, "\n{}", help)?;
    }
    Ok(())
}

#[inline]
fn format_line<T, M>(
    title: T,
    msg: M,
    color: Option<Color>,
    bold: bool,
    f: &mut io::StderrLock,
) -> io::Result<()>
where
    T: AsRef<str>,
    M: AsRef<str>,
{
    match color {
        Some(color) => {
            let mut title = title.as_ref().color(color);
            if bold {
                title = title.bold();
            }
            writeln!(f, "{}{} {}", title, ":".white().bold(), msg.as_ref(),)
        }
        None => writeln!(f, "{}: {}", title.as_ref(), msg.as_ref()),
    }
}

#[inline]
fn format_status<T, M>(
    title: T,
    msg: M,
    color: Option<Color>,
    f: &mut io::StderrLock,
) -> io::Result<()>
where
    T: AsRef<str>,
    M: AsRef<str>,
{
    match color {
        Some(color) => {
            let title = title.as_ref().color(color).bold();
            writeln!(f, "{:>12} {}", title, msg.as_ref(),)
        }
        None => writeln!(f, "{:>12} {}", title.as_ref(), msg.as_ref()),
    }
}
