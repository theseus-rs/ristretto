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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enabled = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.DebugSettings.setCTracingOn(Z)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/awt/DebugSettings.setCTracingOn(ZLjava/lang/String;)V", Any)]
#[async_method]
pub async fn set_c_tracing_on_2<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _file = parameters.pop_reference()?;
    let _enabled = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/DebugSettings.setCTracingOn(ZLjava/lang/String;I)V", Any)]
#[async_method]
pub async fn set_c_tracing_on_3<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _line = parameters.pop_int()?;
    let _file = parameters.pop_reference()?;
    let _enabled = parameters.pop_bool()?;
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
        let result = set_c_tracing_on_1(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun.awt.DebugSettings.setCTracingOn(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_c_tracing_on_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_c_tracing_on_2(
            thread,
            Parameters::new(vec![Value::from(false), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_c_tracing_on_3() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_c_tracing_on_3(
            thread,
            Parameters::new(vec![Value::from(false), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.awt.DebugSettings.setCTracingOn(ZLjava/lang/String;I)V",
            result.unwrap_err().to_string()
        );
    }
}
