use std::{error::Error as StdError, fmt};

use crate::{Chain, CliError, Error, ExitCode};

mod chain;
pub(crate) mod wrap;

impl Error {
    /// Create a new error object from any error type.
    ///
    /// The error type must be thread-safe and `'static`, so that the `Error`
    /// will be as well.
    pub fn new<E>(error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self {
            inner: error.into(),
            help: None,
        }
    }

    /// Convert an [`anyhow::Error`] into an error object.
    ///
    /// Due to the generic implementation of [`From`] for [`Error`]: we cannot
    /// add a `From<anyhow::Error>` impl. Use this instead.
    #[inline]
    pub fn from_anyhow(error: anyhow::Error) -> Self {
        Self {
            inner: error,
            help: None,
        }
    }

    /// Wrap the error value with additional context.
    ///
    /// For attaching context to a `Result` as it is propagated, the
    /// [`ErrorWrap`](crate::ErrorWrap) extension trait may be more convenient than this function.
    ///
    /// The primary reason to use `error.warp(...)` instead of
    /// `result.warp(...)` via the `ErrorWrap` trait would be if the context
    /// needs to depend on some data held by the underlying error:
    ///
    /// ```
    /// # use std::fmt::{self, Debug, Display};
    /// #
    /// # type T = ();
    /// #
    /// # impl std::error::Error for ParseError {}
    /// # impl Debug for ParseError {
    /// #     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    /// #         unimplemented!()
    /// #     }
    /// # }
    /// # impl Display for ParseError {
    /// #     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    /// #         unimplemented!()
    /// #     }
    /// # }
    /// #
    /// use narrate::Result;
    /// use std::fs::File;
    /// use std::path::Path;
    ///
    /// struct ParseError {
    ///     line: usize,
    ///     column: usize,
    /// }
    ///
    /// fn parse_impl(file: File) -> Result<T, ParseError> {
    ///     # /*
    ///     ...
    ///     # */
    ///     # unimplemented!()
    /// }
    ///
    /// pub fn parse(path: impl AsRef<Path>) -> Result<T> {
    ///     let file = File::open(&path)?;
    ///     parse_impl(file).map_err(|error| {
    ///         let context = format!(
    ///             "only the first {} lines of {} are valid",
    ///             error.line, path.as_ref().display(),
    ///         );
    ///         narrate::Error::new(error).wrap(context)
    ///     })
    /// }
    /// ```
    pub fn wrap<C>(self, context: C) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        Self {
            inner: self.inner.context(context),
            help: self.help,
        }
    }

    /// Returns true if `E` is the type held by this error object.
    ///
    /// For wrapped errors, this method returns true if `E` matches the
    /// type of the context `C` **or** the type of the error on which the
    /// context has been attached.
    pub fn is<E>(&self) -> bool
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.downcast_ref::<E>().is_some()
    }

    /// Attempt to downcast the error object to a concrete type.
    pub fn downcast<E>(self) -> Result<E, anyhow::Error>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast()
    }

    /// Downcast this error object by reference.
    ///
    /// # Example
    ///
    /// ```
    /// # use narrate::error_from;
    /// # use std::fmt::{self, Display};
    /// # use std::task::Poll;
    /// #
    /// # #[derive(Debug)]
    /// # enum DataStoreError {
    /// #     Censored(()),
    /// # }
    /// #
    /// # impl Display for DataStoreError {
    /// #     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    /// #         unimplemented!()
    /// #     }
    /// # }
    /// #
    /// # impl std::error::Error for DataStoreError {}
    /// #
    /// # const REDACTED_CONTENT: () = ();
    /// #
    /// # let error = error_from!("...");
    /// # let root_cause = &error;
    /// #
    /// # let ret =
    /// // If the error was caused by redaction, then return a tombstone instead
    /// // of the content.
    /// match root_cause.downcast_ref::<DataStoreError>() {
    ///     Some(DataStoreError::Censored(_)) => Ok(Poll::Ready(REDACTED_CONTENT)),
    ///     None => Err(error),
    /// }
    /// # ;
    /// ```
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast_ref()
    }

    /// Downcast this error object by mutable reference.
    pub fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast_mut()
    }

    /// An iterator of the chain of source errors contained by this Error.
    ///
    /// This iterator will visit every error in the cause chain of this error
    /// object, beginning with the error that this error object was created
    /// from.
    ///
    /// # Example
    ///
    /// ```
    /// use narrate::Error;
    /// use std::io;
    ///
    /// pub fn underlying_io_error_kind(error: &Error) -> Option<io::ErrorKind> {
    ///     for cause in error.chain() {
    ///         if let Some(io_error) = cause.downcast_ref::<io::Error>() {
    ///             return Some(io_error.kind());
    ///         }
    ///     }
    ///     None
    /// }
    /// ```
    pub fn chain(&self) -> Chain {
        self.inner.chain().into()
    }

    /// The lowest level cause of this error &mdash; this error's cause's
    /// cause's cause etc.
    ///
    /// The root cause is the last error in the iterator produced by
    /// [`chain()`][Error::chain].
    pub fn root_cause(&self) -> &(dyn StdError + 'static) {
        self.inner.root_cause()
    }

    /// Get a reference to this error's help message
    #[inline]
    pub fn help(&self) -> Option<&str> {
        self.help.as_ref().map(AsRef::as_ref)
    }

    /// Set this error's help message to an owned [`String`]
    #[inline]
    #[deprecated]
    pub fn set_help_owned(&mut self, msg: String) {
        self.help = Some(HelpMsg::Owned(msg));
    }

    /// Set this error's help message to a static `&str`
    #[inline]
    #[deprecated]
    pub fn set_help(&mut self, msg: &'static str) {
        self.help = Some(HelpMsg::Static(msg));
    }

    pub fn add_help(&mut self, help: &'static str) {
        match self.help {
            Some(HelpMsg::Owned(ref mut existing)) => {
                existing.push('\n');
                existing.push_str(help);
            }
            Some(HelpMsg::Static(existing)) => {
                self.help = Some(HelpMsg::Owned(format!("{}\n{}", existing, help)))
            }

            None => self.help = Some(HelpMsg::Static(help)),
        }
    }

    pub fn add_help_with<C>(&mut self, help: C)
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.help = Some(HelpMsg::Owned(match self.help() {
            Some(existing) => format!("{}\n{}", existing, help),
            None => help.to_string(),
        }));
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
            inner: err.into(),
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
        self.inner.fmt(f)
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

impl fmt::Display for HelpMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HelpMsg::Owned(help) => help.fmt(f),
            HelpMsg::Static(help) => help.fmt(f),
        }
    }
}

impl<'a> PartialEq<&'a str> for HelpMsg {
    fn eq(&self, r: &&'a str) -> bool {
        match self {
            Self::Owned(l) => l == r,
            Self::Static(l) => l == r,
        }
    }
}

impl ExitCode for anyhow::Error {
    fn exit_code(&self) -> i32 {
        if let Some(err) = self.downcast_ref::<CliError>() {
            err.exit_code()
        } else if let Some(err) = self.downcast_ref::<Error>() {
            err.exit_code()
        } else {
            exitcode::SOFTWARE
        }
    }
}

impl From<Error> for anyhow::Error {
    fn from(err: Error) -> Self {
        err.inner
    }
}
