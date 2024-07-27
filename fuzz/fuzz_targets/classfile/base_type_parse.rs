#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::BaseType;

fuzz_target!(|data: &[u8]| {
    if data.len() < 1 {
        return;
    }
    let _ = BaseType::parse(data[0] as char);
});
