# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `error_from` macro copied from `anyhow::anyhow`.
- `bail` macro copied from `anyhow::bail`.
- `Chain` type for iterating through wrapped errors.
- Re-export colored crate as narrate publically depends on their
  features.
- Re-exprot `Color` from colored for use in `report::status`.

### Fixed

- `anyhow::Chain` type not used as return type of `Error::chain` method.

## [0.1.2] - 2022-09-15

### Fixed

- Multiple `Error` help messages.
- Wrapping a `Result` with an `Error`.

## [0.1.1] - 2022-09-15

### Added

- Console status and error messaging with `report::status` and `report::err`.
- `Error` methods delegating to inner `anyhow::Error`.
- Convertion from `Error` to `anyhow::Error`.
- More robust test-suite.

## [0.1.0] - 2022-09-14

### Added

- `Error` type to wrap around `anyhow::Error` with a help message.
- `ErrorWrap` trait to add context to errors.
- `Result` convience type.
- `ExitCode` trait for [sysexits.h](https://man.openbsd.org/sysexits.3) program
  exit codes.

[Unreleased]: https://github.com/sonro/narrate/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/sonro/narrate/compare/v0.1.2
[0.1.1]: https://github.com/sonro/narrate/releases/tag/v0.1.1
[0.1.0]: https://github.com/sonro/narrate/releases/tag/v0.1.0
