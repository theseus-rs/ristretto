use crate::native_methods::registry::{MethodRegistry, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/KQueue";

/// Register all native methods for `sun.nio.ch.KQueue`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "keventPoll", "(IJI)I", kevent_poll);
        registry.register(CLASS_NAME, "keventRegister", "(IIII)I", kevent_register);
        registry.register(CLASS_NAME, "kqueue", "()I", kqueue);
    } else {
        registry.register(CLASS_NAME, "create", "()I", create);
        registry.register(CLASS_NAME, "poll", "(IJIJ)I", poll);
        registry.register(CLASS_NAME, "register", "(IIII)I", register_0);
    }

    registry.register(CLASS_NAME, "filterOffset", "()I", filter_offset);
    registry.register(CLASS_NAME, "flagsOffset", "()I", flags_offset);
    registry.register(CLASS_NAME, "identOffset", "()I", ident_offset);
    registry.register(CLASS_NAME, "keventSize", "()I", kevent_size);
}

#[async_recursion(?Send)]
async fn create(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.create()I");
}

#[async_recursion(?Send)]
async fn filter_offset(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.filterOffset()I");
}

#[async_recursion(?Send)]
async fn flags_offset(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.flagsOffset()I");
}

#[async_recursion(?Send)]
async fn ident_offset(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.identOffset()I");
}

#[async_recursion(?Send)]
async fn kevent_poll(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventPoll(IJI)I");
}

#[async_recursion(?Send)]
async fn kevent_register(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventRegister(IIII)I");
}

#[async_recursion(?Send)]
async fn kevent_size(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventSize()I");
}

#[async_recursion(?Send)]
async fn kqueue(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.kqueue()I");
}

#[async_recursion(?Send)]
async fn poll(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.poll(IJIJ)I");
}

#[async_recursion(?Send)]
async fn register_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.register(IIII)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.create()I")]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.filterOffset()I")]
    async fn test_filter_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = filter_offset(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.flagsOffset()I")]
    async fn test_flags_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flags_offset(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.identOffset()I")]
    async fn test_ident_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ident_offset(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.keventPoll(IJI)I")]
    async fn test_kevent_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kevent_poll(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.keventRegister(IIII)I")]
    async fn test_kevent_register() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kevent_register(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.keventSize()I")]
    async fn test_kevent_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kevent_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.kqueue()I")]
    async fn test_kqueue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kqueue(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.poll(IJIJ)I")]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueue.register(IIII)I")]
    async fn test_register_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_0(thread, Parameters::default()).await;
    }
}
