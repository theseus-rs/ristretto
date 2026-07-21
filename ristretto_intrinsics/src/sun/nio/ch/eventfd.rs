use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/EventFD.eventfd0()I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn eventfd0<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[expect(unsafe_code)]
    // SAFETY: eventfd has no pointer arguments.
    let fd = unsafe { libc::eventfd(0, libc::EFD_CLOEXEC) };
    if fd == -1 {
        return Err(super::posix::last_io_exception("eventfd"));
    }
    let vm = thread.vm()?;
    super::posix::register_descriptor(&*vm, fd)?;
    Ok(Some(Value::Int(fd)))
}

#[intrinsic_method("sun/nio/ch/EventFD.set0(I)I", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn set0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let efd = parameters.pop_int()?;
    let result = super::posix::write_descriptor(efd, &1_u64.to_ne_bytes())?;
    Ok(Some(Value::Int(result)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_eventfd0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = eventfd0(thread, Parameters::default())
            .await
            .expect("eventfd");
        assert!(matches!(result, Some(Value::Int(fd)) if fd >= 0));
    }

    #[tokio::test]
    async fn test_set0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let Some(Value::Int(fd)) = eventfd0(thread.clone(), Parameters::default())
            .await
            .expect("eventfd")
        else {
            panic!("expected descriptor");
        };
        let result = set0(thread, Parameters::new(vec![Value::Int(fd)]))
            .await
            .expect("set");
        assert_eq!(Some(Value::Int(8)), result);
    }
}
