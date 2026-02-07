use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/KQueueArrayWrapper.init()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.init()I");
}

#[intrinsic_method(
    "sun/nio/ch/KQueueArrayWrapper.initStructSizes()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_struct_sizes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.initStructSizes()V");
}

#[intrinsic_method("sun/nio/ch/KQueueArrayWrapper.interrupt(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn interrupt<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.interrupt(I)V");
}

#[intrinsic_method(
    "sun/nio/ch/KQueueArrayWrapper.kevent0(IJIJ)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn kevent_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.kevent0(IJIJ)I");
}

#[intrinsic_method(
    "sun/nio/ch/KQueueArrayWrapper.register0(IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn register_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.register0(IIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.init()I")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.initStructSizes()V"
    )]
    async fn test_init_struct_sizes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_struct_sizes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.interrupt(I)V")]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = interrupt(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.kevent0(IJIJ)I")]
    async fn test_kevent_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = kevent_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.KQueueArrayWrapper.register0(IIII)V"
    )]
    async fn test_register_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = register_0(thread, Parameters::default()).await;
    }
}
