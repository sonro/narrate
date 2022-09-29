# narrate

This library provides CLI application error and status reporting utilities. The
coloured output formatting aims to be similar to [Cargo]. Error type is a
wrapper around [Anyhow].

[![Crates.io](https://img.shields.io/crates/v/narrate.svg)](https://crates.io/crates/narrate)
[![tests](https://github.com/sonro/narrate/actions/workflows/tests.yml/badge.svg?branch=main)](https://github.com/sonro/narrate/actions/workflows/tests.yml)
[![license](https://img.shields.io/crates/l/narrate.svg)](#license)

[API Docs](https://docs.rs/narrate/latest/narrate)

## Features

- User facing status messages and error reporting
- Wrap any error with additional context
- Optional help messages for errors
- Set of standard CLI errors with exit codes conforming to
  [sysexits.h]
- Convenience `Result` type
- Drop in replacement for [Anyhow]

## How to use

- Use
  [`narrate::Result<T>`](https://docs.rs/narrate/latest/narrate/type.Result.html)
  as a return type of any fallible function.

   Within the function, use `?` to propagate any error that implements the
  [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
  trait. Same as
  [`anyhow::Result<T>`](https://docs.rs/anyhow/1.0/anyhow/type.Result.html).

  ```rust
  use narrate::Result;

  fn get_user() -> Result<User> {
      let json = std::fs::read_to_string("user.json")?;
      let user: User = serde_json::from_str(&json)?;
      Ok(user)
  }
  ```

- Wrap an error with more context by importing the
  [`narrate::ErrorWrap`](https://docs.rs/narrate/latest/narrate/trait.ErrorWrap.html)
  trait. Similar to
  [`anyhow::Context`](https://docs.rs/anyhow/1.0/anyhow/trait.Context.html),
  this can give your users more information as to why an error happened.

  ```rust
  use narrate::{CliError, ErrorWrap, Result};

  fn run() -> Result<()> {
      ...
      // wrap with contextual information
      data.acquire().wrap(|| "unable to acquire data")?;

      // wrap with another error
      config.load().wrap(|| CliError::Config)?;

      // wrap with help information
      create_dir().wrap_help(|| "project directory already exists", "Try using cargo init")?;
      ...
  }
  ```

  ```console
  error: project directory already exists
  cause: Is a directory (os error 20)

  Try using cargo init
  ```

- Use the
  [`narrate::ExitCode`](https://docs.rs/narrate/latest/narrate/trait.ExitCode.html)
  trait to get the [sysexits.h] conforming exit code from a `narrate::Error`. By
  default this is just `70 (software error)` but it can be easily implemented
  for any type.

- [`narrate::CliError`](https://docs.rs/narrate/latest/narrate/struct.CliError.html)
  collection of typical command line errors. Use them to add context to deeper
  application errors. Use their `exit_code` to conform to [sysexits.h].

  ```rust
  use narrate::{CliError, ErrorWrap, ExitCode, Result};

  fn main() {
      let res = run();

      if let Err(ref err) = res {
          std::process::exit(err.exit_code());
      }
  }

  fn run() -> Result<()> {
      will_error().wrap(|| CliError::OsErr)?
      Ok(())
  }
  ```

- Report errors to the command line with either
  [`report::err`](https://docs.rs/narrate/latest/narrate/report/fn.err.html)
  or
  [`report::err_full`](https://docs.rs/narrate/latest/narrate/report/fn.err_full.html)
  for the complete error chain.

  ```rust
  use narrate::{CliError, Error, report};

  fn main() {
      let res = run();

      if let Err(ref err) = res {
          report::err_full(&err);
          std::process::exit(err.exit_code());
      }
  }

  fn run() -> Result<()> {
      ...
      let config: Config = serde_json::from_str(&json)
          .wrap(|| "bad config file `/app/config.toml`")
          .wrap_help(
              || CliError::Config,
              "see https://docs.example.rs/config for more help",
          )?;
      ...
  }
  ```

  ![report::err_full output](/docs/report_err_full.png?raw=true)

- Report application status to the command line with
  [`report::status`](https://docs.rs/narrate/latest/narrate/report/status.err.html).
  Modeled on the output from [Cargo].

  ```rust
  use colored::Color;
  use narrate::report;

  fn main() {
      report::status("Created", "new project `spacetime`", Color::Green);
  }
  ```

  ![report::status output](/docs/report_status.png?raw=true)

## License

narrate is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

[Cargo]: https://github.com/rust-lang/cargo
[Anyhow]: https://github.com/dtolnay/anyhow
[sysexits.h]: (https://man.openbsd.org/sysexits.3)
