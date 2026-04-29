use crate::java::nio::mapped_regions::MappedRegions;
use crate::sun::nio::fs::managed_files;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError::IoException;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// `MappedMemoryUtils.force0(FileDescriptor fd, long address, long length)` (Java >= 17).
///
/// Same semantics as `MappedByteBuffer.force0` for older Java versions.
#[intrinsic_method(
    "java/nio/MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn force_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length = parameters.pop_long()?;
    let address = parameters.pop_long()?;
    let _fd_value = parameters.pop()?;

    if length <= 0 {
        return Ok(None);
    }
    let length_usize = usize::try_from(length)?;

    let vm = thread.vm()?;
    let regions = vm.resource_manager().get_or_init(MappedRegions::new)?;
    let Some((base, region)) = regions.find_containing(address, length_usize) else {
        return Ok(None);
    };
    if !region.mode.is_writable_back_to_file() {
        return Ok(None);
    }

    let offset_in_region = u64::try_from(address - base)?;
    let file_position = region.position + offset_in_region;
    let bytes = vm.native_memory().read_bytes(address, length_usize);
    let fd = region.fd;
    if let Err(e) = managed_files::write_at(vm.file_handles(), fd, &bytes, file_position).await {
        return Err(IoException(format!("force0: {e}")).into());
    }
    if let Err(e) = managed_files::sync_data(vm.file_handles(), fd).await {
        return Err(IoException(format!("force0: {e}")).into());
    }
    Ok(None)
}

/// `MappedMemoryUtils.isLoaded0(long address, long length, long pageCount)` (Java >= 17).
///
/// Returns whether the entire region is resident in physical memory. Always `true` in our
/// model.
#[intrinsic_method(
    "java/nio/MappedMemoryUtils.isLoaded0(JJJ)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn is_loaded_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

/// `MappedMemoryUtils.load0(long address, long length)` (Java >= 17). No-op.
#[intrinsic_method("java/nio/MappedMemoryUtils.load0(JJ)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn load_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/nio/MappedMemoryUtils.registerNatives()V",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

/// `MappedMemoryUtils.unload0(long address, long length)` (Java >= 17). No-op.
#[intrinsic_method("java/nio/MappedMemoryUtils.unload0(JJ)V", GreaterThanOrEqual(JAVA_17))]
#[async_method]
pub async fn unload_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_force_0_no_region() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut params = Parameters::default();
        params.push(Value::Object(None));
        params.push_long(0xdead_beef);
        params.push_long(16);
        let result = force_0(thread, params).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_force_0_zero_length() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut params = Parameters::default();
        params.push(Value::Object(None));
        params.push_long(0);
        params.push_long(0);
        let result = force_0(thread, params).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_is_loaded_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut params = Parameters::default();
        params.push_long(0);
        params.push_long(16);
        params.push_long(1);
        let result = is_loaded_0(thread, params).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut params = Parameters::default();
        params.push_long(0);
        params.push_long(16);
        let result = load_0(thread, params).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_unload_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut params = Parameters::default();
        params.push_long(0);
        params.push_long(16);
        let result = unload_0(thread, params).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert!(result.is_none());
        Ok(())
    }
}
