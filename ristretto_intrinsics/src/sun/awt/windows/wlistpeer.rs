use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WListPeer.addItems([Ljava/lang/String;II)V", Any)]
#[async_method]
pub async fn add_items<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _width = parameters.pop_int()?;
    let _index = parameters.pop_int()?;
    let _items = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WListPeer.addItems([Ljava/lang/String;II)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WListPeer.create(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WListPeer.create(Lsun/awt/windows/WComponentPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WListPeer.delItems(II)V", Any)]
#[async_method]
pub async fn del_items<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _end = parameters.pop_int()?;
    let _start = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WListPeer.delItems(II)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WListPeer.deselect(I)V", Any)]
#[async_method]
pub async fn deselect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pos = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WListPeer.deselect(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WListPeer.getMaxWidth()I", Any)]
#[async_method]
pub async fn get_max_width<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WListPeer.getMaxWidth()I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WListPeer.isSelected(I)Z", Any)]
#[async_method]
pub async fn is_selected<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WListPeer.isSelected(I)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WListPeer.makeVisible(I)V", Any)]
#[async_method]
pub async fn make_visible<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pos = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WListPeer.makeVisible(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WListPeer.select(I)V", Any)]
#[async_method]
pub async fn select<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pos = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WListPeer.select(I)V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WListPeer.setMultipleSelections(Z)V", Any)]
#[async_method]
pub async fn set_multiple_selections<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _on = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WListPeer.setMultipleSelections(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WListPeer.updateMaxItemWidth()V", Any)]
#[async_method]
pub async fn update_max_item_width<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WListPeer.updateMaxItemWidth()V".to_string(),
    )
    .into())
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
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WListPeer.addItems([Ljava/lang/String;II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WListPeer.create(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_del_items() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = del_items(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WListPeer.delItems(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_deselect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = deselect(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WListPeer.deselect(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_max_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_max_width(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WListPeer.getMaxWidth()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_selected() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_selected(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WListPeer.isSelected(I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_make_visible() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_visible(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WListPeer.makeVisible(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_select() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = select(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WListPeer.select(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_multiple_selections() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_multiple_selections(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WListPeer.setMultipleSelections(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_update_max_item_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_max_item_width(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WListPeer.updateMaxItemWidth()V",
            result.unwrap_err().to_string()
        );
    }
}
