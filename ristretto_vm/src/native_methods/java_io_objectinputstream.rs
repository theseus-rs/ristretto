use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.io.ObjectInputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/ObjectInputStream";
    registry.register(class_name, "bytesToDoubles", "([BI[DII)V", bytes_to_doubles);
    registry.register(class_name, "bytesToFloats", "([BI[FII)V", bytes_to_floats);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn bytes_to_doubles(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn bytes_to_floats(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
