use std::{fmt, path::PathBuf};

use crate::ExitCode;

#[derive(Debug)]
pub enum CliError {
    Config,

    CreateFile(PathBuf),

    InputData,

    InputFileNotFound(PathBuf),

    NoUser(String),

    NoHost(String),

    OperationPermission(String),

    OsErr,

    OsFileNotFound(PathBuf),

    ReadFile(PathBuf),

    ResourceNotFound(String),

    Protocol,

    Temporary,

    Usage,

    WriteFile(PathBuf),
}

impl std::error::Error for CliError {}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CliError::*;
        match self {
            Config => write!(f, "invalid configuration"),

            CreateFile(file) => write!(f, "cannot create file: {}", file.display()),

            InputData => write!(f, "invalid input data"),

            InputFileNotFound(file) => write!(f, "file not found: {}", file.display()),

            NoUser(user) => write!(f, "user not found: {}", user),

            NoHost(host) => write!(f, "host not found: {}", host),

            OperationPermission(op) => write!(f, "no permission for operation: {}", op),

            OsErr => write!(f, "operating system error"),

            OsFileNotFound(file) => write!(f, "system file not found: {}", file.display()),

            ReadFile(file) => write!(f, "cannot read file: {}", file.display()),

            ResourceNotFound(resource) => write!(f, "resource not found: {}", resource),

            Protocol => write!(f, "protocol not possible"),

            Temporary => write!(f, "temporary failure"),

            Usage => write!(f, "incorrect usage"),

            WriteFile(file) => write!(f, "cannot write to file: {}", file.display()),
        }
    }
}

impl ExitCode for CliError {
    fn exit_code(&self) -> i32 {
        self.into()
    }
}

impl From<CliError> for i32 {
    fn from(err: CliError) -> Self {
        (&err).into()
    }
}

impl From<&CliError> for i32 {
    fn from(err: &CliError) -> Self {
        use exitcode::*;
        use CliError::*;
        match err {
            Config => CONFIG,
            CreateFile(_) => CANTCREAT,
            InputData | ResourceNotFound(_) => DATAERR,
            InputFileNotFound(_) => NOINPUT,
            NoUser(_) => NOUSER,
            NoHost(_) => NOHOST,
            OperationPermission(_) => NOPERM,
            OsErr => OSERR,
            OsFileNotFound(_) => OSFILE,
            ReadFile(_) | WriteFile(_) => IOERR,
            Protocol => PROTOCOL,
            Temporary => TEMPFAIL,
            Usage => USAGE,
        }
    }
}
