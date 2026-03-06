#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::byte_reader::ByteReader;
use ristretto_classfile::{ConstantPool, Field};

fuzz_target!(|data: &[u8]| {
    let constant_pool = ConstantPool::default();
    let mut cursor = ByteReader::new(data);
    let _ = Field::from_bytes(&constant_pool, &mut cursor);
});
