use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CDropTarget";

/// Register all intrinsic methods for `sun.lwawt.macosx.CDropTarget`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "createNativeDropTarget",
            "(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;Ljava/awt/peer/ComponentPeer;J)J",
            create_native_drop_target,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "createNativeDropTarget",
            "(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J",
            create_native_drop_target,
        );
    }

    registry.register(
        CLASS_NAME,
        "releaseNativeDropTarget",
        "(J)V",
        release_native_drop_target,
    );
}

#[async_recursion(?Send)]
async fn create_native_drop_target(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J"
    )
}

#[async_recursion(?Send)]
async fn release_native_drop_target(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTarget.releaseNativeDropTarget(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J"
    )]
    async fn test_create_native_drop_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_drop_target(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDropTarget.releaseNativeDropTarget(J)V"
    )]
    async fn test_release_native_drop_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = release_native_drop_target(thread, Parameters::default()).await;
    }
}
