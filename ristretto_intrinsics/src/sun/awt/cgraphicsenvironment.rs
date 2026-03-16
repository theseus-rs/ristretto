use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/CGraphicsEnvironment.deregisterDisplayReconfiguration(J)V",
    Any
)]
#[async_method]
pub async fn deregister_display_reconfiguration<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsEnvironment.deregisterDisplayReconfiguration(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/CGraphicsEnvironment.getDisplayIDs()[I", Any)]
#[async_method]
pub async fn get_display_i_ds<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsEnvironment.getDisplayIDs()[I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/CGraphicsEnvironment.getMainDisplayID()I", Any)]
#[async_method]
pub async fn get_main_display_id<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsEnvironment.getMainDisplayID()I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/CGraphicsEnvironment.initCocoa()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_cocoa<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.CGraphicsEnvironment.initCocoa()V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/awt/CGraphicsEnvironment.registerDisplayReconfiguration()J", Any)]
#[async_method]
pub async fn register_display_reconfiguration<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.CGraphicsEnvironment.registerDisplayReconfiguration()J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_deregister_display_reconfiguration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = deregister_display_reconfiguration(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_display_i_ds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_display_i_ds(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_main_display_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_main_display_id(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_cocoa() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_cocoa(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_display_reconfiguration() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = register_display_reconfiguration(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
