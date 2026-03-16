use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.abortRead(J)V", Any)]
#[async_method]
pub async fn abort_read<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.abortRead(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.clearNativeReadAbortFlag(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn clear_native_read_abort_flag<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.clearNativeReadAbortFlag(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.disposeReader(J)V", Any)]
#[async_method]
pub async fn dispose_reader<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.disposeReader(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.initJPEGImageReader()J",
    Any
)]
#[async_method]
pub async fn init_jpeg_image_reader<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.initJPEGImageReader()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_reader_i_ds<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V".to_string()).into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z",
    Any
)]
#[async_method]
pub async fn read_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("com.sun.imageio.plugins.jpeg.JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z".to_string()).into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.readImageHeader(JZZ)Z",
    Any
)]
#[async_method]
pub async fn read_image_header<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.readImageHeader(JZZ)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.resetLibraryState(J)V",
    Any
)]
#[async_method]
pub async fn reset_library_state<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.resetLibraryState(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.resetReader(J)V", Any)]
#[async_method]
pub async fn reset_reader<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.resetReader(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.setOutColorSpace(JI)V",
    Any
)]
#[async_method]
pub async fn set_out_color_space<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.setOutColorSpace(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.setSource(J)V", Any)]
#[async_method]
pub async fn set_source<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.setSource(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_abort_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = abort_read(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_clear_native_read_abort_flag() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = clear_native_read_abort_flag(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dispose_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose_reader(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_jpeg_image_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_jpeg_image_reader(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_reader_i_ds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_reader_i_ds(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_read_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_read_image_header() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_image_header(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_library_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_library_state(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_reader(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_out_color_space() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_out_color_space(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_source(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
