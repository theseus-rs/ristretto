use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/Win32GraphicsConfig.getBounds(I)Ljava/awt/Rectangle;", Any)]
#[async_method]
pub async fn get_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsConfig.getBounds(I)Ljava/awt/Rectangle;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsConfig.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/Win32GraphicsConfig.initIDs()V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_bounds(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/Win32GraphicsConfig.getBounds(I)Ljava/awt/Rectangle;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/Win32GraphicsConfig.initIDs()V",
            result.unwrap_err().to_string()
        );
    }
}
