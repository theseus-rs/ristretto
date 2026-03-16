use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.abortWrite(J)V", Any)]
#[async_method]
pub async fn abort_write<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.abortWrite(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.disposeWriter(J)V", Any)]
#[async_method]
pub async fn dispose_writer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.disposeWriter(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.initJPEGImageWriter()J",
    Any
)]
#[async_method]
pub async fn init_jpeg_image_writer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.initJPEGImageWriter()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_writer_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageWriter.initWriterIDs(Ljava/lang/Class;Ljava/lang/Class;)V".to_string()).into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.resetWriter(J)V", Any)]
#[async_method]
pub async fn reset_writer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.resetWriter(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageWriter.setDest(J)V", Any)]
#[async_method]
pub async fn set_dest<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageWriter.setDest(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z",
    Any
)]
#[async_method]
pub async fn write_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeImage(J[BIII[IIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;Z[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;ZZZI[I[I[I[I[IZI)Z".to_string()).into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V",
    Any
)]
#[async_method]
pub async fn write_tables<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageWriter.writeTables(J[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_abort_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = abort_write(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dispose_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_writer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_jpeg_image_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_jpeg_image_writer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_writer_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_writer_ids(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_writer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_writer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_dest() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_dest(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_write_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_write_tables() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_tables(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
