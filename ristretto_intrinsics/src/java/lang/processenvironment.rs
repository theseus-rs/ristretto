#[cfg(not(target_family = "wasm"))]
use ristretto_classfile::VersionSpecification::Any;
#[cfg(target_family = "unix")]
use ristretto_classfile::mutf8;
#[cfg(target_family = "unix")]
use ristretto_classloader::Reference;
#[cfg(not(target_family = "wasm"))]
use ristretto_classloader::Value;
#[cfg(not(target_family = "wasm"))]
use ristretto_macros::async_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_macros::intrinsic_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::Thread;
#[cfg(target_family = "unix")]
use ristretto_types::VM;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::{Parameters, Result};
#[cfg(not(target_family = "wasm"))]
use std::sync::Arc;
#[cfg(target_family = "unix")]
use zerocopy::transmute_ref;

#[cfg(target_family = "unix")]
#[intrinsic_method("java/lang/ProcessEnvironment.environ()[[B", Any)]
#[async_method]
pub async fn environ<T: Thread + 'static>(
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

/// Returns the environment block as a single string with null-separated KEY=VALUE pairs,
/// terminated by double null. This is the Windows-specific native method.
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "java/lang/ProcessEnvironment.environmentBlock()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn environment_block<T: Thread + 'static>(
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
    let string_value = thread.intern_string(&block).await?;
    Ok(Some(string_value))
}
