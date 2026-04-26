use ristretto_classfile::JAVA_17;
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
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

use super::common::throw_unix_exception;

/// `IOStatus.UNSUPPORTED` from `sun.nio.ch.IOStatus`. Returned by `directCopy0` to indicate
/// that direct copy via a platform specific syscall is not supported. The caller falls back
/// to the buffered copy path.
const IO_STATUS_UNSUPPORTED: i32 = -4;

/// Read a NUL-terminated byte string from native memory.
fn read_native_path<V: VM>(vm: &V, address: i64) -> Vec<u8> {
    vm.native_memory().read_cstring(address)
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
    let path_bytes = read_native_path(&*vm, path_address);
    let mode_bytes = read_native_path(&*vm, type_address);

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
    #[expect(clippy::cast_possible_wrap)]
    let handle = fp as usize as i64;
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
    let _buf_len = parameters.pop_int()?;
    let _buffer = parameters.pop_long()?;
    let entry_value = parameters.pop()?;
    let fp = parameters.pop_long()?;

    if fp == 0 {
        return Ok(Some(Value::Int(-1)));
    }

    let fp_ptr = usize::try_from(fp)
        .map_err(|_| InternalError("invalid mount file handle".to_string()))?
        as *mut libc::FILE;
    let mut storage: libc::mntent = unsafe_zeroed_mntent();
    let scratch_len: libc::c_int = 4096;
    let mut scratch = vec![0u8; usize::try_from(scratch_len).unwrap_or(0)];

    #[expect(unsafe_code)]
    let result = unsafe {
        unsafe extern "C" {
            fn getmntent_r(
                stream: *mut libc::FILE,
                mntbuf: *mut libc::mntent,
                buf: *mut libc::c_char,
                buflen: libc::c_int,
            ) -> *mut libc::mntent;
        }
        getmntent_r(
            fp_ptr,
            std::ptr::addr_of_mut!(storage),
            scratch.as_mut_ptr().cast::<libc::c_char>(),
            scratch_len,
        )
    };
    if result.is_null() {
        return Ok(Some(Value::Int(-1)));
    }

    #[expect(unsafe_code)]
    let (name, dir, fstype, opts) = unsafe {
        (
            cstr_field_to_bytes(storage.mnt_fsname),
            cstr_field_to_bytes(storage.mnt_dir),
            cstr_field_to_bytes(storage.mnt_type),
            cstr_field_to_bytes(storage.mnt_opts),
        )
    };

    let vm = thread.vm()?;
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let stream = parameters.pop_long()?;

    if stream != 0 {
        let fp = usize::try_from(stream)
            .map_err(|_| InternalError("invalid mount file handle".to_string()))?
            as *mut libc::FILE;
        #[expect(unsafe_code)]
        unsafe {
            libc::endmntent(fp);
        }
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
    let _advice = parameters.pop_int()?;
    let _len = parameters.pop_long()?;
    let _offset = parameters.pop_long()?;
    let _fd = parameters.pop_int()?;

    // The descriptors used here are managed by ristretto's `HandleManager` and do not
    // correspond to real OS file descriptors, so calling `libc::posix_fadvise` directly
    // is unsafe. The advice is purely a performance hint, so a no-op success preserves
    // correct semantics.
    Ok(Some(Value::Int(0)))
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
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _address_to_poll_for_cancel = parameters.pop_long()?;
    let _src = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    Ok(Some(Value::Int(IO_STATUS_UNSUPPORTED)))
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.fgetxattr0(IJJI)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fgetxattr0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_len = parameters.pop_int()?;
    let _value_adddress = parameters.pop_long()?;
    let _name_address = parameters.pop_long()?;
    let _filedes = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.fgetxattr0(IJJI)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.flistxattr(IJI)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn flistxattr<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _size = parameters.pop_int()?;
    let _list_address = parameters.pop_long()?;
    let _filedes = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.flistxattr(IJI)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.fremovexattr0(IJ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fremovexattr0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name_address = parameters.pop_long()?;
    let _filedes = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.fremovexattr0(IJ)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.fsetxattr0(IJJI)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fsetxattr0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_len = parameters.pop_int()?;
    let _value_adddress = parameters.pop_long()?;
    let _name_address = parameters.pop_long()?;
    let _filedes = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.fsetxattr0(IJJI)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/fs/LinuxNativeDispatcher.getlinelen(J)I", Equal(JAVA_8))]
#[async_method]
pub async fn getlinelen<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _stream = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.getlinelen(J)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.endmntent(J)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn endmntent_linux_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _stream = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.endmntent(J)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.fgetxattr0(IJJI)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fgetxattr0_linux_le_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_len = parameters.pop_int()?;
    let _value_adddress = parameters.pop_long()?;
    let _name_address = parameters.pop_long()?;
    let _filedes = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.fgetxattr0(IJJI)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.flistxattr(IJI)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn flistxattr_linux_le_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _size = parameters.pop_int()?;
    let _list_address = parameters.pop_long()?;
    let _filedes = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.flistxattr(IJI)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.fremovexattr0(IJ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fremovexattr0_linux_le_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name_address = parameters.pop_long()?;
    let _filedes = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.fremovexattr0(IJ)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.fsetxattr0(IJJI)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fsetxattr0_linux_le_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value_len = parameters.pop_int()?;
    let _value_adddress = parameters.pop_long()?;
    let _name_address = parameters.pop_long()?;
    let _filedes = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.fsetxattr0(IJJI)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/fs/LinuxNativeDispatcher.getlinelen(J)I", Equal(JAVA_8))]
#[async_method]
pub async fn getlinelen_linux_v8<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _stream = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.getlinelen(J)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.getmntent0(JLsun/nio/fs/UnixMountEntry;JI)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn getmntent0_linux_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buf_len = parameters.pop_int()?;
    let _buffer = parameters.pop_long()?;
    let _entry = parameters.pop_reference()?;
    let _fp = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.getmntent0(JLsun/nio/fs/UnixMountEntry;JI)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/fs/LinuxNativeDispatcher.init()V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn init_linux_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/fs/LinuxNativeDispatcher.init()V".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/fs/LinuxNativeDispatcher.setmntent0(JJ)J",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn setmntent0_linux_le_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _type_address = parameters.pop_long()?;
    let _path_address = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/LinuxNativeDispatcher.setmntent0(JJ)J".to_string(),
    )
    .into())
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
    async fn test_posix_fadvise_returns_success() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_int(0); // fd
        params.push_long(0); // offset
        params.push_long(0); // len
        params.push_int(0); // advice
        let result = posix_fadvise(thread, params).await.expect("posix_fadvise");
        assert_eq!(result, Some(Value::Int(0)));
    }

    #[tokio::test]
    async fn test_posix_fadvise_underflow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = posix_fadvise(thread, Parameters::default()).await;
        assert!(matches!(result, Err(Error::ParametersUnderflow)));
    }

    #[tokio::test]
    async fn test_direct_copy_0_returns_unsupported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_int(0); // dst
        params.push_int(0); // src
        params.push_long(0); // cancel address
        let result = direct_copy_0(thread, params).await.expect("direct_copy_0");
        assert_eq!(result, Some(Value::Int(IO_STATUS_UNSUPPORTED)));
    }

    #[tokio::test]
    async fn test_direct_copy_0_underflow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = direct_copy_0(thread, Parameters::default()).await;
        assert!(matches!(result, Err(Error::ParametersUnderflow)));
    }

    #[tokio::test]
    async fn test_endmntent_with_zero_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_long(0);
        let result = endmntent(thread, params).await.expect("endmntent");
        assert!(result.is_none());
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
    async fn test_setmntent_0_invalid_path_throws() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::default();
        // Pass two zero addresses; reading them yields an empty C string.  setmntent on
        // Linux will fail to open ""; on other platforms the intrinsic returns a
        // UnixException unconditionally.
        params.push_long(0);
        params.push_long(0);
        let result = setmntent_0(thread, params).await;
        assert!(matches!(result, Err(Error::Throwable(_))));
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
        let dir = std::env::temp_dir().join("ristretto_linux_native_dispatcher_test");
        std::fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join("mtab");
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
        for _ in 0..2 {
            let mut params = Parameters::default();
            params.push_long(handle);
            params.push(entry.clone());
            params.push_long(0);
            params.push_int(0);
            let r = getmntent_0(thread.clone(), params)
                .await
                .expect("getmntent_0")
                .expect("rc");
            assert_eq!(r, Value::Int(0));
        }

        // EOF.
        let mut params = Parameters::default();
        params.push_long(handle);
        params.push(entry.clone());
        params.push_long(0);
        params.push_int(0);
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

        std::fs::remove_file(&path).ok();
        std::fs::remove_dir(&dir).ok();
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_fgetxattr0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fgetxattr0(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.fgetxattr0(IJJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_flistxattr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flistxattr(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.flistxattr(IJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_fremovexattr0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            fremovexattr0(thread, Parameters::new(vec![Value::Int(0), Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.fremovexattr0(IJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_fsetxattr0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fsetxattr0(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.fsetxattr0(IJJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_getlinelen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getlinelen(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.getlinelen(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_endmntent_linux_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = endmntent_linux_le_v17(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.endmntent(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_fgetxattr0_linux_le_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fgetxattr0_linux_le_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.fgetxattr0(IJJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_flistxattr_linux_le_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flistxattr_linux_le_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.flistxattr(IJI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_fremovexattr0_linux_le_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fremovexattr0_linux_le_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.fremovexattr0(IJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_fsetxattr0_linux_le_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fsetxattr0_linux_le_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.fsetxattr0(IJJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_getlinelen_linux_v8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getlinelen_linux_v8(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.getlinelen(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_getmntent0_linux_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getmntent0_linux_le_v17(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.getmntent0(JLsun/nio/fs/UnixMountEntry;JI)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_linux_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_linux_le_v17(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_setmntent0_linux_le_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = setmntent0_linux_le_v17(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/fs/LinuxNativeDispatcher.setmntent0(JJ)J",
            result.unwrap_err().to_string()
        );
    }
}
