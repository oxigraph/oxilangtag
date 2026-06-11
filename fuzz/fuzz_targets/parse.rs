#![no_main]
use libfuzzer_sys::fuzz_target;
use oxilangtag::LanguageTag;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let parsed = LanguageTag::parse(s).ok();
        let normalized = LanguageTag::parse_and_normalize(s).ok();
        let normalized_upper = LanguageTag::parse_and_normalize(&s.to_ascii_uppercase()).ok();
        let normalized_lower = LanguageTag::parse_and_normalize(&s.to_ascii_lowercase()).ok();
        assert_eq!(parsed.is_some(), normalized.is_some());
        assert_eq!(normalized, normalized_upper);
        assert_eq!(normalized, normalized_lower);
    }
});
