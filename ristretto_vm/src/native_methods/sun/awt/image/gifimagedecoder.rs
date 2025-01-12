use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.GifImageDecoder`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/GifImageDecoder";
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "parseImage",
        "(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z",
        parse_image,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn parse_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.GifImageDecoder.parseImage(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/image/GifImageDecoder";
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "parseImage",
                "(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z"
            )
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
    #[should_panic(
        expected = "sun.awt.image.GifImageDecoder.parseImage(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z"
    )]
    async fn test_parse_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = parse_image(thread, Arguments::default()).await;
    }
}
