use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CFRetainedResource.nativeCFRelease(JZ)V", Any)]
#[async_method]
pub async fn native_cf_release<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
