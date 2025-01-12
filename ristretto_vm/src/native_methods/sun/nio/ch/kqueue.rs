use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.nio.ch.KQueue`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/KQueue";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "keventPoll", "(IJI)I", kevent_poll);
        registry.register(class_name, "keventRegister", "(IIII)I", kevent_register);
        registry.register(class_name, "kqueue", "()I", kqueue);
    } else {
        registry.register(class_name, "create", "()I", create);
        registry.register(class_name, "poll", "(IJIJ)I", poll);
        registry.register(class_name, "register", "(IIII)I", register_0);
    }

    registry.register(class_name, "filterOffset", "()I", filter_offset);
    registry.register(class_name, "flagsOffset", "()I", flags_offset);
    registry.register(class_name, "identOffset", "()I", ident_offset);
    registry.register(class_name, "keventSize", "()I", kevent_size);
}

#[async_recursion(?Send)]
async fn create(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.create()I");
}

#[async_recursion(?Send)]
async fn filter_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.filterOffset()I");
}

#[async_recursion(?Send)]
async fn flags_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.flagsOffset()I");
}

#[async_recursion(?Send)]
async fn ident_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.identOffset()I");
}

#[async_recursion(?Send)]
async fn kevent_poll(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventPoll(IJI)I");
}

#[async_recursion(?Send)]
async fn kevent_register(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventRegister(IIII)I");
}

#[async_recursion(?Send)]
async fn kevent_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventSize()I");
}

#[async_recursion(?Send)]
async fn kqueue(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.kqueue()I");
}

#[async_recursion(?Send)]
async fn poll(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.poll(IJIJ)I");
}

#[async_recursion(?Send)]
async fn register_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.register(IIII)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java9 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/nio/ch/KQueue";
        assert!(registry.method(class_name, "create", "()I").is_some());
        assert!(registry.method(class_name, "poll", "(IJIJ)I").is_some());
        assert!(registry.method(class_name, "register", "(IIII)I").is_some());
        assert!(registry.method(class_name, "filterOffset", "()I").is_some());
        assert!(registry.method(class_name, "flagsOffset", "()I").is_some());
        assert!(registry.method(class_name, "identOffset", "()I").is_some());
        assert!(registry.method(class_name, "keventSize", "()I").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.create()I")]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.filterOffset()I")]
    async fn test_filter_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = filter_offset(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.flagsOffset()I")]
    async fn test_flags_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flags_offset(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.identOffset()I")]
    async fn test_ident_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ident_offset(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.keventPoll(IJI)I")]
    async fn test_kevent_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kevent_poll(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.keventRegister(IIII)I")]
    async fn test_kevent_register() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kevent_register(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.keventSize()I")]
    async fn test_kevent_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kevent_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.kqueue()I")]
    async fn test_kqueue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kqueue(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.poll(IJIJ)I")]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.KQueue.register(IIII)I")]
    async fn test_register_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_0(thread, Arguments::default()).await;
    }
}
