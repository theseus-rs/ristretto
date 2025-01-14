use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.JPEGImageDecoder`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/JPEGImageDecoder";
    registry.register(class_name, "initIDs", "(Ljava/lang/Class;)V", init_ids);
    registry.register(
        class_name,
        "readImage",
        "(Ljava/io/InputStream;[B)V",
        read_image,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn read_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.JPEGImageDecoder.readImage(Ljava/io/InputStream;[B)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/image/JPEGImageDecoder";
        assert!(registry
            .method(class_name, "initIDs", "(Ljava/lang/Class;)V")
            .is_some());
        assert!(registry
            .method(class_name, "readImage", "(Ljava/io/InputStream;[B)V")
            .is_some());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.image.JPEGImageDecoder.readImage(Ljava/io/InputStream;[B)V")]
    async fn test_read_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_image(thread, Arguments::default()).await;
    }
}
