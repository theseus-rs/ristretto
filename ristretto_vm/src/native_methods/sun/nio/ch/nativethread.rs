use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_18};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/NativeThread";

/// Register all native methods for `sun.nio.ch.NativeThread`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_18 {
        registry.register(CLASS_NAME, "current", "()J", current);
        registry.register(CLASS_NAME, "signal", "(J)V", signal);
    } else {
        registry.register(CLASS_NAME, "current0", "()J", current_0);
        registry.register(CLASS_NAME, "signal0", "(J)V", signal_0);
    }

    registry.register(CLASS_NAME, "init", "()V", init);
}

#[async_recursion(?Send)]
async fn current(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeThread.current()J");
}

#[async_recursion(?Send)]
async fn current_0(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    current(thread, arguments).await
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn signal(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeThread.signal(J)V");
}

#[async_recursion(?Send)]
async fn signal_0(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    signal(thread, arguments).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeThread.current()J")]
    async fn test_current() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeThread.current()J")]
    async fn test_current_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeThread.signal(J)V")]
    async fn test_signal() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = signal(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.NativeThread.signal(J)V")]
    async fn test_signal_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = signal_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
