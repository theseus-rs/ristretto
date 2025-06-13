use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, NotEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/CGraphicsDevice.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_bounds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;")
}

#[intrinsic_method(
    "sun/awt/CGraphicsDevice.nativeGetDisplayMode(I)Ljava/awt/DisplayMode;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_display_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetDisplayMode(I)Ljava/awt/DisplayMode;")
}

#[intrinsic_method(
    "sun/awt/CGraphicsDevice.nativeGetDisplayModes(I)[Ljava/awt/DisplayMode;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_display_modes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetDisplayModes(I)[Ljava/awt/DisplayMode;")
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeGetScaleFactor(I)D", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_scale_factor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetScaleFactor(I)D")
}

#[intrinsic_method(
    "sun/awt/CGraphicsDevice.nativeGetScreenInsets(I)Ljava/awt/Insets;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_screen_insets(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetScreenInsets(I)Ljava/awt/Insets;")
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeGetXResolution(I)D", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_x_resolution(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetXResolution(I)D")
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeGetYResolution(I)D", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_y_resolution(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetYResolution(I)D")
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeResetDisplayMode()V", NotEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn native_reset_display_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeResetDisplayMode()V")
}

#[intrinsic_method("sun/awt/CGraphicsDevice.nativeSetDisplayMode(IIIII)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_display_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeSetDisplayMode(IIIII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsDevice.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;"
    )]
    async fn test_native_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_bounds(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsDevice.nativeGetDisplayMode(I)Ljava/awt/DisplayMode;"
    )]
    async fn test_native_get_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_display_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsDevice.nativeGetDisplayModes(I)[Ljava/awt/DisplayMode;"
    )]
    async fn test_native_get_display_modes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_display_modes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsDevice.nativeGetScaleFactor(I)D"
    )]
    async fn test_native_get_scale_factor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_scale_factor(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsDevice.nativeGetScreenInsets(I)Ljava/awt/Insets;"
    )]
    async fn test_native_get_screen_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_screen_insets(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsDevice.nativeGetXResolution(I)D"
    )]
    async fn test_native_get_x_resolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_x_resolution(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsDevice.nativeGetYResolution(I)D"
    )]
    async fn test_native_get_y_resolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_y_resolution(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsDevice.nativeResetDisplayMode()V"
    )]
    async fn test_native_reset_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_reset_display_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.CGraphicsDevice.nativeSetDisplayMode(IIIII)V"
    )]
    async fn test_native_set_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_display_mode(thread, Parameters::default()).await;
    }
}
