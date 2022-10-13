use std::{env, fs, path::PathBuf};

use narrate::{CliError, ErrorWrap};

use crate::util::{assert_function_error, error_stub_res, ExpectedErr};

#[test]
fn error_stub() {
    assert_function_error(&ExpectedErr::new("context"), || {
        error_stub_res().wrap("context")
    });
}

#[test]
fn cli_error_protocol() {
    assert_function_error(&ExpectedErr::new(CliError::Protocol), || {
        error_stub_res().wrap(CliError::Protocol)
    });
}

#[test]
fn cli_error_create_file_with_help() {
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
