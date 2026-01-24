use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/nio/fs/UnixCopyFile.transfer(IIJ)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub(crate) async fn transfer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixCopyFile.transfer(IIJ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.UnixCopyFile.transfer(IIJ)V")]
    async fn test_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transfer(thread, Parameters::default()).await;
    }
}
