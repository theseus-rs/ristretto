use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

use crate::sun::nio::fs::managed_files;

#[intrinsic_method(
    "sun/nio/ch/UnixDispatcher.close0(Ljava/io/FileDescriptor;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn close_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd_value = parameters.pop()?;
    let fd = {
        let guard = fd_value.as_reference()?;
        let Reference::Object(object) = &*guard else {
            return Ok(None);
        };
        object.value("fd")?.as_i32()?
    };
    let vm = thread.vm()?;
    managed_files::close(vm.nio_file_handles(), fd).await;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/UnixDispatcher.init()V", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/UnixDispatcher.preClose0(Ljava/io/FileDescriptor;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn pre_close_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_pre_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = pre_close_0(thread, Parameters::default()).await;
        assert!(result.is_ok());
    }
}
