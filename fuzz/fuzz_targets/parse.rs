#![no_main]
use libfuzzer_sys::fuzz_target;
use oxilangtag::LanguageTag;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let _ = LanguageTag::parse(s);
    }
});
