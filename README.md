# narrate

This library provides CLI application error and status reporting utilities. The
coloured output formatting aims to be similar to [Cargo]. Error type is a
wrapper around [Anyhow].

[![Crates.io](https://img.shields.io/crates/v/narrate.svg)](https://crates.io/crates/narrate)
[![license](https://img.shields.io/crates/l/narrate.svg)](#license)

## Features

- Wrap any error with additional context
- Optional help messages for errors
- Set of standard CLI errors with exit codes conforming to
  [sysexits.h]
- Convenience `Result` type

### Planned features

- Error chain output
- Status messages

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
      data.acquire().wrap(|| "unable to aquire data")?;

      // wrap with another error
      config.load().wrap(|| CliError::Config)?;

      // wrap with help information
      create_dir().wrap_help(|| "project directory already exists", "try using cargo init")?;
      ...
  }
  ```

  ```console
  error: project directory already exists
  cause: Is a directory (os error 20)

  help: try using cargo init
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

## License

narrate is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

[Cargo]: https://github.com/rust-lang/cargo
[Anyhow]: https://github.com/dtolnay/anyhow
[sysexits.h]: (https://man.openbsd.org/sysexits.3)
