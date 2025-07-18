use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/eawt/Application.nativeInitializeApplicationDelegate()V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_initialize_application_delegate(
    _thread: Arc<Thread>,
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
