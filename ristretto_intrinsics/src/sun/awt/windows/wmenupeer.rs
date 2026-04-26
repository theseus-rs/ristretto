use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WMenuPeer.addSeparator()V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn add_separator<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WMenuPeer.addSeparator()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WMenuPeer.createMenu(Lsun/awt/windows/WMenuBarPeer;)V",
    Any
)]
#[async_method]
pub async fn create_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _menu_bar = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WMenuPeer.createMenu(Lsun/awt/windows/WMenuBarPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WMenuPeer.createSubMenu(Lsun/awt/windows/WMenuPeer;)V",
    Any
)]
#[async_method]
pub async fn create_sub_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _menu = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WMenuPeer.createSubMenu(Lsun/awt/windows/WMenuPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WMenuPeer.delItem(I)V", Any)]
#[async_method]
pub async fn del_item<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WMenuPeer.delItem(I)V".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_add_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_separator(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WMenuPeer.addSeparator()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_menu(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuPeer.createMenu(Lsun/awt/windows/WMenuBarPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_sub_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_sub_menu(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuPeer.createSubMenu(Lsun/awt/windows/WMenuPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_del_item() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = del_item(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WMenuPeer.delItem(I)V",
            result.unwrap_err().to_string()
        );
    }
}
