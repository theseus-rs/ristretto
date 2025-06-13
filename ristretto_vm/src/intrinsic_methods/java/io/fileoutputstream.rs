use crate::Error::InternalError;
use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::{Object, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/io/FileOutputStream.close0()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn close_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.FileOutputStream.close0()V")
}

#[intrinsic_method("java/io/FileOutputStream.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/io/FileOutputStream.open0(Ljava/lang/String;Z)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn open_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileOutputStream.open0(Ljava/lang/String;Z)V")
}

#[intrinsic_method("java/io/FileOutputStream.write(IZ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn write(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileOutputStream.write(IZ)V")
}

#[intrinsic_method("java/io/FileOutputStream.writeBytes([BIIZ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn write_bytes(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _append = parameters.pop_bool()?;
    let length = usize::try_from(parameters.pop_int()?)?;
    let offset = usize::try_from(parameters.pop_int()?)?;
    let bytes: Vec<u8> = parameters.pop()?.try_into()?;
    let file_output_stream = parameters.pop_object()?;
    let file_descriptor: Object = file_output_stream.value("fd")?.try_into()?;
    let handle = file_descriptor.value("handle")?.to_long()?;

    match handle {
        1 => {
            let vm = thread.vm()?;
            let configuration = vm.configuration();
            let stdout_lock = configuration.stdout();
            let mut stdout = stdout_lock.lock().await;
            stdout
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| InternalError(error.to_string()))?;
            stdout
                .flush()
                .map_err(|error| InternalError(error.to_string()))?;
        }
        2 => {
            let vm = thread.vm()?;
            let configuration = vm.configuration();
            let stderr_lock = configuration.stderr();
            let mut stderr = stderr_lock.lock().await;
            stderr
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| InternalError(error.to_string()))?;
            stderr
                .flush()
                .map_err(|error| InternalError(error.to_string()))?;
        }
        _ => {
            return Err(InternalError(format!("Invalid file handle: {handle}")));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileOutputStream.close0()V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.FileOutputStream.open0(Ljava/lang/String;Z)V"
    )]
    async fn test_open_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = open_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.io.FileOutputStream.write(IZ)V")]
    async fn test_write() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = write(thread, Parameters::default()).await;
    }
}
