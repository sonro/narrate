#[cfg(feature = "error")]
use std::fmt::Display;
#[cfg(feature = "cli-error")]
use std::path::PathBuf;

#[cfg(feature = "error")]
use error::HelpMsg;

#[cfg(feature = "cli-error")]
mod cli_error;
#[cfg(feature = "error")]
mod error;
#[cfg(all(feature = "cli-error"))]
mod exit_code;
#[cfg(feature = "error")]
mod macros;

#[cfg(feature = "report")]
pub mod report;

#[cfg(feature = "anyhow")]
pub use anyhow;
#[cfg(feature = "report")]
pub use colored;

#[cfg(feature = "report")]
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
#[cfg(feature = "error")]
pub struct Error {
    inner: anyhow::Error,
    help: Option<HelpMsg>,
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
#[cfg(feature = "error")]
pub struct Chain<'a> {
    inner: anyhow::Chain<'a>,
}

/// `Result<T, Error>`
///
/// This is a reasonable return type to use throughout your application.
///
/// `narrate::Result` may be used with one *or* two type parameters. Therefore
/// you can import it and not worry about which `Result` you are using. Using it
/// with two types is functionally the same as rust's standard
/// [`Result`](core::result::Result) type.
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
#[cfg(feature = "error")]
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Provides `wrap`, `wrap_help` and `wrap_help_owned` methods for `Result`.
///
/// This trait is sealed and cannot be implemented for types outside of
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
///     it.detach().wrap("Failed to detach the important thing")?;
///
///     let path = &it.path;
///     let content = fs::read(path)
///         .wrap_with(|| format!("Failed to read instrs from {}", path.display()))?;
///
///     Ok(content)
/// }
/// ```
#[cfg(feature = "error")]
pub trait ErrorWrap<T, E>: error::wrap::private::Sealed
where
    E: Send + Sync + 'static,
{
    /// Wrap an error value with additional context
    fn wrap<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static;

    /// Wrap an error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn wrap_with<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;

    /// Lazily evaluated error wrapper, with an additional static help message
    fn add_help(self, help: &'static str) -> Result<T, Error>;

    fn add_help_with<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

/// Provide `exit_code` method for [CliError]. Intended to be passed to
/// [`std::process::exit`].
#[cfg(feature = "cli-error")]
pub trait ExitCode: exit_code::private::Sealed {
    /// CLI application exit code
    fn exit_code(&self) -> i32 {
        exitcode::SOFTWARE
    }
}

/// Standard command line application error
#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg(feature = "cli-error")]
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

    /// Incorrect usage
    Usage,

    /// Cannot write to file
    WriteFile(PathBuf),
}
