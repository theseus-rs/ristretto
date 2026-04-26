use crate::java::io::filedescriptor::file_descriptor_from_java_object;
use crate::java::nio::mapped_regions::MappedRegions;
use crate::sun::nio::fs::managed_files;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError::IoException;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

/// `MappedByteBuffer.force0(FileDescriptor fd, long address, long length)` (Java <= 11).
///
/// Flushes any changes made to the mapped region back to the underlying file. For ranges that
/// are not part of a writable mapping or are not registered, this is a no-op (matching the
/// best-effort semantics required by the JVM specification; the OS may also no-op msync if
/// the page is clean or if the file does not require sync).
#[intrinsic_method(
    "java/nio/MappedByteBuffer.force0(Ljava/io/FileDescriptor;JJ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn force_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length = parameters.pop_long()?;
    let address = parameters.pop_long()?;
    let fd_value = parameters.pop()?;
    let _this = parameters.pop()?;

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

    // The address may point into the middle of the region (sliced buffer); compute file offset
    // and read from native memory accordingly.
    let offset_in_region = u64::try_from(address - base)?;
    let file_position = region.position + offset_in_region;
    let bytes = vm.native_memory().read_bytes(address, length_usize);
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;
    if let Err(e) = managed_files::write_at(vm.file_handles(), fd, &bytes, file_position).await {
        return Err(IoException(format!("force0: {e}")).into());
    }
    if let Err(e) = managed_files::sync_data(vm.file_handles(), fd).await {
        return Err(IoException(format!("force0: {e}")).into());
    }
    Ok(None)
}

/// `MappedByteBuffer.isLoaded0(long address, long length, int pageCount)` (Java <= 11).
///
/// Returns whether the entire region is resident in physical memory. In our virtualized model
/// memory is always resident, so we return `true`. This is spec-compliant: the result is a
/// hint with no behavioral guarantees.
#[intrinsic_method("java/nio/MappedByteBuffer.isLoaded0(JJI)Z", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn is_loaded_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "windows")]
    let loaded = 0;
    #[cfg(not(target_os = "windows"))]
    let loaded = 1;
    Ok(Some(Value::Int(loaded)))
}

/// `MappedByteBuffer.load0(long address, long length)` (Java <= 11).
///
/// Loads the region into physical memory. In our model memory is always resident, so this is a
/// no-op. The JVM specification only requires best-effort behavior.
#[intrinsic_method("java/nio/MappedByteBuffer.load0(JJ)V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn load_0<T: Thread + 'static>(
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
        let (_vm, thread) = crate::test::java11_thread().await?;
        let mut params = Parameters::default();
        params.push(Value::Object(None)); // this
        params.push(Value::Object(None)); // fd
        params.push_long(0xdead_beef);
        params.push_long(16);
        let result = force_0(thread, params).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_force_0_zero_length() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await?;
        let mut params = Parameters::default();
        params.push(Value::Object(None));
        params.push(Value::Object(None));
        params.push_long(0);
        params.push_long(0);
        let result = force_0(thread, params).await?;
        assert!(result.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_is_loaded_0() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await?;
        let mut params = Parameters::default();
        params.push_long(0);
        params.push_long(16);
        params.push_int(1);
        let result = is_loaded_0(thread, params).await?;
        #[cfg(target_os = "windows")]
        let expected = 0;
        #[cfg(not(target_os = "windows"))]
        let expected = 1;
        assert_eq!(result, Some(Value::Int(expected)));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_0() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await?;
        let mut params = Parameters::default();
        params.push_long(0);
        params.push_long(16);
        let result = load_0(thread, params).await?;
        assert!(result.is_none());
        Ok(())
    }
}
