use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.ImageRepresentation`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/ImageRepresentation";
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "setDiffICM",
        "(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z",
        set_diff_icm,
    );
    registry.register(
        class_name,
        "setICMpixels",
        "(IIII[I[BIILsun/awt/image/IntegerComponentRaster;)Z",
        set_icm_pixels,
    );
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_diff_icm(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImageRepresentation.setDiffICM(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z")
}

#[async_recursion(?Send)]
async fn set_icm_pixels(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImageRepresentation.setICMpixels(IIII[I[BIILsun/awt/image/IntegerComponentRaster;)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/image/ImageRepresentation";
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "setDiffICM",
                "(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "setICMpixels",
                "(IIII[I[BIILsun/awt/image/IntegerComponentRaster;)Z"
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
        expected = "sun.awt.image.ImageRepresentation.setDiffICM(IIII[IIILjava/awt/image/IndexColorModel;[BIILsun/awt/image/ByteComponentRaster;I)Z"
    )]
    async fn test_set_diff_icm() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_diff_icm(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.image.ImageRepresentation.setICMpixels(IIII[I[BIILsun/awt/image/IntegerComponentRaster;)Z"
    )]
    async fn test_set_icm_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_icm_pixels(thread, Arguments::default()).await;
    }
}
