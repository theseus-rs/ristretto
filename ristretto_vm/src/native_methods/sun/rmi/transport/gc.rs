use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.rmi.transport.GC`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/rmi/transport/GC";
    registry.register(
        class_name,
        "maxObjectInspectionAge",
        "()J",
        max_object_inspection_age,
    );
}

#[async_recursion(?Send)]
async fn max_object_inspection_age(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.rmi.transport.GC.maxObjectInspectionAge()J")
}
