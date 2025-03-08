use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::mutf8;
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/ProcessEnvironment";

/// Register all native methods for `java.lang.ProcessEnvironment`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "environ", "()[[B", environ);
}

#[async_recursion(?Send)]
async fn environ(thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let mut values = Vec::new();
    for (key, value) in std::env::vars() {
        let key = Some(Reference::from(mutf8::to_bytes(key)?));
        values.push(key);
        let value = Some(Reference::from(mutf8::to_bytes(value)?));
        values.push(value);
    }
    let vm = thread.vm()?;
    let class = vm.class("[[B").await?;
    let result = Reference::from((class, values));
    Ok(Some(Value::Object(Some(result))))
}
