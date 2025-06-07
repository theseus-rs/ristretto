use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/fs/BsdFileSystem.directCopy0(IIJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn direct_copy_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.BsdFileSystem.directCopy0(IJJ)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.fs.BsdFileSystem.directCopy0(IJJ)I")]
    async fn test_direct_copy_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = direct_copy_0(thread, Parameters::default()).await;
    }
}
