use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CDropTargetContextPeer.addTransfer(JJJ)V", Any)]
#[async_method]
pub async fn add_transfer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    let _native_drop_transfer = parameters.pop_long()?;
    let _native_drop_target = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDropTargetContextPeer.addTransfer(JJJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CDropTargetContextPeer.dropDone(JJZZI)V", Any)]
#[async_method]
pub async fn drop_done<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _drop_action = parameters.pop_int()?;
    let _success = parameters.pop_bool()?;
    let _is_local = parameters.pop_bool()?;
    let _native_drop_transfer = parameters.pop_long()?;
    let _native_drop_target = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDropTargetContextPeer.dropDone(JJZZI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CDropTargetContextPeer.startTransfer(JJ)J", Any)]
#[async_method]
pub async fn start_transfer<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    let _native_drop_target = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDropTargetContextPeer.startTransfer(JJ)J".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_transfer(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CDropTargetContextPeer.addTransfer(JJJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_drop_done() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = drop_done(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::from(false),
                Value::from(false),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CDropTargetContextPeer.dropDone(JJZZI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_start_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = start_transfer(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CDropTargetContextPeer.startTransfer(JJ)J",
            result.unwrap_err().to_string()
        );
    }
}
