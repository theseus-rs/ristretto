use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, Thread, VM};
use std::sync::Arc;

/// Implements `FileSystemPreferences.chmod(String filename, int permission) -> int`.
///
/// On Unix: calls POSIX chmod on the file. Returns 0 on success, errno on failure.
/// On WASM: returns -1 (unsupported).
#[intrinsic_method(
    "java/util/prefs/FileSystemPreferences.chmod(Ljava/lang/String;I)I",
    Any
)]
#[async_method]
pub async fn chmod<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let permission = parameters.pop_int()?;
    let filename = parameters.pop()?.as_string()?;

    let result = platform::chmod(&filename, permission).await;
    Ok(Some(Value::Int(result)))
}

/// Implements `FileSystemPreferences.lockFile0(String filename, int permission, boolean shared) -> int\[\]`.
///
/// Opens a file and acquires an advisory lock. Returns an int\[2\] array:
/// - result\[0\]: file descriptor on success, 0 on failure
/// - result\[1\]: errno value (0 on success)
#[intrinsic_method(
    "java/util/prefs/FileSystemPreferences.lockFile0(Ljava/lang/String;IZ)[I",
    Any
)]
#[async_method]
pub async fn lock_file_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let shared = parameters.pop_bool()?;
    let permission = parameters.pop_int()?;
    let filename = parameters.pop()?.as_string()?;

    let result = platform::lock_file(&filename, permission, shared).await;
    let int_array: Vec<i32> = vec![result[0], result[1]];
    let reference = Reference::from(int_array);
    let vm = thread.vm()?;
    let value = Value::new_object(vm.garbage_collector(), reference);
    Ok(Some(value))
}

/// Implements `FileSystemPreferences.unlockFile0(int fd) -> int`.
///
/// Unlocks a previously locked file and closes the file descriptor.
/// Returns 0 on success, errno on failure.
#[intrinsic_method("java/util/prefs/FileSystemPreferences.unlockFile0(I)I", Any)]
#[async_method]
pub async fn unlock_file_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let result = platform::unlock_file(fd).await;
    Ok(Some(Value::Int(result)))
}

/// Unix-specific implementations using POSIX APIs.
#[cfg(target_family = "unix")]
mod platform {
    use std::ffi::CString;

    fn errno() -> i32 {
        std::io::Error::last_os_error().raw_os_error().unwrap_or(-1)
    }

    pub async fn chmod(filename: &str, permission: i32) -> i32 {
        let filename = filename.to_string();
        #[expect(clippy::cast_sign_loss)]
        #[cfg_attr(target_os = "macos", expect(clippy::cast_possible_truncation))]
        let permission = permission as libc::mode_t;
        tokio::task::spawn_blocking(move || {
            let Ok(c_path) = CString::new(filename) else {
                return -1;
            };
            #[expect(unsafe_code)]
            // SAFETY: c_path is a valid null-terminated C string and permission is a valid mode_t.
            // libc::chmod is a standard POSIX function that takes these arguments.
            let result = unsafe { libc::chmod(c_path.as_ptr(), permission) };
            if result != 0 { errno() } else { 0 }
        })
        .await
        .unwrap_or(-1)
    }

    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub async fn lock_file(filename: &str, permission: i32, shared: bool) -> [i32; 2] {
        let filename = filename.to_string();
        tokio::task::spawn_blocking(move || {
            let Ok(c_path) = CString::new(filename) else {
                return [0, -1];
            };

            let fd;
            let mut result_errno;

            if shared {
                #[expect(unsafe_code)]
                // SAFETY: c_path is a valid null-terminated C string. O_RDONLY is a valid flag.
                let raw_fd = unsafe { libc::open(c_path.as_ptr(), libc::O_RDONLY, 0) };
                fd = raw_fd;
                result_errno = errno();
            } else {
                #[expect(unsafe_code)]
                // SAFETY: umask is a standard POSIX function; we save and restore it.
                let old_umask = unsafe { libc::umask(0) };
                #[expect(unsafe_code)]
                // SAFETY: c_path is a valid null-terminated C string. O_WRONLY|O_CREAT are valid
                // flags and permission is cast to a valid mode_t.
                let raw_fd = unsafe {
                    libc::open(
                        c_path.as_ptr(),
                        libc::O_WRONLY | libc::O_CREAT,
                        libc::c_uint::from(permission as libc::mode_t),
                    )
                };
                fd = raw_fd;
                result_errno = errno();
                #[expect(unsafe_code)]
                // SAFETY: restoring the previously saved umask value.
                unsafe {
                    libc::umask(old_umask);
                }
            }

            if fd < 0 {
                return [0, result_errno];
            }

            #[expect(unsafe_code)]
            // SAFETY: zeroing a flock struct is safe; all-zeros is a valid representation.
            let mut fl: libc::flock = unsafe { std::mem::zeroed() };
            fl.l_whence = libc::SEEK_SET as i16;
            fl.l_len = 0;
            fl.l_start = 0;
            fl.l_type = if shared {
                i64::from(libc::F_RDLCK) as i16
            } else {
                i64::from(libc::F_WRLCK) as i16
            };

            #[expect(unsafe_code)]
            // SAFETY: fd is a valid open file descriptor and fl is a properly initialized flock.
            let rc = unsafe { libc::fcntl(fd, libc::F_SETLK, &fl) };
            result_errno = errno();

            if rc < 0 {
                #[expect(unsafe_code)]
                // SAFETY: fd is a valid open file descriptor.
                unsafe {
                    libc::close(fd);
                }
                [0, result_errno]
            } else {
                [fd, 0]
            }
        })
        .await
        .unwrap_or([0, -1])
    }

