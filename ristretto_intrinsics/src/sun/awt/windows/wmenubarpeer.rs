use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WMenuBarPeer.addMenu(Ljava/awt/Menu;)V", Any)]
#[async_method]
pub async fn add_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _menu = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WMenuBarPeer.addMenu(Ljava/awt/Menu;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WMenuBarPeer.create(Lsun/awt/windows/WFramePeer;)V",
    Any
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _frame = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WMenuBarPeer.create(Lsun/awt/windows/WFramePeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WMenuBarPeer.delMenu(I)V", Any)]
#[async_method]
pub async fn del_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WMenuBarPeer.delMenu(I)V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_add_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_menu(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuBarPeer.addMenu(Ljava/awt/Menu;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuBarPeer.create(Lsun/awt/windows/WFramePeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_del_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = del_menu(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuBarPeer.delMenu(I)V",
            result.unwrap_err().to_string()
        );
    }
}
