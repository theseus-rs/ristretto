use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(not(target_os = "linux"))]
use ristretto_types::JavaError;
use ristretto_types::Thread;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use ristretto_types::VM;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use ristretto_types::handles::{FileHandle, FileModeFlags};
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[cfg(target_os = "linux")]
use super::common::throw_unix_exception;

/// Read native errno from the last OS error.
#[cfg(target_os = "linux")]
fn last_errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(5)
}

/// Register a raw Unix file descriptor in the VM `file_handles` map so it can be
/// read from / written to via `UnixNativeDispatcher.read`/`write`.
#[cfg(any(target_os = "linux", target_os = "macos"))]
async fn register_raw_fd<T: Thread + 'static>(thread: &Arc<T>, raw_fd: i32) -> Result<()> {
    use std::os::fd::FromRawFd;
    #[expect(unsafe_code)]
    let std_file = unsafe { std::fs::File::from_raw_fd(raw_fd) };
    let tokio_file = tokio::fs::File::from_std(std_file);
    let handle = FileHandle {
        file: tokio_file,
        append: false,
        mode: FileModeFlags::READ_WRITE,
    };
    let vm = thread.vm()?;
    let file_handles = vm.file_handles();
    file_handles
        .insert(i64::from(raw_fd), handle)
        .await
        .map_err(|error| {
            ristretto_types::Error::InternalError(format!(
                "Failed to register file descriptor {raw_fd}: {error}"
            ))
        })?;
    Ok(())
}

#[intrinsic_method("sun/nio/fs/LinuxWatchService.configureBlocking(IZ)V", Any)]
#[async_method]
pub async fn configure_blocking<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let blocking = parameters.pop_bool()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_os = "linux")]
    {
        #[expect(unsafe_code)]
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
        if flags < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        let new_flags = if blocking {
            flags & !libc::O_NONBLOCK
        } else {
            flags | libc::O_NONBLOCK
        };
        #[expect(unsafe_code)]
        let result = unsafe { libc::fcntl(fd, libc::F_SETFL, new_flags) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd, blocking);
        Err(JavaError::UnsatisfiedLinkError(
            "sun/nio/fs/LinuxWatchService.configureBlocking(IZ)V".to_string(),
        )
        .into())
    }
}

#[intrinsic_method("sun/nio/fs/LinuxWatchService.eventOffsets()[I", Any)]
#[async_method]
pub async fn event_offsets<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // struct inotify_event { __s32 wd; __u32 mask; __u32 cookie; __u32 len; char name[]; };
    // offsets: wd=0, mask=4, cookie=8, len=12, name=16
    let array: Box<[i32]> = vec![0i32, 4, 8, 12, 16].into_boxed_slice();
    Ok(Some(Value::new_object(
        thread.vm()?.garbage_collector(),
        Reference::IntArray(array),
    )))
}

#[intrinsic_method("sun/nio/fs/LinuxWatchService.eventSize()I", Any)]
#[async_method]
pub async fn event_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // sizeof(struct inotify_event) without flexible array member = 16 bytes
    Ok(Some(Value::Int(16)))
}

#[intrinsic_method("sun/nio/fs/LinuxWatchService.inotifyAddWatch(IJI)I", Any)]
#[async_method]
pub async fn inotify_add_watch<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let mask = parameters.pop_int()?;
    let path_address = parameters.pop_long()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_os = "linux")]
    {
        let vm = thread.vm()?;
        let path_bytes = vm
            .native_memory()
            .try_read_cstring(path_address)
            .ok_or_else(|| {
                ristretto_types::Error::InternalError(
                    "inotifyAddWatch: invalid path address".to_string(),
                )
            })?;
        let c_path = std::ffi::CString::new(path_bytes).map_err(|error| {
            ristretto_types::Error::InternalError(format!("Invalid path: {error}"))
        })?;
        #[expect(clippy::cast_sign_loss, unsafe_code)]
        let wd = unsafe { libc::inotify_add_watch(fd, c_path.as_ptr(), mask as u32) };
        if wd < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(Some(Value::Int(wd)))
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd, path_address, mask);
        Err(JavaError::UnsatisfiedLinkError(
            "sun/nio/fs/LinuxWatchService.inotifyAddWatch(IJI)I".to_string(),
        )
        .into())
    }
}

#[intrinsic_method("sun/nio/fs/LinuxWatchService.inotifyInit()I", Any)]
#[async_method]
pub async fn inotify_init<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        #[expect(unsafe_code)]
        let fd = unsafe { libc::inotify_init1(libc::IN_NONBLOCK | libc::IN_CLOEXEC) };
        if fd < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        register_raw_fd(&thread, fd).await?;
        Ok(Some(Value::Int(fd)))
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = thread;
        Err(JavaError::UnsatisfiedLinkError(
            "sun/nio/fs/LinuxWatchService.inotifyInit()I".to_string(),
        )
        .into())
    }
}

