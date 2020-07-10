# Changelog

## [0.1.1] - 2020-07-10

### Added
- `PartialEq` and `From` implementations between `LanguageTag` and some string types.
- `LanguageTag` order and hash is now the same as `str`.
- `Borrow<Target=&str>` and `AsRef<Target=&str>` implementations for `LanguageTag`.

## [0.1.0] - 2020-05-01

### Added
- `LanguageTag` struct with a parser, case normalization and components accessors.
