use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/KQueueArrayWrapper.init()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueueArrayWrapper.init()I".to_string()).into())
}

#[intrinsic_method(
    "sun/nio/ch/KQueueArrayWrapper.initStructSizes()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_struct_sizes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.KQueueArrayWrapper.initStructSizes()V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/nio/ch/KQueueArrayWrapper.interrupt(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn interrupt<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueueArrayWrapper.interrupt(I)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/nio/ch/KQueueArrayWrapper.kevent0(IJIJ)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn kevent_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg3 = parameters.pop_long()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_long()?;
    let _arg0 = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueueArrayWrapper.kevent0(IJIJ)I".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/nio/ch/KQueueArrayWrapper.register0(IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn register_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg3 = parameters.pop_int()?;
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_int()?;
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.nio.ch.KQueueArrayWrapper.register0(IIII)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.KQueueArrayWrapper.init()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_struct_sizes() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init_struct_sizes(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.KQueueArrayWrapper.initStructSizes()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = interrupt(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.nio.ch.KQueueArrayWrapper.interrupt(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_kevent_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = kevent_0(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.KQueueArrayWrapper.kevent0(IJIJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_register_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = register_0(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.KQueueArrayWrapper.register0(IIII)V",
            result.unwrap_err().to_string()
        );
    }
}
