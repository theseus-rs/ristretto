use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/util/logging/FileHandler.isSetUID()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn is_set_uid<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.logging.FileHandler.isSetUID()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.util.logging.FileHandler.isSetUID()Z")]
    async fn test_is_set_uid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_set_uid(thread, Parameters::default()).await;
    }
}
