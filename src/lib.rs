mod cli_error;
mod error;
mod macros;

pub mod report;

use std::{fmt::Display, path::PathBuf};

use error::HelpMsg;

pub use colored;

pub use colored::Color;

/// Wrapper around a dynamic error type with an optional help message.
///
/// `Error` works a lot like `Box<dyn std::error::Error>`, but with these
/// differences:
///
/// - `Error` requires that the error is `Send`, `Sync`, and `'static`.
/// - `Error` is represented as a narrow pointer &mdash; exactly one word in
///   size instead of two.
/// - `Error` may contain a help message in order to suggest further actions a
///   user might take.
#[derive(Debug)]
pub struct Error {
    pub(crate) inner: anyhow::Error,
    pub(crate) help: Option<HelpMsg>,
}

/// Iterator of a chain of source errors.
///
/// This type is the iterator returned by [`Error::chain`].
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
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct Chain<'a> {
    inner: anyhow::Chain<'a>,
}

/// `Result<T, Error>`
///
/// This is a reasonable return type to use throughout your application.
///
/// `narrate::Result` may be used with one *or* two type parameters. Therefore
/// you can import it and not worry about which `Result` you are using. Using it
/// with two types is functionally the same as rust's standard `Result` type.
///
/// ```
/// use narrate::Result;
///
/// # /*
/// fn demo1() -> Result<T> {...}
///            // ^ equivalent to std::result::Result<T, narrate::Error>
///
/// fn demo2() -> Result<T, OtherError> {...}
///            // ^ equivalent to std::result::Result<T, OtherError>
/// */
/// ```
///
/// # Example
///
/// ```
/// # pub trait Deserialize {}
/// #
/// # mod serde_json {
/// #     use super::Deserialize;
/// #     use std::io;
/// #
/// #     pub fn from_str<T: Deserialize>(json: &str) -> io::Result<T> {
/// #         unimplemented!()
/// #     }
/// # }
/// #
/// # #[derive(Debug)]
/// # struct ClusterMap;
/// #
/// # impl Deserialize for ClusterMap {}
/// #
/// # fn main() {
/// # run();
/// # }
/// #
/// use narrate::Result;
///
/// fn run() -> Result<()> {
///     # return Ok(());
///     let config = std::fs::read_to_string("cluster.json")?;
///     let map: ClusterMap = serde_json::from_str(&config)?;
///     println!("cluster info: {:#?}", map);
///     Ok(())
/// }
///
/// ```
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Provides `wrap`, `wrap_help` and `wrap_help_owned` methods for `Result`.
///
/// This trait will be sealed and should not be implemented for types outside of
/// `narrate`.
///
/// # Example
///
/// ```
/// use narrate::{ErrorWrap, Result};
/// use std::fs;
/// use std::path::PathBuf;
///
/// pub struct ImportantThing {
///     path: PathBuf,
/// }
///
/// impl ImportantThing {
///     # /**
///     pub fn detach(&mut self) -> Result<()> {...}
///     # */
///     # fn detach(&mut self) -> Result<()> {
///     #     unimplemented!()
///     # }
/// }
///
/// pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
///     it.detach().wrap(|| "Failed to detach the important thing")?;
///
///     let path = &it.path;
///     let content = fs::read(path)
///         .wrap(|| format!("Failed to read instrs from {}", path.display()))?;
///
///     Ok(content)
/// }
/// ```
///
pub trait ErrorWrap<T, E>
where
    E: Send + Sync + 'static,
{
    /// Wrap an error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn wrap<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;

    /// Lazily evaluated error wrapper, with an additional static help message
    fn wrap_help<C, F>(self, f: F, help: &'static str) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;

    /// Lazily evaluated error wrapper, with an addition owned help message
    fn wrap_help_owned<C, F>(self, f: F, help: String) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

/// Provide `exit_code` method for errors. Intended to be passed to
/// [`std::process::exit`].
///
/// Conforms to sysexits.h and defaults to `70` for "software error". Implementing
/// this trait for your custom error types allows your application to return the
/// correct code &mdash; even when wrapped in an [`Error`].
pub trait ExitCode {
    /// CLI application exit code
    fn exit_code(&self) -> i32 {
        exitcode::SOFTWARE
    }
}

/// Standard command line application error
#[derive(Debug)]
pub enum CliError {
    /// Invalid configuration
    Config,

    /// Cannot create file
    CreateFile(PathBuf),

    /// Invalid input data
    InputData,

    /// Supplied file not found
    InputFileNotFound(PathBuf),

    /// User not found
    NoUser(String),

    /// Host not found
    NoHost(String),

    /// No permission to perform operation
    OperationPermission(String),

    /// Operating system error
    OsErr,

    /// System file not found
    OsFileNotFound(PathBuf),

    /// Cannot read file
    ReadFile(PathBuf),

    /// Resource not found
    ResourceNotFound(String),

    /// Protocol not possible
    Protocol,

    /// Temporary/non fatal error
    Temporary,

    /// Inccorect usage
    Usage,

    /// Cannot write to file
    WriteFile(PathBuf),
}
