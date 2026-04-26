use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/ThemeReader.closeTheme(J)V", Any)]
#[async_method]
pub async fn close_theme<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _theme = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/ThemeReader.closeTheme(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/ThemeReader.getBoolean(JIII)Z", Any)]
#[async_method]
pub async fn get_boolean<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.getBoolean(JIII)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/ThemeReader.getColor(JIII)Ljava/awt/Color;", Any)]
#[async_method]
pub async fn get_color<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _type_ = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.getColor(JIII)Ljava/awt/Color;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/ThemeReader.getEnum(JIII)I", Any)]
#[async_method]
pub async fn get_enum<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/ThemeReader.getEnum(JIII)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/ThemeReader.getInt(JIII)I", Any)]
#[async_method]
pub async fn get_int<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/ThemeReader.getInt(JIII)I".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/ThemeReader.getPartSize(JII)Ljava/awt/Dimension;",
    Any
)]
#[async_method]
pub async fn get_part_size<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.getPartSize(JII)Ljava/awt/Dimension;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/ThemeReader.getPoint(JIII)Ljava/awt/Point;", Any)]
#[async_method]
pub async fn get_point<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.getPoint(JIII)Ljava/awt/Point;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/ThemeReader.getPosition(JIII)Ljava/awt/Dimension;",
    Any
)]
#[async_method]
pub async fn get_position<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.getPosition(JIII)Ljava/awt/Dimension;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/ThemeReader.getSysBoolean(JI)Z", Any)]
#[async_method]
pub async fn get_sys_boolean<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.getSysBoolean(JI)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/ThemeReader.getThemeBackgroundContentMargins(JIIII)Ljava/awt/Insets;",
    Any
)]
#[async_method]
pub async fn get_theme_background_content_margins<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _bounding_height = parameters.pop_int()?;
    let _bounding_width = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _h_theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.getThemeBackgroundContentMargins(JIIII)Ljava/awt/Insets;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/ThemeReader.getThemeMargins(JIII)Ljava/awt/Insets;",
    Any
)]
#[async_method]
pub async fn get_theme_margins<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _property = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.getThemeMargins(JIII)Ljava/awt/Insets;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/ThemeReader.getThemeTransitionDuration(JIIII)J", Any)]
#[async_method]
pub async fn get_theme_transition_duration<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prop_id = parameters.pop_int()?;
    let _state_to = parameters.pop_int()?;
    let _state_from = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.getThemeTransitionDuration(JIIII)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/ThemeReader.initThemes()Z", Any)]
#[async_method]
pub async fn init_themes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/ThemeReader.initThemes()Z".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/ThemeReader.isGetThemeTransitionDurationDefined()Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_get_theme_transition_duration_defined<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.isGetThemeTransitionDurationDefined()Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/ThemeReader.isThemePartDefined(JII)Z", Any)]
#[async_method]
pub async fn is_theme_part_defined<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.isThemePartDefined(JII)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/ThemeReader.openTheme(Ljava/lang/String;)J",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn open_theme<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.openTheme(Ljava/lang/String;)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/ThemeReader.openTheme(Ljava/lang/String;I)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn open_theme_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dpi = parameters.pop_int()?;
    let _widget = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.openTheme(Ljava/lang/String;I)J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/ThemeReader.paintBackground([IJIIIIIII)V", Any)]
#[async_method]
pub async fn paint_background<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _stride = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _rect_bottom = parameters.pop_int()?;
    let _rect_right = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _part = parameters.pop_int()?;
    let _theme = parameters.pop_long()?;
    let _array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.paintBackground([IJIIIIIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/ThemeReader.setWindowTheme(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn set_window_theme<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sub_app_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/ThemeReader.setWindowTheme(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_theme() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_theme(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.closeTheme(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_boolean() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_boolean(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getBoolean(JIII)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_color() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_color(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getColor(JIII)Ljava/awt/Color;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_enum() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_enum(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getEnum(JIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_int() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_int(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getInt(JIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_part_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_part_size(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getPartSize(JII)Ljava/awt/Dimension;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_point() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_point(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getPoint(JIII)Ljava/awt/Point;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_position(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getPosition(JIII)Ljava/awt/Dimension;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_sys_boolean() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_sys_boolean(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getSysBoolean(JI)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_theme_background_content_margins() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_theme_background_content_margins(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getThemeBackgroundContentMargins(JIIII)Ljava/awt/Insets;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_theme_margins() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_theme_margins(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getThemeMargins(JIII)Ljava/awt/Insets;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_theme_transition_duration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_theme_transition_duration(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.getThemeTransitionDuration(JIIII)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_themes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_themes(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.initThemes()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_get_theme_transition_duration_defined() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_get_theme_transition_duration_defined(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.isGetThemeTransitionDurationDefined()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_theme_part_defined() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_theme_part_defined(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.isThemePartDefined(JII)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_theme() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_theme(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.openTheme(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_theme_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_theme_windows_ge_v11(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.openTheme(Ljava/lang/String;I)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_paint_background() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = paint_background(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.paintBackground([IJIIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_window_theme() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_window_theme(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/ThemeReader.setWindowTheme(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}
