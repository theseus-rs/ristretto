use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WDropTargetContextPeer.dropDone(JZI)V", Any)]
#[async_method]
pub async fn drop_done<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _actions = parameters.pop_int()?;
    let _success = parameters.pop_bool()?;
    let _drop_target = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDropTargetContextPeer.dropDone(JZI)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WDropTargetContextPeer.getData(JJ)Ljava/lang/Object;",
    Any
)]
#[async_method]
pub async fn get_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    let _drop_target = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDropTargetContextPeer.getData(JJ)Ljava/lang/Object;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_drop_done() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = drop_done(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WDropTargetContextPeer.dropDone(JZI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_data(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WDropTargetContextPeer.getData(JJ)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }
}
