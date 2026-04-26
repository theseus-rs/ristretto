use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WScrollbarPeer.create(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollbarPeer.create(Lsun/awt/windows/WComponentPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WScrollbarPeer.getScrollbarSize(I)I", Any)]
#[async_method]
pub async fn get_scrollbar_size<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _orientation = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollbarPeer.getScrollbarSize(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WScrollbarPeer.setLineIncrement(I)V", Any)]
#[async_method]
pub async fn set_line_increment<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _increment = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollbarPeer.setLineIncrement(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WScrollbarPeer.setPageIncrement(I)V", Any)]
#[async_method]
pub async fn set_page_increment<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _increment = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollbarPeer.setPageIncrement(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WScrollbarPeer.setValues(IIII)V", Any)]
#[async_method]
pub async fn set_values<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _maximum = parameters.pop_int()?;
    let _minimum = parameters.pop_int()?;
    let _visible = parameters.pop_int()?;
    let _value = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollbarPeer.setValues(IIII)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WScrollbarPeer.create(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_scrollbar_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_scrollbar_size(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WScrollbarPeer.getScrollbarSize(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_line_increment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_line_increment(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WScrollbarPeer.setLineIncrement(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_page_increment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_page_increment(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WScrollbarPeer.setPageIncrement(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_values() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_values(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WScrollbarPeer.setValues(IIII)V",
            result.unwrap_err().to_string()
        );
    }
}
