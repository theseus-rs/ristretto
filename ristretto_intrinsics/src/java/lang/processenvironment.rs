#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_classfile::VersionSpecification::Any;
#[cfg(any(target_family = "unix", target_os = "wasi"))]
use ristretto_classfile::mutf8;
#[cfg(any(target_family = "unix", target_os = "wasi"))]
use ristretto_classloader::Reference;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_classloader::Value;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_macros::intrinsic_method;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_types::Thread;
#[cfg(any(target_family = "unix", target_os = "wasi"))]
use ristretto_types::VM;
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use ristretto_types::{Parameters, Result};
#[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
use std::sync::Arc;
#[cfg(any(target_family = "unix", target_os = "wasi"))]
use zerocopy::transmute_ref;

#[cfg(any(target_family = "unix", target_os = "wasi"))]
#[intrinsic_method("java/lang/ProcessEnvironment.environ()[[B", Any)]
pub async fn environ<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let mut values = Vec::new();
    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
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
