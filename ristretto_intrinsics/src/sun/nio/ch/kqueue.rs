use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/KQueue.create()I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.create()I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueue.filterOffset()I", Any)]
#[async_method]
pub async fn filter_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.filterOffset()I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueue.flagsOffset()I", Any)]
#[async_method]
pub async fn flags_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.flagsOffset()I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueue.identOffset()I", Any)]
#[async_method]
pub async fn ident_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.identOffset()I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueue.keventPoll(IJI)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn kevent_poll<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _nevents = parameters.pop_int()?;
    let _poll_address = parameters.pop_long()?;
    let _kqpfd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.keventPoll(IJI)I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueue.keventRegister(IIII)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn kevent_register<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_int()?;
    let _filter = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    let _kqpfd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.keventRegister(IIII)I".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/nio/ch/KQueue.keventSize()I", Any)]
#[async_method]
pub async fn kevent_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.keventSize()I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueue.kqueue()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn kqueue<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.kqueue()I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueue.poll(IJIJ)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn poll<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_long()?;
    let _nevents = parameters.pop_int()?;
    let _poll_address = parameters.pop_long()?;
    let _kqfd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.poll(IJIJ)I".to_string()).into())
}

#[intrinsic_method("sun/nio/ch/KQueue.register(IIII)I", GreaterThan(JAVA_8))]
#[async_method]
pub async fn register_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_int()?;
    let _filter = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    let _kqfd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun.nio.ch.KQueue.register(IIII)I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.KQueue.create()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_filter_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = filter_offset(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.KQueue.filterOffset()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_flags_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flags_offset(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.KQueue.flagsOffset()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ident_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ident_offset(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.KQueue.identOffset()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_kevent_poll() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = kevent_poll(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.nio.ch.KQueue.keventPoll(IJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_kevent_register() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = kevent_register(
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
            "sun.nio.ch.KQueue.keventRegister(IIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_kevent_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = kevent_size(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.KQueue.keventSize()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_kqueue() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = kqueue(thread, Parameters::default()).await;
        assert_eq!(
            "sun.nio.ch.KQueue.kqueue()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = poll(
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
            "sun.nio.ch.KQueue.poll(IJIJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_register_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
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
            "sun.nio.ch.KQueue.register(IIII)I",
            result.unwrap_err().to_string()
        );
    }
}
