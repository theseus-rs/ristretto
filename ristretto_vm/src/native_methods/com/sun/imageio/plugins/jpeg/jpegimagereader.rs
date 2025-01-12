use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `com.sun.imageio.plugins.jpeg.JPEGImageReader`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/imageio/plugins/jpeg/JPEGImageReader";
    let java_version = registry.java_version();

    if java_version >= &JAVA_11 {
        registry.register(
            class_name,
            "clearNativeReadAbortFlag",
            "(J)V",
            clear_native_read_abort_flag,
        );
    }

    registry.register(class_name, "abortRead", "(J)V", abort_read);
    registry.register(class_name, "disposeReader", "(J)V", dispose_reader);
    registry.register(
        class_name,
        "initJPEGImageReader",
        "()J",
        init_jpeg_image_reader,
    );
    registry.register(
        class_name,
        "initReaderIDs",
        "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
        init_reader_i_ds,
    );
    registry.register(class_name, "readImage", "(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z", read_image);
    registry.register(class_name, "readImageHeader", "(JZZ)Z", read_image_header);
    registry.register(class_name, "resetLibraryState", "(J)V", reset_library_state);
    registry.register(class_name, "resetReader", "(J)V", reset_reader);
    registry.register(class_name, "setOutColorSpace", "(JI)V", set_out_color_space);
    registry.register(class_name, "setSource", "(J)V", set_source);
}

#[async_recursion(?Send)]
async fn abort_read(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.abortRead(J)V")
}

#[async_recursion(?Send)]
async fn clear_native_read_abort_flag(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.clearNativeReadAbortFlag(J)V")
}

#[async_recursion(?Send)]
async fn dispose_reader(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.disposeReader(J)V")
}

#[async_recursion(?Send)]
async fn init_jpeg_image_reader(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.initJPEGImageReader()J")
}

#[async_recursion(?Send)]
async fn init_reader_i_ds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V")
}

#[async_recursion(?Send)]
async fn read_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z")
}

#[async_recursion(?Send)]
async fn read_image_header(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.readImageHeader(JZZ)Z")
}

#[async_recursion(?Send)]
async fn reset_library_state(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.resetLibraryState(J)V")
}

#[async_recursion(?Send)]
async fn reset_reader(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.resetReader(J)V")
}

#[async_recursion(?Send)]
async fn set_out_color_space(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.setOutColorSpace(JI)V")
}

#[async_recursion(?Send)]
async fn set_source(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.imageio.plugins.jpeg.JPEGImageReader.setSource(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java11 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "com/sun/imageio/plugins/jpeg/JPEGImageReader";
        assert!(registry.method(class_name, "abortRead", "(J)V").is_some());
        assert!(registry
            .method(class_name, "clearNativeReadAbortFlag", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "disposeReader", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "initJPEGImageReader", "()J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "initReaderIDs",
                "(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "readImage",
                "(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z"
            )
            .is_some());
        assert!(registry
            .method(class_name, "readImageHeader", "(JZZ)Z")
            .is_some());
        assert!(registry
            .method(class_name, "resetLibraryState", "(J)V")
            .is_some());
        assert!(registry.method(class_name, "resetReader", "(J)V").is_some());
        assert!(registry
            .method(class_name, "setOutColorSpace", "(JI)V")
            .is_some());
        assert!(registry.method(class_name, "setSource", "(J)V").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.abortRead(J)V"
    )]
    async fn test_abort_read() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = abort_read(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.clearNativeReadAbortFlag(J)V"
    )]
    async fn test_clear_native_read_abort_flag() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_native_read_abort_flag(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.disposeReader(J)V"
    )]
    async fn test_dispose_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose_reader(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.initJPEGImageReader()J"
    )]
    async fn test_init_jpeg_image_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_jpeg_image_reader(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.initReaderIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V"
    )]
    async fn test_init_reader_i_ds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_reader_i_ds(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.readImage(IJ[BI[I[IIIIIII[Ljavax/imageio/plugins/jpeg/JPEGQTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;[Ljavax/imageio/plugins/jpeg/JPEGHuffmanTable;IIZ)Z"
    )]
    async fn test_read_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_image(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.readImageHeader(JZZ)Z"
    )]
    async fn test_read_image_header() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = read_image_header(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.resetLibraryState(J)V"
    )]
    async fn test_reset_library_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_library_state(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.resetReader(J)V"
    )]
    async fn test_reset_reader() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_reader(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.setOutColorSpace(JI)V"
    )]
    async fn test_set_out_color_space() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_out_color_space(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.imageio.plugins.jpeg.JPEGImageReader.setSource(J)V"
    )]
    async fn test_set_source() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_source(thread, Arguments::default()).await;
    }
}
