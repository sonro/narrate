#[cfg(feature = "anyhow")]
impl crate::ExitCode for anyhow::Error {
    fn exit_code(&self) -> i32 {
        if let Some(err) = self.downcast_ref::<crate::CliError>() {
            return err.exit_code();
        }

        #[cfg(feature = "error")]
        if let Some(err) = self.downcast_ref::<crate::Error>() {
            return err.exit_code();
        }

        exitcode::SOFTWARE
    }
}

#[cfg(feature = "error")]
impl crate::ExitCode for crate::Error {
    fn exit_code(&self) -> i32 {
        self.inner.exit_code()
    }
}

pub(crate) mod private {
    pub trait Sealed {}

    #[cfg(feature = "anyhow")]
    impl Sealed for anyhow::Error {}
    #[cfg(feature = "error")]
    impl Sealed for crate::Error {}
    impl Sealed for crate::CliError {}
}
