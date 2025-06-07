use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.abortWrite(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn abort_write(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.abortWrite(J)V")
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.disposeWriter(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn dispose_writer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.disposeWriter(J)V")
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.initJPEGImageWriter()J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn init_jpeg_image_writer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.initJPEGImageWriter()J")
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn init_writer_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V"
    )
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.resetWriter(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn reset_writer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.resetWriter(J)V")
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.setDest(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_dest(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageWriter.setDest(J)V")
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn write_image(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z"
    )
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn write_tables(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V"
    )
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