#[intrinsic_method("sun/nio/fs/LinuxWatchService.inotifyRmWatch(II)V", Any)]
#[async_method]
pub async fn inotify_rm_watch<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let wd = parameters.pop_int()?;
    let fd = parameters.pop_int()?;

    #[cfg(target_os = "linux")]
    {
        #[expect(unsafe_code)]
        let result = unsafe { libc::inotify_rm_watch(fd, wd) };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        Ok(None)
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd, wd);
        Err(JavaError::UnsatisfiedLinkError(
            "sun/nio/fs/LinuxWatchService.inotifyRmWatch(II)V".to_string(),
        )
        .into())
    }
}

#[intrinsic_method("sun/nio/fs/LinuxWatchService.poll(II)I", Any)]
#[async_method]
pub async fn poll<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd2 = parameters.pop_int()?;
    let fd1 = parameters.pop_int()?;

    #[cfg(target_os = "linux")]
    {
        let result = tokio::task::spawn_blocking(move || {
            let mut fds = [
                libc::pollfd {
                    fd: fd1,
                    events: libc::POLLIN,
                    revents: 0,
                },
                libc::pollfd {
                    fd: fd2,
                    events: libc::POLLIN,
                    revents: 0,
                },
            ];
            loop {
                #[expect(unsafe_code)]
                let n = unsafe { libc::poll(fds.as_mut_ptr(), 2, -1) };
                if n < 0 {
                    let err = last_errno();
                    if err == libc::EINTR {
                        continue;
                    }
                    return Err(err);
                }
                return Ok(n);
            }
        })
        .await
        .map_err(|error| {
            ristretto_types::Error::InternalError(format!("poll task failed: {error}"))
        })?;

        match result {
            Ok(n) => Ok(Some(Value::Int(n))),
            Err(errno) => Err(throw_unix_exception(&thread, errno).await),
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd1, fd2);
        Err(
            JavaError::UnsatisfiedLinkError("sun/nio/fs/LinuxWatchService.poll(II)I".to_string())
                .into(),
        )
    }
}

#[intrinsic_method("sun/nio/fs/LinuxWatchService.socketpair([I)V", Any)]
#[async_method]
pub async fn socketpair<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let sv_ref = parameters.pop_reference()?;

    #[cfg(target_os = "linux")]
    {
        let array = sv_ref.ok_or_else(|| {
            ristretto_types::Error::InternalError("socketpair: null int[]".to_string())
        })?;
        let array_value = Value::from(array);
        {
            let int_vec = array_value.as_int_vec_ref()?;
            if int_vec.len() < 2 {
                return Err(ristretto_types::Error::InternalError(
                    "socketpair: int[] too small".to_string(),
                ));
            }
        }

        let mut sv: [i32; 2] = [0; 2];
        #[expect(unsafe_code)]
        let result = unsafe {
            libc::socketpair(
                libc::AF_UNIX,
                libc::SOCK_STREAM | libc::SOCK_CLOEXEC,
                0,
                sv.as_mut_ptr(),
            )
        };
        if result < 0 {
            return Err(throw_unix_exception(&thread, last_errno()).await);
        }
        let [first_fd, second_fd] = sv;
        register_raw_fd(&thread, first_fd).await?;
        if let Err(error) = register_raw_fd(&thread, second_fd).await {
            let vm = thread.vm()?;
            super::managed_files::close(vm.file_handles(), i64::from(first_fd)).await;
            return Err(error);
        }
        // Write the two fds into the int[] argument.
        let assigned = {
            let mut int_vec = array_value.as_int_vec_mut()?;
            if let Some([first_value, second_value]) = int_vec.get_mut(..2) {
                *first_value = first_fd;
                *second_value = second_fd;
                true
            } else {
                false
            }
        };
        if !assigned {
            let vm = thread.vm()?;
            super::managed_files::close(vm.file_handles(), i64::from(first_fd)).await;
            super::managed_files::close(vm.file_handles(), i64::from(second_fd)).await;
            return Err(ristretto_types::Error::InternalError(
                "socketpair: int[] became too small".to_string(),
            ));
        }
        Ok(None)
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, sv_ref);
        Err(JavaError::UnsatisfiedLinkError(
            "sun/nio/fs/LinuxWatchService.socketpair([I)V".to_string(),
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_offsets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = event_offsets(thread, Parameters::default()).await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_event_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = event_size(thread, Parameters::default()).await.unwrap();
        assert_eq!(Some(Value::Int(16)), result);
    }

    #[cfg(not(target_os = "linux"))]
    #[tokio::test]
    async fn test_inotify_init_unsupported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = inotify_init(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
