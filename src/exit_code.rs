use crate::{CliError, Error, ExitCode};

impl ExitCode for anyhow::Error {
    fn exit_code(&self) -> i32 {
        if let Some(err) = self.downcast_ref::<CliError>() {
            return err.exit_code();
        }

        #[cfg(feature = "error")]
        if let Some(err) = self.downcast_ref::<Error>() {
            return err.exit_code();
        }

        exitcode::SOFTWARE
    }
}

#[cfg(feature = "error")]
impl ExitCode for Error {
    fn exit_code(&self) -> i32 {
        self.inner.exit_code()
    }
}

pub(crate) mod private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for anyhow::Error {}
    impl Sealed for Error {}
    impl Sealed for CliError {}
}
