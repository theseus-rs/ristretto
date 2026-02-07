use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/KQueue.create()I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn create<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.create()I");
}

#[intrinsic_method("sun/nio/ch/KQueue.filterOffset()I", Any)]
#[async_method]
pub async fn filter_offset<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.filterOffset()I");
}

#[intrinsic_method("sun/nio/ch/KQueue.flagsOffset()I", Any)]
#[async_method]
pub async fn flags_offset<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.flagsOffset()I");
}

#[intrinsic_method("sun/nio/ch/KQueue.identOffset()I", Any)]
#[async_method]
pub async fn ident_offset<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.identOffset()I");
}

#[intrinsic_method("sun/nio/ch/KQueue.keventPoll(IJI)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn kevent_poll<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventPoll(IJI)I");
}

#[intrinsic_method("sun/nio/ch/KQueue.keventRegister(IIII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn kevent_register<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventRegister(IIII)I");
}

#[intrinsic_method("sun/nio/ch/KQueue.keventSize()I", Any)]
#[async_method]
pub async fn kevent_size<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventSize()I");
}

#[intrinsic_method("sun/nio/ch/KQueue.kqueue()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn kqueue<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.kqueue()I");
}

#[intrinsic_method("sun/nio/ch/KQueue.poll(IJIJ)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn poll<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.poll(IJIJ)I");
}

#[intrinsic_method("sun/nio/ch/KQueue.register(IIII)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn register_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
