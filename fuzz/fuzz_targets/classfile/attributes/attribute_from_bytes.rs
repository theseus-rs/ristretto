#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::attributes::Attribute;
use ristretto_classfile::ConstantPool;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let data = data.to_vec();
    let mut cursor = Cursor::new(data);
    let constant_pool = ConstantPool::default();
    let _ = Attribute::from_bytes(&constant_pool, &mut cursor);
});
