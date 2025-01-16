use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/PollArrayWrapper";

/// Register all native methods for `sun.nio.ch.PollArrayWrapper`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "interrupt", "(I)V", interrupt);
    registry.register(CLASS_NAME, "poll0", "(JIJ)I", poll_0);
}

#[async_recursion(?Send)]
async fn interrupt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.PollArrayWrapper.interrupt(I)V")
}

#[async_recursion(?Send)]
async fn poll_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.PollArrayWrapper.poll0(JIJ)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.PollArrayWrapper.interrupt(I)V")]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = interrupt(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.PollArrayWrapper.poll0(JIJ)I")]
    async fn test_poll_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll_0(thread, Arguments::default()).await;
    }
}
