//! [![github]](https://github.com/sonro/narrate)&ensp;[![crates-io]](https://crates.io/crates/narrate)&ensp;[![docs-rs]](https://docs.rs/narrate)
//!
//! [github]:
//!     https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]:
//!     https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]:
//!     https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! This library provides Rust CLI applications with console reporting and
//! error-handling utilities. Console output is modeled after
//! [Cargo](https://github.com/rust-lang/cargo), and the [`Error`] type is
//! similar to [anyhow's](https://github.com/dtolnay/anyhow)
//! [`Error`](https://docs.rs/anyhow/1/anyhow/struct.Error.html), but with
//! optional help messages.
//!
//! Minimum supported Rust version: **1.61.1**
//!
//! # Features
//!
//! - Ergonomic [error-handling](#error-handling).
//! - A set of typical CLI application [errors](#cli-errors) (with exit codes).
//! - Errors and app status [reporting](report).
//!
//! #### Cargo Feature Flags
//!
//! All features are enabled by default, but they can be imported individually
//! using [Cargo feature
//! flags](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features):
//!
//! - `error`: Enables error-handling with [`Error`], [`Result`] and
//!   [`ErrorWrap`].
//! - `cli-error`: Enables set of [`CliError`]s with their associated
//!   [`exit_code`](ExitCode).
//! - `report`: Enables reporting errors and statuses to the console with the
//!   [`report`] module.
//!
//! ##### Example `Cargo.toml`
//!
//! ```toml
//! ...
//! [dependencies]
//! narrate = { version = "0.4.0", default-features = false, features = ["report"] }
//! ...
//! ```
//!
//! ## Error Handling
//!
//! Use [`Result<T>`] as a return type for any fallible function. Within the
//! function, use `?` to propagate any error that implements the
//! [`std::error::Error`] trait. Same as
//! [`anyhow::Result<T>`](https://docs.rs/anyhow/1.0/anyhow/type.Result.html).
//!
//! ```
//! # struct User;
//! use narrate::Result;
//!
//! fn get_user() -> Result<User> {
//! # /*
//!     let json = std::fs::read_to_string("user.json")?;
//!     let user: User = serde_json::from_str(&json)?;
//!     Ok(user)
//! # */
//! # Ok(User)
//! }
//! ```
//!
//! #### Returning from `main()`
//!
//! [`Result<T>`] can be used to return from `main()`. If any errors occur it
//! prints the `Debug` implementation for [`Error`].
//!
//! ```no_run
//! use narrate::{bail, Result};
//!
//! fn main() -> Result<()> {
//!     inner_fn()?;
//!     Ok(())
//! }
//!
//! fn inner_fn() -> Result<()> {
//!     bail!("internal error")
//! }
//! ```
//!
//! Console output:
//!
//! ```console
//! Error: internal error
//! ```
//!
//! ### Error Wrap
//!
//! Wrap an error with more context by importing [`ErrorWrap`]. Similar to
//! [`anyhow::Context`](https://docs.rs/anyhow/1.0/anyhow/trait.Context.html).
//! Just add `.wrap(context)` after any function call that returns a `Result`.
//!
//! Context can be anything that implements [`Debug`](std::fmt::Debug),
//! [`Display`](std::fmt::Display), [`Sync`] and [`Send`] -- including [`&str`],
//! [`String`] and errors.
//!
//! ```rust
//! use narrate::{ErrorWrap, Result, CliError};
//!
//! fn run() -> Result<()> {
//! # /*
//!     ...
//!     // wrap with contextual &str
//!     acquire().wrap("unable to acquire data")?;
//!
//!     // or wrap with another error
//!     config.load().wrap(CliError::Config)?;
//!     ...
//! # */
//! # Ok(())
//! }
//!
//! # /*
//! fn acquire() -> Result<(), io::Error> {
//!     ...
//! }
//! # */
//! ```
//!
//! Console output:
//!
//! ```console
//! Error: unable to acquire data
//! Cause: oh no!
//! ```
//!
//! #### Lazy evaluation
//!
//! If your context requires some work to create/format, you should use
//! [`wrap_with`](ErrorWrap::wrap_with) instead.
//!
//! ```rust
//! use narrate::{ErrorWrap, Result, CliError};
//!
//! fn run() -> Result<()> {
//! # /*
//!     ...
//!     // wrap with a formatted string
//!     data.store(path).wrap_with(|| format!("can't save to: {path}"))?;
//!
//!     // wrap with a computed error
//!     data.store(path)
//!         .wrap_with(|| CliError::WriteFile(PathBuf::from(path)))?;
//!     ...
//! # */
//! # Ok(())
//! }
//! ```
//!
//! ### Help Message Wrap
//!
//! Add separate help text to an error. By importing [`ErrorWrap`] you also get
//! the `add_help` method and its lazy version `add_help_with`.
//!
//! ```rust
//! use narrate::{ErrorWrap, Result};
//!
//! fn run() -> Result<()> {
//! # /*
//!     Project::new(path).add_help("try using `project init`")?;
//!     ...
//! # */
//! # Ok(())
//! }
//! ```
//!
//! Console output:
//!
//! ```console
//! Error: directory already exists: '/home/dev/cool-project'
//!
//! try using `project init`
//! ```
//!
//! #### Combination
//!
//! Mix and match the `ErrorWrap` methods throughout your application to make
//! sure the user gets all the information they need.
//!
//! ```rust
//! use narrate::{ErrorWrap, Result};
//! # use std::path::Path;
//!
//! fn run() -> Result<()> {
//! # /*
//!     ...
//!     new_project(&path).wrap("cannot create project")?;
//!     ...
//! # */
//! # Ok(())
//! }
//!
//! fn new_project(path: &Path) -> Result<()> {
//! # /*
//!     ...
//!     create_dir(path)
//!         .wrap_with(|| format!(
//!             "unable to create directory: '{}'",
//!             path.display()
//!         ))
//!         .add_help(
//!             "try using `project init` inside your existing directory"
//!         )?;
//!     ...
//! # */
//! # Ok(())
//! }
//! ```
//!
//! Console output:
//!
//! ```console
//! Error: cannot create project
//! Cause: unable to create directory: '/home/dev/cool-project'
//! Cause: Is a directory (os error 20)
//!
//! try using `project init` inside your existing directory
//! ```
//!
//! ### Convenience Macros
//!
//! Use the [`error_from`] macro to create an ad-hoc [`Error`] from a string or
//! another error. Similar to [`anyhow!`](anyhow::anyhow).
//!
//! ```
//! # use std::collections::HashMap;
//! # use narrate::{error_from, Result};
//! # fn run(map: HashMap<&'static str, String>, key: &str) -> Result<()> {
//! let val = map.get(key).ok_or(error_from!("unknown key"))?;
//! # Ok(())
//! # }
//! ```
//!
//! Use [`bail`] to return early with an error. Equivalent to `return Err(error_from!(...))`.
//!
//!
//! ```
//! # use std::collections::HashMap;
//! # use narrate::{bail, Result};
//! # fn run(map: HashMap<&'static str, String>, key: &str) -> Result<()> {
//! if !map.contains_key(key) {
//!     bail!("unknown key");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## CLI Errors
//!
//! Use [`CliError`] for a set of common errors that can occur in a command-line
//! application. These can be used to avoid adding repetitive context for IO
//! errors.
//!
//! ```
//! use narrate::{bail, ErrorWrap, Result, CliError};
//!
//! fn run() -> Result<()> {
//! # /*
//!     ...
//!     match args.operation {
//!         Op::Get => fetch_data(res_name).wrap_with(|| CliError::ResourceNotFound(res_name))?,
//!         Op::Set(data) => set_data(res_name, data).wrap(CliError::InputData)?,
//!         _ => bail!(CliError::Protocol),
//!     }
//!     ...
//! # */
//! # Ok(())
//! }
//! ```
//!
//! ### Exit Codes
//!
//! As this selection of errors can often be fatal for an application, this
//! library provides access to a set of standard program exit codes via the
//! [`ExitCode`] trait. These adhere to
//! [sysexits.h](https://man.openbsd.org/sysexits).
//!
//! Both [`anyhow::Error`] and [`narrate::Error`](Error) implement this trait,
//! thus can provide exit codes. If no [`CliError`] is found as an underlying
//! error, the code will be `70` (for internal software error).
//!
//! Import the [`ExitCode`] trait to use the `exit_code` function, and use
//! [`std::process::exit`] to exit the program with the appropriate code.
//!
//! ```
//! # /*
//! use narrate::ExitCode;
//!
//! if let Err(err) = run() {
//!     std::process::exit(err.exit_code());
//! }
//! # */
//! ```
//!

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
#[cfg(feature = "cli-error")]
mod exit_code;

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

/// Provides `wrap` and `add_help` methods for [`Result`](core::result::Result).
///
/// This trait is sealed and cannot be implemented for types outside of
/// `narrate`.
///
/// Useful for wrapping a potential error with additional context and/or help
/// message.
///
/// ## Lazy evaluation
///
/// Use `wrap_with` and `add_help_with` methods for lazily evaluation of the
/// added context.
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
///         .wrap_with(|| format!("Failed to read instrs from {}", path.display()))
///         .add_help("list of instr in README.md")?;
///
///     Ok(content)
/// }
/// ```
#[cfg(feature = "error")]
pub trait ErrorWrap<T, E>: error::wrap::private::Sealed
where
    E: Send + Sync + 'static,
{
    /// Wrap an error value with additional context.
    fn wrap<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static;

    /// Wrap an error value with lazily evaluated context.
    fn wrap_with<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;

    /// Add a help message to an error value.
    fn add_help(self, help: &'static str) -> Result<T, Error>;

    /// Add a lazily evaluated help message to an error value.
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
#[non_exhaustive]
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
