use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/management/ClassLoadingImpl.setVerboseClass(Z)V", Any)]
#[async_method]
pub async fn set_verbose_class<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ClassLoadingImpl.setVerboseClass(Z)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_verbose_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_verbose_class(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
