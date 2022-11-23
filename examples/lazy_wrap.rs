//! Use lazy evaluation to wrap errors in more context.
//!
//! Will print (with colors):
//!
//! ```console
//! error: unable to load data from file: `/not/an/exsisting/file`
//! cause: file not found: /not/an/exsisting/file
//! cause: No such file or directory (os error 2)
//! ```

use std::{
    fs,
    path::{Path, PathBuf},
};

use narrate::{report, CliError, ErrorWrap, ExitCode, Result};

fn main() {
    let path = PathBuf::from("/not/an/exsisting/file");
    let res = run(path);

    if let Err(ref err) = res {
        report::err_full(err);
        std::process::exit(err.exit_code());
    }
}

fn run(path: PathBuf) -> Result<()> {
    // lazily create context string
    let _ = load_data(&path)
        .wrap_with(|| format!("unable to load data from file: `{}`", path.display()))?;
    Ok(())
}

fn load_data(path: &Path) -> Result<String> {
    // lazily create CliError
    let contents =
        fs::read_to_string(path).wrap_with(|| CliError::InputFileNotFound(path.to_owned()))?;

    // lazily create context string
    let data =
        parse(&contents).wrap_with(|| format!("unable to parse file: `{}`", path.display()))?;

    Ok(data)
}

fn parse(raw: &str) -> Result<String> {
    // fake parsing
    Ok(raw.to_ascii_lowercase())
}
