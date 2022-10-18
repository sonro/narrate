use anyhow::anyhow;
use narrate::{CliError, Error};

use crate::util::{assert_error, test_error_stub, ErrorStub, ExpectedErr, TestError};

#[test]
fn new() {
    let error = Error::new(ErrorStub);
    assert_error(&ExpectedErr::new(ErrorStub), error)
}

#[test]
fn from_anyhow() {
    let msg = "anyhow error";
    let any_err = anyhow!(msg);
    let error = Error::from_anyhow(any_err);
    assert_error(&ExpectedErr::new(msg), error)
}

#[test]
fn wrap_transparent_display() {
    let error = Error::new(ErrorStub).wrap(CliError::Temporary);
    assert_error(&ExpectedErr::new(CliError::Temporary), error)
}

#[test]
fn is_original() {
    let error = Error::new(ErrorStub);
    assert!(error.is::<ErrorStub>());
}

#[test]
fn is_not_original() {
    let error = Error::new(ErrorStub);
    assert!(!error.is::<CliError>());
}

#[test]
fn downcast_to_original() {
    let error = Error::new(ErrorStub);
    if let Ok(original) = error.downcast::<ErrorStub>() {
        assert_eq!(ErrorStub, original);
    }
}

#[test]
fn downcast_ref_to_original() {
    let error = Error::new(ErrorStub);
    if let Some(original) = error.downcast_ref::<ErrorStub>() {
        assert_eq!(&ErrorStub, original);
    }
}

#[test]
fn downcast_mut_to_original() {
    let mut error = Error::new(ErrorStub);
    if let Some(original) = error.downcast_mut::<ErrorStub>() {
        assert_eq!(&ErrorStub, original);
    }
}

#[test]
fn wrapped_error_can_use_is() {
    let error = Error::new(ErrorStub).wrap(CliError::Temporary);
    assert!(error.is::<CliError>());
    assert!(error.is::<ErrorStub>());
}

#[test]
fn wrapped_error_can_downcast() {
    let error = Error::new(ErrorStub).wrap(CliError::Temporary);
    if let Ok(original) = error.downcast::<ErrorStub>() {
        assert_eq!(ErrorStub, original);
    }
}

#[test]
fn wrapped_error_chain_downcast() {
    let error = Error::from(ErrorStub).wrap("context");
    assert!(error.chain().any(|cause| cause.is::<ErrorStub>()));
}

#[test]
fn root_cause_from_function() {
    let error = Error::from(test_error_stub().expect_err("should error"));
    assert_ne!(
        TestError::Stub(ErrorStub).to_string(),
        error.root_cause().to_string()
    );
    assert_eq!(ErrorStub.to_string(), error.root_cause().to_string());
}

#[test]
fn add_help_once() {
    let help = "help message";
    let mut error = Error::new(ErrorStub);
    error.add_help(help);
    assert_error(&ExpectedErr::new_with_help(ErrorStub, help), error);
}

#[test]
fn add_help_twice() {
    let help_1 = "first help";
    let help_2 = "second help";
    let mut error = Error::new(ErrorStub);
    error.add_help(help_1);
    error.add_help(help_2);
    let combined = format!("{}\n{}", help_1, help_2);
    assert_error(&ExpectedErr::new_with_help(ErrorStub, &combined), error);
}

#[test]
fn add_help_with() {
    let msg = "help";
    let mut error = Error::new(ErrorStub);
    error.add_help_with(|| msg);
    assert_error(&ExpectedErr::new_with_help(ErrorStub, msg), error);
}

#[test]
fn add_help_with_twice() {
    let help_1 = "first help";
    let help_2 = "second help";
    let mut error = Error::new(ErrorStub);
    error.add_help_with(|| help_1);
    error.add_help_with(|| help_2);
    let combined = format!("{}\n{}", help_1, help_2);
    assert_error(&ExpectedErr::new_with_help(ErrorStub, &combined), error);
}
