use std::{env, fs, path::PathBuf};

use narrate::{CliError, ErrorWrap, ExitCode, Result};

#[test]
fn wrap_err_help_function() {
    let mut path = env::temp_dir();
    path.push("unknown_dir");
    assert!(!path.exists(), "dir should not exist: `{}`", path.display());
    path.push("file_name");

    let help_msg = "Consider a better file name";

    fn fun(path: PathBuf, help: &'static str) -> Result<()> {
        fs::File::create(&path).wrap_help(|| CliError::CreateFile(path), help)?;
        Ok(())
    }

    let err = fun(path, help_msg).expect_err("function should error");
    assert_eq!(exitcode::CANTCREAT, err.exit_code());

    let err_help = err.help().expect("error has help message");
    assert_eq!(help_msg, err_help);
}
