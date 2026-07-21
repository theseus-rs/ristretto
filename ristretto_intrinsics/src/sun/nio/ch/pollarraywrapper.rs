use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/PollArrayWrapper.interrupt(I)V", LessThanOrEqual(JAVA_8))]
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

#[intrinsic_method("sun/nio/ch/PollArrayWrapper.poll0(JIJ)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn poll_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_long()?;
    let num_fds = parameters.pop_int()?;
    let poll_address = parameters.pop_long()?;
    let result = super::posix::poll(&thread, poll_address, num_fds, timeout).await?;
    Ok(Some(Value::Int(result)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_interrupt() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = interrupt(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_poll_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = poll_0(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(Some(Value::Int(0)), result.expect("poll"));
    }
}
