#![allow(dead_code)]

use std::{
    error::Error as StdError,
    fmt::{Debug, Display},
};

use narrate::{CliError, Error, Result};

pub fn assert_function_error<E, F>(expected: &ExpectedErr<E>, function: F)
where
    E: Display + Debug + Send + Sync + 'static,
    F: FnOnce() -> Result<()>,
{
    let error = function().expect_err("function should error");
    assert_error(expected, error);
}

pub fn assert_error<E>(expected: &ExpectedErr<E>, error: Error)
where
    E: Display + Debug + Send + Sync + 'static,
{
    assert!(error.is::<E>());
    assert_eq!(expected.to_string(), error.to_string());
    assert_eq!(expected.help_msg, error.help());
}

#[derive(Debug, PartialEq, Eq)]
pub struct ErrorStub;

#[derive(Debug, PartialEq, Eq)]
pub enum TestError {
    Stub(ErrorStub),
    Cli(CliError),
}

impl Display for ErrorStub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorStub")
    }
}

impl StdError for ErrorStub {}

macro_rules! fmt_err {
    ($fmt:ident, $err:ident) => {
        $fmt($err.to_string())
    };
}

impl Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = |err| write!(f, "TestError: {}", err);
        match self {
            Self::Stub(err) => fmt_err!(fmt, err),
            Self::Cli(err) => fmt_err!(fmt, err),
        }
    }
}

impl StdError for TestError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            Self::Stub(err) => err,
            Self::Cli(err) => err,
        })
    }
}

impl From<ErrorStub> for TestError {
    fn from(err: ErrorStub) -> Self {
        Self::Stub(err)
    }
}

pub fn error_stub_res() -> Result<(), ErrorStub> {
    Err(ErrorStub)
}

pub fn ok_res() -> Result<(), ErrorStub> {
    Ok(())
}

pub fn test_error_stub() -> Result<(), TestError> {
    Ok(error_stub_res()?)
}

pub struct ExpectedErr<'a, E> {
    error: E,
    help_msg: Option<&'a str>,
}

impl<'a, E> ExpectedErr<'a, E> {
    pub fn new(err: E) -> Self {
        Self {
            error: err,
            help_msg: None,
        }
    }

    pub fn new_with_help(err: E, help_msg: &'a str) -> Self {
        Self {
            error: err,
            help_msg: Some(help_msg),
        }
    }
}

impl<'a, E: Display> Display for ExpectedErr<'a, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}
