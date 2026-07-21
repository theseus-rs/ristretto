use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/EPollPort.close0(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn close0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let _ = super::posix::close_descriptor(&*vm, fd)?;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/EPollPort.drain1(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn drain1<T: Thread + 'static>(
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

#[intrinsic_method("sun/nio/ch/EPollPort.interrupt(I)V", Equal(JAVA_8))]
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

#[intrinsic_method("sun/nio/ch/EPollPort.socketpair([I)V", Equal(JAVA_8))]
#[async_method]
pub async fn socketpair<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let socket_pair = parameters.pop_reference()?;
    let Some(socket_pair) = socket_pair else {
        return Err(ristretto_types::Error::InternalError(
            "EPollPort.socketpair: null array".to_string(),
        ));
    };
    let socket_pair = Value::from(socket_pair);
    if socket_pair.as_int_vec_ref()?.len() < 2 {
        return Err(ristretto_types::Error::InternalError(
            "EPollPort.socketpair: array too small".to_string(),
        ));
    }
    let mut descriptors = [0_i32; 2];
    #[expect(unsafe_code)]
    // SAFETY: descriptors points to two writable ints as required by socketpair.
    let result = unsafe {
        libc::socketpair(
            libc::AF_UNIX,
            libc::SOCK_STREAM | libc::SOCK_CLOEXEC,
            0,
            descriptors.as_mut_ptr(),
        )
    };
    if result == -1 {
        return Err(super::posix::last_io_exception("socketpair"));
    }
    let vm = thread.vm()?;
    super::posix::register_descriptor(&*vm, descriptors[0])?;
    super::posix::register_descriptor(&*vm, descriptors[1])?;
    let mut values = socket_pair.as_int_vec_mut()?;
    let [first, second, ..] = values.as_mut() else {
        return Err(ristretto_types::Error::InternalError(
            "EPollPort.socketpair: array too small".to_string(),
        ));
    };
    *first = descriptors[0];
    *second = descriptors[1];
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(None, result.expect("close"));
    }

    #[tokio::test]
    async fn test_drain1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = drain1(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = interrupt(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_socketpair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socketpair(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert!(result.is_err());
    }
}
