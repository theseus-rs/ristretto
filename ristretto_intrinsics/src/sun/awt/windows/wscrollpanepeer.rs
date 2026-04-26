use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WScrollPanePeer._getHScrollbarHeight()I", Any)]
#[async_method]
pub async fn get_hscrollbar_height<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollPanePeer._getHScrollbarHeight()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WScrollPanePeer._getVScrollbarWidth()I", Any)]
#[async_method]
pub async fn get_vscrollbar_width<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollPanePeer._getVScrollbarWidth()I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WScrollPanePeer.create(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollPanePeer.create(Lsun/awt/windows/WComponentPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WScrollPanePeer.getOffset(I)I", Any)]
#[async_method]
pub async fn get_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _orient = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollPanePeer.getOffset(I)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WScrollPanePeer.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WScrollPanePeer.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WScrollPanePeer.setInsets()V", Any)]
#[async_method]
pub async fn set_insets<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WScrollPanePeer.setInsets()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WScrollPanePeer.setScrollPosition(II)V", Any)]
#[async_method]
pub async fn set_scroll_position<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollPanePeer.setScrollPosition(II)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WScrollPanePeer.setSpans(IIII)V", Any)]
#[async_method]
pub async fn set_spans<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _child_height = parameters.pop_int()?;
    let _child_width = parameters.pop_int()?;
    let _parent_height = parameters.pop_int()?;
    let _parent_width = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WScrollPanePeer.setSpans(IIII)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_hscrollbar_height() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_hscrollbar_height(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WScrollPanePeer._getHScrollbarHeight()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_vscrollbar_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_vscrollbar_width(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WScrollPanePeer._getVScrollbarWidth()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WScrollPanePeer.create(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_offset(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WScrollPanePeer.getOffset(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WScrollPanePeer.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_insets(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WScrollPanePeer.setInsets()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_scroll_position() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_scroll_position(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WScrollPanePeer.setScrollPosition(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_spans() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_spans(
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
            "sun/awt/windows/WScrollPanePeer.setSpans(IIII)V",
            result.unwrap_err().to_string()
        );
    }
}
