use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/fs/MacOSXNativeDispatcher";

/// Register all native methods for `sun.nio.fs.MacOSXNativeDispatcher`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "normalizepath", "([CI)[C", normalizepath);
}

#[async_recursion(?Send)]
async fn normalizepath(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
