use crate::java::io::filedescriptor::file_descriptor_from_java_object;
use crate::java::nio::mapped_regions::{MapMode, MappedRegion, MappedRegions};
use crate::sun::nio::fs::managed_files;
use ristretto_classfile::VersionSpecification::{Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError::{IllegalArgumentException, IoException};
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/FileChannelImpl.initIDs()J", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(0)))
}

/// Maps `length` bytes from the file (starting at `position`) into a freshly-allocated
/// `NativeMemory` block, returning its base address.
async fn do_map<V: VM>(vm: &Arc<V>, fd: i64, prot: i32, position: i64, length: i64) -> Result<i64> {
    if position < 0 {
        return Err(IllegalArgumentException(format!("Invalid position: {position}")).into());
    }
    if length < 0 {
        return Err(IllegalArgumentException(format!("Invalid length: {length}")).into());
    }
    let mode = MapMode::from_int(prot)
        .ok_or_else(|| IllegalArgumentException(format!("Invalid map mode: {prot}")))?;
    let length_usize = usize::try_from(length)?;
    let position_u64 = u64::try_from(position)?;

    let address = vm.native_memory().allocate(length_usize.max(1));

    if length_usize > 0 {
        // Read up to `length` bytes from the file. If the file is shorter, the tail of the
        // mapping remains zero (as the JDK / OS would extend the file when MAP_RW; we mirror
        // by leaving zeros in our buffer for the read-only case and writing zeros back on
        // force for read-write).
        let mut buf = vec![0u8; length_usize];
        let n = match managed_files::read_at(vm.file_handles(), fd, &mut buf, position_u64).await {
            Ok(n) => n,
            Err(e) => {
                vm.native_memory().free(address);
                return Err(IoException(format!("map0: {e}")).into());
            }
        };
        // Truncate to the actual number of bytes read; everything past this remains zero.
        if n < length_usize {
            for byte in &mut buf[n..] {
                *byte = 0;
            }
        }
        vm.native_memory().write_bytes(address, &buf);
    }

    let regions = vm.resource_manager().get_or_init(MappedRegions::new)?;
    regions.insert(
        address,
        MappedRegion {
            fd,
            position: position_u64,
            length: length_usize,
            mode,
            file_key: None,
            path: None,
        },
    );

    Ok(address)
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.map0(IJJ)J", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn map_0_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length = parameters.pop_long()?;
    let position = parameters.pop_long()?;
    let prot = parameters.pop_int()?;
    // The instance method receives `this` (the FileChannelImpl) which holds the FileDescriptor;
    // OpenJDK's native takes (prot, position, length);`this` is passed as JNIEnv self. In our
    // intrinsic ABI the receiver is on the stack; the caller however passes the FileDescriptor
    // explicitly via reflection elsewhere. To stay compatible with the OpenJDK signature, the
    // surrounding Java glue calls these natives with `this` already discarded; the
    // FileDescriptor is held on the FileChannelImpl, which Java passes through the receiver.
    let this = parameters.pop()?;
    let vm = thread.vm()?;
    let fd_value = {
        let receiver = this.as_object_ref()?;
        receiver.value("fd")?
    };
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;
    let address = do_map(&vm, fd, prot, position, length).await?;
    Ok(Some(Value::Long(address)))
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.map0(IJJZ)J", Equal(JAVA_17))]
#[async_method]
pub async fn map_0_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_sync = parameters.pop_int()? != 0;
    let length = parameters.pop_long()?;
    let position = parameters.pop_long()?;
    let prot = parameters.pop_int()?;
    let this = parameters.pop()?;
    let vm = thread.vm()?;
    let fd_value = {
        let receiver = this.as_object_ref()?;
        receiver.value("fd")?
    };
    let fd = file_descriptor_from_java_object(&vm, &fd_value)?;
    let address = do_map(&vm, fd, prot, position, length).await?;
    Ok(Some(Value::Long(address)))
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.maxDirectTransferSize0()I", Equal(JAVA_17))]
#[async_method]
pub async fn max_direct_transfer_size_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(i32::MAX)))
}

#[intrinsic_method(
    "sun/nio/ch/FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn transfer_to_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let dst_fd_value = parameters.pop()?;
    let count = parameters.pop_long()?;
    let position = parameters.pop_long()?;
    let src_fd_value = parameters.pop()?;
    if count <= 0 {
        return Ok(Some(Value::Long(0)));
    }
    let vm = thread.vm()?;
    let src_fd = file_descriptor_from_java_object(&vm, &src_fd_value)?;
    let dst_fd = file_descriptor_from_java_object(&vm, &dst_fd_value)?;
    let count_usize = usize::try_from(count)?;
    let position_u64 = u64::try_from(position)?;
    let mut buf = vec![0u8; count_usize];
    let n = managed_files::read_at(vm.file_handles(), src_fd, &mut buf, position_u64)
        .await
        .map_err(|e| IoException(format!("transferTo0: {e}")))?;
    if n == 0 {
        return Ok(Some(Value::Long(0)));
    }
    let written = managed_files::write(vm.file_handles(), dst_fd, &buf[..n])
        .await
        .map_err(|e| IoException(format!("transferTo0: {e}")))?;
    Ok(Some(Value::Long(
        i64::try_from(written).map_err(|e| InternalError(e.to_string()))?,
    )))
}

