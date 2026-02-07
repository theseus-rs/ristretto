use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::mutf8;
use ristretto_classloader::Reference;
use ristretto_classloader::Value;
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
