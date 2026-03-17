use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/rmi/transport/GC.maxObjectInspectionAge()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn max_object_inspection_age<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.rmi.transport.GC.maxObjectInspectionAge()J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_max_object_inspection_age() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = max_object_inspection_age(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
