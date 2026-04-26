use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WLabelPeer.create(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WLabelPeer.create(Lsun/awt/windows/WComponentPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WLabelPeer.lazyPaint()V", Any)]
#[async_method]
pub async fn lazy_paint<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WLabelPeer.lazyPaint()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WLabelPeer.setAlignment(I)V", Any)]
#[async_method]
pub async fn set_alignment<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _alignment = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WLabelPeer.setAlignment(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WLabelPeer.setText(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn set_text<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _text = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WLabelPeer.setText(Ljava/lang/String;)V".to_string(),
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
            "sun/awt/windows/WLabelPeer.create(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lazy_paint() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lazy_paint(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WLabelPeer.lazyPaint()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_alignment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_alignment(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WLabelPeer.setAlignment(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_text() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_text(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WLabelPeer.setText(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }
}
