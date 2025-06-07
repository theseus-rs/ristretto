use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/nio/fs/MacOSXNativeDispatcher.normalizepath([CI)[C", Any)]
#[async_recursion(?Send)]
pub(crate) async fn normalizepath(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.fs.MacOSXNativeDispatcher.normalizepath([CI)[C");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.MacOSXNativeDispatcher.normalizepath([CI)[C"
    )]
    async fn test_normalizepath() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = normalizepath(thread, Parameters::default()).await;
    }
}
