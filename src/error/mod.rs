use std::{error::Error as StdError, fmt};

use crate::{Chain, Error};

mod chain;
mod macros;
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

    /// Create a new error object from a printable error message.
    ///
    /// If the argument implements [`std::error::Error`], prefer [`Error::new`]
    /// instead which preserves the underlying error's cause chain and
    /// backtrace. If the argument may or may not implement `std::error::Error`
    /// now or in the future, use [`error_from!(err)`](`crate::error_from`)
    /// which handles either way correctly.
    ///
    /// `Error::msg("...")` is equivalent to `error_from!("...")` but
    /// occasionally convenient in places where a function is preferable over a
    /// macro, such as iterator or stream combinators:
    ///
    /// ```
    /// # /*
    /// use narrate::{Error, Result};
    /// use futures::stream::{Stream, StreamExt, TryStreamExt};
    ///
    /// async fn demo<S>(stream: S) -> Result<Vec<Output>>
    /// where
    ///     S: Stream<Item = Input>,
    /// {
    ///     stream
    ///         .then(ffi::do_some_work) // returns Result<Output, &str>
    ///         .map_err(Error::msg)
    ///         .try_collect()
    ///         .await
    /// }
    /// # */
    /// ```
    pub fn msg<M>(message: M) -> Self
    where
        M: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        Self {
            inner: anyhow::Error::msg(message),
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
    pub fn downcast<E>(self) -> Result<E, Self>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast().map_err(Self::from_anyhow)
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

    /// Add a 'static help message to the Error.
    ///
    /// Use this method to add a plain help [`str`]. If you need to format the
    /// message, or add an owned [`String`], use
    /// [`add_help_with`](Self::add_help_with) instead.
    ///
    /// If the Error already has existing help text, this method will append the
    /// new message to it (on a new line). This is done deliberately because the
    /// last help message should be most visible.
    ///
    /// You can add help to any `Result` that might contain a
    /// [`std::error::Error`], by using the [`ErrorWrap`](crate::ErrorWrap)
    /// trait.
    ///
    /// # Examples
    ///
    /// Add help to a constructed `Error`.
    ///
    /// ```
    /// # use std::fmt;
    /// #
    /// # #[derive(Debug)]
    /// # struct AppError;
    /// #
    /// # impl fmt::Display for AppError {
    /// #   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    /// #       write!(f, "application error")
    /// #   }
    /// # }
    /// #
    /// # impl std::error::Error for AppError {}
    /// #
    /// use narrate::Error;
    ///
    /// let mut error = Error::new(AppError);
    /// error.add_help("help message");
    /// ```
    ///
    /// Add multiple help messages.
    ///
    ///
    /// ```
    /// # use std::fmt;
    /// #
    /// # #[derive(Debug)]
    /// # struct AppError;
    /// #
    /// # impl fmt::Display for AppError {
    /// #   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    /// #       write!(f, "application error")
    /// #   }
    /// # }
    /// #
    /// # impl std::error::Error for AppError {}
    /// #
    /// use narrate::{Error, ErrorWrap, Result};
    ///
    /// fn main() {
    ///     let err = outer().unwrap_err();
    ///     // outer help appended on a new line
    ///     let expected = "inner help\nouter help";
    ///     assert_eq!(err.help(), Some(expected));
    /// }
    ///
    /// fn outer() -> Result<()> {
    ///     // uses the ErrorWrap trait to add help to the Result
    ///     inner().add_help("outer help")?;
    ///     Ok(())
    /// }
    ///
    /// fn inner() -> Result<()> {
    ///     let mut error = Error::new(AppError);
    ///     error.add_help("inner help");
    ///     Err(error)
    /// }
    /// ```
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

    /// Add a computed help message to the Error.
    ///
    /// Use this method to add a formatted or computed [`String`]. If you are
    /// adding a `'static str` you should use [`add_help`](Self::add_help)
    /// instead.
    ///
    /// This method takes any closure that produces a value that implements
    /// [`Display`](fmt::Display), [`Send`], [`Sync`] and `'static`.
    ///
    /// If the Error already has existing help text, this method will append the
    /// new message to it (on a new line). This is done deliberately because the
    /// last help message should be most visible.
    ///
    /// You can add help to any `Result` that might contain a
    /// [`std::error::Error`], by using the [`ErrorWrap`](crate::ErrorWrap)
    /// trait.
    ///
    /// # Examples
    ///
    /// Add formatted help.
    ///
    /// ```
    /// # use std::path::PathBuf;
    /// use narrate::{CliError, Error};
    ///
    /// let path = PathBuf::from("/path/to/file");
    /// let mut error = Error::new(CliError::CreateFile(path.clone()));
    /// error.add_help_with(|| format!("helpful message about path: {}", path.display()));
    /// ```
    ///
    /// Add existing value.
    ///
    /// ```
    /// use narrate::error_from;
    ///
    /// let mut error = error_from!("error msg");
    /// let help = String::from("help msg");
    /// error.add_help_with(|| help);
    /// ```
    pub fn add_help_with<C, F>(&mut self, f: F)
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.help = Some(HelpMsg::Owned(match self.help() {
            Some(existing) => format!("{}\n{}", existing, f()),
            None => f().to_string(),
        }));
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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            return fmt::Debug::fmt(&self.inner, f);
        }

        write!(f, "{}", self.inner)?;

        for cause in self.inner.chain().skip(1) {
            write!(f, "\nCause: {cause}")?;
        }

        if let Some(ref help) = self.help {
            write!(f, "\n\n{help}")?;
        }

        Ok(())
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

impl From<Error> for anyhow::Error {
    fn from(err: Error) -> Self {
        err.inner
    }
}
