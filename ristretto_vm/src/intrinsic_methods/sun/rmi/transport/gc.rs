use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/rmi/transport/GC.maxObjectInspectionAge()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn max_object_inspection_age(
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
