use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CDropTargetContextPeer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CDropTargetContextPeer";
    registry.register(class_name, "addTransfer", "(JJJ)V", add_transfer);
    registry.register(class_name, "dropDone", "(JJZZI)V", drop_done);
    registry.register(class_name, "startTransfer", "(JJ)J", start_transfer);
}

#[async_recursion(?Send)]
async fn add_transfer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTargetContextPeer.addTransfer(JJJ)V")
}

#[async_recursion(?Send)]
async fn drop_done(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTargetContextPeer.dropDone(JJZZI)V")
}

#[async_recursion(?Send)]
async fn start_transfer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTargetContextPeer.startTransfer(JJ)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CDropTargetContextPeer";
        assert!(registry
            .method(class_name, "addTransfer", "(JJJ)V")
            .is_some());
        assert!(registry
            .method(class_name, "dropDone", "(JJZZI)V")
            .is_some());
        assert!(registry
            .method(class_name, "startTransfer", "(JJ)J")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CDropTargetContextPeer.addTransfer(JJJ)V")]
    async fn test_add_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_transfer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CDropTargetContextPeer.dropDone(JJZZI)V")]
    async fn test_drop_done() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = drop_done(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CDropTargetContextPeer.startTransfer(JJ)J")]
    async fn test_start_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = start_transfer(thread, Arguments::default()).await;
    }
}
