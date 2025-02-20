use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CFRetainedResource";

/// Register all native methods for `sun.lwawt.macosx.CFRetainedResource`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "nativeCFRelease", "(JZ)V", native_cf_release);
}

#[async_recursion(?Send)]
async fn native_cf_release(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
