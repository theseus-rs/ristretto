use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/image/GifImageDecoder";

/// Register all native methods for `sun.awt.image.GifImageDecoder`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
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

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.GifImageDecoder.parseImage(IIIIZI[B[BLjava/awt/image/IndexColorModel;)Z"
    )]
    async fn test_parse_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = parse_image(thread, Arguments::default()).await;
    }
}
