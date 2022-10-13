use anyhow::anyhow;
use narrate::{CliError, Error};

use crate::util::{test_error_stub, ErrorStub, TestError};

#[test]
fn new_transparent_display() {
    let error = Error::new(ErrorStub);
    assert_eq!(ErrorStub.to_string(), error.to_string());
}

#[test]
fn from_anyhow_transparent_display() {
    let any_err = anyhow!("anyhow error");
    let original_msg = any_err.to_string();
    let error = Error::from_anyhow(any_err);
    assert_eq!(original_msg, error.to_string());
}

#[test]
fn wrap_transparent_display() {
    let error = Error::new(ErrorStub).wrap(CliError::Temporary);
    assert_eq!(CliError::Temporary.to_string(), error.to_string());
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
fn help() {
    let error = Error::new(ErrorStub);
    assert_eq!(None, error.help());
}

#[test]
fn add_help_once() {
    let help = "help message";
    let mut error = Error::new(ErrorStub);
    error.add_help(help);
    assert_eq!(Some(help), error.help());
}

#[test]
fn add_help_twice() {
    let help_1 = "first help";
    let help_2 = "second help";
    let mut error = Error::new(ErrorStub);
    error.add_help(help_1);
    error.add_help(help_2);
    let combined = format!("{}\n{}", help_1, help_2);
    let expected = Some(combined.as_str());
    assert_eq!(expected, error.help());
}

#[test]
fn add_help_with() {
    let msg = "help";
    let mut error = Error::new(ErrorStub);
    error.add_help_with(|| msg);
    assert_eq!(Some(msg), error.help());
}

#[test]
fn add_help_with_twice() {
    let help_1 = "first help";
    let help_2 = "second help";
    let mut error = Error::new(ErrorStub);
    error.add_help_with(|| help_1);
    error.add_help_with(|| help_2);
    let combined = format!("{}\n{}", help_1, help_2);
    let expected = Some(combined.as_str());
    assert_eq!(expected, error.help());
}
