use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CImage`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CImage";
    registry.register(
        class_name,
        "nativeCopyNSImageIntoArray",
        "(J[IIIII)V",
        native_copy_ns_image_into_array,
    );
    registry.register(
        class_name,
        "nativeCreateNSImageFromArray",
        "([III)J",
        native_create_ns_image_from_array,
    );
    registry.register(
        class_name,
        "nativeCreateNSImageFromArrays",
        "([[I[I[I)J",
        native_create_ns_image_from_arrays,
    );
    registry.register(
        class_name,
        "nativeCreateNSImageFromBytes",
        "([B)J",
        native_create_ns_image_from_bytes,
    );
    registry.register(
        class_name,
        "nativeCreateNSImageFromFileContents",
        "(Ljava/lang/String;)J",
        native_create_ns_image_from_file_contents,
    );
    registry.register(
        class_name,
        "nativeCreateNSImageFromIconSelector",
        "(I)J",
        native_create_ns_image_from_icon_selector,
    );
    registry.register(
        class_name,
        "nativeCreateNSImageFromImageName",
        "(Ljava/lang/String;)J",
        native_create_ns_image_from_image_name,
    );
    registry.register(
        class_name,
        "nativeCreateNSImageOfFileFromLaunchServices",
        "(Ljava/lang/String;)J",
        native_create_ns_image_of_file_from_launch_services,
    );
    registry.register(
        class_name,
        "nativeGetNSImageRepresentationSizes",
        "(JDD)[Ljava/awt/geom/Dimension2D;",
        native_get_ns_image_representation_sizes,
    );
    registry.register(
        class_name,
        "nativeGetNSImageSize",
        "(J)Ljava/awt/geom/Dimension2D;",
        native_get_ns_image_size,
    );
    registry.register(
        class_name,
        "nativeGetPlatformImageBytes",
        "([III)[B",
        native_get_platform_image_bytes,
    );
    registry.register(
        class_name,
        "nativeResizeNSImageRepresentations",
        "(JDD)V",
        native_resize_ns_image_representations,
    );
    registry.register(
        class_name,
        "nativeSetNSImageSize",
        "(JDD)V",
        native_set_ns_image_size,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_copy_ns_image_into_array(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_ns_image_from_array(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_ns_image_from_arrays(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_ns_image_from_bytes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_ns_image_from_file_contents(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_ns_image_from_icon_selector(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_ns_image_from_image_name(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_create_ns_image_of_file_from_launch_services(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_ns_image_representation_sizes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_ns_image_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_get_platform_image_bytes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_resize_ns_image_representations(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_set_ns_image_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
