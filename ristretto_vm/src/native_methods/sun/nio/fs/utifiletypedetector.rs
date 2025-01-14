use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.fs.UTIFileTypeDetector`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/fs/UTIFileTypeDetector";
    registry.register(
        class_name,
        "probe0",
        "(Ljava/lang/String;)Ljava/lang/String;",
        probe_0,
    );
}

#[async_recursion(?Send)]
async fn probe_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UTIFileTypeDetector.probe0(Ljava/lang/String;)Ljava/lang/String;");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/fs/UTIFileTypeDetector";
        assert!(registry
            .method(
                class_name,
                "probe0",
                "(Ljava/lang/String;)Ljava/lang/String;"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.fs.UTIFileTypeDetector.probe0(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_probe_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = probe_0(thread, Arguments::default()).await;
    }
}
