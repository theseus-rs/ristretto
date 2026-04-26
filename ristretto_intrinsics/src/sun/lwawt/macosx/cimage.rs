use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCopyNSImageIntoArray(J[IIIII)V", Any)]
#[async_method]
pub async fn native_copy_ns_image_into_array<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dh = parameters.pop_int()?;
    let _dw = parameters.pop_int()?;
    let _sh = parameters.pop_int()?;
    let _sw = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    let _image = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeCopyNSImageIntoArray(J[IIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCreateNSImageFromArray([III)J", Any)]
#[async_method]
pub async fn native_create_ns_image_from_array<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageFromArray([III)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCreateNSImageFromArrays([[I[I[I)J", Any)]
#[async_method]
pub async fn native_create_ns_image_from_arrays<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_reference()?;
    let _w = parameters.pop_reference()?;
    let _buffers = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageFromArrays([[I[I[I)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCreateNSImageFromBytes([B)J", Any)]
#[async_method]
pub async fn native_create_ns_image_from_bytes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buffer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageFromBytes([B)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeCreateNSImageFromFileContents(Ljava/lang/String;)J",
    Any
)]
#[async_method]
pub async fn native_create_ns_image_from_file_contents<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _file = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageFromFileContents(Ljava/lang/String;)J"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeCreateNSImageFromIconSelector(I)J", Any)]
#[async_method]
pub async fn native_create_ns_image_from_icon_selector<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _selector = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageFromIconSelector(I)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeCreateNSImageFromImageName(Ljava/lang/String;)J",
    Any
)]
#[async_method]
pub async fn native_create_ns_image_from_image_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageFromImageName(Ljava/lang/String;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeCreateNSImageOfFileFromLaunchServices(Ljava/lang/String;)J",
    Any
)]
#[async_method]
pub async fn native_create_ns_image_of_file_from_launch_services<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _file = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeCreateNSImageOfFileFromLaunchServices(Ljava/lang/String;)J"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeGetNSImageRepresentationSizes(JDD)[Ljava/awt/geom/Dimension2D;",
    Any
)]
#[async_method]
pub async fn native_get_ns_image_representation_sizes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _image = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CImage.nativeGetNSImageRepresentationSizes(JDD)[Ljava/awt/geom/Dimension2D;".to_string()).into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeGetNSImageSize(J)Ljava/awt/geom/Dimension2D;",
    Any
)]
#[async_method]
pub async fn native_get_ns_image_size<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _image = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeGetNSImageSize(J)Ljava/awt/geom/Dimension2D;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeGetPlatformImageBytes([III)[B", Any)]
#[async_method]
pub async fn native_get_platform_image_bytes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeGetPlatformImageBytes([III)[B".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CImage.nativeResizeNSImageRepresentations(JDD)V",
    Any
)]
#[async_method]
pub async fn native_resize_ns_image_representations<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _image = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeResizeNSImageRepresentations(JDD)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CImage.nativeSetNSImageSize(JDD)V", Any)]
#[async_method]
pub async fn native_set_ns_image_size<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_double()?;
    let _w = parameters.pop_double()?;
    let _image = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CImage.nativeSetNSImageSize(JDD)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_copy_ns_image_into_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_copy_ns_image_into_array(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeCopyNSImageIntoArray(J[IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_create_ns_image_from_array() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_ns_image_from_array(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeCreateNSImageFromArray([III)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_create_ns_image_from_arrays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_ns_image_from_arrays(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeCreateNSImageFromArrays([[I[I[I)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_create_ns_image_from_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_create_ns_image_from_bytes(thread, Parameters::new(vec![Value::Object(None)]))
                .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeCreateNSImageFromBytes([B)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_create_ns_image_from_file_contents() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_ns_image_from_file_contents(
            thread,
            Parameters::new(vec![Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeCreateNSImageFromFileContents(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_create_ns_image_from_icon_selector() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_create_ns_image_from_icon_selector(thread, Parameters::new(vec![Value::Int(0)]))
                .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeCreateNSImageFromIconSelector(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_create_ns_image_from_image_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_ns_image_from_image_name(
            thread,
            Parameters::new(vec![Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeCreateNSImageFromImageName(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_create_ns_image_of_file_from_launch_services() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_image_of_file_from_launch_services(thread, Parameters::default())
            .await;
    }

    #[tokio::test]
    async fn test_native_get_ns_image_representation_sizes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_ns_image_representation_sizes(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Double(0.0), Value::Double(0.0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeGetNSImageRepresentationSizes(JDD)[Ljava/awt/geom/Dimension2D;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_ns_image_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_ns_image_size(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeGetNSImageSize(J)Ljava/awt/geom/Dimension2D;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_platform_image_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_platform_image_bytes(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeGetPlatformImageBytes([III)[B",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_resize_ns_image_representations() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_resize_ns_image_representations(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Double(0.0), Value::Double(0.0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeResizeNSImageRepresentations(JDD)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_ns_image_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_image_size(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Double(0.0), Value::Double(0.0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CImage.nativeSetNSImageSize(JDD)V",
            result.unwrap_err().to_string()
        );
    }
}
