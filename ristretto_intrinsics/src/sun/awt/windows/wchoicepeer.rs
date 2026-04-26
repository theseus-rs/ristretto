use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WChoicePeer.addItems([Ljava/lang/String;I)V", Any)]
#[async_method]
pub async fn add_items<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    let _items = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WChoicePeer.addItems([Ljava/lang/String;I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WChoicePeer.closeList()V", Any)]
#[async_method]
pub async fn close_list<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WChoicePeer.closeList()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WChoicePeer.create(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WChoicePeer.create(Lsun/awt/windows/WComponentPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WChoicePeer.remove(I)V", Any)]
#[async_method]
pub async fn remove<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WChoicePeer.remove(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WChoicePeer.removeAll()V", Any)]
#[async_method]
pub async fn remove_all<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WChoicePeer.removeAll()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WChoicePeer.reshape(IIII)V", Any)]
#[async_method]
pub async fn reshape<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WChoicePeer.reshape(IIII)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WChoicePeer.select(I)V", Any)]
#[async_method]
pub async fn select<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WChoicePeer.select(I)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_add_items() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_items(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WChoicePeer.addItems([Ljava/lang/String;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_list(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WChoicePeer.closeList()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WChoicePeer.create(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_remove() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WChoicePeer.remove(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_remove_all() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_all(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WChoicePeer.removeAll()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_reshape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reshape(
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
            "sun/awt/windows/WChoicePeer.reshape(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_select() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = select(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WChoicePeer.select(I)V",
            result.unwrap_err().to_string()
        );
    }
}
