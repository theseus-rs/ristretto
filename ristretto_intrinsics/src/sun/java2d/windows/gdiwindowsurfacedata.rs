use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/windows/GDIWindowSurfaceData.initIDs(Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _xor_comp = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/windows/GDIWindowSurfaceData.initIDs(Ljava/lang/Class;)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/java2d/windows/GDIWindowSurfaceData.initOps(Lsun/awt/windows/WComponentPeer;IIIII)V",
    Any
)]
#[async_method]
pub async fn init_ops<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    let _blue_mask = parameters.pop_int()?;
    let _green_mask = parameters.pop_int()?;
    let _red_mask = parameters.pop_int()?;
    let _depth = parameters.pop_int()?;
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/windows/GDIWindowSurfaceData.initOps(Lsun/awt/windows/WComponentPeer;IIIII)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/java2d/windows/GDIWindowSurfaceData.invalidateSD()V", Any)]
#[async_method]
pub async fn invalidate_sd<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/windows/GDIWindowSurfaceData.invalidateSD()V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/java2d/windows/GDIWindowSurfaceData.initIDs(Ljava/lang/Class;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ops(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/windows/GDIWindowSurfaceData.initOps(Lsun/awt/windows/WComponentPeer;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_invalidate_sd() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invalidate_sd(thread, Parameters::default()).await;
        assert_eq!(
            "sun/java2d/windows/GDIWindowSurfaceData.invalidateSD()V",
            result.unwrap_err().to_string()
        );
    }
}
