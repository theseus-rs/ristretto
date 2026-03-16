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
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;Ljava/awt/peer/ComponentPeer;J)J".to_string()).into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn create_native_drop_target_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/CDropTarget.releaseNativeDropTarget(J)V", Any)]
#[async_method]
pub async fn release_native_drop_target<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_drop_target_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_native_drop_target_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_native_drop_target_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_release_native_drop_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = release_native_drop_target(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
