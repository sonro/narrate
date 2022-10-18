use std::{env, fs, path::PathBuf};

use narrate::{CliError, ErrorWrap};

use crate::util::{assert_function_error, error_stub_res, ok_res, ErrorStub, ExpectedErr};

#[test]
fn context_str() {
    let context = "context";
    assert_function_error(&ExpectedErr::new(context), || {
        error_stub_res().wrap(context)
    });
}

#[test]
fn error_type() {
    assert_function_error(&ExpectedErr::new(CliError::Protocol), || {
        error_stub_res().wrap(CliError::Protocol)
    });
}

#[test]
fn lazy_error_type() {
    let path = PathBuf::from("file_path");
    assert_function_error(
        &ExpectedErr::new(CliError::CreateFile(path.clone())),
        || error_stub_res().wrap_with(|| CliError::CreateFile(path.clone())),
    );
}

#[test]
fn wrap_with_is_lazy() {
    let mut touched = false;
    let _ = ok_res().wrap_with(|| {
        touched = true;
        ""
    });
    assert!(!touched);

    let _ = error_stub_res().wrap_with(|| {
        touched = true;
        ""
    });
    assert!(touched);
}

#[test]
fn add_help_no_error() {
    let help = "help msg";
    assert_function_error(&ExpectedErr::new_with_help(ErrorStub, help), || {
        error_stub_res().add_help(help)
    });
}

#[test]
fn add_help_lazy() {
    assert_function_error(&ExpectedErr::new_with_help(ErrorStub, "12"), || {
        error_stub_res().add_help_with(|| format!("1{}", 2))
    });
}

#[test]
fn add_help_with_is_lazy() {
    let mut touched = false;
    let _ = ok_res().add_help_with(|| {
        touched = true;
        ""
    });
    assert!(!touched);

    let _ = error_stub_res().add_help_with(|| {
        touched = true;
        ""
    });
    assert!(touched);
}

#[test]
fn lazy_wrap_fs_function_with_help() {
    let mut path = non_existent_dir();
    path.push("file_name");
    let help_msg = "Consider a better file name";

    let expected = ExpectedErr::new_with_help(CliError::CreateFile(path.clone()), help_msg);
    let function = || {
        fs::File::create(&path)
            .wrap_with(|| CliError::CreateFile(path.clone()))
            .add_help(help_msg)?;
        Ok(())
    };
    assert_function_error(&expected, function)
}

fn non_existent_dir() -> PathBuf {
    let mut path = env::temp_dir();
    path.push("unknown_dir");
    assert!(!path.exists(), "dir should not exist: `{}`", path.display());
    path
}
