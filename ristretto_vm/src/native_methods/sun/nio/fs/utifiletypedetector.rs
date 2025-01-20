use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/fs/UTIFileTypeDetector";

/// Register all native methods for `sun.nio.fs.UTIFileTypeDetector`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "probe0",
        "(Ljava/lang/String;)Ljava/lang/String;",
        probe_0,
    );
}

#[async_recursion(?Send)]
async fn probe_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UTIFileTypeDetector.probe0(Ljava/lang/String;)Ljava/lang/String;");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.fs.UTIFileTypeDetector.probe0(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_probe_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = probe_0(thread, Parameters::default()).await;
    }
}
