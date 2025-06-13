use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CFRetainedResource.nativeCFRelease(JZ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_cf_release(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CFRetainedResource.nativeCFRelease(JZ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CFRetainedResource.nativeCFRelease(JZ)V"
    )]
    async fn test_native_cf_release() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_cf_release(thread, Parameters::default()).await;
    }
}
