use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/fs/UnixFileSystem.bufferedCopy0(IIJIJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn buffered_copy_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixFileSystem.bufferedCopy0(IIJIJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UnixFileSystem.bufferedCopy0(IIJIJ)V"
    )]
    async fn test_buffered_copy_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = buffered_copy_0(thread, Parameters::default()).await;
    }
}
