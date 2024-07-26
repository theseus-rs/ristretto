#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::{ConstantPool, Field};
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let constant_pool = ConstantPool::default();
    let data = data.to_vec();
    let mut cursor = Cursor::new(data);
    let _ = Field::from_bytes(&constant_pool, &mut cursor);
});
