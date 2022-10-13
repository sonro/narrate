# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `add_help` and `add_help_with` methods for `Error`.

### Changed

- Deprecate `set_help` and `set_help_owned` methods for `Error` type.
- [**BREAKING**] `Display` implementation simplified for `Error`.
- [**BREAKING**] `report::err` module now only prints relevant help.
- [**BREAKING**] `Error::downcast` method no longer returns an `anyhow::Result`.

## [0.2.0] - 2022-09-29

### Added

- Re-export anyhow crate.

### Changed

#### [**BREAKING**] `ErrorWrap`

- Seal trait so it cannot be implemented for user's types.
- Lazily evaluated method renamed `wrap_with`.
- Method `wrap` is no longer lazily evaluated.
- Adding help is now done through the method `add_help` and lazy version
  `add_help_with`.

#### Other

- [**BREAKING**] Fix public API typo for `Error` method `route_cause` to
  `root_cause`.

## [0.1.3] - 2022-09-28

### Added

- `error_from` macro copied from `anyhow::anyhow`.
- `bail` macro copied from `anyhow::bail`.
- `Chain` type for iterating through wrapped errors.
- Re-export colored crate as narrate publicly depends on their
  features.
- Re-export `Color` from colored for use in `report::status`.

### Changed

- `anyhow::Chain` type not used as return type of `Error::chain` method.

### Documentation

- Add rustdoc for the crate's public API.

## [0.1.2] - 2022-09-15

### Changed

- Multiple `Error` help messages.
- Wrapping a `Result` with an `Error`.

## [0.1.1] - 2022-09-15

### Added

- Console status and error messaging with `report::status` and `report::err`.
- `Error` methods delegating to inner `anyhow::Error`.
- Conversion from `Error` to `anyhow::Error`.
- More robust test-suite.

## [0.1.0] - 2022-09-14

### Added

- `Error` type to wrap around `anyhow::Error` with a help message.
- `ErrorWrap` trait to add context to errors.
- `Result` convince type.
- `ExitCode` trait for [sysexits.h](https://man.openbsd.org/sysexits.3) program
  exit codes.

[Unreleased]: https://github.com/sonro/narrate/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/sonro/narrate/compare/v0.2.0
[0.1.3]: https://github.com/sonro/narrate/compare/v0.1.3
[0.1.2]: https://github.com/sonro/narrate/compare/v0.1.2
[0.1.1]: https://github.com/sonro/narrate/releases/tag/v0.1.1
[0.1.0]: https://github.com/sonro/narrate/releases/tag/v0.1.0
