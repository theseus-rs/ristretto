use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WMouseInfoPeer.fillPointWithCoords(Ljava/awt/Point;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fill_point_with_coords<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _point = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WMouseInfoPeer.fillPointWithCoords(Ljava/awt/Point;)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WMouseInfoPeer.isWindowUnderMouse(Ljava/awt/Window;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_window_under_mouse<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WMouseInfoPeer.isWindowUnderMouse(Ljava/awt/Window;)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_fill_point_with_coords() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            fill_point_with_coords(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WMouseInfoPeer.fillPointWithCoords(Ljava/awt/Point;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_window_under_mouse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            is_window_under_mouse(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WMouseInfoPeer.isWindowUnderMouse(Ljava/awt/Window;)Z",
            result.unwrap_err().to_string()
        );
    }
}
