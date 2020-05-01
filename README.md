oxilangtag
=========

[![actions status](https://github.com/oxigraph/oxilangtag/workflows/build/badge.svg)](https://github.com/oxigraph/oxilangtag/actions)
[![Latest Version](https://img.shields.io/crates/v/oxilangtag.svg)](https://crates.io/crates/oxilangtag)
[![Released API docs](https://docs.rs/oxilangtag/badge.svg)](https://docs.rs/oxilangtag)

OxiLangTag to validate and normalize language tags following [RFC 5646](https://tools.ietf.org/html/rfc5646)
([BCP 47](https://tools.ietf.org/html/bcp47)).

It allows zero stack allocation language tag validation.

Example:
```rust
use oxilangtag::LanguageTag;

let language_tag = LanguageTag::parse("en-US").unwrap();
assert_eq!("en-US", language_tag.into_inner());
```


## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)
   
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Futures by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
