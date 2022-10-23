use anyhow::{anyhow, Context};
use narrate::{error_from, CliError, ErrorWrap, ExitCode};

use crate::util::{cli_config_res, error_stub_res, ErrorStub};

#[test]
fn anyhow_error() {
    let err = anyhow!("err msg");
    assert_eq!(exitcode::SOFTWARE, err.exit_code());
}

#[test]
fn narrate_error() {
    let err = error_from!("err msg");
    assert_eq!(exitcode::SOFTWARE, err.exit_code());
}

#[test]
fn anyhow_cli_error() {
    let err = anyhow!(CliError::Config);
    assert_eq!(exitcode::CONFIG, err.exit_code());
}

#[test]
fn narrate_cli_error() {
    let err = error_from!(CliError::Config);
    assert_eq!(exitcode::CONFIG, err.exit_code());
}

#[test]
fn anyhow_wrapped_cli_error() {
    let err = anyhow!(CliError::Config).context(ErrorStub);
    assert_eq!(exitcode::CONFIG, err.exit_code());
}

#[test]
fn narrate_wrapped_cli_error() {
    let err = error_from!(CliError::Config).wrap(ErrorStub);
    assert_eq!(exitcode::CONFIG, err.exit_code());
}

#[test]
fn anyhow_result_wrapped_cli_error() {
    let err = cli_config_res().context(ErrorStub).unwrap_err();
    assert_eq!(exitcode::CONFIG, err.exit_code());
}

#[test]
fn narrate_result_wrapped_cli_error() {
    let err = cli_config_res().wrap(ErrorStub).unwrap_err();
    assert_eq!(exitcode::CONFIG, err.exit_code());
}

#[test]
fn anyhow_result_wrapping_cli_error() {
    let err = error_stub_res().context(CliError::Config).unwrap_err();
    assert_eq!(exitcode::CONFIG, err.exit_code());
}

#[test]
fn narrate_result_wrapping_cli_error() {
    let err = error_stub_res().wrap(CliError::Config).unwrap_err();
    assert_eq!(exitcode::CONFIG, err.exit_code());
}
