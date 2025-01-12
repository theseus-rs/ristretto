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

#[async_recursion(?Send)]
async fn native_copy_ns_image_into_array(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCopyNSImageIntoArray(J[IIIII)V")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_array(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromArray([III)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_arrays(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromArrays([[I[I[I)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_bytes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromBytes([B)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_file_contents(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromFileContents(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_icon_selector(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromIconSelector(I)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_from_image_name(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromImageName(Ljava/lang/String;)J")
}

#[async_recursion(?Send)]
async fn native_create_ns_image_of_file_from_launch_services(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageOfFileFromLaunchServices(Ljava/lang/String;)J"
    )
}

#[async_recursion(?Send)]
async fn native_get_ns_image_representation_sizes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeGetNSImageRepresentationSizes(JDD)[Ljava/awt/geom/Dimension2D;")
}

#[async_recursion(?Send)]
async fn native_get_ns_image_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeGetNSImageSize(J)Ljava/awt/geom/Dimension2D;")
}

#[async_recursion(?Send)]
async fn native_get_platform_image_bytes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeGetPlatformImageBytes([III)[B")
}

#[async_recursion(?Send)]
async fn native_resize_ns_image_representations(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeResizeNSImageRepresentations(JDD)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_image_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeSetNSImageSize(JDD)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CImage";
        assert!(registry
            .method(class_name, "nativeCopyNSImageIntoArray", "(J[IIIII)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeCreateNSImageFromArray", "([III)J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeCreateNSImageFromArrays", "([[I[I[I)J")
            .is_some());
        assert!(registry
            .method(class_name, "nativeCreateNSImageFromBytes", "([B)J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeCreateNSImageFromFileContents",
                "(Ljava/lang/String;)J"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nativeCreateNSImageFromIconSelector", "(I)J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeCreateNSImageFromImageName",
                "(Ljava/lang/String;)J"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeCreateNSImageOfFileFromLaunchServices",
                "(Ljava/lang/String;)J"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeGetNSImageRepresentationSizes",
                "(JDD)[Ljava/awt/geom/Dimension2D;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeGetNSImageSize",
                "(J)Ljava/awt/geom/Dimension2D;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nativeGetPlatformImageBytes", "([III)[B")
            .is_some());
        assert!(registry
            .method(class_name, "nativeResizeNSImageRepresentations", "(JDD)V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetNSImageSize", "(JDD)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CImage.nativeCopyNSImageIntoArray(J[IIIII)V")]
    async fn test_native_copy_ns_image_into_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_copy_ns_image_into_array(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CImage.nativeCreateNSImageFromArray([III)J")]
    async fn test_native_create_ns_image_from_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_array(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CImage.nativeCreateNSImageFromArrays([[I[I[I)J")]
    async fn test_native_create_ns_image_from_arrays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_arrays(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CImage.nativeCreateNSImageFromBytes([B)J")]
    async fn test_native_create_ns_image_from_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_bytes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CImage.nativeCreateNSImageFromFileContents(Ljava/lang/String;)J"
    )]
    async fn test_native_create_ns_image_from_file_contents() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_file_contents(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CImage.nativeCreateNSImageFromIconSelector(I)J")]
    async fn test_native_create_ns_image_from_icon_selector() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_icon_selector(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CImage.nativeCreateNSImageFromImageName(Ljava/lang/String;)J"
    )]
    async fn test_native_create_ns_image_from_image_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_from_image_name(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CImage.nativeCreateNSImageOfFileFromLaunchServices(Ljava/lang/String;)J"
    )]
    async fn test_native_create_ns_image_of_file_from_launch_services() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ =
            native_create_ns_image_of_file_from_launch_services(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CImage.nativeGetNSImageRepresentationSizes(JDD)[Ljava/awt/geom/Dimension2D;"
    )]
    async fn test_native_get_ns_image_representation_sizes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_image_representation_sizes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CImage.nativeGetNSImageSize(J)Ljava/awt/geom/Dimension2D;"
    )]
    async fn test_native_get_ns_image_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_image_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CImage.nativeGetPlatformImageBytes([III)[B")]
    async fn test_native_get_platform_image_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_platform_image_bytes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CImage.nativeResizeNSImageRepresentations(JDD)V")]
    async fn test_native_resize_ns_image_representations() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_resize_ns_image_representations(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CImage.nativeSetNSImageSize(JDD)V")]
    async fn test_native_set_ns_image_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_image_size(thread, Arguments::default()).await;
    }
}
