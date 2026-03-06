#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::ReferenceKind;
use ristretto_classfile::byte_reader::ByteReader;

fuzz_target!(|data: &[u8]| {
    let mut cursor = ByteReader::new(data);
    let _ = ReferenceKind::from_bytes(&mut cursor);
});
