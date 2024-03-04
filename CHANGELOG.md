# Changelog

## [0.1.4] - 2024-03-04

### Added

- Support for `no_std`: Rust std usage is now behind the enabled by default `std` feature.

### Changed

- Rust minimum supported version is set to 1.63.

## [0.1.3] - 2022-03-26

### Added

- `LanguageTag` now implements Serde `Serialize` and `Deserialize` trait if the `serde` crate is present.
  The serialization is a plain string.

## [0.1.2] - 2021-04-16

### Added

- `LanguageTag` struct with a parser, case normalization and components accessors.

### Changed

- Proper attribution from [`language-tags`](https://github.com/pyfisch/rust-language-tags/).
