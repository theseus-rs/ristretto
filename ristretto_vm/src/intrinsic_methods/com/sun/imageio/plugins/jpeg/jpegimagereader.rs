use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.abortRead(J)V", Any)]
#[async_method]
pub(crate) async fn abort_read(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.abortRead(J)V")
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.clearNativeReadAbortFlag(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn clear_native_read_abort_flag(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.clearNativeReadAbortFlag(J)V")
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.disposeReader(J)V", Any)]
#[async_method]
pub(crate) async fn dispose_reader(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.disposeReader(J)V")
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.initJPEGImageReader()J",
    Any
)]
#[async_method]
pub(crate) async fn init_jpeg_image_reader(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.initJPEGImageReader()J")
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub(crate) async fn init_reader_i_ds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V"
    )
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z",
    Any
)]
#[async_method]
pub(crate) async fn read_image(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z"
    )
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.readImageHeader(JZZ)Z",
    Any
)]
#[async_method]
pub(crate) async fn read_image_header(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.readImageHeader(JZZ)Z")
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.resetLibraryState(J)V",
    Any
)]
#[async_method]
pub(crate) async fn reset_library_state(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.resetLibraryState(J)V")
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.resetReader(J)V", Any)]
#[async_method]
pub(crate) async fn reset_reader(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.resetReader(J)V")
}

#[intrinsic_method(
    "com/sun/imageio/plugins/jpeg/JPEGImageReader.setOutColorSpace(JI)V",
    Any
)]
#[async_method]
pub(crate) async fn set_out_color_space(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.setOutColorSpace(JI)V")
}

#[intrinsic_method("com/sun/imageio/plugins/jpeg/JPEGImageReader.setSource(J)V", Any)]
#[async_method]
pub(crate) async fn set_source(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.setSource(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.abortRead(J)V"
    )]
    async fn test_abort_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = abort_read(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.clearNativeReadAbortFlag(J)V"
    )]
    async fn test_clear_native_read_abort_flag() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_native_read_abort_flag(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.disposeReader(J)V"
    )]
    async fn test_dispose_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_reader(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.initJPEGImageReader()J"
    )]
    async fn test_init_jpeg_image_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_jpeg_image_reader(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V"
    )]
    async fn test_init_reader_i_ds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_reader_i_ds(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z"
    )]
    async fn test_read_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_image(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.readImageHeader(JZZ)Z"
    )]
    async fn test_read_image_header() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_image_header(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.resetLibraryState(J)V"
    )]
    async fn test_reset_library_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_library_state(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.resetReader(J)V"
    )]
    async fn test_reset_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_reader(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.setOutColorSpace(JI)V"
    )]
    async fn test_set_out_color_space() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_out_color_space(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.setSource(J)V"
    )]
    async fn test_set_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_source(thread, Parameters::default()).await;
    }
}
