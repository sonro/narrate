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
        self.map_err(|err| err.ext_add_help_with(f()))
    }
}

mod ext {
    use crate::error::HelpMsg;

    use super::*;

    pub trait StdError {
        fn ext_context<C>(self, context: C) -> Error
        where
            C: Display + Send + Sync + 'static;

        fn ext_add_help(self, help: &'static str) -> Error;

        fn ext_add_help_with<C>(self, help: C) -> Error
        where
            C: Display + Send + Sync + 'static;
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
            err.set_help(help);
            err
        }

        fn ext_add_help_with<C>(self, help: C) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            let mut err = Error::from(self);
            err.set_help_owned(help.to_string());
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
            match self.help {
                Some(HelpMsg::Owned(ref mut msg)) => {
                    msg.push('\n');
                    msg.push_str(help);
                }
                Some(HelpMsg::Static(msg)) => self.set_help_owned(format!("{}\n{}", msg, help)),

                None => self.set_help(help),
            }
            self
        }

        fn ext_add_help_with<C>(mut self, help: C) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            match self.help() {
                Some(msg) => self.set_help_owned(format!("{}\n{}", msg, help)),
                None => self.set_help_owned(help.to_string()),
            }
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
