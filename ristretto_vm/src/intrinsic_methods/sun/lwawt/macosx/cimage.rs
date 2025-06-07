use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCopyNSImageIntoArray(J[IIIII)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_copy_ns_image_into_array(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCopyNSImageIntoArray(J[IIIII)V")
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCreateNSImageFromArray([III)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_ns_image_from_array(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromArray([III)J")
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCreateNSImageFromArrays([[I[I[I)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_ns_image_from_arrays(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromArrays([[I[I[I)J")
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCreateNSImageFromBytes([B)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_ns_image_from_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromBytes([B)J")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeCreateNSImageFromFileContents(Ljava/lang/String;)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_ns_image_from_file_contents(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromFileContents(Ljava/lang/String;)J")
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCreateNSImageFromIconSelector(I)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_ns_image_from_icon_selector(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromIconSelector(I)J")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeCreateNSImageFromImageName(Ljava/lang/String;)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_ns_image_from_image_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeCreateNSImageFromImageName(Ljava/lang/String;)J")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeCreateNSImageOfFileFromLaunchServices(Ljava/lang/String;)J",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_ns_image_of_file_from_launch_services(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageOfFileFromLaunchServices(Ljava/lang/String;)J"
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeGetNSImageRepresentationSizes(JDD)[Ljava/awt/geom/Dimension2D;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_ns_image_representation_sizes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CImage.nativeGetNSImageRepresentationSizes(JDD)[Ljava/awt/geom/Dimension2D;"
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeGetNSImageSize(J)Ljava/awt/geom/Dimension2D;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_ns_image_size(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeGetNSImageSize(J)Ljava/awt/geom/Dimension2D;")
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeGetPlatformImageBytes([III)[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_platform_image_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeGetPlatformImageBytes([III)[B")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeResizeNSImageRepresentations(JDD)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_resize_ns_image_representations(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CImage.nativeResizeNSImageRepresentations(JDD)V")
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeSetNSImageSize(JDD)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_ns_image_size(
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
