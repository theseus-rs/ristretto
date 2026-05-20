use bitflags::bitflags;
#[cfg(target_family = "unix")]
use ristretto_classfile::VersionSpecification::{
    Any, Between, Equal, GreaterThanOrEqual, LessThanOrEqual,
};
#[cfg(target_family = "unix")]
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17, JAVA_21, JAVA_25};
#[cfg(target_family = "unix")]
use ristretto_classloader::Reference;
use ristretto_classloader::Value;
#[cfg(target_family = "unix")]
use ristretto_macros::async_method;
#[cfg(target_family = "unix")]
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
#[cfg(target_family = "unix")]
use ristretto_types::JavaError::NullPointerException;
#[cfg(target_family = "unix")]
use ristretto_types::Parameters;
use ristretto_types::Result;
use ristretto_types::VM;
#[cfg(target_family = "unix")]
use std::ffi::CString;
#[cfg(target_family = "unix")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_family = "unix")]
use std::sync::Arc;

#[cfg(target_family = "unix")]
use super::managed_files;
#[cfg(target_family = "unix")]
use ristretto_types::Thread;

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SupportsFlags: i32 {
        /// Supports openat and other *at calls.
        const OPENAT = 1 << 1;  // syscalls
        /// Supports futimes or futimesat
        const FUTIMES = 1 << 2;
        /// Supports futimens
        const FUTIMENS = 1 << 3;
        /// Supports lutimes
        const LUTIMES = 1 << 4;
        /// Supports extended attributes
        const XATTR = 1 << 5;
        /// Supports file birth (creation) time attribute
        const BIRTHTIME = 1 << 16; // other features
    }
}

#[cfg(target_family = "unix")]
#[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn set_unix_metadata_fields(
    object: &mut ristretto_classloader::Object,
    metadata: &std::fs::Metadata,
) -> Result<()> {
    object.set_value("st_mode", Value::Int(metadata.mode() as i32))?;
    object.set_value("st_ino", Value::Long(metadata.ino() as i64))?;
    object.set_value("st_dev", Value::Long(metadata.dev() as i64))?;
    object.set_value("st_rdev", Value::Long(metadata.rdev() as i64))?;
    object.set_value("st_nlink", Value::Int(metadata.nlink() as i32))?;
    object.set_value("st_uid", Value::Int(metadata.uid() as i32))?;
    object.set_value("st_gid", Value::Int(metadata.gid() as i32))?;
    object.set_value("st_size", Value::Long(metadata.size() as i64))?;
    object.set_value("st_atime_sec", Value::Long(metadata.atime()))?;
    object.set_value("st_atime_nsec", Value::Long(metadata.atime_nsec()))?;
    object.set_value("st_mtime_sec", Value::Long(metadata.mtime()))?;
    object.set_value("st_mtime_nsec", Value::Long(metadata.mtime_nsec()))?;
    object.set_value("st_ctime_sec", Value::Long(metadata.ctime()))?;
    object.set_value("st_ctime_nsec", Value::Long(metadata.ctime_nsec()))?;
    Ok(())
}

#[cfg(not(target_family = "unix"))]
fn set_unix_metadata_fields(
    object: &mut ristretto_classloader::Object,
    _metadata: &std::fs::Metadata,
) -> Result<()> {
    object.set_value("st_mode", Value::Int(0))?;
    object.set_value("st_ino", Value::Long(0))?;
    object.set_value("st_dev", Value::Long(0))?;
    object.set_value("st_rdev", Value::Long(0))?;
    object.set_value("st_nlink", Value::Int(0))?;
    object.set_value("st_uid", Value::Int(0))?;
    object.set_value("st_gid", Value::Int(0))?;
    object.set_value("st_size", Value::Long(0))?;
    object.set_value("st_atime_sec", Value::Long(0))?;
    object.set_value("st_atime_nsec", Value::Long(0))?;
    object.set_value("st_mtime_sec", Value::Long(0))?;
    object.set_value("st_mtime_nsec", Value::Long(0))?;
    object.set_value("st_ctime_sec", Value::Long(0))?;
    object.set_value("st_ctime_nsec", Value::Long(0))?;
    Ok(())
}

/// Read a null-terminated path from native memory
fn read_native_path<V: VM>(vm: &V, address: i64) -> Result<String> {
    let path_bytes = vm.native_memory().read_cstring(address);
    String::from_utf8(path_bytes)
        .map_err(|error| InternalError(format!("Invalid path encoding: {error}")))
}

/// Get errno from last OS error
#[cfg(target_family = "unix")]
fn last_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(5)
}

#[cfg(target_family = "unix")]
use super::common::throw_unix_exception;

