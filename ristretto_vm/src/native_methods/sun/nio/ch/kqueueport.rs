use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.KQueuePort`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/KQueuePort";
    registry.register(class_name, "close0", "(I)V", close_0);
    registry.register(class_name, "drain1", "(I)V", drain_1);
    registry.register(class_name, "interrupt", "(I)V", interrupt);
    registry.register(class_name, "socketpair", "([I)V", socketpair);
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.close0(I)V");
}

#[async_recursion(?Send)]
async fn drain_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.drain1(I)V");
}

#[async_recursion(?Send)]
async fn interrupt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.interrupt(I)V");
}

#[async_recursion(?Send)]
async fn socketpair(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueuePort.socketpair([I)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/ch/KQueuePort";
        assert!(registry.method(class_name, "close0", "(I)V").is_some());
        assert!(registry.method(class_name, "drain1", "(I)V").is_some());
        assert!(registry.method(class_name, "interrupt", "(I)V").is_some());
        assert!(registry.method(class_name, "socketpair", "([I)V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueuePort.close0(I)V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueuePort.drain1(I)V")]
    async fn test_drain_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = drain_1(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueuePort.interrupt(I)V")]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = interrupt(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueuePort.socketpair([I)V")]
    async fn test_socketpair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socketpair(thread, Arguments::default()).await;
    }
}
