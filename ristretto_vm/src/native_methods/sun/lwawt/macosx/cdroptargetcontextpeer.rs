use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CDropTargetContextPeer";

/// Register all native methods for `sun.lwawt.macosx.CDropTargetContextPeer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "addTransfer", "(JJJ)V", add_transfer);
    registry.register(CLASS_NAME, "dropDone", "(JJZZI)V", drop_done);
    registry.register(CLASS_NAME, "startTransfer", "(JJ)J", start_transfer);
}

#[async_recursion(?Send)]
async fn add_transfer(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTargetContextPeer.addTransfer(JJJ)V")
}

#[async_recursion(?Send)]
async fn drop_done(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTargetContextPeer.dropDone(JJZZI)V")
}

#[async_recursion(?Send)]
async fn start_transfer(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTargetContextPeer.startTransfer(JJ)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDropTargetContextPeer.addTransfer(JJJ)V"
    )]
    async fn test_add_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_transfer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDropTargetContextPeer.dropDone(JJZZI)V"
    )]
    async fn test_drop_done() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = drop_done(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CDropTargetContextPeer.startTransfer(JJ)J"
    )]
    async fn test_start_transfer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = start_transfer(thread, Parameters::default()).await;
    }
}
