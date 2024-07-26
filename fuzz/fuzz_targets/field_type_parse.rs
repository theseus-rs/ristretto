#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::FieldType;

fuzz_target!(|data: &[u8]| {
    let data = String::from_utf8_lossy(data).to_string();
    let _ = FieldType::parse(&data);
});
