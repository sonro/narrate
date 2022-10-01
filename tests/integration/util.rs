use std::{
    error::Error as StdError,
    fmt::{Debug, Display},
};

use narrate::Result;

pub fn assert_function_error<E, F>(expected: &ExpectedErr<E>, function: F)
where
    E: Display + Debug + Send + Sync + 'static,
    F: FnOnce() -> Result<()>,
{
    let err = function().expect_err("function should error");
    assert!(err.is::<E>());
    assert_eq!(err.to_string(), expected.to_string());
    assert_eq!(expected.help_msg, err.help());
}

#[derive(Debug)]
pub struct ErrorStub;

impl Display for ErrorStub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ErrorStub")
    }
}

impl StdError for ErrorStub {}

pub fn always_errors() -> Result<(), ErrorStub> {
    Err(ErrorStub)
}

pub struct ExpectedErr<E> {
    error: E,
    help_msg: Option<&'static str>,
}

impl<E> ExpectedErr<E> {
    pub fn new(err: E) -> Self {
        Self {
            error: err,
            help_msg: None,
        }
    }

    pub fn new_with_help(err: E, help_msg: &'static str) -> Self {
        Self {
            error: err,
            help_msg: Some(help_msg),
        }
    }
}

impl<E: Display> Display for ExpectedErr<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error.fmt(f)
    }
}
