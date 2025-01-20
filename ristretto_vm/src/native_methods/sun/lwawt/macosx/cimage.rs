use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CImage";

/// Register all native methods for `sun.lwawt.macosx.CImage`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeCopyNSImageIntoArray",
        "(J[IIIII)V",
        native_copy_ns_image_into_array,
    );
    registry.register(
        CLASS_NAME,
        "nativeCreateNSImageFromArray",
        "([III)J",
        native_create_ns_image_from_array,
    );
    registry.register(
        CLASS_NAME,
        "nativeCreateNSImageFromArrays",
        "([[I[I[I)J",
        native_create_ns_image_from_arrays,
    );
    registry.register(
        CLASS_NAME,
        "nativeCreateNSImageFromBytes",
        "([B)J",
        native_create_ns_image_from_bytes,
    );
    registry.register(
        CLASS_NAME,
        "nativeCreateNSImageFromFileContents",
        "(Ljava/lang/String;)J",
        native_create_ns_image_from_file_contents,
    );
    registry.register(
        CLASS_NAME,
        "nativeCreateNSImageFromIconSelector",
        "(I)J",
        native_create_ns_image_from_icon_selector,
    );
    registry.register(
        CLASS_NAME,
        "nativeCreateNSImageFromImageName",
        "(Ljava/lang/String;)J",
        native_create_ns_image_from_image_name,
    );
    registry.register(
        CLASS_NAME,
        "nativeCreateNSImageOfFileFromLaunchServices",
        "(Ljava/lang/String;)J",
        native_create_ns_image_of_file_from_launch_services,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetNSImageRepresentationSizes",
        "(JDD)[Ljava/awt/geom/Dimension2D;",
        native_get_ns_image_representation_sizes,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetNSImageSize",
        "(J)Ljava/awt/geom/Dimension2D;",
        native_get_ns_image_size,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetPlatformImageBytes",
        "([III)[B",
        native_get_platform_image_bytes,
    );
    registry.register(
        CLASS_NAME,
        "nativeResizeNSImageRepresentations",
        "(JDD)V",
        native_resize_ns_image_representations,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetNSImageSize",
        "(JDD)V",
        native_set_ns_image_size,
    );
}

#[async_recursion(?Send)]
async fn native_copy_ns_image_into_array(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCopyNSImageIntoArray(J[IIIII)V")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_array(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromArray([III)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_arrays(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromArrays([[I[I[I)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromBytes([B)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_file_contents(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromFileContents(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_icon_selector(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromIconSelector(I)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_image_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromImageName(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_of_file_from_launch_services(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageOfFileFromLaunchServices(Ljava/lang/String;)J"
    )
}

#[async_recursion(?Send)]
async fn native_get_ns_image_representation_sizes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeGetNSImageRepresentationSizes(JDD)[Ljava/awt/geom/Dimension2D;")
}

#[async_recursion(?Send)]
async fn native_get_ns_image_size(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeGetNSImageSize(J)Ljava/awt/geom/Dimension2D;")
}

#[async_recursion(?Send)]
async fn native_get_platform_image_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeGetPlatformImageBytes([III)[B")
}

#[async_recursion(?Send)]
async fn native_resize_ns_image_representations(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeResizeNSImageRepresentations(JDD)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_image_size(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeSetNSImageSize(JDD)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeCopyNSImageIntoArray(J[IIIII)V"
    )]
    async fn test_native_copy_ns_image_into_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_copy_ns_image_into_array(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeCreateNSImageFromArray([III)J"
    )]
    async fn test_native_create_ns_image_from_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_array(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeCreateNSImageFromArrays([[I[I[I)J"
    )]
    async fn test_native_create_ns_image_from_arrays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_arrays(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeCreateNSImageFromBytes([B)J"
    )]
    async fn test_native_create_ns_image_from_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeCreateNSImageFromFileContents(Ljava/lang/String;)J"
    )]
    async fn test_native_create_ns_image_from_file_contents() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_file_contents(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeCreateNSImageFromIconSelector(I)J"
    )]
    async fn test_native_create_ns_image_from_icon_selector() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_icon_selector(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeCreateNSImageFromImageName(Ljava/lang/String;)J"
    )]
    async fn test_native_create_ns_image_from_image_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_image_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeCreateNSImageOfFileFromLaunchServices(Ljava/lang/String;)J"
    )]
    async fn test_native_create_ns_image_of_file_from_launch_services() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_of_file_from_launch_services(thread, Parameters::default())
            .await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeGetNSImageRepresentationSizes(JDD)[Ljava/awt/geom/Dimension2D;"
    )]
    async fn test_native_get_ns_image_representation_sizes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_image_representation_sizes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeGetNSImageSize(J)Ljava/awt/geom/Dimension2D;"
    )]
    async fn test_native_get_ns_image_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_image_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeGetPlatformImageBytes([III)[B"
    )]
    async fn test_native_get_platform_image_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_platform_image_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeResizeNSImageRepresentations(JDD)V"
    )]
    async fn test_native_resize_ns_image_representations() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_resize_ns_image_representations(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CImage.nativeSetNSImageSize(JDD)V"
    )]
    async fn test_native_set_ns_image_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_image_size(thread, Parameters::default()).await;
    }
}