#[intrinsic_method("sun/nio/ch/FileChannelImpl.unmap0(JJ)I", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn unmap_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _length = parameters.pop_long()?;
    let address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let regions = vm.resource_manager().get_or_init(MappedRegions::new)?;
    regions.remove(address);
    vm.native_memory().free(address);
    Ok(Some(Value::Int(0)))
}

/// `FileChannelImpl.map0(FileDescriptor fd, int prot, long position, long length, boolean isSync)`
/// (Java >= 21). The native became static and explicitly receives the `FileDescriptor`.
/// In `OpenJDK` 21+ this signature is implemented on `UnixFileDispatcherImpl` instead, so we do
/// not register it here.
#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use ristretto_types::handles::{FileHandle, FileModeFlags};
    use std::io::Write;
    use tempfile::NamedTempFile;

    async fn open_test_file(
        initial: &[u8],
    ) -> (
        Arc<ristretto_vm::VM>,
        Arc<ristretto_vm::Thread>,
        NamedTempFile,
        i64,
    ) {
        let (vm, thread) = crate::test::java17_thread().await.expect("thread");
        let mut tmp = NamedTempFile::new().expect("tmp");
        tmp.write_all(initial).expect("write");
        tmp.flush().expect("flush");
        let path = tmp.path().to_path_buf();
        let std_file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("open");
        let tokio_file = tokio::fs::File::from_std(std_file);
        let handle: FileHandle = (tokio_file, FileModeFlags::READ_WRITE).into();
        let fd = i64::from(vm.next_nio_fd());
        vm.file_handles().insert(fd, handle).await.expect("insert");
        (vm, thread, tmp, fd)
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_max_direct_transfer_size_0() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = max_direct_transfer_size_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Int(i32::MAX)));
        Ok(())
    }

    #[tokio::test]
    async fn test_do_map_and_unmap() -> Result<()> {
        let initial = b"abcdefghij".to_vec();
        let (vm, _thread, _tmp, fd) = open_test_file(&initial).await;
        let address = do_map(&vm, fd, 1 /* MAP_RW */, 0, 10).await?;
        assert!(address >= 0x1000_0000);
        // Verify the bytes are present in NativeMemory.
        let bytes = vm.native_memory().read_bytes(address, 10);
        assert_eq!(bytes, initial);
        // Verify it was registered.
        let regions = vm.resource_manager().get_or_init(MappedRegions::new)?;
        let region = regions.get(address).expect("registered");
        assert_eq!(region.fd, fd);
        assert_eq!(region.length, 10);
        // Now unmap.
        let mut params = Parameters::default();
        params.push_long(address);
        params.push_long(10);
        let (_vm2, thread2) = crate::test::java17_thread().await?;
        // Reuse the original VM/thread for unmap to operate on the same registry.
        let weak = std::sync::Arc::downgrade(&vm);
        let thread = ristretto_vm::Thread::new(&weak, 99);
        let _ = thread2;
        unmap_0(thread, params).await?;
        let regions = vm.resource_manager().get_or_init(MappedRegions::new)?;
        assert!(regions.get(address).is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_do_map_invalid_mode() -> Result<()> {
        let initial = b"abc".to_vec();
        let (vm, _thread, _tmp, fd) = open_test_file(&initial).await;
        let result = do_map(&vm, fd, 99, 0, 3).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_do_map_negative_position() -> Result<()> {
        let initial = b"abc".to_vec();
        let (vm, _thread, _tmp, fd) = open_test_file(&initial).await;
        let result = do_map(&vm, fd, 0, -1, 3).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_do_map_negative_length() -> Result<()> {
        let initial = b"abc".to_vec();
        let (vm, _thread, _tmp, fd) = open_test_file(&initial).await;
        let result = do_map(&vm, fd, 0, 0, -1).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_do_map_partial_file() -> Result<()> {
        let initial = b"hello".to_vec();
        let (vm, _thread, _tmp, fd) = open_test_file(&initial).await;
        // Map more than the file contains; the tail is zero.
        let address = do_map(&vm, fd, 0, 0, 10).await?;
        let bytes = vm.native_memory().read_bytes(address, 10);
        assert_eq!(&bytes[..5], b"hello");
        assert_eq!(&bytes[5..], &[0u8; 5]);
        Ok(())
    }

    #[tokio::test]
    async fn test_unmap_unknown_address() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let mut params = Parameters::default();
        params.push_long(0xdead_beef);
        params.push_long(16);
        let result = unmap_0(thread, params).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_transfer_to_0_zero_count() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let mut params = Parameters::default();
        params.push(Value::Object(None));
        params.push_long(0);
        params.push_long(0);
        params.push(Value::Object(None));
        let result = transfer_to_0(thread, params).await?;
        assert_eq!(result, Some(Value::Long(0)));
        Ok(())
    }
}
