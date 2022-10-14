use std::path::PathBuf;

use narrate::{CliError, ExitCode};

#[test]
fn outputs() {
    cli_error_array().iter().for_each(assert_error_msg_and_code);
}

#[test]
fn impl_std_error() {
    fn assert_std_error(_e: impl std::error::Error) {}
    assert_std_error(CliError::Config);
}

struct ErrorTest {
    error: CliError,
    msg: String,
    code: i32,
}

fn assert_error_msg_and_code(error_test: &ErrorTest) {
    let ErrorTest { error, msg, code } = error_test;
    assert_eq!(
        *code,
        error.exit_code(),
        "CliError: `{:?}` ExitCode should be: `{}`, got: `{}`",
        error,
        code,
        error.exit_code()
    );
    assert_eq!(
        *msg,
        error.to_string(),
        "\nCliError: `{:?}`\nshould display: `{}`\n           got: `{}`\n",
        error,
        msg,
        error,
    );
}

fn cli_error_array() -> [ErrorTest; 15] {
    let path_buf = PathBuf::from("path");
    [
        ErrorTest {
            error: CliError::Config,
            msg: "invalid configuration".into(),
            code: exitcode::CONFIG,
        },
        ErrorTest {
            error: CliError::CreateFile(path_buf.clone()),
            msg: format!("cannot create file: {}", path_buf.display()),
            code: exitcode::CANTCREAT,
        },
        ErrorTest {
            error: CliError::InputData,
            msg: "invalid input data".into(),
            code: exitcode::DATAERR,
        },
        ErrorTest {
            error: CliError::InputFileNotFound(path_buf.clone()),
            msg: format!("file not found: {}", path_buf.display()),
            code: exitcode::NOINPUT,
        },
        ErrorTest {
            error: CliError::NoUser("username".into()),
            msg: "user not found: username".into(),
            code: exitcode::NOUSER,
        },
        ErrorTest {
            error: CliError::NoHost("hostname".into()),
            msg: "host not found: hostname".into(),
            code: exitcode::NOHOST,
        },
        ErrorTest {
            error: CliError::OperationPermission("operation".into()),
            msg: "no permission for operation: operation".into(),
            code: exitcode::NOPERM,
        },
        ErrorTest {
            error: CliError::OsErr,
            msg: "operating system error".into(),
            code: exitcode::OSERR,
        },
        ErrorTest {
            error: CliError::OsFileNotFound(path_buf.clone()),
            msg: format!("system file not found: {}", path_buf.display()),
            code: exitcode::OSFILE,
        },
        ErrorTest {
            error: CliError::ReadFile(path_buf.clone()),
            msg: format!("cannot read file: {}", path_buf.display()),
            code: exitcode::IOERR,
        },
        ErrorTest {
            error: CliError::ResourceNotFound("resource".into()),
            msg: "resource not found: resource".into(),
            code: exitcode::DATAERR,
        },
        ErrorTest {
            error: CliError::Protocol,
            msg: "protocol not possible".into(),
            code: exitcode::PROTOCOL,
        },
        ErrorTest {
            error: CliError::Temporary,
            msg: "temporary failure".into(),
            code: exitcode::TEMPFAIL,
        },
        ErrorTest {
            error: CliError::Usage,
            msg: "incorrect usage".into(),
            code: exitcode::USAGE,
        },
        ErrorTest {
            error: CliError::WriteFile(path_buf.clone()),
            msg: format!("cannot write to file: {}", path_buf.display()),
            code: exitcode::IOERR,
        },
    ]
}
