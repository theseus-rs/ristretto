use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CDropTargetContextPeer.addTransfer(JJJ)V", Any)]
#[async_method]
pub(crate) async fn add_transfer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTargetContextPeer.addTransfer(JJJ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CDropTargetContextPeer.dropDone(JJZZI)V", Any)]
#[async_method]
pub(crate) async fn drop_done(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CDropTargetContextPeer.dropDone(JJZZI)V")
}

#[intrinsic_method("sun/lwawt/macosx/CDropTargetContextPeer.startTransfer(JJ)J", Any)]
#[async_method]
pub(crate) async fn start_transfer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
