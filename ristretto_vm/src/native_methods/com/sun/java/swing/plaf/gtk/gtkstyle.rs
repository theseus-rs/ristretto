use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/java/swing/plaf/gtk/GTKStyle";

/// Register all native methods for `com.sun.java.swing.plaf.gtk.GTKStyle`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "nativeGetClassValue",
        "(ILjava/lang/String;)Ljava/lang/Object;",
        native_get_class_value,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetColorForState",
        "(III)I",
        native_get_color_for_state,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetPangoFontName",
        "(I)Ljava/lang/String;",
        native_get_pango_font_name,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetXThickness",
        "(I)I",
        native_get_x_thickness,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetYThickness",
        "(I)I",
        native_get_y_thickness,
    );
}

#[async_recursion(?Send)]
async fn native_get_class_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn native_get_color_for_state(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetColorForState(III)I")
}

#[async_recursion(?Send)]
async fn native_get_pango_font_name(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn native_get_x_thickness(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetXThickness(I)I")
}

#[async_recursion(?Send)]
async fn native_get_y_thickness(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetYThickness(I)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;"
    )]
    async fn test_native_get_class_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_class_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetColorForState(III)I"
    )]
    async fn test_native_get_color_for_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_color_for_state(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;"
    )]
    async fn test_native_get_pango_font_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_pango_font_name(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetXThickness(I)I"
    )]
    async fn test_native_get_x_thickness() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_x_thickness(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetYThickness(I)I"
    )]
    async fn test_native_get_y_thickness() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_y_thickness(thread, Arguments::default()).await;
    }
}
