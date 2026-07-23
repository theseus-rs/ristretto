use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.epollCreate()I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_create<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    Ok(Some(Value::Int(super::epoll::create_epoll(&*vm)?)))
}

#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.epollCtl(IIII)V", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_ctl<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let events = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    let opcode = parameters.pop_int()?;
    let epfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let errno = super::epoll::control(&*vm, epfd, opcode, fd, events).await;
    if errno != 0 {
        return Err(super::posix::io_exception(
            "epoll_ctl",
            &std::io::Error::from_raw_os_error(errno),
        ));
    }
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.epollWait(JIJI)I", Equal(JAVA_8))]
#[async_method]
pub async fn epoll_wait<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let epfd = parameters.pop_int()?;
    let timeout = parameters.pop_long()?;
    let num_fds = parameters.pop_int()?;
    let poll_address = parameters.pop_long()?;
    let timeout = if timeout < -1 {
        -1
    } else {
        i32::try_from(timeout).unwrap_or(i32::MAX)
    };
    let result =
        super::epoll::wait_for_events(&thread, epfd, poll_address, num_fds, timeout).await?;
    Ok(Some(Value::Int(result)))
}

#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.init()V", Equal(JAVA_8))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.interrupt(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn interrupt<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let raw_fd = super::posix::raw_descriptor(&*vm, fd).await;
    let _ = super::posix::write_descriptor(raw_fd, &[1])?;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.offsetofData()I", Equal(JAVA_8))]
#[async_method]
pub async fn offsetof_data<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(4)))
}

#[intrinsic_method("sun/nio/ch/EPollArrayWrapper.sizeofEPollEvent()I", Equal(JAVA_8))]
#[async_method]
pub async fn sizeof_epoll_event<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(12)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_epoll_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = epoll_create(thread, Parameters::default())
            .await
            .expect("create");
        assert!(matches!(result, Some(Value::Int(fd)) if fd >= 0));
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
        assert!(result.is_err());
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
        assert_eq!(Some(Value::Int(0)), result.expect("wait"));
    }

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await.expect("init");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = interrupt(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_offsetof_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = offsetof_data(thread, Parameters::default())
            .await
            .expect("offset");
        assert_eq!(Some(Value::Int(4)), result);
    }

    #[tokio::test]
    async fn test_sizeof_epoll_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = sizeof_epoll_event(thread, Parameters::default())
            .await
            .expect("size");
        assert_eq!(Some(Value::Int(12)), result);
    }
}
