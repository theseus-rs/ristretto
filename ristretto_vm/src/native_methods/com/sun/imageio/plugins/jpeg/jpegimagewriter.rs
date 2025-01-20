use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/imageio/plugins/jpeg/JPEGImageWriter";

/// Register all native methods for `com.sun.imageio.plugins.jpeg.JPEGImageWriter`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "abortWrite", "(J)V", abort_write);
    registry.register(CLASS_NAME, "disposeWriter", "(J)V", dispose_writer);
    registry.register(
        CLASS_NAME,
        "initJPEGImageWriter",
        "()J",
        init_jpeg_image_writer,
    );
    registry.register(
        CLASS_NAME,
        "initWriterIDs",
        "(Ljava/lang/Class;Ljava/lang/Class;)V",
        init_writer_ids,
    );
    registry.register(CLASS_NAME, "resetWriter", "(J)V", reset_writer);
    registry.register(CLASS_NAME, "setDest", "(J)V", set_dest);
    registry.register(CLASS_NAME, "writeImage", "(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z", write_image);
    registry.register(CLASS_NAME, "writeTables", "(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V", write_tables);
}

#[async_recursion(?Send)]
async fn abort_write(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.abortWrite(J)V")
}

#[async_recursion(?Send)]
async fn dispose_writer(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.disposeWriter(J)V")
}

#[async_recursion(?Send)]
async fn init_jpeg_image_writer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.initJPEGImageWriter()J")
}

#[async_recursion(?Send)]
async fn init_writer_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V")
}

#[async_recursion(?Send)]
async fn reset_writer(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.resetWriter(J)V")
}

#[async_recursion(?Send)]
async fn set_dest(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.setDest(J)V")
}

#[async_recursion(?Send)]
async fn write_image(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z")
}

#[async_recursion(?Send)]
async fn write_tables(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.abortWrite(J)V"
    )]
    async fn test_abort_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = abort_write(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.disposeWriter(J)V"
    )]
    async fn test_dispose_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_writer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.initJPEGImageWriter()J"
    )]
    async fn test_init_jpeg_image_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_jpeg_image_writer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V"
    )]
    async fn test_init_writer_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_writer_ids(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.resetWriter(J)V"
    )]
    async fn test_reset_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_writer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.setDest(J)V"
    )]
    async fn test_set_dest() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_dest(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z"
    )]
    async fn test_write_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V"
    )]
    async fn test_write_tables() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write_tables(thread, Parameters::default()).await;
    }
}
