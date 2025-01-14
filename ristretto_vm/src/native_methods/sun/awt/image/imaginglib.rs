use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.image.ImagingLib`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/image/ImagingLib";
    registry.register(
        class_name,
        "convolveBI",
        "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I",
        convolve_bi,
    );
    registry.register(
        class_name,
        "convolveRaster",
        "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I",
        convolve_raster,
    );
    registry.register(class_name, "init", "()Z", init);
    registry.register(
        class_name,
        "lookupByteBI",
        "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I",
        lookup_byte_bi,
    );
    registry.register(
        class_name,
        "lookupByteRaster",
        "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I",
        lookup_byte_raster,
    );
    registry.register(
        class_name,
        "transformBI",
        "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I",
        transform_bi,
    );
    registry.register(
        class_name,
        "transformRaster",
        "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I",
        transform_raster,
    );
}

#[async_recursion(?Send)]
async fn convolve_bi(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImagingLib.convolveBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I")
}

#[async_recursion(?Send)]
async fn convolve_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImagingLib.convolveRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImagingLib.init()Z")
}

#[async_recursion(?Send)]
async fn lookup_byte_bi(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImagingLib.lookupByteBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I")
}

#[async_recursion(?Send)]
async fn lookup_byte_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImagingLib.lookupByteRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I")
}

#[async_recursion(?Send)]
async fn transform_bi(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImagingLib.transformBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I")
}

#[async_recursion(?Send)]
async fn transform_raster(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.image.ImagingLib.transformRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/image/ImagingLib";
        assert!(registry
            .method(
                class_name,
                "convolveBI",
                "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "convolveRaster",
                "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I"
            )
            .is_some());
        assert!(registry.method(class_name, "init", "()Z").is_some());
        assert!(registry
            .method(
                class_name,
                "lookupByteBI",
                "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "lookupByteRaster",
                "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "transformBI",
                "(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "transformRaster",
                "(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.image.ImagingLib.convolveBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;Ljava/awt/image/Kernel;I)I"
    )]
    async fn test_convolve_bi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = convolve_bi(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.image.ImagingLib.convolveRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;Ljava/awt/image/Kernel;I)I"
    )]
    async fn test_convolve_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = convolve_raster(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.image.ImagingLib.init()Z")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.image.ImagingLib.lookupByteBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[[B)I"
    )]
    async fn test_lookup_byte_bi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lookup_byte_bi(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.image.ImagingLib.lookupByteRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[[B)I"
    )]
    async fn test_lookup_byte_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lookup_byte_raster(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.image.ImagingLib.transformBI(Ljava/awt/image/BufferedImage;Ljava/awt/image/BufferedImage;[DI)I"
    )]
    async fn test_transform_bi() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transform_bi(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.image.ImagingLib.transformRaster(Ljava/awt/image/Raster;Ljava/awt/image/Raster;[DI)I"
    )]
    async fn test_transform_raster() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transform_raster(thread, Arguments::default()).await;
    }
}
