use std::{error::Error as StdError, fmt};

use super::{Error, HelpMsg};

pub trait ErrorWrap<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    /// Wrap an error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn wrap<C, F>(self, f: F) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;

    /// Lazily evaluated error wrapper, with an additional static help message
    fn wrap_help<C, F>(self, f: F, help: &'static str) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;

    /// Lazily evaluated error wrapper, with an addition owned help message
    fn wrap_help_owned<C, F>(self, f: F, help: String) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> ErrorWrap<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn wrap<C, F>(self, f: F) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|err| Error {
            inner: anyhow::Error::from(err).context(f()),
            help: None,
        })
    }

    fn wrap_help<C, F>(self, f: F, help: &'static str) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|err| Error {
            inner: anyhow::Error::from(err).context(f()),
            help: Some(HelpMsg::Static(help)),
        })
    }

    fn wrap_help_owned<C, F>(self, f: F, help: String) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|err| Error {
            inner: anyhow::Error::from(err).context(f()),
            help: Some(HelpMsg::Owned(help)),
        })
    }
}
