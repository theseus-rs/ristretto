use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WGlobalCursorManager.findHeavyweightUnderCursor(Z)Ljava/awt/Component;",
    Any
)]
#[async_method]
pub async fn find_heavyweight_under_cursor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _use_cache = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WGlobalCursorManager.findHeavyweightUnderCursor(Z)Ljava/awt/Component;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WGlobalCursorManager.getCursorPos(Ljava/awt/Point;)V",
    Any
)]
#[async_method]
pub async fn get_cursor_pos<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _point = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WGlobalCursorManager.getCursorPos(Ljava/awt/Point;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WGlobalCursorManager.getLocationOnScreen(Ljava/awt/Component;)Ljava/awt/Point;",
    Any
)]
#[async_method]
pub async fn get_location_on_screen<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _component = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WGlobalCursorManager.getLocationOnScreen(Ljava/awt/Component;)Ljava/awt/Point;".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/windows/WGlobalCursorManager.setCursor(Ljava/awt/Component;Ljava/awt/Cursor;Z)V",
    Any
)]
#[async_method]
pub async fn set_cursor<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_bool()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WGlobalCursorManager.setCursor(Ljava/awt/Component;Ljava/awt/Cursor;Z)V"
            .to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_find_heavyweight_under_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            find_heavyweight_under_cursor(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WGlobalCursorManager.findHeavyweightUnderCursor(Z)Ljava/awt/Component;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_cursor_pos() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_cursor_pos(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WGlobalCursorManager.getCursorPos(Ljava/awt/Point;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_location_on_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_location_on_screen(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WGlobalCursorManager.getLocationOnScreen(Ljava/awt/Component;)Ljava/awt/Point;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_cursor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_cursor(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WGlobalCursorManager.setCursor(Ljava/awt/Component;Ljava/awt/Cursor;Z)V",
            result.unwrap_err().to_string()
        );
    }
}
