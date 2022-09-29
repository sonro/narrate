use std::fmt::Display;

use crate::{Error, ErrorWrap};

impl<T, E> ErrorWrap<T, E> for Result<T, E>
where
    E: ext::StdError + Send + Sync + 'static,
{
    fn wrap<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|err| err.ext_context(f()))
    }

    fn wrap_help<C, F>(self, f: F, help: &'static str) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|err| err.ext_context_help(f(), help))
    }

    fn wrap_help_owned<C, F>(self, f: F, help: String) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|err| err.ext_context_help_owned(f(), help))
    }
}

mod ext {
    use crate::error::HelpMsg;

    use super::*;

    pub trait StdError {
        fn ext_context<C>(self, context: C) -> Error
        where
            C: Display + Send + Sync + 'static;

        fn ext_context_help<C>(self, context: C, help: &'static str) -> Error
        where
            C: Display + Send + Sync + 'static;

        fn ext_context_help_owned<C>(self, context: C, help: String) -> Error
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

        fn ext_context_help<C>(self, context: C, help: &'static str) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            let mut err = Error::from(self).wrap(context);
            err.set_help(help);
            err
        }

        fn ext_context_help_owned<C>(self, context: C, help: String) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            let mut err = Error::from(self).wrap(context);
            err.set_help_owned(help);
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

        fn ext_context_help<C>(self, context: C, help: &'static str) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            let mut err = self.wrap(context);
            match err.help {
                Some(HelpMsg::Owned(ref mut msg)) => {
                    msg.push('\n');
                    msg.push_str(help);
                }
                Some(HelpMsg::Static(msg)) => err.set_help_owned(format!("{}\n{}", msg, help)),

                None => err.set_help(help),
            }
            err
        }

        fn ext_context_help_owned<C>(self, context: C, help: String) -> Error
        where
            C: Display + Send + Sync + 'static,
        {
            let mut err = self.wrap(context);
            match err.help() {
                Some(msg) => err.set_help_owned(format!("{}\n{}", msg, help)),
                None => err.set_help_owned(help),
            }
            err
        }
    }
}

pub(crate) mod private {
    use super::*;

    pub trait Sealed {}

    impl<T, E> Sealed for Result<T, E> where E: ext::StdError {}
    impl<T> Sealed for Option<T> {}
}
