use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::{Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
#[cfg(target_os = "linux")]
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

use super::common::throw_unix_exception;
#[cfg(target_os = "linux")]
use super::native_resources;

#[cfg(target_os = "linux")]
fn last_errno() -> i32 {
    std::io::Error::last_os_error()
        .raw_os_error()
        .unwrap_or(libc::EIO)
}

const IO_STATUS_UNAVAILABLE: i32 = -2;
const IO_STATUS_UNSUPPORTED_CASE: i32 = -6;

/// Read a NUL-terminated byte string from native memory.
fn read_native_path<V: VM>(vm: &V, address: i64) -> Result<Vec<u8>> {
    vm.native_memory()
        .try_read_cstring(address)
        .ok_or_else(|| InternalError(format!("Invalid native string address: {address}")))
}

/// Read a C string field from a `libc::mntent` struct as a Java byte array.
#[expect(unsafe_code)]
unsafe fn cstr_field_to_bytes(ptr: *const libc::c_char) -> Vec<u8> {
    if ptr.is_null() {
        Vec::new()
    } else {
        unsafe { std::ffi::CStr::from_ptr(ptr).to_bytes().to_vec() }
    }
}

/// `LinuxNativeDispatcher.init()V`
///
/// The native version caches `jfieldID`s used to populate `UnixMountEntry` instances and
/// performs no other initialization. There is nothing equivalent for the intrinsic
/// implementation, so this is a no-op.
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/fs/LinuxNativeDispatcher.init()V", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

/// `LinuxNativeDispatcher.setmntent0(JJ)J`
///
/// Opens the mount table file specified by `pathAddress` using the mode specified by
/// `typeAddress` (both addresses point to NUL-terminated strings in native memory) and
/// returns a `FILE *` cast to `long`. Throws a `UnixException` on failure.
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/fs/LinuxNativeDispatcher.setmntent0(JJ)J", Any)]
#[async_method]
pub async fn setmntent_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let type_address = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;

    let vm = thread.vm()?;
    let path_bytes = read_native_path(&*vm, path_address)?;
    let mode_bytes = read_native_path(&*vm, type_address)?;

    let Ok(path) = std::ffi::CString::new(path_bytes) else {
        return Err(throw_unix_exception(&thread, libc::EINVAL).await);
    };
    let Ok(mode) = std::ffi::CString::new(mode_bytes) else {
        return Err(throw_unix_exception(&thread, libc::EINVAL).await);
    };

    #[expect(unsafe_code)]
    let fp = unsafe { libc::setmntent(path.as_ptr(), mode.as_ptr()) };
    if fp.is_null() {
        let errno = std::io::Error::last_os_error()
            .raw_os_error()
            .unwrap_or(libc::EIO);
        return Err(throw_unix_exception(&thread, errno).await);
    }
    let handle = native_resources::insert_file(&*vm, fp)?;
    Ok(Some(Value::Long(handle)))
}

/// `LinuxNativeDispatcher.getmntent0(JLsun/nio/fs/UnixMountEntry;JI)I`
///
/// Reads the next entry from the mount file `fp` and populates `entry`'s `name`, `dir`,
/// `fstype`, and `opts` byte-array fields. The `buffer`/`bufLen` parameters point at a
/// scratch buffer allocated by the JDK for `getmntent_r`; the intrinsic does not need it.
/// Returns 0 on success or -1 at end-of-file. The native version returns -1 on out-of-memory
/// when allocating Java byte arrays as well, but that does not apply to the intrinsic.
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.getmntent0(JLsun/nio/fs/UnixMountEntry;JI)I",
    Any
)]
#[async_method]
pub async fn getmntent_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let buf_len = parameters.pop_int()?;
    let buffer = parameters.pop_long()?;
    let entry_value = parameters.pop()?;
    let fp = parameters.pop_long()?;

    if fp == 0 {
        return Ok(Some(Value::Int(-1)));
    }

    let vm = thread.vm()?;
    let scratch_len = usize::try_from(buf_len).unwrap_or(0);
    if scratch_len == 0
        || vm
            .native_memory()
            .try_read_bytes(buffer, scratch_len)
            .is_none()
    {
        return Err(throw_unix_exception(&thread, libc::EFAULT).await);
    }
    let entry = {
        let mut storage: libc::mntent = unsafe_zeroed_mntent();
        let mut scratch = vec![0u8; scratch_len];
        #[expect(unsafe_code)]
        native_resources::with_file(&*vm, fp, |fp_ptr| unsafe {
            unsafe extern "C" {
                fn getmntent_r(
                    stream: *mut libc::FILE,
                    mntbuf: *mut libc::mntent,
                    buf: *mut libc::c_char,
                    buflen: libc::c_int,
                ) -> *mut libc::mntent;
            }
            let result = getmntent_r(
                fp_ptr,
                std::ptr::addr_of_mut!(storage),
                scratch.as_mut_ptr().cast::<libc::c_char>(),
                buf_len,
            );
            if result.is_null() {
                None
            } else {
                Some((
                    cstr_field_to_bytes(storage.mnt_fsname),
                    cstr_field_to_bytes(storage.mnt_dir),
                    cstr_field_to_bytes(storage.mnt_type),
                    cstr_field_to_bytes(storage.mnt_opts),
                ))
            }
        })?
    };
    let Some(entry) = entry else {
        return Err(throw_unix_exception(&thread, libc::EBADF).await);
    };
    let Some((name, dir, fstype, opts)) = entry else {
        return Ok(Some(Value::Int(-1)));
    };

    let gc = vm.garbage_collector();
    let mut guard = entry_value.as_reference_mut()?;
    let Reference::Object(object) = &mut *guard else {
        return Ok(Some(Value::Int(-1)));
    };
    object.set_value("name", Value::new_object(gc, Reference::from(name)))?;
    object.set_value("dir", Value::new_object(gc, Reference::from(dir)))?;
    object.set_value("fstype", Value::new_object(gc, Reference::from(fstype)))?;
    object.set_value("opts", Value::new_object(gc, Reference::from(opts)))?;

    Ok(Some(Value::Int(0)))
}

