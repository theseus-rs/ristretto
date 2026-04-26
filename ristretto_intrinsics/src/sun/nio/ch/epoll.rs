use ristretto_classfile::VersionSpecification::{Any, Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/EPoll.create()I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPoll.create()I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPoll.ctl(IIII)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn ctl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _events = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    let _opcode = parameters.pop_int()?;
    let _epfd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPoll.ctl(IIII)I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPoll.dataOffset()I", Any)]
#[async_method]
pub async fn data_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPoll.dataOffset()I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPoll.epollCreate()I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPoll.epollCreate()I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPoll.epollCtl(IIII)I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_ctl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _events = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    let _opcode = parameters.pop_int()?;
    let _epfd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPoll.epollCtl(IIII)I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPoll.epollWait(IJI)I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_wait<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _numfds = parameters.pop_int()?;
    let _poll_address = parameters.pop_long()?;
    let _epfd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPoll.epollWait(IJI)I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPoll.eventSize()I", Any)]
#[async_method]
pub async fn event_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPoll.eventSize()I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPoll.eventsOffset()I", Any)]
#[async_method]
pub async fn events_offset<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPoll.eventsOffset()I".to_string()).into())
}
#[intrinsic_method("sun/nio/ch/EPoll.wait(IJII)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn wait<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_int()?;
    let _numfds = parameters.pop_int()?;
    let _address = parameters.pop_long()?;
    let _epfd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/EPoll.wait(IJII)I".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EPoll.create()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_ctl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = ctl(
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
            "sun/nio/ch/EPoll.ctl(IIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_data_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = data_offset(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EPoll.dataOffset()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_epoll_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = epoll_create(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EPoll.epollCreate()I",
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
            "sun/nio/ch/EPoll.epollCtl(IIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_epoll_wait() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = epoll_wait(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/EPoll.epollWait(IJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_event_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = event_size(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EPoll.eventSize()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_events_offset() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = events_offset(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/EPoll.eventsOffset()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_wait() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = wait(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/EPoll.wait(IJII)I",
            result.unwrap_err().to_string()
        );
    }
}
