#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::mutf8;

fuzz_target!(|data: &[u8]| {
    let _ = mutf8::from_bytes(data);
});
