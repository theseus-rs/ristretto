use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/image/JPEGImageDecoder";

/// Register all intrinsic methods for `sun.awt.image.JPEGImageDecoder`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "initIDs", "(Ljava/lang/Class;)V", init_ids);
    registry.register(
        CLASS_NAME,
        "readImage",
        "(Ljava/io/InputStream;[B)V",
        read_image,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn read_image(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.image.JPEGImageDecoder.readImage(Ljava/io/InputStream;[B)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.image.JPEGImageDecoder.readImage(Ljava/io/InputStream;[B)V"
    )]
    async fn test_read_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_image(thread, Parameters::default()).await;
    }
}
