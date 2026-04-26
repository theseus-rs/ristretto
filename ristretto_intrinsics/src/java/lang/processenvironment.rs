use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::mutf8;
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use zerocopy::transmute_ref;

#[intrinsic_method("java/lang/ProcessEnvironment.environ()[[B", Any)]
#[async_method]
pub async fn environ<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let mut values = Vec::new();
    let vm = thread.vm()?;
    let collector = &vm.garbage_collector();
    for (key, value) in std::env::vars() {
        let key_bytes = mutf8::to_bytes(&key)?;
        let key_bytes: &[i8] = transmute_ref!(key_bytes.as_slice());
        let key = Value::new_object(collector, Reference::from(key_bytes.to_vec()));
        values.push(key);
        let value_bytes = mutf8::to_bytes(&value)?;
        let value_bytes: &[i8] = transmute_ref!(value_bytes.as_slice());
        let value = Value::new_object(collector, Reference::from(value_bytes.to_vec()));
        values.push(value);
    }
    let class = thread.class("[[B").await?;
    let reference = Reference::try_from((class, values))?;
    let value = Value::new_object(collector, reference);
    Ok(Some(value))
}

#[intrinsic_method(
    "java/lang/ProcessEnvironment.environmentBlock()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn environment_block<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let mut block = String::new();
    for (key, value) in std::env::vars() {
        block.push_str(&key);
        block.push('=');
        block.push_str(&value);
        block.push('\0');
    }
    block.push('\0');

    let string_class = thread.class("java/lang/String").await?;
    let mut object = Object::new(string_class)?;
    let vm = thread.vm()?;
    let collector = &vm.garbage_collector();
    let use_latin1 = block.chars().all(|c| (c as u32) <= 0xFF);
    let (coder, bytes): (i32, Vec<i8>) = if use_latin1 {
        (0, block.chars().map(|c| c as i8).collect())
    } else {
        let utf16_bytes: Vec<u8> = block.encode_utf16().flat_map(u16::to_ne_bytes).collect();
        let signed_bytes: &[i8] = transmute_ref!(utf16_bytes.as_slice());
        (1, signed_bytes.to_vec())
    };
    object.set_value("coder", Value::Int(coder))?;
    object.set_value(
        "value",
        Value::new_object(collector, Reference::from(bytes)),
    )?;
    object.set_value("hash", Value::Int(0))?;
    object.set_value("hashIsZero", Value::Int(0))?;
    let value = Value::from_object(collector, object);
    Ok(Some(value))
}
