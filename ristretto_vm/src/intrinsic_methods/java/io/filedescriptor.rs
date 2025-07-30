use crate::JavaError::IoException;
use crate::intrinsic_methods::java::io::fileoutputstream::file_handle_identifier;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::{Result, VM};
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use tokio::fs::File;
#[cfg(not(target_family = "wasm"))]
use tokio::io::AsyncWriteExt;

/// Returns the file descriptor for a given Java `java.io.FileDescriptor` object taking into account
/// the Java version. For Java 11 and later, it uses the `handle` field for windows, while for
/// earlier versions, or on non-windows platforms, it uses the `fd` field.
pub(crate) fn file_descriptor_from_java_object(
    vm: &Arc<VM>,
    file_descriptor: &Value,
) -> Result<i64> {
    let file_descriptor = file_descriptor.as_object_ref()?;
    let fd = if vm.java_class_file_version() >= &JAVA_11 {
        #[cfg(not(target_os = "windows"))]
        {
            let fd = file_descriptor.value("fd")?.as_i32()?;
            i64::from(fd)
        }
        #[cfg(target_os = "windows")]
        {
            let fd = file_descriptor.value("handle")?.as_i64()?;
            fd
        }
    } else {
        let fd = file_descriptor.value("fd")?.as_i32()?;
        i64::from(fd)
    };
    Ok(fd)
}

#[intrinsic_method("java/io/FileDescriptor.close0()V", GreaterThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn close_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file_descriptor = parameters.pop()?;
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let handle_identifier = file_handle_identifier(fd);

    {
        let mut file_descriptor = file_descriptor.as_object_mut()?;
        file_descriptor.set_value("fd", Value::Int(-1))?;
        if vm.java_class_file_version() >= &JAVA_11 {
            file_descriptor.set_value("handle", Value::Long(-1))?;
        }
    }

    let Some(handle) = file_handles.remove(&handle_identifier).await else {
        return Err(IoException(format!(
            "File handle not found for identifier: {handle_identifier}"
        ))
        .into());
    };

    #[cfg(target_family = "wasm")]
    {
        let _ = handle;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let mut file_handle: File = handle.try_into()?;
        file_handle.shutdown().await?;
    }

    Ok(None)
}

#[intrinsic_method("java/io/FileDescriptor.getAppend(I)Z", GreaterThanOrEqual(JAVA_11))]
#[expect(clippy::match_same_arms)]
#[async_recursion(?Send)]
pub(crate) async fn get_append(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_int()?;
    let append = match handle {
        0 => {
            // true if stdin is in append mode
            false
        }
        1 => {
            // true if stdout is in append mode
            false
        }
        2 => {
            // true if stderr is in append mode
            false
        }
        _ => {
            let vm = thread.vm()?;
            let file_handles = vm.file_handles();
            let fd = i64::from(handle);
            let handle_identifier = file_handle_identifier(fd);
            let file_handle = file_handles.get(&handle_identifier).await.ok_or_else(|| {
                IoException(format!("File handle not found: {handle_identifier}"))
            })?;
            file_handle.append
        }
    };
    Ok(Some(Value::from(append)))
}

#[intrinsic_method("java/io/FileDescriptor.getHandle(I)J", GreaterThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn get_handle(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_int()?;
    let handle = i64::from(handle);
    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method("java/io/FileDescriptor.initIDs()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/io/FileDescriptor.sync()V", LessThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn sync(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    sync_0(thread, parameters).await
}

#[intrinsic_method("java/io/FileDescriptor.sync0()V", GreaterThan(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn sync_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let file_descriptor = parameters.pop()?;
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();
    let fd = file_descriptor_from_java_object(&vm, &file_descriptor)?;
    let handle_identifier = file_handle_identifier(fd);
    let file_handle = file_handles
        .get_mut(&handle_identifier)
        .await
        .ok_or_else(|| IoException(format!("File handle not found: {handle_identifier}")))?;
    let file = &file_handle.file;

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = file;
    }

    #[cfg(target_os = "wasi")]
    {
        file.sync_all()?;
    }

    #[cfg(not(target_family = "wasm"))]
    {
        file.sync_all().await?;
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_append() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let file_handles = [0, 1, 2];

        for handle in file_handles {
            let result =
                get_append(thread.clone(), Parameters::new(vec![Value::Int(handle)])).await?;
            assert_eq!(Some(Value::from(false)), result);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
