use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/Signal";

/// Register all intrinsic methods for `sun.misc.Signal`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "findSignal",
        "(Ljava/lang/String;)I",
        find_signal,
    );
    registry.register(CLASS_NAME, "handle0", "(IJ)J", handle_0);
    registry.register(CLASS_NAME, "raise0", "(I)V", raise_0);
}

#[async_recursion(?Send)]
async fn find_signal(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Signal.findSignal(Ljava/lang/String;)I")
}

#[async_recursion(?Send)]
async fn handle_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Signal.handle0(IJ)J")
}

#[async_recursion(?Send)]
async fn raise_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.misc.Signal.raise0(I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.Signal.findSignal(Ljava/lang/String;)I"
    )]
    async fn test_find_signal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_signal(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Signal.handle0(IJ)J")]
    async fn test_handle_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = handle_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.misc.Signal.raise0(I)V")]
    async fn test_raise_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = raise_0(thread, Parameters::default()).await;
    }
}
