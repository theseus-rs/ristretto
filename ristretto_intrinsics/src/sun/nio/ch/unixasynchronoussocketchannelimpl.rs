use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/UnixAsynchronousSocketChannelImpl.checkConnect(I)V", Any)]
#[async_method]
pub async fn check_connect<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.UnixAsynchronousSocketChannelImpl.checkConnect(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.UnixAsynchronousSocketChannelImpl.checkConnect(I)V"
    )]
    async fn test_check_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_connect(thread, Parameters::default()).await;
    }
}
