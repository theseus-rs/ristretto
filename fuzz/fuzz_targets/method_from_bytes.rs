#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use ristretto_classfile::{ConstantPool, Method};
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let constant_pool = ConstantPool::default();
    let data = data.to_vec();
    let mut cursor = Cursor::new(data);
    let _ = Method::from_bytes(&constant_pool, &mut cursor);
});
