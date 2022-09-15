use std::{env, fs, path::PathBuf};

use narrate::{CliError, ErrorWrap};

use crate::util::{always_errors, assert_function_error, ExpectedErr};

#[test]
fn error_stub() {
    assert_function_error(&ExpectedErr::new("context"), || {
        always_errors().wrap(|| "context")?;
        Ok(())
    });
}

#[test]
fn cli_error_protocol() {
    assert_function_error(&ExpectedErr::new(CliError::Protocol), || {
        always_errors().wrap(|| CliError::Protocol)?;
        Ok(())
    });
}

#[test]
fn cli_error_create_file_with_help() {
    let mut path = non_existant_dir();
    path.push("file_name");
    let help_msg = "Consider a better file name";

    let expected = ExpectedErr::new_with_help(CliError::CreateFile(path.clone()), help_msg);

    let function = || {
        fs::File::create(&path).wrap_help(|| CliError::CreateFile(path.clone()), help_msg)?;
        Ok(())
    };

    assert_function_error(&expected, function)
}

fn non_existant_dir() -> PathBuf {
    let mut path = env::temp_dir();
    path.push("unknown_dir");
    assert!(!path.exists(), "dir should not exist: `{}`", path.display());
    path
}
