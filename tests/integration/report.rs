use std::process::{Command, Output, Stdio};

use narrate::{error_from, Error};

const STATUS_TEST_NAME: &str = "status_test";
const ERR_TEST_NAME: &str = "report_err_test";
const ERR_FULL_TEST_NAME: &str = "report_err_full_test";

#[test]
fn status_output_to_stderr() {
    let output = test_bin(STATUS_TEST_NAME, &["hi", "world", "green"]);
    // don't test for color because `status()` only outputs it to a TTY
    // by piping stderr in `test_bin` we therefore remove the color output
    let expected = format!("{:>12} {}\n", "hi", "world");
    assert_stderr(&expected, &output);
}

mod err {
    use super::*;

    fn err_check(errors: &[Error]) {
        let args = format_error_test_args(errors);
        let expected = format_error_test_expected(&errors[(errors.len() - 1)..]);
        let output = test_bin(ERR_TEST_NAME, &args);
        assert_stderr(&expected, &output);
    }

    #[test]
    fn single_error() {
        err_check(&[error_from!("test error string")]);
    }

    #[test]
    fn double_error() {
        err_check(&[error_from!("first error"), error_from!("second error")]);
    }

    #[test]
    fn single_error_with_help() {
        err_check(&[{
            let mut err = error_from!("error message");
            err.add_help("help message");
            err
        }]);
    }

    #[test]
    fn double_error_with_helps() {
        err_check(&[
            {
                let mut err = error_from!("inner error message");
                err.add_help("inner help message");
                err
            },
            {
                let mut err = error_from!("outer error message");
                err.add_help("outer help message");
                err
            },
        ]);
    }
}

mod err_full {
    use super::*;

    fn err_full_check(errors: &[Error]) {
        let args = format_error_test_args(errors);
        let expected = format_error_test_expected(errors);
        let output = test_bin(ERR_FULL_TEST_NAME, &args);
        assert_stderr(&expected, &output);
    }

    #[test]
    fn single_error() {
        err_full_check(&[error_from!("error message")]);
    }

    #[test]
    fn single_error_with_help() {
        err_full_check(&[{
            let mut err = error_from!("error message");
            err.add_help("help message");
            err
        }]);
    }

    #[test]
    fn double_error() {
        err_full_check(&[
            error_from!("inner error message"),
            error_from!("outer help message"),
        ]);
    }

    #[test]
    fn double_error_with_one_help() {
        err_full_check(&[
            {
                let mut err = error_from!("inner error message");
                err.add_help("inner help message");
                err
            },
            error_from!("outer error message"),
        ]);
    }

    #[test]
    fn double_error_with_helps() {
        err_full_check(&[
            {
                let mut err = error_from!("inner error message");
                err.add_help("inner help message");
                err
            },
            {
                let mut err = error_from!("outer error message");
                err.add_help("outer help message");
                err
            },
        ]);
    }
}

fn format_error_test_expected(errors: &[Error]) -> String {
    let mut list = Vec::new();
    let mut helps = Vec::new();
    let mut iter = errors.iter().rev();

    // push to error list and help list
    let mut pusher = |error: &Error, title: &str| {
        list.push(format!("{}: {}\n", title, error));
        if let Some(help) = error.help() {
            helps.push(help.to_owned());
        }
    };

    // outer error
    let first_error = iter.next().expect("at least 1 error");
    pusher(first_error, "error");

    // causes
    iter.for_each(|e| pusher(e, "cause"));

    let mut output = list.join("");

    if !helps.is_empty() {
        helps.reverse();
        let help_output = helps.join("\n");
        output = format!("{}\n{}\n", output, help_output);
    }

    output
}

fn format_error_test_args(errors: &[Error]) -> Vec<String> {
    let mut args = vec![];

    for error in errors {
        args.push(error.to_string());
        if let Some(help) = error.help() {
            args.push("-h".to_string());
            args.push(help.to_string());
        }
    }

    args
}

fn test_bin<S: AsRef<str>>(binary: &str, args: &[S]) -> Output {
    let mut full_args = vec!["run", "-q", "--all-features", "--bin", binary];
    let target = std::env::var("RUSTC_TARGET");
    if let Ok(ref target) = target {
        full_args.extend_from_slice(&["--target", target]);
    }
    full_args.push("--");
    full_args.extend(args.iter().map(|a| a.as_ref()));

    Command::new("cargo")
        .args(full_args)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| panic!("Failed to execute binary for testing: {}. {}", binary, err))
        .wait_with_output()
        .unwrap_or_else(|err| {
            panic!(
                "Failed to wait for test binary to finish: {}. {}",
                binary, err
            )
        })
}

fn assert_stderr(expected: &str, output: &Output) {
    let actual = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        expected, actual,
        "\n# EXPECTED:\n{}# ACTUAL:\n{}",
        expected, actual
    );
}