/// Convert a path string to a `CString` for libc calls
#[cfg(target_family = "unix")]
fn to_cstring(path: &str) -> Result<CString> {
    CString::new(path.as_bytes())
        .map_err(|e| InternalError(format!("Invalid path (contains null byte): {e}")))
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.access0(JI)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn access_0_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let amode = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(unsafe_code)]
        let result = unsafe { libc::access(c_path.as_ptr(), amode) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = amode;
        if std::path::Path::new(&path_str).exists() {
            Ok(None)
        } else {
            Err(throw_unix_exception(&thread, 2).await)
        }
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.access0(JI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn access_0_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mode = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;

    let vm = thread.vm()?;
    let path_bytes = vm.native_memory().read_cstring(path_address);
    let path_str = String::from_utf8(path_bytes)
        .map_err(|error| InternalError(format!("Invalid path encoding: {error}")))?;

    if std::path::Path::new(&path_str).exists() {
        Ok(Some(Value::Int(0)))
    } else {
        Ok(Some(Value::Int(-1)))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.chmod0(JI)V", Any)]
#[async_method]
pub async fn chmod_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mode = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(clippy::cast_sign_loss, unsafe_code)]
        #[cfg_attr(target_os = "macos", expect(clippy::cast_possible_truncation))]
        let result = unsafe { libc::chmod(c_path.as_ptr(), mode as libc::mode_t) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = mode;
        Err(InternalError(
            "chmod is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.chown0(JII)V", Any)]
#[async_method]
pub async fn chown_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let gid = parameters.pop_int()?;
    let uid = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(clippy::cast_sign_loss, unsafe_code)]
        let result =
            unsafe { libc::chown(c_path.as_ptr(), uid as libc::uid_t, gid as libc::gid_t) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (uid, gid);
        Err(InternalError(
            "chown is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.close(I)V", Equal(JAVA_8))]
#[async_method]
pub async fn close<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    close_0(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.close0(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn close_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    managed_files::close(vm.file_handles(), i64::from(fd)).await;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.closedir(J)V", Any)]
#[async_method]
pub async fn closedir<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let dir = parameters.pop_long()?;

    #[cfg(target_family = "unix")]
    {
        if dir != 0 {
            #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let dir_ptr = dir as usize as *mut libc::DIR;
            #[expect(unsafe_code)]
            unsafe {
                libc::closedir(dir_ptr)
            };
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = dir;
        Err(InternalError(
            "closedir is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.dup(I)I", Any)]
#[async_method]
pub async fn dup<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(unsafe_code)]
        let result = unsafe { libc::dup(fd) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(Some(Value::Int(result)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = fd;
        Err(InternalError(
            "dup is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.exists0(J)Z",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn exists_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;
    let exists = std::path::Path::new(&path_str).exists();
    Ok(Some(Value::from(exists)))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.fclose(J)V", LessThanOrEqual(JAVA_11))]
#[async_method]
pub async fn fclose_1<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fp = parameters.pop_long()?;

    #[cfg(target_family = "unix")]
    {
        if fp != 0 {
            #[expect(unsafe_code, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            let result = unsafe { libc::fclose(fp as usize as *mut libc::FILE) };
            if result != 0 {
                return Err(throw_unix_exception(&thread, last_errno()).await);
            }
        }
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = fp;
    }
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fchmod(II)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn fchmod<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    fchmod_0(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fchmod0(II)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn fchmod_0<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mode = parameters.pop_int()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(clippy::cast_sign_loss, unsafe_code)]
        #[cfg_attr(target_os = "macos", expect(clippy::cast_possible_truncation))]
        let result = unsafe { libc::fchmod(fd, mode as libc::mode_t) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (fd, mode);
        Err(InternalError(
            "fchmod is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.fchmodat0(IJII)V", Equal(JAVA_25))]
#[async_method]
pub async fn fchmodat_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags = parameters.pop_int()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let mode = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let dirfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(clippy::cast_sign_loss, unsafe_code)]
        #[cfg_attr(target_os = "macos", expect(clippy::cast_possible_truncation))]
        let result = unsafe { libc::fchmodat(dirfd, c_path.as_ptr(), mode as libc::mode_t, flags) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (dirfd, flags);
        Err(InternalError(
            "fchmodat is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fchmodatNoFollowSupported0()Z",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn fchmodat_no_follow_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "macos")]
    {
        Ok(Some(Value::from(true)))
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(Some(Value::from(false)))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fchown(III)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn fchown<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    fchown_0(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fchown0(III)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn fchown_0<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let gid = parameters.pop_int()?;
    let uid = parameters.pop_int()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(clippy::cast_sign_loss)]
        #[expect(unsafe_code)]
        let result = unsafe { libc::fchown(fd, uid as libc::uid_t, gid as libc::gid_t) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (fd, uid, gid);
        Err(InternalError(
            "fchown is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.fdopendir(I)J", Any)]
#[async_method]
pub async fn fdopendir<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(unsafe_code)]
        let dir = unsafe { libc::fdopendir(fd) };
        if dir.is_null() {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        #[expect(clippy::cast_possible_wrap)]
        let dir = dir as usize as i64;
        let vm = thread.vm()?;
        if let Some(handle) = vm.file_handles().remove(&i64::from(fd)).await {
            std::mem::forget(handle);
        }
        Ok(Some(Value::Long(dir)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = fd;
        Err(InternalError(
            "fdopendir is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fgetxattr0(IJJI)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn fgetxattr_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value_len = parameters.pop_int()?;
    let value_address = parameters.pop_long()?;
    let name_address = parameters.pop_long()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let name_str = read_native_path(&*vm, name_address)?;

    #[cfg(target_os = "macos")]
    {
        let c_name = to_cstring(&name_str)?;
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        #[expect(unsafe_code)]
        let result = unsafe {
            libc::fgetxattr(
                fd,
                c_name.as_ptr(),
                value_address as usize as *mut libc::c_void,
                value_len as usize,
                0,
                0,
            )
        };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(Some(Value::Int(i32::try_from(result)?)))
    }
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    {
        let c_name = to_cstring(&name_str)?;
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        #[expect(unsafe_code)]
        let result = unsafe {
            libc::fgetxattr(
                fd,
                c_name.as_ptr(),
                value_address as usize as *mut libc::c_void,
                value_len as usize,
            )
        };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(Some(Value::Int(i32::try_from(result)?)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (fd, value_address, value_len);
        Err(InternalError(
            "fgetxattr is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.flistxattr(IJI)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn flistxattr<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let size = parameters.pop_int()?;
    let list_address = parameters.pop_long()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_os = "macos")]
    {
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        #[expect(unsafe_code)]
        let result = unsafe {
            libc::flistxattr(
                fd,
                list_address as usize as *mut libc::c_char,
                size as usize,
                0,
            )
        };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(Some(Value::Int(i32::try_from(result)?)))
    }
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    {
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        #[expect(unsafe_code)]
        let result = unsafe {
            libc::flistxattr(
                fd,
                list_address as usize as *mut libc::c_char,
                size as usize,
            )
        };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(Some(Value::Int(i32::try_from(result)?)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (fd, list_address, size);
        Err(InternalError(
            "flistxattr is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fopen0(JJ)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fopen_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mode_address = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let mode_str = read_native_path(&*vm, mode_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        let c_mode = to_cstring(&mode_str)?;
        #[expect(unsafe_code)]
        let fp = unsafe { libc::fopen(c_path.as_ptr(), c_mode.as_ptr()) };
        if fp.is_null() {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        #[expect(clippy::cast_possible_wrap)]
        {
            Ok(Some(Value::Long(fp as usize as i64)))
        }
    }
    #[cfg(not(target_family = "unix"))]
    {
        Err(InternalError(
            "fopen is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fpathconf(II)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn fpathconf<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name = parameters.pop_int()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(unsafe_code)]
        let result = unsafe { libc::fpathconf(fd, name) };
        if result == -1 {
            let errno = last_errno();
            if errno != 0 {
                return Err(throw_unix_exception(&thread, errno).await);
            }
        }
        Ok(Some(Value::Long(result)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (fd, name);
        Err(InternalError(
            "fpathconf is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fremovexattr0(IJ)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn fremovexattr_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name_address = parameters.pop_long()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let name_str = read_native_path(&*vm, name_address)?;

    #[cfg(target_os = "macos")]
    {
        let c_name = to_cstring(&name_str)?;
        #[expect(unsafe_code)]
        let result = unsafe { libc::fremovexattr(fd, c_name.as_ptr(), 0) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    {
        let c_name = to_cstring(&name_str)?;
        #[expect(unsafe_code)]
        let result = unsafe { libc::fremovexattr(fd, c_name.as_ptr()) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = fd;
        Err(InternalError(
            "fremovexattr is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fsetxattr0(IJJI)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn fsetxattr_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value_len = parameters.pop_int()?;
    let value_address = parameters.pop_long()?;
    let name_address = parameters.pop_long()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let name_str = read_native_path(&*vm, name_address)?;

    #[cfg(target_os = "macos")]
    {
        let c_name = to_cstring(&name_str)?;
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        #[expect(unsafe_code)]
        let result = unsafe {
            libc::fsetxattr(
                fd,
                c_name.as_ptr(),
                value_address as usize as *const libc::c_void,
                value_len as usize,
                0,
                0,
            )
        };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(all(target_family = "unix", not(target_os = "macos")))]
    {
        let c_name = to_cstring(&name_str)?;
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        #[expect(unsafe_code)]
        let result = unsafe {
            libc::fsetxattr(
                fd,
                c_name.as_ptr(),
                value_address as usize as *const libc::c_void,
                value_len as usize,
                0,
            )
        };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (fd, value_address, value_len);
        Err(InternalError(
            "fsetxattr is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fstat(ILsun/nio/fs/UnixFileAttributes;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn fstat<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    fstat_0(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fstat0(ILsun/nio/fs/UnixFileAttributes;)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn fstat_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let attributes = parameters.pop()?;
    if attributes.is_null() {
        return Err(NullPointerException(Some("attributes is null".to_string())).into());
    }
    let fd = parameters.pop_int()?;

    let vm = thread.vm()?;
    let metadata = match managed_files::metadata(vm.file_handles(), i64::from(fd)).await {
        Ok(m) => m,
        Err(e) => {
            let errno = e.raw_os_error().unwrap_or(9 /* EBADF */);
            return Err(throw_unix_exception(&thread, errno).await);
        }
    };

    let mut guard = attributes.as_reference_mut()?;
    let Reference::Object(object) = &mut *guard else {
        return Err(InternalError(
            "fstat0: attributes is not an object".to_string(),
        ));
    };

    set_unix_metadata_fields(object, &metadata)?;

    #[cfg(target_os = "macos")]
    {
        use std::os::macos::fs::MetadataExt as MacMetadataExt;
        object.set_value("st_birthtime_sec", Value::Long(metadata.st_birthtime()))?;
        object.set_value(
            "st_birthtime_nsec",
            Value::Long(metadata.st_birthtime_nsec()),
        )?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        object.set_value("st_birthtime_sec", Value::Long(0))?;
        object.set_value("st_birthtime_nsec", Value::Long(0))?;
    }

    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.fstatat0(IJILsun/nio/fs/UnixFileAttributes;)V",
    Any
)]
#[async_method]
pub async fn fstatat_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let attributes = parameters.pop()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let flag = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let dfd = parameters.pop_int()?;

    if attributes.is_null() {
        return Err(NullPointerException(Some("attributes is null".to_string())).into());
    }

    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    // Determine whether to follow symlinks based on the AT_SYMLINK_NOFOLLOW flag.
    // The dfd parameter is ignored; the path is expected to be absolute.
    #[cfg(target_family = "unix")]
    let follow_links = flag & libc::AT_SYMLINK_NOFOLLOW == 0;
    #[cfg(not(target_family = "unix"))]
    let follow_links = true;

    let _ = dfd;

    let metadata = if follow_links {
        std::fs::metadata(&path_str)
    } else {
        std::fs::symlink_metadata(&path_str)
    };

    let metadata = match metadata {
        Ok(m) => m,
        Err(e) => {
            let errno = e.raw_os_error().unwrap_or(2);
            return Err(throw_unix_exception(&thread, errno).await);
        }
    };

    let mut guard = attributes.as_reference_mut()?;
    let Reference::Object(object) = &mut *guard else {
        return Err(InternalError(
            "fstatat0: attributes is not an object".to_string(),
        ));
    };

    set_unix_metadata_fields(object, &metadata)?;

    #[cfg(target_os = "macos")]
    {
        use std::os::macos::fs::MetadataExt as MacMetadataExt;
        object.set_value("st_birthtime_sec", Value::Long(metadata.st_birthtime()))?;
        object.set_value(
            "st_birthtime_nsec",
            Value::Long(metadata.st_birthtime_nsec()),
        )?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        object.set_value("st_birthtime_sec", Value::Long(0))?;
        object.set_value("st_birthtime_nsec", Value::Long(0))?;
    }

    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.futimens(IJJ)V", Equal(JAVA_17))]
#[async_method]
pub async fn futimens<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    futimens_0(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.futimens0(IJJ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn futimens_0<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let modification_time = parameters.pop_long()?;
    let access_time = parameters.pop_long()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_family = "unix")]
    {
        let times = [
            libc::timespec {
                tv_sec: access_time / 1_000_000_000,
                tv_nsec: access_time % 1_000_000_000,
            },
            libc::timespec {
                tv_sec: modification_time / 1_000_000_000,
                tv_nsec: modification_time % 1_000_000_000,
            },
        ];
        #[expect(unsafe_code)]
        let result = unsafe { libc::futimens(fd, times.as_ptr()) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (fd, access_time, modification_time);
        Err(InternalError(
            "futimens is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.futimes(IJJ)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn futimes<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    futimes_0(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.futimes0(IJJ)V", Equal(JAVA_21))]
#[async_method]
pub async fn futimes_0<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let modification_time = parameters.pop_long()?;
    let access_time = parameters.pop_long()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_family = "unix")]
    {
        let times = [
            libc::timeval {
                tv_sec: access_time / 1_000_000,
                tv_usec: (access_time % 1_000_000) as _,
            },
            libc::timeval {
                tv_sec: modification_time / 1_000_000,
                tv_usec: (modification_time % 1_000_000) as _,
            },
        ];
        #[expect(unsafe_code)]
        let result = unsafe { libc::futimes(fd, times.as_ptr()) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (fd, access_time, modification_time);
        Err(InternalError(
            "futimes is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getcwd()[B", Any)]
#[async_method]
pub async fn getcwd<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let current_dir_path =
        std::env::current_dir().map_err(|error| InternalError(format!("getcwd: {error}")))?;
    let current_dir_str = current_dir_path.to_string_lossy();
    let current_dir_bytes = current_dir_str.as_bytes();
    let current_dir: &[i8] = zerocopy::transmute_ref!(current_dir_bytes);
    let current_dir_bytes = Value::new_object(
        thread.vm()?.garbage_collector(),
        Reference::from(current_dir.to_vec()),
    );
    Ok(Some(current_dir_bytes))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getgrgid(I)[B", Any)]
#[async_method]
pub async fn getgrgid<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let gid = parameters.pop_int()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(clippy::cast_sign_loss)]
        #[expect(unsafe_code)]
        let gr = unsafe { libc::getgrgid(gid as libc::gid_t) };
        if gr.is_null() {
            return Ok(Some(Value::Object(None)));
        }
        #[expect(unsafe_code)]
        let name = unsafe { std::ffi::CStr::from_ptr((*gr).gr_name) };
        let name_bytes = name.to_bytes();
        let name_i8: &[i8] = zerocopy::transmute_ref!(name_bytes);
        let vm = thread.vm()?;
        Ok(Some(Value::new_object(
            vm.garbage_collector(),
            Reference::from(name_i8.to_vec()),
        )))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = gid;
        Err(InternalError(
            "getgrgid is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getgrnam0(J)I", Any)]
#[async_method]
pub async fn getgrnam_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let name_str = read_native_path(&*vm, name_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_name = to_cstring(&name_str)?;
        #[expect(unsafe_code)]
        let gr = unsafe { libc::getgrnam(c_name.as_ptr()) };
        if gr.is_null() {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        #[expect(clippy::cast_possible_wrap)]
        #[expect(unsafe_code)]
        let gid = unsafe { (*gr).gr_gid as i32 };
        Ok(Some(Value::Int(gid)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        Err(InternalError(
            "getgrnam is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.getlinelen(J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn getlinelen<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fp = parameters.pop_long()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let fp_ptr = fp as usize as *mut libc::FILE;
        let mut line_buf: *mut libc::c_char = std::ptr::null_mut();
        let mut line_len: libc::size_t = 0;
        #[expect(unsafe_code)]
        let result = unsafe {
            libc::getline(
                std::ptr::from_mut(&mut line_buf),
                std::ptr::from_mut(&mut line_len),
                fp_ptr,
            )
        };
        let len = if result == -1 {
            -1
        } else {
            #[expect(unsafe_code)]
            let len = unsafe { libc::strlen(line_buf) };
            i32::try_from(len).unwrap_or(-1)
        };
        if !line_buf.is_null() {
            #[expect(unsafe_code)]
            unsafe {
                libc::free(line_buf.cast::<libc::c_void>());
            };
        }
        Ok(Some(Value::Int(len)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = fp;
        Err(InternalError(
            "getlinelen is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getpwnam0(J)I", Any)]
#[async_method]
pub async fn getpwnam_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let name_str = read_native_path(&*vm, name_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_name = to_cstring(&name_str)?;
        #[expect(unsafe_code)]
        let pw = unsafe { libc::getpwnam(c_name.as_ptr()) };
        if pw.is_null() {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        #[expect(clippy::cast_possible_wrap)]
        #[expect(unsafe_code)]
        let uid = unsafe { (*pw).pw_uid as i32 };
        Ok(Some(Value::Int(uid)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        Err(InternalError(
            "getpwnam is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.getpwuid(I)[B", Any)]
#[async_method]
pub async fn getpwuid<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let uid = parameters.pop_int()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(clippy::cast_sign_loss)]
        #[expect(unsafe_code)]
        let pw = unsafe { libc::getpwuid(uid as libc::uid_t) };
        if pw.is_null() {
            return Ok(Some(Value::Object(None)));
        }
        #[expect(unsafe_code)]
        let name = unsafe { std::ffi::CStr::from_ptr((*pw).pw_name) };
        let name_bytes = name.to_bytes();
        let name_i8: &[i8] = zerocopy::transmute_ref!(name_bytes);
        let vm = thread.vm()?;
        Ok(Some(Value::new_object(
            vm.garbage_collector(),
            Reference::from(name_i8.to_vec()),
        )))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = uid;
        Err(InternalError(
            "getpwuid is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.init()I", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg_attr(
        not(any(target_family = "unix", target_os = "macos", target_os = "linux")),
        expect(unused_mut)
    )]
    let mut capabilities = SupportsFlags::empty();

    #[cfg(target_family = "unix")]
    {
        capabilities |= SupportsFlags::OPENAT;
        capabilities |= SupportsFlags::FUTIMENS;
        capabilities |= SupportsFlags::XATTR;
    }

    #[cfg(target_os = "macos")]
    {
        capabilities |= SupportsFlags::FUTIMES;
        capabilities |= SupportsFlags::LUTIMES;
        capabilities |= SupportsFlags::BIRTHTIME;
    }

    #[cfg(target_os = "linux")]
    {
        capabilities |= SupportsFlags::FUTIMES;
        capabilities |= SupportsFlags::LUTIMES;
    }

    Ok(Some(Value::Int(capabilities.bits())))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.lchown0(JII)V", Any)]
#[async_method]
pub async fn lchown_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let gid = parameters.pop_int()?;
    let uid = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(clippy::cast_sign_loss, unsafe_code)]
        let result =
            unsafe { libc::lchown(c_path.as_ptr(), uid as libc::uid_t, gid as libc::gid_t) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (uid, gid);
        Err(InternalError(
            "lchown is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.link0(JJ)V", Any)]
#[async_method]
pub async fn link_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let new_address = parameters.pop_long()?;
    let existing_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let existing_path = read_native_path(&*vm, existing_address)?;
    let new_path = read_native_path(&*vm, new_address)?;

    if let Err(e) = std::fs::hard_link(&existing_path, &new_path) {
        let errno = e.raw_os_error().unwrap_or(5);
        return Err(throw_unix_exception(&thread, errno).await);
    }
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.lstat0(JLsun/nio/fs/UnixFileAttributes;)V",
    Any
)]
#[async_method]
pub async fn lstat_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let attributes = parameters.pop()?;
    let path_address = parameters.pop_long()?;

    if attributes.is_null() {
        return Err(NullPointerException(Some("attributes is null".to_string())).into());
    }

    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    let metadata = match std::fs::symlink_metadata(&path_str) {
        Ok(m) => m,
        Err(e) => {
            let errno = e.raw_os_error().unwrap_or(2);
            return Err(throw_unix_exception(&thread, errno).await);
        }
    };

    let mut guard = attributes.as_reference_mut()?;
    let Reference::Object(object) = &mut *guard else {
        return Err(InternalError(
            "lstat0: attributes is not an object".to_string(),
        ));
    };

    set_unix_metadata_fields(object, &metadata)?;

    #[cfg(target_os = "macos")]
    {
        use std::os::macos::fs::MetadataExt as MacMetadataExt;
        object.set_value("st_birthtime_sec", Value::Long(metadata.st_birthtime()))?;
        object.set_value(
            "st_birthtime_nsec",
            Value::Long(metadata.st_birthtime_nsec()),
        )?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        object.set_value("st_birthtime_sec", Value::Long(0))?;
        object.set_value("st_birthtime_nsec", Value::Long(0))?;
    }

    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.lutimes0(JJJ)V",
    Between(JAVA_17, JAVA_21)
)]
#[async_method]
pub async fn lutimes_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let modification_time = parameters.pop_long()?;
    let access_time = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        let times = [
            libc::timeval {
                tv_sec: access_time / 1_000_000,
                tv_usec: (access_time % 1_000_000) as _,
            },
            libc::timeval {
                tv_sec: modification_time / 1_000_000,
                tv_usec: (modification_time % 1_000_000) as _,
            },
        ];
        #[expect(unsafe_code)]
        let result = unsafe { libc::lutimes(c_path.as_ptr(), times.as_ptr()) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (access_time, modification_time);
        Err(InternalError(
            "lutimes is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.mkdir0(JI)V", Any)]
#[async_method]
pub async fn mkdir_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mode = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(clippy::cast_sign_loss, unsafe_code)]
        #[cfg_attr(target_os = "macos", expect(clippy::cast_possible_truncation))]
        let result = unsafe { libc::mkdir(c_path.as_ptr(), mode as libc::mode_t) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = mode;
        if let Err(e) = std::fs::create_dir(&path_str) {
            let errno = e.raw_os_error().unwrap_or(5);
            return Err(throw_unix_exception(&thread, errno).await);
        }
        Ok(None)
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.mknod0(JIJ)V", Any)]
#[async_method]
pub async fn mknod_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let dev = parameters.pop_long()?;
    let mode = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(clippy::cast_sign_loss)]
        #[cfg_attr(target_os = "macos", expect(clippy::cast_possible_truncation))]
        #[expect(unsafe_code)]
        let result =
            unsafe { libc::mknod(c_path.as_ptr(), mode as libc::mode_t, dev as libc::dev_t) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (mode, dev);
        Err(InternalError(
            "mknod is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.open0(JII)I", Any)]
#[async_method]
pub async fn open_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mode = parameters.pop_int()?;
    let flags = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;

    let vm = thread.vm()?;
    // Read the null-terminated path from managed native memory
    let path_bytes = vm.native_memory().read_cstring(path_address);
    let path_str = String::from_utf8(path_bytes)
        .map_err(|error| InternalError(format!("Invalid path encoding: {error}")))?;

    match managed_files::open(
        vm.file_handles(),
        vm.resource_manager(),
        &path_str,
        flags,
        mode,
    )
    .await
    {
        Ok(fd) => Ok(Some(Value::Int(i32::try_from(fd)?))),
        Err(error) => {
            let errno = error.raw_os_error().unwrap_or(2);
            Err(throw_unix_exception(&thread, errno).await)
        }
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.openat0(IJII)I", Any)]
#[async_method]
pub async fn openat_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mode = parameters.pop_int()?;
    let flags = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let dfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(clippy::cast_sign_loss)]
        #[expect(unsafe_code)]
        let result = unsafe { libc::openat(dfd, c_path.as_ptr(), flags, mode as libc::c_uint) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(Some(Value::Int(result)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (dfd, flags, mode);
        Err(InternalError(
            "openat is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.opendir0(J)J", Any)]
#[async_method]
pub async fn opendir_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(unsafe_code)]
        let dir = unsafe { libc::opendir(c_path.as_ptr()) };
        if dir.is_null() {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        #[expect(clippy::cast_possible_wrap)]
        {
            Ok(Some(Value::Long(dir as usize as i64)))
        }
    }
    #[cfg(not(target_family = "unix"))]
    {
        Err(InternalError(
            "opendir is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.pathconf0(JI)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn pathconf_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(unsafe_code)]
        let result = unsafe { libc::pathconf(c_path.as_ptr(), name) };
        if result == -1 {
            let errno = last_errno();
            if errno != 0 {
                return Err(throw_unix_exception(&thread, errno).await);
            }
        }
        Ok(Some(Value::Long(result)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = name;
        Err(InternalError(
            "pathconf is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.read(IJI)I", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn read<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    read_0(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.read0(IJI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn read_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd = parameters.pop_int()?;

    let count = usize::try_from(count)?;
    let mut buf = vec![0u8; count];

    let vm = thread.vm()?;
    match managed_files::read(vm.file_handles(), i64::from(fd), &mut buf).await {
        Ok(n) => {
            if n > 0 {
                vm.native_memory().write_bytes(address, &buf[..n]);
            }
            Ok(Some(Value::Int(i32::try_from(n)?)))
        }
        Err(error) => {
            let errno = error.raw_os_error().unwrap_or(5 /* EIO */);
            Err(throw_unix_exception(&thread, errno).await)
        }
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.readdir(J)[B",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn readdir<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    readdir_0(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.readdir0(J)[B",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn readdir_0<T: Thread + 'static>(
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))] thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let dir = parameters.pop_long()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let dir_ptr = dir as usize as *mut libc::DIR;
        #[expect(unsafe_code)]
        let entry = unsafe { libc::readdir(dir_ptr) };
        if entry.is_null() {
            return Ok(Some(Value::Object(None)));
        }
        #[expect(unsafe_code)]
        let d_name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) };
        let name_bytes = d_name.to_bytes();
        let name_i8: &[i8] = zerocopy::transmute_ref!(name_bytes);
        let vm = thread.vm()?;
        Ok(Some(Value::new_object(
            vm.garbage_collector(),
            Reference::from(name_i8.to_vec()),
        )))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = dir;
        Err(InternalError(
            "readdir is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.readlink0(J)[B", Any)]
#[async_method]
pub async fn readlink_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    let target = match std::fs::read_link(&path_str) {
        Ok(t) => t,
        Err(e) => {
            let errno = e.raw_os_error().unwrap_or(22);
            return Err(throw_unix_exception(&thread, errno).await);
        }
    };

    let target_bytes = target.to_string_lossy().into_owned().into_bytes();
    let target_i8: &[i8] = zerocopy::transmute_ref!(target_bytes.as_slice());
    Ok(Some(Value::new_object(
        vm.garbage_collector(),
        Reference::from(target_i8.to_vec()),
    )))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.realpath0(J)[B", Any)]
#[async_method]
pub async fn realpath_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;

    let vm = thread.vm()?;
    let path_bytes = vm.native_memory().read_cstring(path_address);
    let path_str = String::from_utf8(path_bytes)
        .map_err(|error| InternalError(format!("Invalid path encoding: {error}")))?;

    let canonical = match std::fs::canonicalize(&path_str) {
        Ok(c) => c,
        Err(error) => {
            let errno = error.raw_os_error().unwrap_or(2);
            return Err(throw_unix_exception(&thread, errno).await);
        }
    };

    let canonical_bytes = canonical.to_string_lossy().into_owned().into_bytes();
    let canonical_i8: &[i8] = zerocopy::transmute_ref!(canonical_bytes.as_slice());
    Ok(Some(Value::new_object(
        thread.vm()?.garbage_collector(),
        Reference::from(canonical_i8.to_vec()),
    )))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.rename0(JJ)V", Any)]
#[async_method]
pub async fn rename_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let to_address = parameters.pop_long()?;
    let from_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let from_path = read_native_path(&*vm, from_address)?;
    let to_path = read_native_path(&*vm, to_address)?;

    if let Err(e) = std::fs::rename(&from_path, &to_path) {
        let errno = e.raw_os_error().unwrap_or(5);
        return Err(throw_unix_exception(&thread, errno).await);
    }
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.renameat0(IJIJ)V", Any)]
#[async_method]
pub async fn renameat_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let to_address = parameters.pop_long()?;
    let tofd = parameters.pop_int()?;
    let from_address = parameters.pop_long()?;
    let fromfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let from_path = read_native_path(&*vm, from_address)?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let to_path = read_native_path(&*vm, to_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_from = to_cstring(&from_path)?;
        let c_to = to_cstring(&to_path)?;
        #[expect(unsafe_code)]
        let result = unsafe { libc::renameat(fromfd, c_from.as_ptr(), tofd, c_to.as_ptr()) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (fromfd, tofd);
        Err(InternalError(
            "renameat is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.rewind(J)V", Any)]
#[async_method]
pub async fn rewind<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let dir = parameters.pop_long()?;

    #[cfg(target_family = "unix")]
    {
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let dir_ptr = dir as usize as *mut libc::DIR;
        #[expect(unsafe_code)]
        unsafe {
            libc::rewinddir(dir_ptr);
        };
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = dir;
        Err(InternalError(
            "rewinddir is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.rmdir0(J)V", Any)]
#[async_method]
pub async fn rmdir_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    if let Err(e) = std::fs::remove_dir(&path_str) {
        let errno = e.raw_os_error().unwrap_or(5);
        return Err(throw_unix_exception(&thread, errno).await);
    }
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.stat0(JLsun/nio/fs/UnixFileAttributes;)V",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn stat_0_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    let _ = stat_0_1(thread, parameters).await?;
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.stat0(JLsun/nio/fs/UnixFileAttributes;)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn stat_0_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let attributes = parameters.pop()?;
    if attributes.is_null() {
        return Err(NullPointerException(Some("attributes is null".to_string())).into());
    }
    let path_address = parameters.pop_long()?;

    // Read the null-terminated path from managed native memory
    let vm = thread.vm()?;
    let path_bytes = vm.native_memory().read_cstring(path_address);
    let path_str = String::from_utf8(path_bytes)
        .map_err(|error| InternalError(format!("Invalid path encoding: {error}")))?;

    let metadata = match std::fs::metadata(path_str) {
        Ok(m) => m,
        Err(error) => {
            let errno = error.raw_os_error().unwrap_or(2);
            return Ok(Some(Value::Int(errno)));
        }
    };

    let mut guard = attributes.as_reference_mut()?;
    let Reference::Object(object) = &mut *guard else {
        return Err(InternalError(
            "stat0: attributes is not an object".to_string(),
        ));
    };

    set_unix_metadata_fields(object, &metadata)?;

    // Birth time; macOS supports it, Linux may not
    #[cfg(target_os = "macos")]
    {
        use std::os::macos::fs::MetadataExt as MacMetadataExt;
        object.set_value("st_birthtime_sec", Value::Long(metadata.st_birthtime()))?;
        object.set_value(
            "st_birthtime_nsec",
            Value::Long(metadata.st_birthtime_nsec()),
        )?;
    }
    #[cfg(not(target_os = "macos"))]
    {
        object.set_value("st_birthtime_sec", Value::Long(0))?;
        object.set_value("st_birthtime_nsec", Value::Long(0))?;
    }

    Ok(Some(Value::Int(0)))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.stat1(J)I", Between(JAVA_11, JAVA_17))]
#[async_method]
pub async fn stat_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    let metadata = match std::fs::metadata(&path_str) {
        Ok(m) => m,
        Err(e) => {
            let errno = e.raw_os_error().unwrap_or(2);
            return Err(throw_unix_exception(&thread, errno).await);
        }
    };

    #[cfg(target_family = "unix")]
    {
        #[expect(clippy::cast_possible_wrap)]
        let mode = metadata.mode() as i32;
        Ok(Some(Value::Int(mode)))
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = metadata;
        Ok(Some(Value::Int(0)))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.statvfs0(JLsun/nio/fs/UnixFileStoreAttributes;)V",
    Any
)]
#[async_method]
pub async fn statvfs_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let attributes = parameters.pop()?;
    let path_address = parameters.pop_long()?;

    if attributes.is_null() {
        return Err(NullPointerException(Some("attributes is null".to_string())).into());
    }

    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(unsafe_code)]
        let mut buf: libc::statvfs = unsafe { std::mem::zeroed() };
        #[expect(unsafe_code)]
        let result = unsafe { libc::statvfs(c_path.as_ptr(), std::ptr::from_mut(&mut buf)) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }

        let mut guard = attributes.as_reference_mut()?;
        let Reference::Object(object) = &mut *guard else {
            return Err(InternalError(
                "statvfs0: attributes is not an object".to_string(),
            ));
        };

        #[expect(clippy::cast_possible_wrap)]
        {
            object.set_value("f_frsize", Value::Long(buf.f_frsize as i64))?;
        }
        #[cfg(target_os = "macos")]
        {
            object.set_value("f_blocks", Value::Long(i64::from(buf.f_blocks)))?;
            object.set_value("f_bfree", Value::Long(i64::from(buf.f_bfree)))?;
            object.set_value("f_bavail", Value::Long(i64::from(buf.f_bavail)))?;
        }
        #[cfg(not(target_os = "macos"))]
        #[expect(clippy::cast_possible_wrap)]
        {
            object.set_value("f_blocks", Value::Long(buf.f_blocks as i64))?;
            object.set_value("f_bfree", Value::Long(buf.f_bfree as i64))?;
            object.set_value("f_bavail", Value::Long(buf.f_bavail as i64))?;
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        Err(InternalError(
            "statvfs is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.strerror(I)[B", Any)]
#[async_method]
pub async fn strerror<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let errno = parameters.pop_int()?;

    let error = std::io::Error::from_raw_os_error(errno);
    let msg = error.to_string();
    let msg_bytes = msg.into_bytes();
    let msg_i8: &[i8] = zerocopy::transmute_ref!(msg_bytes.as_slice());
    let vm = thread.vm()?;
    Ok(Some(Value::new_object(
        vm.garbage_collector(),
        Reference::from(msg_i8.to_vec()),
    )))
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.symlink0(JJ)V", Any)]
#[async_method]
pub async fn symlink_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let link_address = parameters.pop_long()?;
    let target_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let target_path = read_native_path(&*vm, target_address)?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let link_path = read_native_path(&*vm, link_address)?;

    #[cfg(target_family = "unix")]
    {
        if let Err(e) = std::os::unix::fs::symlink(&target_path, &link_path) {
            let errno = e.raw_os_error().unwrap_or(5);
            return Err(throw_unix_exception(&thread, errno).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        Err(InternalError(
            "symlink is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.unlink0(J)V", Any)]
#[async_method]
pub async fn unlink_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    let path_str = read_native_path(&*vm, path_address)?;

    if let Err(e) = std::fs::remove_file(&path_str) {
        let errno = e.raw_os_error().unwrap_or(5);
        return Err(throw_unix_exception(&thread, errno).await);
    }
    Ok(None)
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.unlinkat0(IJI)V", Any)]
#[async_method]
pub async fn unlinkat_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flag = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let dfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        #[expect(unsafe_code)]
        let result = unsafe { libc::unlinkat(dfd, c_path.as_ptr(), flag) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (dfd, flag);
        Err(InternalError(
            "unlinkat is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.utimes0(JJJ)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn utimes_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let modification_time = parameters.pop_long()?;
    let access_time = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        let times = [
            libc::timeval {
                tv_sec: access_time / 1_000_000,
                tv_usec: (access_time % 1_000_000) as _,
            },
            libc::timeval {
                tv_sec: modification_time / 1_000_000,
                tv_usec: (modification_time % 1_000_000) as _,
            },
        ];
        #[expect(unsafe_code)]
        let result = unsafe { libc::utimes(c_path.as_ptr(), times.as_ptr()) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (access_time, modification_time);
        Err(InternalError(
            "utimes is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method("sun/nio/fs/UnixNativeDispatcher.utimensat0(IJJJI)V", Equal(JAVA_25))]
#[async_method]
pub async fn utimensat_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags = parameters.pop_int()?;
    let modification_time = parameters.pop_long()?;
    let access_time = parameters.pop_long()?;
    let path_address = parameters.pop_long()?;
    let dfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    #[cfg_attr(not(target_family = "unix"), expect(unused_variables))]
    let path_str = read_native_path(&*vm, path_address)?;

    #[cfg(target_family = "unix")]
    {
        let c_path = to_cstring(&path_str)?;
        let times = [
            libc::timespec {
                tv_sec: access_time / 1_000_000_000,
                tv_nsec: access_time % 1_000_000_000,
            },
            libc::timespec {
                tv_sec: modification_time / 1_000_000_000,
                tv_nsec: modification_time % 1_000_000_000,
            },
        ];
        #[expect(unsafe_code)]
        let result = unsafe { libc::utimensat(dfd, c_path.as_ptr(), times.as_ptr(), flags) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }
    #[cfg(not(target_family = "unix"))]
    {
        let _ = (dfd, access_time, modification_time, flags);
        Err(InternalError(
            "utimensat is not supported on this platform".to_string(),
        ))
    }
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.write(IJI)I",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn write<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    write_0(thread, parameters).await
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/nio/fs/UnixNativeDispatcher.write0(IJI)I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn write_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let fd = parameters.pop_int()?;

    let count = usize::try_from(count)?;
    let vm = thread.vm()?;
    let data = vm.native_memory().read_bytes(address, count);

    match managed_files::write(vm.file_handles(), i64::from(fd), &data).await {
        Ok(n) => Ok(Some(Value::Int(i32::try_from(n)?))),
        Err(error) => {
            let errno = error.raw_os_error().unwrap_or(5 /* EIO */);
            Err(throw_unix_exception(&thread, errno).await)
        }
    }
}

#[cfg(all(test, target_family = "unix", not(target_family = "wasm")))]
mod tests {
    use super::*;
    use zerocopy::transmute_ref;

    /// Write a null-terminated C string into native memory and return its address.
    fn write_cstring_to_native<V: VM>(vm: &V, s: &str) -> i64 {
        let mut bytes = s.as_bytes().to_vec();
        bytes.push(0);
        let addr = vm.native_memory().allocate(bytes.len());
        vm.native_memory().write_bytes(addr, &bytes);
        addr
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_access_0_0() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await.expect("thread");
        let path_addr = write_cstring_to_native(&*vm, "/");
        let mut parameters = Parameters::new(vec![]);
        parameters.push_long(path_addr);
        parameters.push_int(libc::F_OK);
        let result = access_0_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_access_0_1_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = access_0_1(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_access_0_1_existing_path() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path_addr = write_cstring_to_native(&*vm, "/");
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        params.push_int(0);
        let result = access_0_1(thread, params).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_access_0_1_nonexistent_path() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path_addr = write_cstring_to_native(&*vm, "/nonexistent_path_test_abc123");
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        params.push_int(0);
        let result = access_0_1(thread, params).await?;
        assert_eq!(result, Some(Value::Int(-1)));
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_chmod_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let temp_file = std::env::current_dir().unwrap().join("_test_chmod_0.tmp");
        std::fs::write(&temp_file, b"test").unwrap();
        let path_addr = write_cstring_to_native(&*vm, temp_file.to_str().unwrap());
        let mut parameters = Parameters::new(vec![]);
        parameters.push_long(path_addr);
        parameters.push_int(0o644);
        let result = chmod_0(thread, parameters).await?;
        assert_eq!(result, None);
        std::fs::remove_file(&temp_file).ok();
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_chown_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let temp_file = std::env::current_dir().unwrap().join("_test_chown_0.tmp");
        std::fs::write(&temp_file, b"test").unwrap();
        #[expect(unsafe_code)]
        let uid = unsafe { libc::getuid() };
        #[expect(unsafe_code)]
        let gid = unsafe { libc::getgid() };
        let path_addr = write_cstring_to_native(&*vm, temp_file.to_str().unwrap());
        let mut parameters = Parameters::new(vec![]);
        parameters.push_long(path_addr);
        #[expect(clippy::cast_possible_wrap)]
        {
            parameters.push_int(uid as i32);
            parameters.push_int(gid as i32);
        }
        let result = chown_0(thread, parameters).await?;
        assert_eq!(result, None);
        std::fs::remove_file(&temp_file).ok();
        Ok(())
    }

    #[tokio::test]
    async fn test_close_default_params() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = close(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_close_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_close_0_invalid_fd() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::new(vec![]);
        params.push_int(99999);
        let result = close_0(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_closedir() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::new(vec![]);
        parameters.push_long(0);
        let result = closedir(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_dup() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::new(vec![]);
        parameters.push_int(1);
        let result = dup(thread, parameters).await?;
        let fd = result.unwrap();
        if let Value::Int(new_fd) = fd {
            assert!(new_fd > 0);
            #[expect(unsafe_code)]
            unsafe {
                libc::close(new_fd)
            };
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_exists_0() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await.expect("thread");
        let path_addr = write_cstring_to_native(&*vm, "/");
        let mut parameters = Parameters::new(vec![]);
        parameters.push_long(path_addr);
        let result = exists_0(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_fchmod_default_params() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = fchmod(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fchmod_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fchmod_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fchown_default_params() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = fchown(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fchown_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fchown_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fclose_1_null_fp() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let mut parameters = Parameters::new(vec![]);
        parameters.push_long(0);
        let result = fclose_1(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_fdopendir_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fdopendir(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_fdopendir_bad_fd() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut params = Parameters::new(vec![]);
        params.push_int(-1);
        let result = fdopendir(thread, params).await;
        assert!(
            matches!(
                result,
                Err(ristretto_types::Error::Throwable(_) | ristretto_types::Error::InternalError(_))
            ),
            "expected Throwable or InternalError, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fgetxattr_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fgetxattr_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_flistxattr_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = flistxattr(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fopen_0_default_params() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = fopen_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_fopen_fclose_success() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await.expect("thread");
        let temp_file = std::env::current_dir()
            .unwrap()
            .join("_test_fopen_fclose.tmp");
        std::fs::write(&temp_file, b"fopen test").unwrap();
        let path_addr = write_cstring_to_native(&*vm, temp_file.to_str().unwrap());
        let mode_addr = write_cstring_to_native(&*vm, "r");
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        params.push_long(mode_addr);
        let result = fopen_0(thread.clone(), params).await?;
        let Some(Value::Long(fp)) = result else {
            panic!("expected Long from fopen_0")
        };
        assert_ne!(fp, 0);

        // Close via fclose_1 which now properly calls libc::fclose
        let mut params = Parameters::new(vec![]);
        params.push_long(fp);
        let result = fclose_1(thread, params).await?;
        assert_eq!(result, None);

        std::fs::remove_file(&temp_file).ok();
        Ok(())
    }

    #[tokio::test]
    async fn test_fpathconf_default_params() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = fpathconf(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fremovexattr_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fremovexattr_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fsetxattr_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fsetxattr_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fstat_default_params() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = fstat(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fstat_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fstat_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fstat_0_null_attributes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let params = Parameters::new(vec![Value::Int(1), Value::Object(None)]);
        let result = fstat_0(thread, params).await;
        assert!(
            matches!(
                result,
                Err(ristretto_types::Error::JavaError(NullPointerException(_)))
            ),
            "expected NullPointerException, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fstatat_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fstatat_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_futimens_default_params() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = futimens(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_futimens_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = futimens_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_futimes_default_params() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = futimes(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_futimes_0_default_params() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = futimes_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_getcwd() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getcwd(thread, Parameters::default()).await?;
        let bytes = result.expect("cwd");
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        let cwd = String::from_utf8_lossy(bytes);
        let current_dir_path =
            std::env::current_dir().map_err(|error| InternalError(format!("getcwd: {error}")))?;
        let expected = current_dir_path.to_string_lossy();
        assert_eq!(cwd, expected);
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_getgrgid() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        #[expect(unsafe_code)]
        let gid = unsafe { libc::getgid() };
        let mut parameters = Parameters::new(vec![]);
        #[expect(clippy::cast_possible_wrap)]
        parameters.push_int(gid as i32);
        let result = getgrgid(thread, parameters).await?;
        let value = result.expect("expected group name");
        let bytes = value.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        let name = String::from_utf8_lossy(bytes);
        assert!(!name.is_empty(), "group name should be non-empty");
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_getgrnam_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        // "wheel" exists on macOS; fall back to "staff" if needed
        let name_addr = write_cstring_to_native(&*vm, "wheel");
        let mut parameters = Parameters::new(vec![]);
        parameters.push_long(name_addr);
        let result = getgrnam_0(thread.clone(), parameters).await;
        if let Ok(Some(Value::Int(gid))) = result {
            assert!(gid >= 0, "gid should be non-negative");
        } else {
            // Try "staff" as fallback
            let name_addr2 = write_cstring_to_native(&*vm, "staff");
            let mut params2 = Parameters::new(vec![]);
            params2.push_long(name_addr2);
            let result2 = getgrnam_0(thread, params2).await?;
            assert!(matches!(result2, Some(Value::Int(_))), "expected gid Int");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_getlinelen_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = getlinelen(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_getpwnam_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let name_addr = write_cstring_to_native(&*vm, "root");
        let mut parameters = Parameters::new(vec![]);
        parameters.push_long(name_addr);
        let result = getpwnam_0(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(0))); // root uid is 0
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_getpwuid() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::new(vec![]);
        parameters.push_int(0); // root
        let result = getpwuid(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await?;
        let value = result.expect("init result");
        if let Value::Int(bits) = value {
            #[cfg(target_family = "unix")]
            {
                let flags = SupportsFlags::from_bits_truncate(bits);
                assert!(flags.contains(SupportsFlags::OPENAT));
                assert!(flags.contains(SupportsFlags::FUTIMENS));
            }
            #[cfg(not(target_family = "unix"))]
            assert_eq!(bits, 0);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_lchown_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lchown_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_link_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = link_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_link_0_success() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let src = std::env::current_dir().unwrap().join("_test_link_src.tmp");
        let dst = std::env::current_dir().unwrap().join("_test_link_dst.tmp");
        std::fs::write(&src, b"link test").unwrap();
        let _ = std::fs::remove_file(&dst);

        let src_addr = write_cstring_to_native(&*vm, src.to_str().unwrap());
        let dst_addr = write_cstring_to_native(&*vm, dst.to_str().unwrap());
        let mut params = Parameters::new(vec![]);
        params.push_long(src_addr);
        params.push_long(dst_addr);
        let result = link_0(thread, params).await?;
        assert_eq!(result, None);
        assert!(dst.exists());

        std::fs::remove_file(&src).ok();
        std::fs::remove_file(&dst).ok();
        Ok(())
    }

    #[tokio::test]
    async fn test_lstat_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lstat_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_lstat_0_success() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let temp_file = std::env::current_dir().unwrap().join("_test_lstat_0.tmp");
        std::fs::write(&temp_file, b"lstat test").unwrap();

        let path_addr = write_cstring_to_native(&*vm, temp_file.to_str().unwrap());
        let attrs = thread
            .object("sun.nio.fs.UnixFileAttributes", "", &[] as &[Value])
            .await?;
        let params = Parameters::new(vec![Value::Long(path_addr), attrs]);
        let result = lstat_0(thread, params).await?;
        assert_eq!(result, None);

        std::fs::remove_file(&temp_file).ok();
        Ok(())
    }

    #[tokio::test]
    async fn test_lutimes_0_default_params() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = lutimes_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_mkdir_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mkdir_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_mknod_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = mknod_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_open_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_read_default_params() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = read(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_read_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_write_default_params() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = write(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_write_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = write_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_open_read_write_close_lifecycle() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let temp_file = std::env::current_dir()
            .unwrap()
            .join("_test_open_rw_close.tmp");
        let _ = std::fs::remove_file(&temp_file);

        // O_CREAT | O_RDWR | O_TRUNC (macOS flags)
        #[cfg(target_os = "macos")]
        let flags = 0x0202 | 0x0400; // O_RDWR | O_CREAT | O_TRUNC
        #[cfg(not(target_os = "macos"))]
        let flags = 0x02 | 0x40 | 0x200; // O_RDWR | O_CREAT | O_TRUNC

        let path_addr = write_cstring_to_native(&*vm, temp_file.to_str().unwrap());
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        params.push_int(flags);
        params.push_int(0o644);
        let result = open_0(thread.clone(), params).await?;
        let Some(Value::Int(fd)) = result else {
            panic!("expected Int fd from open_0")
        };
        assert!(fd >= 0);

        // Write data
        let test_data = b"hello ristretto";
        let buf_addr = vm.native_memory().allocate(test_data.len());
        vm.native_memory().write_bytes(buf_addr, test_data);
        let mut params = Parameters::new(vec![]);
        params.push_int(fd);
        params.push_long(buf_addr);
        #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
        params.push_int(test_data.len() as i32);
        let result = write_0(thread.clone(), params).await?;
        #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
        let expected_written = test_data.len() as i32;
        assert_eq!(result, Some(Value::Int(expected_written)));

        // Close and reopen for reading
        let mut params = Parameters::new(vec![]);
        params.push_int(fd);
        close_0(thread.clone(), params).await?;

        let path_addr2 = write_cstring_to_native(&*vm, temp_file.to_str().unwrap());
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr2);
        params.push_int(0); // O_RDONLY
        params.push_int(0);
        let result = open_0(thread.clone(), params).await?;
        let Some(Value::Int(read_fd)) = result else {
            panic!("expected Int fd from open_0")
        };

        // Read data back
        let read_buf_addr = vm.native_memory().allocate(test_data.len());
        let mut params = Parameters::new(vec![]);
        params.push_int(read_fd);
        params.push_long(read_buf_addr);
        #[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
        params.push_int(test_data.len() as i32);
        let result = read_0(thread.clone(), params).await?;
        assert_eq!(result, Some(Value::Int(expected_written)));
        let read_back = vm
            .native_memory()
            .read_bytes(read_buf_addr, test_data.len());
        assert_eq!(&read_back, test_data);

        // Close
        let mut params = Parameters::new(vec![]);
        params.push_int(read_fd);
        close_0(thread, params).await?;

        std::fs::remove_file(&temp_file).ok();
        Ok(())
    }

    #[tokio::test]
    async fn test_openat_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = openat_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_opendir_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = opendir_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_pathconf_0_default_params() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = pathconf_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_pathconf_0_success() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await.expect("thread");
        let path_addr = write_cstring_to_native(&*vm, "/");
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        params.push_int(libc::_PC_NAME_MAX);
        let result = pathconf_0(thread, params).await?;
        let Some(Value::Long(value)) = result else {
            panic!("expected Long from pathconf_0")
        };
        assert!(value > 0, "NAME_MAX should be positive, got {value}");
        Ok(())
    }

    #[tokio::test]
    async fn test_readdir_default_params() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = readdir(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_readdir_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = readdir_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_readlink_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = readlink_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_symlink_readlink_success() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let target = std::env::current_dir()
            .unwrap()
            .join("_test_symlink_target.tmp");
        let link = std::env::current_dir()
            .unwrap()
            .join("_test_symlink_link.tmp");
        std::fs::write(&target, b"symlink test").unwrap();
        let _ = std::fs::remove_file(&link);

        let target_addr = write_cstring_to_native(&*vm, target.to_str().unwrap());
        let link_addr = write_cstring_to_native(&*vm, link.to_str().unwrap());
        let mut params = Parameters::new(vec![]);
        params.push_long(target_addr);
        params.push_long(link_addr);
        let result = symlink_0(thread.clone(), params).await?;
        assert_eq!(result, None);
        assert!(link.exists());

        // readlink
        let link_addr2 = write_cstring_to_native(&*vm, link.to_str().unwrap());
        let mut params = Parameters::new(vec![]);
        params.push_long(link_addr2);
        let result = readlink_0(thread, params).await?;
        let bytes = result.expect("readlink result");
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        let read_target = String::from_utf8_lossy(bytes);
        assert_eq!(read_target, target.to_str().unwrap());

        std::fs::remove_file(&link).ok();
        std::fs::remove_file(&target).ok();
        Ok(())
    }

    #[tokio::test]
    async fn test_realpath_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = realpath_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_realpath_0_success() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path_addr = write_cstring_to_native(&*vm, "/");
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        let result = realpath_0(thread, params).await?;
        let bytes = result.expect("realpath result");
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        let resolved = String::from_utf8_lossy(bytes);
        assert!(
            resolved.starts_with('/'),
            "resolved path should be absolute, got {resolved}"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_rename_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = rename_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_renameat_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = renameat_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_rewind_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = rewind(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_rmdir_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = rmdir_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_stat_0_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let unix_file_attributes = thread
            .object("sun.nio.fs.UnixFileAttributes", "", &[] as &[Value])
            .await?;
        let parameters = Parameters::new(vec![Value::Long(0), unix_file_attributes]);
        let result = stat_0_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_stat_0_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let unix_file_attributes = thread
            .object("sun.nio.fs.UnixFileAttributes", "", &[] as &[Value])
            .await?;
        let parameters = Parameters::new(vec![Value::Long(0), unix_file_attributes]);
        let result = stat_0_1(thread, parameters).await?;
        // With address 0, path resolves to empty string; stat returns an errno
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_stat_1_default_params() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = stat_1(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_stat_1_success() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await.expect("thread");
        let path_addr = write_cstring_to_native(&*vm, "/");
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        let result = stat_1(thread, params).await?;
        let Some(Value::Int(mode)) = result else {
            panic!("expected Int mode from stat_1")
        };
        // Root directory should have S_IFDIR bit set (0o40000)
        assert_ne!(
            mode & 0o40000,
            0,
            "expected directory mode bit, got {mode:#o}"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_statvfs_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = statvfs_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_strerror() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::new(vec![]);
        parameters.push_int(2); // ENOENT
        let result = strerror(thread, parameters).await?;
        let bytes = result.expect("strerror result");
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        let msg = String::from_utf8_lossy(bytes);
        assert!(!msg.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_symlink_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = symlink_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_unlink_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unlink_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_unlinkat_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unlinkat_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_utimes_0_default_params() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = utimes_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_utimensat_0_default_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = utimensat_0(thread, Parameters::default()).await;
        assert!(
            matches!(result, Err(ristretto_types::Error::ParametersUnderflow)),
            "expected ParametersUnderflow, got {result:?}"
        );
    }

    #[tokio::test]
    async fn test_fchmodat_no_follow_supported_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fchmodat_no_follow_supported_0(thread, Parameters::default()).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_opendir_readdir_closedir() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path_addr = write_cstring_to_native(&*vm, "/");

        // Open directory
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        let result = opendir_0(thread.clone(), params).await?;
        let Some(Value::Long(dir_ptr)) = result else {
            panic!("expected long")
        };
        assert_ne!(dir_ptr, 0);

        // Read at least one entry
        let mut params = Parameters::new(vec![]);
        params.push_long(dir_ptr);
        let result = readdir_0(thread.clone(), params).await?;
        assert!(result.is_some());

        // Close directory
        let mut params = Parameters::new(vec![]);
        params.push_long(dir_ptr);
        closedir(thread, params).await?;
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_mkdir_rmdir() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let dir_path = std::env::current_dir()
            .unwrap()
            .join("_test_mkdir_rmdir_tmp");
        let _ = std::fs::remove_dir(&dir_path);
        let path_addr = write_cstring_to_native(&*vm, dir_path.to_str().unwrap());

        // mkdir
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        params.push_int(0o755);
        mkdir_0(thread.clone(), params).await?;
        assert!(dir_path.exists());

        // rmdir
        let path_addr2 = write_cstring_to_native(&*vm, dir_path.to_str().unwrap());
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr2);
        rmdir_0(thread, params).await?;
        assert!(!dir_path.exists());
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_rename() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let from = std::env::current_dir()
            .unwrap()
            .join("_test_rename_from.tmp");
        let to = std::env::current_dir().unwrap().join("_test_rename_to.tmp");
        std::fs::write(&from, b"test").unwrap();
        let _ = std::fs::remove_file(&to);

        let from_addr = write_cstring_to_native(&*vm, from.to_str().unwrap());
        let to_addr = write_cstring_to_native(&*vm, to.to_str().unwrap());
        let mut params = Parameters::new(vec![]);
        params.push_long(from_addr);
        params.push_long(to_addr);
        rename_0(thread, params).await?;

        assert!(!from.exists());
        assert!(to.exists());
        std::fs::remove_file(&to).ok();
        Ok(())
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_unlink() -> Result<()> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let path = std::env::current_dir().unwrap().join("_test_unlink_0.tmp");
        std::fs::write(&path, b"test").unwrap();

        let path_addr = write_cstring_to_native(&*vm, path.to_str().unwrap());
        let mut params = Parameters::new(vec![]);
        params.push_long(path_addr);
        unlink_0(thread, params).await?;
        assert!(!path.exists());
        Ok(())
    }
}
