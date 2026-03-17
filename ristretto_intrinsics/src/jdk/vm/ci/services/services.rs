use ristretto_classfile::JAVA_25;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/vm/ci/services/Services.readSystemPropertiesInfo([I)J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn read_system_properties_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.vm.ci.services.Services.readSystemPropertiesInfo([I)J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_system_properties_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_system_properties_info(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
