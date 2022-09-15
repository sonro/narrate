use std::io::{self, stderr, Write};

use colored::{Color, Colorize};

use crate::error::Error;

const STDERR: &str = "writing to stderr";

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

pub fn err(err: &Error) {
    let color = atty::is(atty::Stream::Stderr);
    let mut f = stderr().lock();
    format_error_title(err, color, &mut f).expect(STDERR);
    format_error_help(err, &mut f).expect(STDERR);
}

pub fn err_full(err: &Error) {
    let color = atty::is(atty::Stream::Stderr);
    let mut f = stderr().lock();
    format_error_title(err, color, &mut f).expect(STDERR);
    format_error_causes(err, color, &mut f).expect(STDERR);
    format_error_help(err, &mut f).expect(STDERR);
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
fn format_error_help(err: &Error, f: &mut io::StderrLock) -> io::Result<()> {
    if let Some(ref help) = err.help {
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
