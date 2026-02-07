use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/smartcardio/PlatformPCSC.initialize(Ljava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn initialize<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.smartcardio.PlatformPCSC.initialize(Ljava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.smartcardio.PlatformPCSC.initialize(Ljava/lang/String;)V"
    )]
    async fn test_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = initialize(thread, Parameters::default()).await;
    }
}
