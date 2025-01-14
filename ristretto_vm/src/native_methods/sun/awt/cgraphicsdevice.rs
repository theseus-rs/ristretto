use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `sun.awt.CGraphicsDevice`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/CGraphicsDevice";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_8 {
        registry.register(
            class_name,
            "nativeResetDisplayMode",
            "()V",
            native_reset_display_mode,
        );
    } else {
        registry.register(
            class_name,
            "nativeGetBounds",
            "(I)Ljava/awt/geom/Rectangle2D;",
            native_get_bounds,
        );
    }

    if java_version >= JAVA_17 {
        registry.register(
            class_name,
            "nativeResetDisplayMode",
            "()V",
            native_reset_display_mode,
        );
    }

    registry.register(
        class_name,
        "nativeGetDisplayMode",
        "(I)Ljava/awt/DisplayMode;",
        native_get_display_mode,
    );
    registry.register(
        class_name,
        "nativeGetDisplayModes",
        "(I)[Ljava/awt/DisplayMode;",
        native_get_display_modes,
    );
    registry.register(
        class_name,
        "nativeGetScaleFactor",
        "(I)D",
        native_get_scale_factor,
    );
    registry.register(
        class_name,
        "nativeGetScreenInsets",
        "(I)Ljava/awt/Insets;",
        native_get_screen_insets,
    );
    registry.register(
        class_name,
        "nativeGetXResolution",
        "(I)D",
        native_get_x_resolution,
    );
    registry.register(
        class_name,
        "nativeGetYResolution",
        "(I)D",
        native_get_y_resolution,
    );
    registry.register(
        class_name,
        "nativeSetDisplayMode",
        "(IIIII)V",
        native_set_display_mode,
    );
}

#[async_recursion(?Send)]
async fn native_get_bounds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;")
}

#[async_recursion(?Send)]
async fn native_get_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetDisplayMode(I)Ljava/awt/DisplayMode;")
}

#[async_recursion(?Send)]
async fn native_get_display_modes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetDisplayModes(I)[Ljava/awt/DisplayMode;")
}

#[async_recursion(?Send)]
async fn native_get_scale_factor(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetScaleFactor(I)D")
}

#[async_recursion(?Send)]
async fn native_get_screen_insets(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetScreenInsets(I)Ljava/awt/Insets;")
}

#[async_recursion(?Send)]
async fn native_get_x_resolution(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetXResolution(I)D")
}

#[async_recursion(?Send)]
async fn native_get_y_resolution(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeGetYResolution(I)D")
}

#[async_recursion(?Send)]
async fn native_reset_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeResetDisplayMode()V")
}

#[async_recursion(?Send)]
async fn native_set_display_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.CGraphicsDevice.nativeSetDisplayMode(IIIII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java21 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/awt/CGraphicsDevice";
        assert!(registry
            .method(
                class_name,
                "nativeGetBounds",
                "(I)Ljava/awt/geom/Rectangle2D;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeGetDisplayMode",
                "(I)Ljava/awt/DisplayMode;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "nativeGetDisplayModes",
                "(I)[Ljava/awt/DisplayMode;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "nativeGetScaleFactor", "(I)D")
            .is_some());
        assert!(registry
            .method(class_name, "nativeGetScreenInsets", "(I)Ljava/awt/Insets;")
            .is_some());
        assert!(registry
            .method(class_name, "nativeGetXResolution", "(I)D")
            .is_some());
        assert!(registry
            .method(class_name, "nativeGetYResolution", "(I)D")
            .is_some());
        assert!(registry
            .method(class_name, "nativeResetDisplayMode", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "nativeSetDisplayMode", "(IIIII)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.CGraphicsDevice.nativeGetBounds(I)Ljava/awt/geom/Rectangle2D;"
    )]
    async fn test_native_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_bounds(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.CGraphicsDevice.nativeGetDisplayMode(I)Ljava/awt/DisplayMode;"
    )]
    async fn test_native_get_display_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_display_mode(thread, Arguments::default()).await;
    }
}
