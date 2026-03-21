use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.nativeFinishPainting([III)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_finish_painting<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativeFinishPainting([III)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.nativeSetRangeValue(IDDDD)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_set_range_value<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativeSetRangeValue(IDDDD)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.nativeStartPainting(II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_start_painting<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativeStartPainting(II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_get_gtk_setting(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_gtk_setting<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativeGetGTKSetting(I)Ljava/lang/Object;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_arrow(IIILjava/lang/String;IIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_arrow<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintArrow(IIILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_background(IIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_background<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBackground(IIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box(IIILjava/lang/String;IIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_box<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBox(IIILjava/lang/String;IIIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box_gap(IIILjava/lang/String;IIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_box_gap<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBoxGap(IIILjava/lang/String;IIIIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_check(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_check<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintCheck(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_expander(IILjava/lang/String;IIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_expander<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintExpander(IILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_extension(IIILjava/lang/String;IIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_extension<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintExtension(IIILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_flat_box(IIILjava/lang/String;IIIIZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_flat_box<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintFlatBox(IIILjava/lang/String;IIIIZ)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_focus(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_focus<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintFocus(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_handle(IIILjava/lang/String;IIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintHandle(IIILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_hline(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_hline<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintHline(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_option(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_option<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintOption(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_shadow(IIILjava/lang/String;IIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_shadow<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintShadow(IIILjava/lang/String;IIIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_slider(IIILjava/lang/String;IIIIIZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_slider<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintSlider(IIILjava/lang/String;IIIIIZ)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_vline(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_vline<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintVline(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_switch_theme()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_switch_theme<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.native_switch_theme()V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_finish_painting() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_finish_painting(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_range_value() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_set_range_value(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_start_painting() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_start_painting(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_gtk_setting() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_get_gtk_setting(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_arrow() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_arrow(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_background() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_background(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_box() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_box(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_box_gap() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_box_gap(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_check() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_check(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_expander() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_expander(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_extension() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_extension(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_flat_box() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_flat_box(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_focus() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_focus(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_handle() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_handle(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_hline() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_hline(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_option() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_option(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_shadow() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_shadow(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_slider() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_slider(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_paint_vline() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_vline(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_switch_theme() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_switch_theme(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
