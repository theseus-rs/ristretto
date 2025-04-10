use crate::Error::InternalError;
use crate::Result;
use crate::native_methods::registry::{JAVA_8, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::{Object, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "java/io/FileOutputStream";

/// Register all native methods for `java.io.FileOutputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(CLASS_NAME, "close0", "()V", close_0);
    }

    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(CLASS_NAME, "open0", "(Ljava/lang/String;Z)V", open_0);
    registry.register(CLASS_NAME, "write", "(IZ)V", write);
    registry.register(CLASS_NAME, "writeBytes", "([BIIZ)V", write_bytes);
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileOutputStream.close0()V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileOutputStream.open0(Ljava/lang/String;Z)V")
}

#[async_recursion(?Send)]
async fn write(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.io.FileOutputStream.write(IZ)V")
}

#[async_recursion(?Send)]
async fn write_bytes(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
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
