#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::mutf8;

fuzz_target!(|data: &[u8]| {
    if let Ok(value) = std::str::from_utf8(data) {
        let _ = mutf8::to_bytes(value);
    }
});
