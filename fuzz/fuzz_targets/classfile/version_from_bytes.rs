#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::Version;
use ristretto_classfile::byte_reader::ByteReader;

fuzz_target!(|data: &[u8]| {
    let mut cursor = ByteReader::new(data);
    let _ = Version::from_bytes(&mut cursor);
});
