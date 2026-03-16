use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/DebugSettings.setCTracingOn(Z)V", Any)]
#[async_method]
pub async fn set_c_tracing_on_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.DebugSettings.setCTracingOn(Z)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/awt/DebugSettings.setCTracingOn(ZLjava/lang/String;)V", Any)]
#[async_method]
pub async fn set_c_tracing_on_2<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/DebugSettings.setCTracingOn(ZLjava/lang/String;I)V", Any)]
#[async_method]
pub async fn set_c_tracing_on_3<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;I)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_c_tracing_on_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_c_tracing_on_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_c_tracing_on_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_c_tracing_on_2(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_c_tracing_on_3() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_c_tracing_on_3(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
