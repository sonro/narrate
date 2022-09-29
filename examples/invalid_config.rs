use narrate::{report, CliError, Error, ErrorWrap, ExitCode, Result};

fn main() {
    let res = config_error()
        .wrap(CliError::Config)
        .add_help("See https://docs.example.rs/config for more info");

    if let Err(ref err) = res {
        report::err_full(err);
        std::process::exit(err.exit_code());
    }
}

/// Equivalent to:
/// ```no_run
/// serde_json::from_str(&json)
///     .wrap(|| format!("bad config file `{}`", path))
/// ```
fn config_error() -> Result<(), Error> {
    // simulate deserialization error
    let error = narrate::error_from!("missing key: 'port'");
    // wrap with config error
    let error = error.wrap("bad config file `/app/config.toml`");

    Err(error)
}
