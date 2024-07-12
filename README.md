# narrate

[![Crates.io](https://img.shields.io/crates/v/narrate.svg)](https://crates.io/crates/narrate)
[![msrv
1.61.1](https://img.shields.io/badge/msrv-1.61.1-dea584.svg?logo=rust)](https://github.com/rust-lang/rust/releases/tag/1.61.1)
[![tests](https://img.shields.io/github/actions/workflow/status/sonro/varj/release.yml?label=tests&logo=github)](https://github.com/sonro/narrate/actions/workflows/tests.yml)
[![Documentation](https://img.shields.io/docsrs/narrate?logo=docs.rs)](https://docs.rs/narrate/)
[![license](https://img.shields.io/crates/l/narrate.svg)](#license)

This library provides CLI application error and status reporting utilities. The
coloured output formatting aims to be similar to [Cargo]. Error type is a
wrapper around [anyhow], with optional help messages.

## Features

- User facing status messages and error reporting
- Wrap any error with additional context
- Optional help messages for errors
- Set of standard CLI errors with exit codes conforming to
  [sysexits.h]
- Convenience `Result` type
- Replace/integrate with [anyhow]

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
      data.acquire().wrap("unable to acquire data")?;

      // wrap with another error
      config.load().wrap(CliError::Config)?;

      // wrap with lazily evaulated string or error
      config.load().wrap_with(|| format!("cannot load {}", path))?;

      // wrap with help information
      create_dir()
        .wrap("project directory already exists")
        .add_help("Try using cargo init")?;
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
  default this is just `70 (software error)`, but using an apropriate `CliError`
  will change this.

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
      will_error().wrap(CliError::OsErr)?
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
          .wrap("bad config file `/app/config.toml`")
          .wrap(CliError::Config)
          .add_help("see https://docs.example.rs/config for more help")?;
      ...
  }
  ```

  ![report::err_full output](/docs/report_err_full.png?raw=true)

- Report application status to the command line with
  [`report::status`](https://docs.rs/narrate/latest/narrate/report/fn.status.html).
  Modeled on the output from [Cargo].

  ```rust
  use narrate::{Color, report};

  fn main() {
      report::status("Created", "new project `spacetime`", Color::Green);
  }
  ```

  ![report::status output](/docs/report_status.png?raw=true)

Please view the [API Docs](https://docs.rs/narrate/) and [examples](examples/)
for more information.

## FAQ

### Should I use narrate instead of [anyhow] or [eyre]?

Anyhow is a great tool for handling errors in your CLI app, but it doesn't come
with its own reporting, common set of errors, or the ability to add separate
help messages.

Eyre and its companion crates offer fine-grained error reporting and is far more
customizable than narrate - which is opinionated in copying Cargo's style. If
you don't need that much control, narrate provides a simpler alternative. Plus
the added benefit of reporting statuses, not just errors.

### Can I just pretty print my [anyhow] errors?

If you just use the `report`
[Cargo feature flag](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features),
you can access the [`report`](https://docs.rs/narrate/latest/narrate/report)
  module and thus the `anyhow_err` and `anyhow_err_full` functions.

```toml
# Cargo.toml
[dependencies]
narrate = { version = "0.4.0", default-features = false, features = ["report"] }
```

```rust
// main.rs
use narrate::report;

fn main() {
    if let Err(err) => run() {
        report::anyhow_err_full(err);    
    }
}

fn run() -> anyhow::Result<()> {
  ...
}
```

## Contributing

**Thank you very much for considering to contribute to this project!**

We welcome any form of contribution:

- New issues (feature requests, bug reports, questions, ideas, ...)
- Pull requests (documentation improvements, code improvements, new features,
  ...)

**Note**: Before you take the time to open a pull request, please open an issue
first.

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

narrate is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

[Cargo]: https://github.com/rust-lang/cargo
[anyhow]: https://github.com/dtolnay/anyhow
[eyre]: https://github.com/yaahc/eyre
[sysexits.h]: (https://man.openbsd.org/sysexits.3)
