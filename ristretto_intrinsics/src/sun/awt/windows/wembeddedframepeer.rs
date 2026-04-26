use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WEmbeddedFramePeer.create(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WEmbeddedFramePeer.create(Lsun/awt/windows/WComponentPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WEmbeddedFramePeer.getBoundsPrivate()Ljava/awt/Rectangle;",
    Any
)]
#[async_method]
pub async fn get_bounds_private<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WEmbeddedFramePeer.getBoundsPrivate()Ljava/awt/Rectangle;".to_string(),
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
            "sun/awt/windows/WEmbeddedFramePeer.create(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_bounds_private() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_bounds_private(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WEmbeddedFramePeer.getBoundsPrivate()Ljava/awt/Rectangle;",
            result.unwrap_err().to_string()
        );
    }
}
