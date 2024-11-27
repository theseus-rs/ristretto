use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.io.ObjectOutputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/ObjectOutputStream";
    registry.register(class_name, "doublesToBytes", "([DI[BII)V", doubles_to_bytes);
    registry.register(class_name, "floatsToBytes", "([FI[BII)V", floats_to_bytes);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn doubles_to_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn floats_to_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
