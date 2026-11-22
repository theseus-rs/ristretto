#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::BaseType;

fuzz_target!(|data: &[u8]| {
    if let Some(&byte) = data.first() {
        let _ = BaseType::parse(char::from(byte));
    }
});
