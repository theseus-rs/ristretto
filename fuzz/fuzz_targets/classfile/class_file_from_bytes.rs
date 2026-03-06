#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::ClassFile;

fuzz_target!(|data: &[u8]| {
    let _ = ClassFile::from_bytes(data);
});
