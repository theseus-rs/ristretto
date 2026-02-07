use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_class_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;"
    )
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetColorForState(III)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_color_for_state<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetColorForState(III)I")
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_pango_font_name<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;")
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetXThickness(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_x_thickness<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetXThickness(I)I")
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetYThickness(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_y_thickness<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
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
        let _ = native_get_class_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetColorForState(III)I"
    )]
    async fn test_native_get_color_for_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_color_for_state(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;"
    )]
    async fn test_native_get_pango_font_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_pango_font_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetXThickness(I)I"
    )]
    async fn test_native_get_x_thickness() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_x_thickness(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetYThickness(I)I"
    )]
    async fn test_native_get_y_thickness() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_y_thickness(thread, Parameters::default()).await;
    }
}
