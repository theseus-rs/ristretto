use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeCreateView(IIIIJ)J", Any)]
#[async_method]
pub async fn native_create_view<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_layer_ptr = parameters.pop_long()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeCreateView(IIIIJ)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;",
    Any
)]
#[async_method]
pub async fn native_get_location_on_screen<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _awt_view = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeGetNSViewDisplayID(J)I", Any)]
#[async_method]
pub async fn native_get_ns_view_display_id<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _awt_view = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeGetNSViewDisplayID(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeIsViewUnderMouse(J)Z", Any)]
#[async_method]
pub async fn native_is_view_under_mouse<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeIsViewUnderMouse(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformView.nativeSetAutoResizable(JZ)V", Any)]
#[async_method]
pub async fn native_set_auto_resizable<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _to_resize = parameters.pop_bool()?;
    let _awt_view = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformView.nativeSetAutoResizable(JZ)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_create_view() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_view(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CPlatformView.nativeCreateView(IIIIJ)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_location_on_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_get_location_on_screen(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CPlatformView.nativeGetLocationOnScreen(J)Ljava/awt/geom/Rectangle2D;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_ns_view_display_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_get_ns_view_display_id(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CPlatformView.nativeGetNSViewDisplayID(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_is_view_under_mouse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_is_view_under_mouse(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CPlatformView.nativeIsViewUnderMouse(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_auto_resizable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_auto_resizable(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CPlatformView.nativeSetAutoResizable(JZ)V",
            result.unwrap_err().to_string()
        );
    }
}
