use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/rmi/transport/GC";

/// Register all native methods for `sun.rmi.transport.GC`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "maxObjectInspectionAge",
        "()J",
        max_object_inspection_age,
    );
}

#[async_recursion(?Send)]
async fn max_object_inspection_age(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.rmi.transport.GC.maxObjectInspectionAge()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.rmi.transport.GC.maxObjectInspectionAge()J"
    )]
    async fn test_max_object_inspection_age() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = max_object_inspection_age(thread, Parameters::default()).await;
    }
}
