use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/KQueueArrayWrapper";

/// Register all native methods for `sun.nio.ch.KQueueArrayWrapper`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "init", "()I", init);
    registry.register(CLASS_NAME, "initStructSizes", "()V", init_struct_sizes);
    registry.register(CLASS_NAME, "interrupt", "(I)V", interrupt);
    registry.register(CLASS_NAME, "kevent0", "(IJIJ)I", kevent_0);
    registry.register(CLASS_NAME, "register0", "(IIII)V", register_0);
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.init()I");
}

#[async_recursion(?Send)]
async fn init_struct_sizes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.initStructSizes()V");
}

#[async_recursion(?Send)]
async fn interrupt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.interrupt(I)V");
}

#[async_recursion(?Send)]
async fn kevent_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.kevent0(IJIJ)I");
}

#[async_recursion(?Send)]
async fn register_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.register0(IIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.init()I")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.initStructSizes()V"
    )]
    async fn test_init_struct_sizes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_struct_sizes(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.interrupt(I)V")]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = interrupt(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.kevent0(IJIJ)I")]
    async fn test_kevent_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kevent_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.register0(IIII)V"
    )]
    async fn test_register_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_0(thread, Arguments::default()).await;
    }
}
