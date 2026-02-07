use crate::java::io::fileoutputstream::file_handle_identifier;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use tokio::io::AsyncWriteExt;

#[intrinsic_method(
    "java/io/FileCleanable.cleanupClose0(IJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn cleanup_close_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();

    #[cfg(not(target_os = "windows"))]
    let fd = {
        let _handle = parameters.pop_long()?;
        let fd = parameters.pop_int()?;
        i64::from(fd)
    };

    #[cfg(target_os = "windows")]
    let fd = {
        let handle = parameters.pop_long()?;
        let _fd = parameters.pop_int()?;
        handle
    };

    if fd < 0 {
        return Ok(None);
    }

    let handle_identifier = file_handle_identifier(fd);
    if let Some(handle) = file_handles.remove(&handle_identifier).await {
        #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
        {
            let _ = handle;
        }

        #[cfg(target_os = "wasi")]
        {
            let file_handle: std::fs::File = handle.try_into()?;
            file_handle.sync_all()?;
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let mut file_handle: tokio::fs::File = handle.try_into()?;
            file_handle.shutdown().await?;
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::io::fileoutputstream::raw_file_descriptor;
    use ristretto_types::handles::FileHandle;
    use tokio::fs::{File, remove_file};

    #[tokio::test]
    async fn test_cleanup_close_0_no_handle() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_int(-1);
        parameters.push_long(-1);
        let result = cleanup_close_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_cleanup_close_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let file_handles = vm.file_handles();

        let file_name = "cleanup_close_0_no_handle_test.txt";
        let file = File::create(file_name).await?;
        let fd = raw_file_descriptor(&file)?;
        let file_handle: FileHandle = (file, false).into();
        let handle_identifier = file_handle_identifier(0);
        file_handles.insert(handle_identifier, file_handle).await?;

        let mut parameters = Parameters::default();
        parameters.push_int(i32::try_from(fd)?);
        parameters.push_long(fd);
        let result = cleanup_close_0(thread, parameters).await?;
        assert_eq!(result, None);

        remove_file(file_name).await?;
        Ok(())
    }
}
