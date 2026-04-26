use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/fs/LinuxWatchService.configureBlocking(IZ)V", Any)]
#[async_method]
pub async fn configure_blocking<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blocking = parameters.pop_bool()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxWatchService.configureBlocking(IZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/fs/LinuxWatchService.eventOffsets()[I", Any)]
#[async_method]
pub async fn event_offsets<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxWatchService.eventOffsets()[I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/fs/LinuxWatchService.eventSize()I", Any)]
#[async_method]
pub async fn event_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/fs/LinuxWatchService.eventSize()I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/nio/fs/LinuxWatchService.inotifyAddWatch(IJI)I", Any)]
#[async_method]
pub async fn inotify_add_watch<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mask = parameters.pop_int()?;
    let _path_address = parameters.pop_long()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxWatchService.inotifyAddWatch(IJI)I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/fs/LinuxWatchService.inotifyInit()I", Any)]
#[async_method]
pub async fn inotify_init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/fs/LinuxWatchService.inotifyInit()I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/nio/fs/LinuxWatchService.inotifyRmWatch(II)V", Any)]
#[async_method]
pub async fn inotify_rm_watch<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _wd = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxWatchService.inotifyRmWatch(II)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/fs/LinuxWatchService.poll(II)I", Any)]
#[async_method]
pub async fn poll<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd2 = parameters.pop_int()?;
    let _fd1 = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/fs/LinuxWatchService.poll(II)I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/nio/fs/LinuxWatchService.socketpair([I)V", Any)]
#[async_method]
pub async fn socketpair<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sv = parameters.pop_reference()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/fs/LinuxWatchService.socketpair([I)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_configure_blocking() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = configure_blocking(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxWatchService.configureBlocking(IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_event_offsets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = event_offsets(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/LinuxWatchService.eventOffsets()[I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_event_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = event_size(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/LinuxWatchService.eventSize()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_inotify_add_watch() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inotify_add_watch(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxWatchService.inotifyAddWatch(IJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_inotify_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inotify_init(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/LinuxWatchService.inotifyInit()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_inotify_rm_watch() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            inotify_rm_watch(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/LinuxWatchService.inotifyRmWatch(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = poll(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/fs/LinuxWatchService.poll(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_socketpair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socketpair(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/nio/fs/LinuxWatchService.socketpair([I)V",
            result.unwrap_err().to_string()
        );
    }
}