/// Helper that returns a zero-initialized `libc::mntent`. `libc::mntent` does not
/// implement `Default`, so we build it via `MaybeUninit::zeroed`.
#[expect(unsafe_code)]
fn unsafe_zeroed_mntent() -> libc::mntent {
    unsafe { std::mem::MaybeUninit::<libc::mntent>::zeroed().assume_init() }
}

/// `LinuxNativeDispatcher.endmntent(J)V`
///
/// Closes the mount file previously opened with `setmntent0`.
#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/fs/LinuxNativeDispatcher.endmntent(J)V", Any)]
#[async_method]
pub async fn endmntent<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let stream = parameters.pop_long()?;

    let vm = thread.vm()?;
    let Some(pointer) = native_resources::take_file(&*vm, stream)? else {
        return Err(throw_unix_exception(&thread, libc::EBADF).await);
    };
    #[expect(unsafe_code)]
    unsafe {
        libc::endmntent(pointer as *mut libc::FILE);
    }

    Ok(None)
}

/// `LinuxNativeDispatcher.posix_fadvise(IJJI)I`
///
/// Issues a `posix_fadvise` advice hint on the supplied file descriptor. On non-Linux
/// platforms we return 0 (success) since the advice is purely advisory. On Linux, errors
/// are surfaced as a `UnixException`.
///
/// Note that the JDK passes the descriptor as a managed handle; on platforms where it does
/// not correspond to a real OS file descriptor we simply ignore the call.
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.posix_fadvise(IJJI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn posix_fadvise<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let advice = parameters.pop_int()?;
    let len = parameters.pop_long()?;
    let offset = parameters.pop_long()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_os = "linux")]
    {
        #[expect(unsafe_code)]
        let result = unsafe { libc::posix_fadvise(fd, offset, len, advice) };
        Ok(Some(Value::Int(result)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (fd, offset, len, advice);
        Ok(Some(Value::Int(libc::ENOTSUP)))
    }
}

/// `LinuxNativeDispatcher.directCopy0(IIJ)I`
///
/// Performs a direct kernel-mediated copy between `src` and `dst` (e.g. `copy_file_range`
/// or `sendfile`). Returns `IOStatus.UNSUPPORTED` so that the JDK falls back to the
/// already-implemented buffered copy. The descriptors are managed handles and not real
/// file descriptors, so issuing the syscall directly is not possible.
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.directCopy0(IIJ)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn direct_copy_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address_to_poll_for_cancel = parameters.pop_long()?;
    let src = parameters.pop_int()?;
    let dst = parameters.pop_int()?;

    #[cfg(target_os = "linux")]
    {
        let vm = thread.vm()?;
        if address_to_poll_for_cancel != 0
            && vm
                .native_memory()
                .read_i32(address_to_poll_for_cancel)
                .is_none()
        {
            return Err(throw_unix_exception(&thread, libc::EFAULT).await);
        }
        let cancelled = || {
            address_to_poll_for_cancel != 0
                && vm
                    .native_memory()
                    .read_i32(address_to_poll_for_cancel)
                    .is_some_and(|value| value != 0)
        };
        if cancelled() {
            return Err(throw_unix_exception(&thread, libc::ECANCELED).await);
        }
        let count = if address_to_poll_for_cancel == 0 {
            0x7fff_f000_usize
        } else {
            1024 * 1024
        };

        loop {
            #[expect(unsafe_code)]
            let copied = unsafe {
                libc::copy_file_range(
                    src,
                    std::ptr::null_mut(),
                    dst,
                    std::ptr::null_mut(),
                    count,
                    0,
                )
            };
            if copied > 0 {
                if cancelled() {
                    return Err(throw_unix_exception(&thread, libc::ECANCELED).await);
                }
                continue;
            }
            if copied == 0 {
                return Ok(Some(Value::Int(0)));
            }
            let error = last_errno();
            if error == libc::EINTR {
                continue;
            }
            if error == libc::EAGAIN {
                return Ok(Some(Value::Int(IO_STATUS_UNAVAILABLE)));
            }
            if !matches!(error, libc::EINVAL | libc::ENOSYS | libc::EXDEV) {
                return Err(throw_unix_exception(&thread, error).await);
            }
            break;
        }

        loop {
            #[expect(unsafe_code)]
            let copied = unsafe { libc::sendfile(dst, src, std::ptr::null_mut(), count) };
            if copied > 0 {
                if cancelled() {
                    return Err(throw_unix_exception(&thread, libc::ECANCELED).await);
                }
                continue;
            }
            if copied == 0 {
                return Ok(Some(Value::Int(0)));
            }
            let error = last_errno();
            if error == libc::EINTR {
                continue;
            }
            if error == libc::EAGAIN {
                return Ok(Some(Value::Int(IO_STATUS_UNAVAILABLE)));
            }
            if matches!(error, libc::EINVAL | libc::ENOSYS) {
                return Ok(Some(Value::Int(IO_STATUS_UNSUPPORTED_CASE)));
            }
            return Err(throw_unix_exception(&thread, error).await);
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, dst, src, address_to_poll_for_cancel);
        Ok(Some(Value::Int(-4)))
    }
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.fgetxattr0(IJJI)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fgetxattr0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::unixnativedispatcher::fgetxattr_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.flistxattr(IJI)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn flistxattr<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::unixnativedispatcher::flistxattr(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.fremovexattr0(IJ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fremovexattr0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::unixnativedispatcher::fremovexattr_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.fsetxattr0(IJJI)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fsetxattr0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::unixnativedispatcher::fsetxattr_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/fs/LinuxNativeDispatcher.getlinelen(J)I", Equal(JAVA_8))]
#[async_method]
pub async fn getlinelen<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let stream = parameters.pop_long()?;
    if stream == 0 {
        return Err(throw_unix_exception(&thread, libc::EBADF).await);
    }
    let vm = thread.vm()?;
    #[expect(unsafe_code)]
    let outcome = native_resources::with_file(&*vm, stream, |pointer| unsafe {
        let mut line = std::ptr::null_mut::<libc::c_char>();
        let mut capacity = 0_usize;
        let result = libc::getline(&raw mut line, &raw mut capacity, pointer);
        let error = last_errno();
        let eof = libc::feof(pointer) != 0;
        libc::free(line.cast::<libc::c_void>());
        (result, error, eof)
    })?;
    let Some((result, error, eof)) = outcome else {
        return Err(throw_unix_exception(&thread, libc::EBADF).await);
    };
    if result < 0 {
        if eof {
            return Ok(Some(Value::Int(-1)));
        }
        return Err(throw_unix_exception(&thread, error).await);
    }
    Ok(Some(Value::Int(i32::try_from(result).map_err(|_| {
        InternalError("line length exceeds jint".to_string())
    })?)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::Error;

    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await.expect("init");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_posix_fadvise_returns_native_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_int(0); // fd
        params.push_long(0); // offset
        params.push_long(0); // len
        params.push_int(0); // advice
        let result = posix_fadvise(thread, params).await.expect("posix_fadvise");
        assert!(matches!(result, Some(Value::Int(_))));
    }

    #[tokio::test]
    async fn test_posix_fadvise_underflow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = posix_fadvise(thread, Parameters::default()).await;
        assert!(matches!(result, Err(Error::ParametersUnderflow)));
    }

    #[tokio::test]
    async fn test_direct_copy_0_underflow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = direct_copy_0(thread, Parameters::default()).await;
        assert!(matches!(result, Err(Error::ParametersUnderflow)));
    }

    #[tokio::test]
    async fn test_endmntent_with_invalid_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_long(0);
        let result = endmntent(thread, params).await;
        assert!(matches!(result, Err(Error::Throwable(_))));
    }

    #[tokio::test]
    async fn test_endmntent_underflow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = endmntent(thread, Parameters::default()).await;
        assert!(matches!(result, Err(Error::ParametersUnderflow)));
    }

    #[tokio::test]
    async fn test_setmntent_0_underflow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = setmntent_0(thread, Parameters::default()).await;
        assert!(matches!(result, Err(Error::ParametersUnderflow)));
    }

    #[tokio::test]
    async fn test_setmntent_0_invalid_native_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        // Invalid guest pointers are rejected instead of being interpreted as empty strings.
        params.push_long(0);
        params.push_long(0);
        let result = setmntent_0(thread, params).await;
        assert!(matches!(result, Err(InternalError(_))));
    }

    #[tokio::test]
    async fn test_getmntent_0_underflow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getmntent_0(thread, Parameters::default()).await;
        assert!(matches!(result, Err(Error::ParametersUnderflow)));
    }

    #[tokio::test]
    async fn test_getmntent_0_with_null_handle() {
        // With fp == 0 we must short-circuit and return -1 without touching the entry.
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_long(0); // fp
        params.push_reference(None); // entry (null reference)
        params.push_long(0); // buffer
        params.push_int(0); // bufLen
        let result = getmntent_0(thread, params).await.expect("getmntent_0");
        assert_eq!(result, Some(Value::Int(-1)));
    }

    #[tokio::test]
    async fn test_setmntent_getmntent_endmntent_roundtrip() {
        use std::io::Write;

        // Build a tiny synthetic mtab-style file we can parse with getmntent_r.
        let directory = tempfile::tempdir().expect("temp directory");
        let path = directory.path().join("mtab");
        {
            let mut f = std::fs::File::create(&path).expect("create");
            writeln!(f, "/dev/sda1 / ext4 rw,relatime 0 0").expect("write");
            writeln!(f, "tmpfs /tmp tmpfs rw,nosuid,nodev 0 0").expect("write");
        }

        let (vm, thread) = crate::test::thread().await.expect("thread");

        // Write the path and mode strings into native memory so the intrinsic can read them.
        let mut path_bytes = path.to_string_lossy().as_bytes().to_vec();
        path_bytes.push(0);
        let mut mode_bytes = b"r".to_vec();
        mode_bytes.push(0);
        let path_addr = vm.native_memory().allocate(path_bytes.len());
        let mode_addr = vm.native_memory().allocate(mode_bytes.len());
        vm.native_memory().write_bytes(path_addr, &path_bytes);
        vm.native_memory().write_bytes(mode_addr, &mode_bytes);

        let mut params = Parameters::default();
        params.push_long(path_addr);
        params.push_long(mode_addr);
        let handle_value = setmntent_0(thread.clone(), params)
            .await
            .expect("setmntent")
            .expect("handle");
        let Value::Long(handle) = handle_value else {
            panic!("expected long handle, got {handle_value:?}");
        };
        assert!(handle != 0, "handle should not be zero");

        // Read both entries.
        let entry = thread
            .object("sun.nio.fs.UnixMountEntry", "", &[] as &[Value])
            .await
            .expect("UnixMountEntry");
        let scratch = vm.native_memory().allocate(1024);
        for _ in 0..2 {
            let mut params = Parameters::default();
            params.push_long(handle);
            params.push(entry.clone());
            params.push_long(scratch);
            params.push_int(1024);
            let r = getmntent_0(thread.clone(), params)
                .await
                .expect("getmntent_0")
                .expect("rc");
            assert_eq!(r, Value::Int(0));
        }
        let directory = {
            let entry_reference = entry.as_reference().expect("entry reference");
            let Reference::Object(entry_object) = &*entry_reference else {
                panic!("expected mount entry object");
            };
            entry_object.value("dir").expect("dir field")
        };
        let directory = {
            let directory_reference = directory.as_reference().expect("dir reference");
            let Reference::ByteArray(directory) = &*directory_reference else {
                panic!("expected byte array mount directory");
            };
            directory
                .iter()
                .map(|byte| byte.cast_unsigned())
                .collect::<Vec<u8>>()
        };
        assert_eq!(directory, b"/tmp");

        // EOF.
        let mut params = Parameters::default();
        params.push_long(handle);
        params.push(entry.clone());
        params.push_long(scratch);
        params.push_int(1024);
        let r = getmntent_0(thread.clone(), params)
            .await
            .expect("getmntent_0")
            .expect("rc");
        assert_eq!(r, Value::Int(-1));

        // Close.
        let mut params = Parameters::default();
        params.push_long(handle);
        let r = endmntent(thread.clone(), params).await.expect("endmntent");
        assert!(r.is_none());
    }
}
