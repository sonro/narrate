use std::{error::Error as StdError, fmt};

use crate::{Chain, CliError, Error, ExitCode};

mod chain;
mod wrap;

impl Error {
    pub fn new<E>(error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self {
            inner: error.into(),
            help: None,
        }
    }

    #[inline]
    pub fn from_anyhow(error: anyhow::Error) -> Self {
        Self {
            inner: error,
            help: None,
        }
    }

    pub fn wrap<C>(self, context: C) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        Self {
            inner: self.inner.context(context),
            help: self.help,
        }
    }

    pub fn is<E>(&self) -> bool
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.downcast_ref::<E>().is_some()
    }

    pub fn downcast<E>(self) -> Result<E, anyhow::Error>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast()
    }

    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast_ref()
    }

    pub fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.inner.downcast_mut()
    }

    pub fn chain(&self) -> Chain {
        self.inner.chain().into()
    }

    pub fn route_cause(&self) -> &(dyn StdError + 'static) {
        self.inner.root_cause()
    }

    #[inline]
    pub fn help(&self) -> Option<&str> {
        self.help.as_ref().map(AsRef::as_ref)
    }

    #[inline]
    pub fn set_help_owned(&mut self, msg: String) {
        self.help = Some(HelpMsg::Owned(msg));
    }

    #[inline]
    pub fn set_help(&mut self, msg: &'static str) {
        self.help = Some(HelpMsg::Static(msg));
    }
}

impl ExitCode for Error {
    fn exit_code(&self) -> i32 {
        self.inner.exit_code()
    }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    fn from(err: E) -> Self {
        Self {
            inner: err.into(),
            help: None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum HelpMsg {
    Owned(String),
    Static(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.help {
            None => self.inner.fmt(f),
            Some(ref help) => write!(f, "{}\n\n{}", self.inner, help),
        }
    }
}

impl AsRef<str> for HelpMsg {
    fn as_ref(&self) -> &str {
        match self {
            HelpMsg::Owned(ref s) => s,
            HelpMsg::Static(s) => s,
        }
    }
}

impl fmt::Display for HelpMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HelpMsg::Owned(help) => help.fmt(f),
            HelpMsg::Static(help) => help.fmt(f),
        }
    }
}

impl<'a> PartialEq<&'a str> for HelpMsg {
    fn eq(&self, r: &&'a str) -> bool {
        match self {
            Self::Owned(l) => l == r,
            Self::Static(l) => l == r,
        }
    }
}

impl ExitCode for anyhow::Error {
    fn exit_code(&self) -> i32 {
        if let Some(err) = self.downcast_ref::<CliError>() {
            err.exit_code()
        } else if let Some(err) = self.downcast_ref::<Error>() {
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
