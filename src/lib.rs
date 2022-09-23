mod cli_error;
mod error;
mod macros;

pub mod report;

use std::{fmt::Display, path::PathBuf};

use error::HelpMsg;

pub use colored;

pub use colored::Color;

#[derive(Debug)]
pub struct Error {
    pub(crate) inner: anyhow::Error,
    pub(crate) help: Option<HelpMsg>,
}

#[derive(Clone, Default)]
#[repr(transparent)]
pub struct Chain<'a> {
    inner: anyhow::Chain<'a>,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

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

pub trait ExitCode {
    fn exit_code(&self) -> i32 {
        exitcode::SOFTWARE
    }
}

#[derive(Debug)]
pub enum CliError {
    Config,

    CreateFile(PathBuf),

    InputData,

    InputFileNotFound(PathBuf),

    NoUser(String),

    NoHost(String),

    OperationPermission(String),

    OsErr,

    OsFileNotFound(PathBuf),

    ReadFile(PathBuf),

    ResourceNotFound(String),

    Protocol,

    Temporary,

    Usage,

    WriteFile(PathBuf),
}
