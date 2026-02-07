use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/management/ClassLoadingImpl.setVerboseClass(Z)V", Any)]
#[async_method]
pub async fn set_verbose_class<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ClassLoadingImpl.setVerboseClass(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ClassLoadingImpl.setVerboseClass(Z)V"
    )]
    async fn test_set_verbose_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_verbose_class(thread, Parameters::default()).await;
    }
}
