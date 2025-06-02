use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/FileKey";

/// Register all intrinsic methods for `sun.nio.ch.FileKey`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_21 {
        registry.register(CLASS_NAME, "init", "(Ljava/io/FileDescriptor;)V", init);
        registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    } else {
        registry.register(CLASS_NAME, "init", "(Ljava/io/FileDescriptor;[J)V", init);
    }
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileKey.init(Ljava/io/FileDescriptor;)V"
    )]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
