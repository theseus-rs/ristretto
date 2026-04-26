use ristretto_classfile::VersionSpecification::{Any, GreaterThan, NotEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/CGraphicsDevice.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn native_get_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsDevice.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/CGraphicsDevice.nativeGetDisplayMode(I)Ljava/awt/DisplayMode;",
    Any
)]
#[async_method]
pub async fn native_get_display_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsDevice.nativeGetDisplayMode(I)Ljava/awt/DisplayMode;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/CGraphicsDevice.nativeGetDisplayModes(I)[Ljava/awt/DisplayMode;",
    Any
)]
#[async_method]
pub async fn native_get_display_modes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsDevice.nativeGetDisplayModes(I)[Ljava/awt/DisplayMode;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeGetScaleFactor(I)D", Any)]
#[async_method]
pub async fn native_get_scale_factor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsDevice.nativeGetScaleFactor(I)D".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/CGraphicsDevice.nativeGetScreenInsets(I)Ljava/awt/Insets;",
    Any
)]
#[async_method]
pub async fn native_get_screen_insets<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsDevice.nativeGetScreenInsets(I)Ljava/awt/Insets;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeGetXResolution(I)D", Any)]
#[async_method]
pub async fn native_get_x_resolution<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsDevice.nativeGetXResolution(I)D".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeGetYResolution(I)D", Any)]
#[async_method]
pub async fn native_get_y_resolution<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsDevice.nativeGetYResolution(I)D".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeResetDisplayMode()V", NotEqual(JAVA_11))]
#[async_method]
pub async fn native_reset_display_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsDevice.nativeResetDisplayMode()V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeSetDisplayMode(IIIII)V", Any)]
#[async_method]
pub async fn native_set_display_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _refrate = parameters.pop_int()?;
    let _bpp = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _display_id = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsDevice.nativeSetDisplayMode(IIIII)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_bounds(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.CGraphicsDevice.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_display_mode(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.CGraphicsDevice.nativeGetDisplayMode(I)Ljava/awt/DisplayMode;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_display_modes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_display_modes(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.CGraphicsDevice.nativeGetDisplayModes(I)[Ljava/awt/DisplayMode;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_scale_factor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_scale_factor(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.CGraphicsDevice.nativeGetScaleFactor(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_screen_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_screen_insets(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.CGraphicsDevice.nativeGetScreenInsets(I)Ljava/awt/Insets;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_x_resolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_x_resolution(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.CGraphicsDevice.nativeGetXResolution(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_y_resolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_y_resolution(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.awt.CGraphicsDevice.nativeGetYResolution(I)D",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_reset_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_reset_display_mode(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.CGraphicsDevice.nativeResetDisplayMode()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_display_mode(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.awt.CGraphicsDevice.nativeSetDisplayMode(IIIII)V",
            result.unwrap_err().to_string()
        );
    }
}
