use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;Ljava/awt/peer/ComponentPeer;J)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_native_drop_target_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg3 = parameters.pop_long()?;
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;Ljava/awt/peer/ComponentPeer;J)J".to_string()).into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn create_native_drop_target_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jnativepeer = parameters.pop_long()?;
    let _jcomponent = parameters.pop_reference()?;
    let _jdroptarget = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/CDropTarget.releaseNativeDropTarget(J)V", Any)]
#[async_method]
pub async fn release_native_drop_target<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _native_drop_target = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CDropTarget.releaseNativeDropTarget(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_native_drop_target_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_native_drop_target_0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;Ljava/awt/peer/ComponentPeer;J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_native_drop_target_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_drop_target_1(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_release_native_drop_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            release_native_drop_target(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CDropTarget.releaseNativeDropTarget(J)V",
            result.unwrap_err().to_string()
        );
    }
}
