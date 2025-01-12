use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.imageio.plugins.jpeg.JPEGImageWriter`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/imageio/plugins/jpeg/JPEGImageWriter";
    registry.register(class_name, "abortWrite", "(J)V", abort_write);
    registry.register(class_name, "disposeWriter", "(J)V", dispose_writer);
    registry.register(
        class_name,
        "initJPEGImageWriter",
        "()J",
        init_jpeg_image_writer,
    );
    registry.register(
        class_name,
        "initWriterIDs",
        "(Ljava/lang/Class;Ljava/lang/Class;)V",
        init_writer_ids,
    );
    registry.register(class_name, "resetWriter", "(J)V", reset_writer);
    registry.register(class_name, "setDest", "(J)V", set_dest);
    registry.register(class_name, "writeImage", "(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z", write_image);
    registry.register(class_name, "writeTables", "(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V", write_tables);
}

#[async_recursion(?Send)]
async fn abort_write(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.abortWrite(J)V")
}

#[async_recursion(?Send)]
async fn dispose_writer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.disposeWriter(J)V")
}

#[async_recursion(?Send)]
async fn init_jpeg_image_writer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.initJPEGImageWriter()J")
}

#[async_recursion(?Send)]
async fn init_writer_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V")
}

#[async_recursion(?Send)]
async fn reset_writer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.resetWriter(J)V")
}

#[async_recursion(?Send)]
async fn set_dest(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.setDest(J)V")
}

#[async_recursion(?Send)]
async fn write_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z")
}

#[async_recursion(?Send)]
async fn write_tables(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "com/sun/imageio/plugins/jpeg/JPEGImageWriter";
        assert!(registry.method(class_name, "abortWrite", "(J)V").is_some());
        assert!(registry
            .method(class_name, "disposeWriter", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "initJPEGImageWriter", "()J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "initWriterIDs",
                "(Ljava/lang/Class;Ljava/lang/Class;)V"
            )
            .is_some());
        assert!(registry.method(class_name, "resetWriter", "(J)V").is_some());
        assert!(registry.method(class_name, "setDest", "(J)V").is_some());
        assert!(registry
            .method(
                class_name,
                "writeImage",
                "(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "writeTables",
                "(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.abortWrite(J)V"
    )]
    async fn test_abort_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = abort_write(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.disposeWriter(J)V"
    )]
    async fn test_dispose_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_writer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.initJPEGImageWriter()J"
    )]
    async fn test_init_jpeg_image_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_jpeg_image_writer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V"
    )]
    async fn test_init_writer_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_writer_ids(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.resetWriter(J)V"
    )]
    async fn test_reset_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_writer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.setDest(J)V"
    )]
    async fn test_set_dest() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_dest(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z"
    )]
    async fn test_write_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_image(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V"
    )]
    async fn test_write_tables() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_tables(thread, Arguments::default()).await;
    }
}
