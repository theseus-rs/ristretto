use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.java.swing.plaf.gtk.GTKStyle`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/java/swing/plaf/gtk/GTKStyle";
    registry.register(
        class_name,
        "nativeGetClassValue",
        "(ILjava/lang/String;)Ljava/lang/Object;",
        native_get_class_value,
    );
    registry.register(
        class_name,
        "nativeGetColorForState",
        "(III)I",
        native_get_color_for_state,
    );
    registry.register(
        class_name,
        "nativeGetPangoFontName",
        "(I)Ljava/lang/String;",
        native_get_pango_font_name,
    );
    registry.register(
        class_name,
        "nativeGetXThickness",
        "(I)I",
        native_get_x_thickness,
    );
    registry.register(
        class_name,
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
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_color_for_state(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_pango_font_name(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_x_thickness(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_get_y_thickness(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
