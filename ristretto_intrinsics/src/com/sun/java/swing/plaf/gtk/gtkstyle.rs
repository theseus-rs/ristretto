use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_class_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _key = parameters.pop_reference()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetColorForState(III)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_color_for_state<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _type_id = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetColorForState(III)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_pango_font_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetXThickness(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_x_thickness<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetXThickness(I)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetYThickness(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_y_thickness<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetYThickness(I)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_get_class_value_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _key = parameters.pop_reference()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;".to_string()).into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetColorForState(III)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_get_color_for_state_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _type_id = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetColorForState(III)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_get_pango_font_name_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetXThickness(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_get_xthickness_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetXThickness(I)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetYThickness(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_get_ythickness_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetYThickness(I)I".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_get_class_value() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_get_class_value(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_color_for_state() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_get_color_for_state(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetColorForState(III)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_pango_font_name() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_get_pango_font_name(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_x_thickness() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_get_x_thickness(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetXThickness(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_y_thickness() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_get_y_thickness(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKStyle.nativeGetYThickness(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_get_class_value_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_class_value_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetClassValue(ILjava/lang/String;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_get_color_for_state_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_color_for_state_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetColorForState(III)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_get_pango_font_name_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_get_pango_font_name_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)]))
                .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetPangoFontName(I)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_get_xthickness_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_get_xthickness_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetXThickness(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_get_ythickness_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_get_ythickness_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKStyle.nativeGetYThickness(I)I",
            result.unwrap_err().to_string()
        );
    }
}
