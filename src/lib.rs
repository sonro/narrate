mod error;
pub mod report;

pub use error::{CliError, Error, ErrorWrap};

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub trait ExitCode {
    fn exit_code(&self) -> i32 {
        exitcode::SOFTWARE
    }
}

impl ExitCode for anyhow::Error {
    fn exit_code(&self) -> i32 {
        if let Some(err) = self.downcast_ref::<error::CliError>() {
            err.exit_code()
        } else if let Some(err) = self.downcast_ref::<error::Error>() {
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
