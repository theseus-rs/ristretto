use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.epollCreate()I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/ch/EPollArrayWrapper.epollCreate()I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.epollCtl(IIII)V", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_ctl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _events = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    let _opcode = parameters.pop_int()?;
    let _epfd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/ch/EPollArrayWrapper.epollCtl(IIII)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.epollWait(JIJI)I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_wait<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _epfd = parameters.pop_int()?;
    let _timeout = parameters.pop_long()?;
    let _numfds = parameters.pop_int()?;
    let _poll_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/EPollArrayWrapper.epollWait(JIJI)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.init()V", Equal(JAVA_8))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPollArrayWrapper.init()V".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.interrupt(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn interrupt<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/ch/EPollArrayWrapper.interrupt(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.offsetofData()I", Equal(JAVA_8))]
#[async_method]
pub async fn offsetof_data<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/ch/EPollArrayWrapper.offsetofData()I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.sizeofEPollEvent()I", Equal(JAVA_8))]
#[async_method]
pub async fn sizeof_epoll_event<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/EPollArrayWrapper.sizeofEPollEvent()I".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_epoll_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = epoll_create(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EPollArrayWrapper.epollCreate()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_epoll_ctl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = epoll_ctl(
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
            "sun/nio/ch/EPollArrayWrapper.epollCtl(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_epoll_wait() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = epoll_wait(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/EPollArrayWrapper.epollWait(JIJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EPollArrayWrapper.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = interrupt(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/EPollArrayWrapper.interrupt(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_offsetof_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = offsetof_data(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EPollArrayWrapper.offsetofData()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_sizeof_epoll_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = sizeof_epoll_event(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EPollArrayWrapper.sizeofEPollEvent()I",
            result.unwrap_err().to_string()
        );
    }
}
