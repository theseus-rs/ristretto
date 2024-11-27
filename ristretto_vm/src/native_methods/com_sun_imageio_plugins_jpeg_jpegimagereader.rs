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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn abort_read(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn clear_native_read_abort_flag(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn dispose_reader(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_jpeg_image_reader(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_reader_i_ds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn read_image(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn read_image_header(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn reset_library_state(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn reset_reader(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_out_color_space(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_source(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
