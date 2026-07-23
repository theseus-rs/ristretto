use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/KQueuePort.close0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn close_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let _ = super::posix::close_descriptor(&*vm, fd)?;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/KQueuePort.drain1(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn drain_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let fd = super::posix::raw_descriptor(&*vm, fd).await;
    let mut byte = [0_u8; 1];
    let _ = super::posix::read_descriptor(fd, &mut byte)?;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/KQueuePort.interrupt(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn interrupt<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let fd = super::posix::raw_descriptor(&*vm, fd).await;
    let _ = super::posix::write_descriptor(fd, &[1])?;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/KQueuePort.socketpair([I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn socketpair<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let pair = parameters.pop_reference()?;
    let Some(pair) = pair else {
        return Err(ristretto_types::Error::InternalError(
            "KQueuePort.socketpair: null array".to_string(),
        ));
    };
    let pair = Value::from(pair);
    if pair.as_int_vec_ref()?.len() < 2 {
        return Err(ristretto_types::Error::InternalError(
            "KQueuePort.socketpair: array too small".to_string(),
        ));
    }
    let mut descriptors = [0_i32; 2];
    #[expect(unsafe_code)]
    // SAFETY: descriptors points to two writable integers.
    let result = unsafe {
        libc::socketpair(
            libc::AF_UNIX,
            libc::SOCK_STREAM,
            0,
            descriptors.as_mut_ptr(),
        )
    };
    if result == -1 {
        return Err(super::posix::last_io_exception("socketpair"));
    }
    for fd in descriptors {
        #[expect(unsafe_code)]
        // SAFETY: fd was returned by socketpair and F_GETFD has no pointer argument.
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
        #[expect(unsafe_code)]
        // SAFETY: fd is live and the flags are valid for F_SETFD.
        unsafe {
            libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC)
        };
    }
    let vm = thread.vm()?;
    super::posix::register_descriptor(&*vm, descriptors[0])?;
    super::posix::register_descriptor(&*vm, descriptors[1])?;
    let mut values = pair.as_int_vec_mut()?;
    let [first, second, ..] = values.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "KQueuePort.socketpair: array too small".to_string(),
        ));
    };
    *first = descriptors[0];
    *second = descriptors[1];
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;
    use ristretto_types::VM;

    async fn socket_pair<T: Thread + 'static>(thread: Arc<T>) -> Result<(i32, i32)> {
        let vm = thread.vm()?;
        let pair = Value::new_object(vm.garbage_collector(), Reference::from(vec![0_i32, 0_i32]));
        let result = socketpair(thread, Parameters::new(vec![pair.clone()])).await?;
        assert_eq!(None, result);
        let values = pair.as_int_vec_ref()?;
        let [first, second, ..] = values.as_ref() else {
            return Err(ristretto_types::Error::InternalError(
                "KQueuePort test socket pair is incomplete".to_string(),
            ));
        };
        Ok((*first, *second))
    }

    async fn close<T: Thread + 'static>(thread: Arc<T>, fd: i32) -> Result<()> {
        let result = close_0(thread, Parameters::new(vec![Value::Int(fd)])).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_close_0() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (first, second) = socket_pair(thread.clone()).await?;
        close(thread.clone(), first).await?;
        close(thread, second).await
    }

    #[tokio::test]
    async fn test_drain_1() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (writer, reader) = socket_pair(thread.clone()).await?;
        let result = interrupt(thread.clone(), Parameters::new(vec![Value::Int(writer)])).await?;
        assert_eq!(None, result);
        let result = drain_1(thread.clone(), Parameters::new(vec![Value::Int(reader)])).await?;
        assert_eq!(None, result);
        close(thread.clone(), writer).await?;
        close(thread, reader).await
    }

    #[tokio::test]
    async fn test_interrupt() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (writer, reader) = socket_pair(thread.clone()).await?;
        let result = interrupt(thread.clone(), Parameters::new(vec![Value::Int(writer)])).await?;
        assert_eq!(None, result);
        let result = drain_1(thread.clone(), Parameters::new(vec![Value::Int(reader)])).await?;
        assert_eq!(None, result);
        close(thread.clone(), writer).await?;
        close(thread, reader).await
    }

    #[tokio::test]
    async fn test_socketpair() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (first, second) = socket_pair(thread.clone()).await?;
        assert!(first >= 0);
        assert!(second >= 0);
        assert_ne!(first, second);
        close(thread.clone(), first).await?;
        close(thread, second).await
    }
}
