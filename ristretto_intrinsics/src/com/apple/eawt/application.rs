use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/eawt/Application.nativeInitializeApplicationDelegate()V",
    Any
)]
#[async_method]
pub async fn native_initialize_application_delegate<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_initialize_application_delegate() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_initialize_application_delegate(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
