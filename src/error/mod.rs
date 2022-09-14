use std::{error::Error as StdError, fmt};

use crate::ExitCode;

pub mod cli;
pub mod wrap;

pub use cli::CliError;
pub use wrap::ErrorWrap;

pub struct Error {
    pub(crate) inner: anyhow::Error,
    pub(crate) help: Option<HelpMsg>,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("inner", &self.inner.to_string())
            .field("help", &self.help)
            .finish()
    }
}

impl Error {
    pub fn help(&self) -> Option<&str> {
        self.help.as_ref().map(AsRef::as_ref)
    }

    pub fn set_help_owned(&mut self, msg: String) {
        self.help = Some(HelpMsg::Owned(msg));
    }

    pub fn set_help_static(&mut self, msg: &'static str) {
        self.help = Some(HelpMsg::Static(msg));
    }
}

impl ExitCode for Error {
    fn exit_code(&self) -> i32 {
        self.inner.exit_code()
    }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    fn from(err: E) -> Self {
        Self {
            inner: anyhow::Error::from(err),
            help: None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum HelpMsg {
    Owned(String),
    Static(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.help {
            None => self.inner.fmt(f),
            Some(ref help) => format_error_with_help(&self.inner, help, f),
        }
    }
}

impl AsRef<str> for HelpMsg {
    fn as_ref(&self) -> &str {
        match self {
            HelpMsg::Owned(ref s) => s,
            HelpMsg::Static(s) => s,
        }
    }
}

#[inline]
fn format_error_with_help(
    err: &anyhow::Error,
    help: &HelpMsg,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    write!(f, "{}\n\n{}", err, help)
}

impl fmt::Display for HelpMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HelpMsg::Owned(help) => help.fmt(f),
            HelpMsg::Static(help) => help.fmt(f),
        }
    }
}
