use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_8, JAVA_17, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/CGraphicsDevice";

/// Register all intrinsic methods for `sun.awt.CGraphicsDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "nativeResetDisplayMode",
            "()V",
            native_reset_display_mode,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "nativeGetBounds",
            "(I)Ljava/awt/geom/Rectangle2D;",
            native_get_bounds,
        );
    }

    if registry.java_major_version() >= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "nativeResetDisplayMode",
            "()V",
            native_reset_display_mode,
        );
    }

    registry.register(
        CLASS_NAME,
        "nativeGetDisplayMode",
        "(I)Ljava/awt/DisplayMode;",
        native_get_display_mode,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetDisplayModes",
        "(I)[Ljava/awt/DisplayMode;",
        native_get_display_modes,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetScaleFactor",
        "(I)D",
        native_get_scale_factor,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetScreenInsets",
        "(I)Ljava/awt/Insets;",
        native_get_screen_insets,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetXResolution",
        "(I)D",
        native_get_x_resolution,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetYResolution",
        "(I)D",
        native_get_y_resolution,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetDisplayMode",
        "(IIIII)V",
        native_set_display_mode,
    );
}

#[async_recursion(?Send)]
async fn native_get_bounds(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;")
}

#[async_recursion(?Send)]
async fn native_get_display_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetDisplayMode(I)Ljava/awt/DisplayMode;")
}

#[async_recursion(?Send)]
async fn native_get_display_modes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetDisplayModes(I)[Ljava/awt/DisplayMode;")
}

#[async_recursion(?Send)]
async fn native_get_scale_factor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetScaleFactor(I)D")
}

#[async_recursion(?Send)]
async fn native_get_screen_insets(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetScreenInsets(I)Ljava/awt/Insets;")
}

#[async_recursion(?Send)]
async fn native_get_x_resolution(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetXResolution(I)D")
}

#[async_recursion(?Send)]
async fn native_get_y_resolution(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetYResolution(I)D")
}

#[async_recursion(?Send)]
async fn native_reset_display_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeResetDisplayMode()V")
}

#[async_recursion(?Send)]
async fn native_set_display_mode(
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
