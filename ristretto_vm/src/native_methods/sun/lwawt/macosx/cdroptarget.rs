use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CDropTarget`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CDropTarget";
    registry.register(
        class_name,
        "createNativeDropTarget",
        "(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J",
        create_native_drop_target,
    );
    registry.register(
        class_name,
        "releaseNativeDropTarget",
        "(J)V",
        release_native_drop_target,
    );
}

#[async_recursion(?Send)]
async fn create_native_drop_target(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J")
}

#[async_recursion(?Send)]
async fn release_native_drop_target(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTarget.releaseNativeDropTarget(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CDropTarget";
        assert!(registry
            .method(
                class_name,
                "createNativeDropTarget",
                "(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J"
            )
            .is_some());
        assert!(registry
            .method(class_name, "releaseNativeDropTarget", "(J)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CDropTarget.createNativeDropTarget(Ljava/awt/dnd/DropTarget;Ljava/awt/Component;J)J"
    )]
    async fn test_create_native_drop_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_native_drop_target(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CDropTarget.releaseNativeDropTarget(J)V")]
    async fn test_release_native_drop_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = release_native_drop_target(thread, Arguments::default()).await;
    }
}
