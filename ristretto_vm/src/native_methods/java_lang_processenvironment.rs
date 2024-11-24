use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::mutf8;
use ristretto_classloader::{ConcurrentVec, Reference, Value};
use std::sync::Arc;

/// Register all native methods for `java.lang.ProcessEnvironment`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ProcessEnvironment";
    registry.register(class_name, "environ", "()[[B", environ);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn environ(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let mut values = Vec::new();
    for (key, value) in std::env::vars() {
        let key = Some(Reference::from(mutf8::to_bytes(key)?));
        values.push(key);
        let value = Some(Reference::from(mutf8::to_bytes(value)?));
        values.push(value);
    }
    let vm = thread.vm()?;
    let class = vm.class("[[B").await?;
    let result = Reference::Array(class, ConcurrentVec::from(values));
    Ok(Some(Value::Object(Some(result))))
}
