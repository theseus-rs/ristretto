use crate::Result;
use crate::native_methods::registry::{JAVA_11, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/imageio/plugins/jpeg/JPEGImageReader";

/// Register all native methods for `com.sun.imageio.plugins.jpeg.JPEGImageReader`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "clearNativeReadAbortFlag",
            "(J)V",
            clear_native_read_abort_flag,
        );
    }

    registry.register(CLASS_NAME, "abortRead", "(J)V", abort_read);
    registry.register(CLASS_NAME, "disposeReader", "(J)V", dispose_reader);
    registry.register(
        CLASS_NAME,
        "initJPEGImageReader",
        "()J",
        init_jpeg_image_reader,
    );
    registry.register(
        CLASS_NAME,
        "initReaderIDs",
        "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
        init_reader_i_ds,
    );
    registry.register(CLASS_NAME, "readImage", "(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z", read_image);
    registry.register(CLASS_NAME, "readImageHeader", "(JZZ)Z", read_image_header);
    registry.register(CLASS_NAME, "resetLibraryState", "(J)V", reset_library_state);
    registry.register(CLASS_NAME, "resetReader", "(J)V", reset_reader);
    registry.register(CLASS_NAME, "setOutColorSpace", "(JI)V", set_out_color_space);
    registry.register(CLASS_NAME, "setSource", "(J)V", set_source);
}

#[async_recursion(?Send)]
async fn abort_read(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.abortRead(J)V")
}

#[async_recursion(?Send)]
async fn clear_native_read_abort_flag(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.clearNativeReadAbortFlag(J)V")
}

#[async_recursion(?Send)]
async fn dispose_reader(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.disposeReader(J)V")
}

#[async_recursion(?Send)]
async fn init_jpeg_image_reader(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.initJPEGImageReader()J")
}

#[async_recursion(?Send)]
async fn init_reader_i_ds(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V"
    )
}

#[async_recursion(?Send)]
async fn read_image(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "com.sun.imageio.plugins.jpeg.JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z"
    )
}

#[async_recursion(?Send)]
async fn read_image_header(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.readImageHeader(JZZ)Z")
}

#[async_recursion(?Send)]
async fn reset_library_state(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.resetLibraryState(J)V")
}

#[async_recursion(?Send)]
async fn reset_reader(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.resetReader(J)V")
}

#[async_recursion(?Send)]
async fn set_out_color_space(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.setOutColorSpace(JI)V")
}

#[async_recursion(?Send)]
async fn set_source(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
