use std::fmt::Display;

use crate::{Error, ErrorWrap};

impl<T, E> ErrorWrap<T, E> for Result<T, E>
where
    E: ext::StdError + Send + Sync + 'static,
{
    fn wrap<C>(self, context: C) -> crate::Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
    {
        self.map_err(|err| err.ext_context(context))
    }

    fn wrap_with<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|err| err.ext_context(f()))
    }

    fn add_help(self, help: &'static str) -> Result<T, Error> {
        self.map_err(|err| err.ext_add_help(help))
    }

    fn add_help_with<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|err| err.ext_add_help_with(f))
    }
}

mod ext {
    use super::*;

    pub trait StdError {
        fn ext_context<C>(self, context: C) -> Error
        where
            C: Display + Send + Sync + 'static;

        fn ext_add_help(self, help: &'static str) -> Error;

        fn ext_add_help_with<C, F>(self, f: F) -> Error
        where
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C;
    }

    impl<E> StdError for E
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        fn ext_context<C>(self, context: C) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            Error::from(self).wrap(context)
        }

        fn ext_add_help(self, help: &'static str) -> Error {
            let mut err = Error::from(self);
            err.add_help(help);
            err
        }

        fn ext_add_help_with<C, F>(self, f: F) -> Error
        where
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C,
        {
            let mut err = Error::from(self);
            err.add_help_with(f);
            err
        }
    }

    impl StdError for Error {
        fn ext_context<C>(self, context: C) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            self.wrap(context)
        }

        fn ext_add_help(mut self, help: &'static str) -> Error {
            self.add_help(help);
            self
        }

        fn ext_add_help_with<C, F>(mut self, f: F) -> Error
        where
            C: Display + Send + Sync + 'static,
            F: FnOnce() -> C,
            F: FnOnce() -> C,
        {
            self.add_help_with(f);
            self
        }
    }
}

pub(crate) mod private {
    use super::*;

    pub trait Sealed {}

    impl<T, E> Sealed for Result<T, E> where E: ext::StdError {}
    impl<T> Sealed for Option<T> {}
}
