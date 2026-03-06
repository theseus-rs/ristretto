#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::byte_reader::ByteReader;
use ristretto_classfile::{ConstantPool, Method};

fuzz_target!(|data: &[u8]| {
    let constant_pool = ConstantPool::default();
    let mut cursor = ByteReader::new(data);
    let _ = Method::from_bytes(&constant_pool, &mut cursor);
});