    #[expect(clippy::cast_possible_truncation)]
    pub async fn unlock_file(fd: i32) -> i32 {
        tokio::task::spawn_blocking(move || {
            #[expect(unsafe_code)]
            // SAFETY: zeroing a flock struct is safe; all-zeros is a valid representation.
            let mut fl: libc::flock = unsafe { std::mem::zeroed() };
            fl.l_whence = libc::SEEK_SET as i16;
            fl.l_len = 0;
            fl.l_start = 0;
            fl.l_type = i64::from(libc::F_UNLCK) as i16;

            #[expect(unsafe_code)]
            // SAFETY: fd is expected to be a valid file descriptor from lock_file.
            let rc = unsafe { libc::fcntl(fd, libc::F_SETLK, &fl) };
            if rc < 0 {
                let err = errno();
                #[expect(unsafe_code)]
                // SAFETY: closing the file descriptor on error.
                unsafe {
                    libc::close(fd);
                }
                return err;
            }
            #[expect(unsafe_code)]
            // SAFETY: fd is a valid file descriptor that we are done with.
            let rc = unsafe { libc::close(fd) };
            if rc < 0 {
                return errno();
            }
            0
        })
        .await
        .unwrap_or(-1)
    }
}

/// WASM-specific implementations (no filesystem access).
#[cfg(target_family = "wasm")]
mod platform {
    #[expect(clippy::unused_async)]
    pub async fn chmod(_filename: &str, _permission: i32) -> i32 {
        -1
    }

    #[expect(clippy::unused_async)]
    pub async fn lock_file(_filename: &str, _permission: i32, _shared: bool) -> [i32; 2] {
        [0, -1]
    }

    #[expect(clippy::unused_async)]
    pub async fn unlock_file(_fd: i32) -> i32 {
        -1
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;

    #[cfg(target_family = "unix")]
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_chmod_success() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let file = NamedTempFile::new()?;
        let path = file.path().to_string_lossy().to_string();
        let path_value: Value = path.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path_value);
        parameters.push_int(0o644);
        let result = chmod(thread, parameters).await?.expect("value");
        let code = result.as_i32()?;
        #[cfg(target_family = "unix")]
        assert_eq!(0, code);
        #[cfg(not(target_family = "unix"))]
        assert_eq!(-1, code);
        Ok(())
    }

    #[tokio::test]
    async fn test_chmod_nonexistent_file() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let path = "/tmp/ristretto_nonexistent_file_for_chmod_test";
        let path_value: Value = path.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path_value);
        parameters.push_int(0o644);
        let result = chmod(thread, parameters).await?.expect("value");
        let code = result.as_i32()?;
        assert_ne!(0, code);
        Ok(())
    }

    #[tokio::test]
    async fn test_lock_file_0_exclusive() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let file = NamedTempFile::new()?;
        let path = file.path().to_string_lossy().to_string();
        let path_value: Value = path.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path_value);
        parameters.push_int(0o644);
        parameters.push_bool(false); // exclusive lock
        let result = lock_file_0(thread.clone(), parameters)
            .await?
            .expect("value");
        let (fd, err) = {
            let lock_result = result.as_reference()?;
            if let Reference::IntArray(arr) = &*lock_result {
                (arr[0], arr[1])
            } else {
                panic!("Expected IntArray result");
            }
        };
        #[cfg(target_family = "unix")]
        {
            assert!(fd > 0, "Expected valid fd, got {fd}");
            assert_eq!(0, err);

            let mut unlock_params = Parameters::default();
            unlock_params.push_int(fd);
            let unlock_result = unlock_file_0(thread, unlock_params).await?.expect("value");
            assert_eq!(0, unlock_result.as_i32()?);
        }
        #[cfg(not(target_family = "unix"))]
        {
            assert_eq!(0, fd);
            assert_eq!(-1, err);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_lock_file_0_shared() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let file = NamedTempFile::new()?;
        let path = file.path().to_string_lossy().to_string();
        let path_value: Value = path.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path_value);
        parameters.push_int(0o644);
        parameters.push_bool(true); // shared lock
        let result = lock_file_0(thread.clone(), parameters)
            .await?
            .expect("value");
        let (fd, err) = {
            let lock_result = result.as_reference()?;
            if let Reference::IntArray(arr) = &*lock_result {
                (arr[0], arr[1])
            } else {
                panic!("Expected IntArray result");
            }
        };
        #[cfg(target_family = "unix")]
        {
            assert!(fd > 0, "Expected valid fd, got {fd}");
            assert_eq!(0, err);

            let mut unlock_params = Parameters::default();
            unlock_params.push_int(fd);
            let unlock_result = unlock_file_0(thread, unlock_params).await?.expect("value");
            assert_eq!(0, unlock_result.as_i32()?);
        }
        #[cfg(not(target_family = "unix"))]
        {
            assert_eq!(0, fd);
            assert_eq!(-1, err);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_lock_file_0_nonexistent() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let path = "/tmp/ristretto_nonexistent_lockfile_test_dir/nonexistent";
        let path_value: Value = path.to_object(&thread).await?;
        let mut parameters = Parameters::default();
        parameters.push(path_value);
        parameters.push_int(0o644);
        parameters.push_bool(true); // shared lock on nonexistent file
        let result = lock_file_0(thread, parameters).await?.expect("value");
        let lock_result = result.as_reference()?;
        if let Reference::IntArray(arr) = &*lock_result {
            assert_eq!(0, arr[0], "fd should be 0 for nonexistent file");
            assert_ne!(0, arr[1], "errno should be non-zero");
        } else {
            panic!("Expected IntArray result");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_unlock_file_0_invalid_fd() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push_int(-1); // invalid fd
        let result = unlock_file_0(thread, parameters).await?.expect("value");
        let code = result.as_i32()?;
        assert_ne!(0, code, "Should fail with invalid fd");
        Ok(())
    }
}
