use std::io::{self, stderr, Write};

use colored::{Color, Colorize};

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

/// Report an error to stderr
///
/// The message will consist of a red `error:` title, followed by the
/// [`Display`](std::fmt::Display) impl for the underlying error.
///
/// If the [`Error`] contains a help message, that will be printed 2 lines
/// below.
///
/// # Examples
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
    format_error_title(err, color, &mut f).expect(STDERR);
    format_error_help(err, &mut f).expect(STDERR);
}

/// Report an error to stderr, printing a list of wrapped causes
///
/// The message will consist of a red `error:` title, followed by the
/// [`Display`](std::fmt::Display) impl for the underlying error.
/// Each subsequent wrapped error will have a plain `cause:` title.
///
/// # Examples
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
///     let res = setup_config().wrap_with(|| "invalid configuration");
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
    format_error_title(err, color, &mut f).expect(STDERR);
    format_error_causes(err, color, &mut f).expect(STDERR);
    format_error_help_all(err, &mut f).expect(STDERR);
}

#[inline]
fn format_error_title(err: &Error, color: bool, f: &mut io::StderrLock) -> io::Result<()> {
    let color = match color {
        true => Some(Color::Red),
        false => None,
    };
    format_line("error", err.inner.to_string(), color, true, f)
}

#[inline]
fn format_error_causes(err: &Error, color: bool, f: &mut io::StderrLock) -> io::Result<()> {
    let color = match color {
        true => Some(Color::Red),
        false => None,
    };
    for cause in err.inner.chain().skip(1) {
        format_line("cause", cause.to_string(), color, false, f)?;
    }
    Ok(())
}

#[inline]
fn format_error_help_all(err: &Error, f: &mut io::StderrLock) -> io::Result<()> {
    if let Some(help) = err.help() {
        writeln!(f, "\n{}", help)?;
    }
    Ok(())
}

#[inline]
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
