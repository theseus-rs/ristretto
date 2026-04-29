use crate::java::io::filedescriptor::raw_file_descriptor;
use ristretto_types::ResourceManager;
use ristretto_types::handles::{FileHandle, FileModeFlags, HandleManager};
#[cfg(target_os = "wasi")]
use std::io::{Read, Seek, Write};
#[cfg(not(target_family = "wasm"))]
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

pub(crate) async fn open(
    file_handles: &HandleManager<i64, FileHandle>,
    resource_manager: &ResourceManager,
    path: &str,
    flags: i32,
    mode: i32,
) -> std::io::Result<i64> {
    // O_RDONLY=0, O_WRONLY=1, O_RDWR=2
    let access_mode = flags & 0x3;
    #[cfg(target_family = "unix")]
    let (create, exclusive, truncate, append) = (
        flags & libc::O_CREAT != 0,
        flags & libc::O_EXCL != 0,
        flags & libc::O_TRUNC != 0,
        flags & libc::O_APPEND != 0,
    );
    #[cfg(not(target_family = "unix"))]
    let (create, exclusive, truncate, append) = (
        flags & 0x40 != 0,  // O_CREAT
        flags & 0x80 != 0,  // O_EXCL
        flags & 0x200 != 0, // O_TRUNC
        flags & 0x400 != 0, // O_APPEND
    );

    #[cfg(not(target_family = "wasm"))]
    let mut open_options = tokio::fs::OpenOptions::new();
    #[cfg(target_family = "wasm")]
    let mut open_options = std::fs::OpenOptions::new();

    match access_mode {
        0 => {
            open_options.read(true);
        }
        1 => {
            open_options.write(true);
        }
        _ => {
            open_options.read(true).write(true);
        }
    }
    if create && exclusive {
        open_options.create_new(true);
        #[cfg(target_family = "unix")]
        {
            open_options.mode(mode.cast_unsigned());
        }
        #[cfg(not(target_family = "unix"))]
        let _ = mode;
    } else if create {
        open_options.create(true);
        #[cfg(target_family = "unix")]
        {
            open_options.mode(mode.cast_unsigned());
        }
        #[cfg(not(target_family = "unix"))]
        let _ = mode;
    } else {
        let _ = mode;
    }
    if truncate {
        open_options.truncate(true);
    }
    if append {
        open_options.append(true);
    }

    #[cfg(not(target_family = "wasm"))]
    let file = open_options.open(path).await?;
    #[cfg(target_os = "wasi")]
    let file = open_options.open(path)?;
    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    let file = {
        let _ = open_options;
        path.to_string()
    };

    let fd;
    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        fd = raw_file_descriptor(path, resource_manager)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
    }
    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let _ = resource_manager;
        fd = raw_file_descriptor(&file).map_err(|e| std::io::Error::other(e.to_string()))?;
    }

    let file_handle = FileHandle {
        file,
        append,
        mode: FileModeFlags::empty(),
    };
    file_handles
        .insert(fd, file_handle)
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(fd)
}

pub(crate) async fn read(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    buf: &mut [u8],
) -> std::io::Result<usize> {
    #[cfg_attr(
        all(target_family = "wasm", not(target_os = "wasi")),
        expect(unused_mut)
    )]
    let mut file_handle = file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file_handle.file.read(buf).await
    }

    #[cfg(target_os = "wasi")]
    {
        file_handle.file.read(buf)
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        drop(file_handle);
        let _ = buf;
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

pub(crate) async fn write(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    buf: &[u8],
) -> std::io::Result<usize> {
    #[cfg_attr(
        all(target_family = "wasm", not(target_os = "wasi")),
        expect(unused_mut)
    )]
    let mut file_handle = file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file_handle.file.write(buf).await
    }

    #[cfg(target_os = "wasi")]
    {
        file_handle.file.write(buf)
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        drop(file_handle);
        let _ = buf;
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

pub(crate) async fn write_all(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    buf: &[u8],
) -> std::io::Result<()> {
    #[cfg_attr(
        all(target_family = "wasm", not(target_os = "wasi")),
        expect(unused_mut)
    )]
    let mut file_handle = file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file_handle.file.write_all(buf).await
    }

    #[cfg(target_os = "wasi")]
    {
        use std::io::Write;
        file_handle.file.write_all(buf)
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        drop(file_handle);
        let _ = buf;
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

pub(crate) async fn close(file_handles: &HandleManager<i64, FileHandle>, fd: i64) {
    #[cfg_attr(
        all(target_family = "wasm", not(target_os = "wasi")),
        expect(unused_variables, unused_mut)
    )]
    if let Some(mut handle) = file_handles.remove(&fd).await {
        #[cfg(not(target_family = "wasm"))]
        {
            let _ = handle.file.flush().await;
            let _ = handle.file.shutdown().await;
        }
        #[cfg(target_os = "wasi")]
        {
            use std::io::Write;
            let _ = handle.file.flush();
        }
    }
}

pub(crate) async fn metadata(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
) -> std::io::Result<std::fs::Metadata> {
    let file_handle = file_handles
        .get(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file_handle.file.metadata().await
    }

    #[cfg(target_os = "wasi")]
    {
        file_handle.file.metadata()
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        drop(file_handle);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

pub(crate) async fn seek(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    pos: std::io::SeekFrom,
) -> std::io::Result<u64> {
    #[cfg_attr(
        all(target_family = "wasm", not(target_os = "wasi")),
        expect(unused_mut)
    )]
    let mut file_handle = file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file_handle.file.seek(pos).await
    }

    #[cfg(target_os = "wasi")]
    {
        file_handle.file.seek(pos)
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        drop(file_handle);
        let _ = pos;
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

pub(crate) async fn sync_all(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
) -> std::io::Result<()> {
    let file_handle = file_handles
        .get(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file_handle.file.sync_all().await
    }

    #[cfg(target_os = "wasi")]
    {
        file_handle.file.sync_all()
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        drop(file_handle);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

pub(crate) async fn sync_data(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
) -> std::io::Result<()> {
    let file_handle = file_handles
        .get(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file_handle.file.sync_data().await
    }

    #[cfg(target_os = "wasi")]
    {
        file_handle.file.sync_data()
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        drop(file_handle);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

pub(crate) async fn set_len(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    size: u64,
) -> std::io::Result<()> {
    let file_handle = file_handles
        .get(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file_handle.file.set_len(size).await
    }

    #[cfg(target_os = "wasi")]
    {
        file_handle.file.set_len(size)
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        drop(file_handle);
        let _ = size;
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

pub(crate) async fn try_clone(
    file_handles: &HandleManager<i64, FileHandle>,
    resource_manager: &ResourceManager,
    fd: i64,
) -> std::io::Result<i64> {
    let file_handle = file_handles
        .get(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    let cloned_file = file_handle.file.try_clone().await?;
    #[cfg(target_os = "wasi")]
    let cloned_file = file_handle.file.try_clone()?;
    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    let cloned_file = file_handle.file.clone();

    let cloned_handle = FileHandle {
        file: cloned_file,
        append: file_handle.append,
        mode: file_handle.mode,
    };

    let new_fd;
    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        new_fd = raw_file_descriptor("", resource_manager)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
    }
    #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
    {
        let _ = resource_manager;
        new_fd = raw_file_descriptor(&cloned_handle.file)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
    }

    drop(file_handle);
    file_handles
        .insert(new_fd, cloned_handle)
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(new_fd)
}

#[cfg_attr(
    all(target_family = "wasm", not(target_os = "wasi")),
    expect(clippy::unused_async)
)]
pub(crate) async fn read_at(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    buf: &mut [u8],
    offset: u64,
) -> std::io::Result<usize> {
    #[cfg(not(target_family = "wasm"))]
    {
        let file_handle = file_handles
            .get(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let std_file = file_handle.file.try_clone().await?.into_std().await;
        drop(file_handle);
        let mut buf_clone = buf.to_vec();
        let result = tokio::task::spawn_blocking(move || {
            #[cfg(target_family = "unix")]
            {
                use std::os::unix::fs::FileExt;
                std_file
                    .read_at(&mut buf_clone, offset)
                    .map(|n| (n, buf_clone))
            }
            #[cfg(target_family = "windows")]
            {
                use std::io::{Seek, SeekFrom};
                use std::os::windows::fs::FileExt;
                let mut std_file = std_file;
                let saved_pos = std_file.stream_position()?;
                let n = std_file.seek_read(&mut buf_clone, offset)?;
                std_file.seek(SeekFrom::Start(saved_pos))?;
                Ok::<_, std::io::Error>((n, buf_clone))
            }
        })
        .await
        .map_err(std::io::Error::other)??;
        buf[..result.0].copy_from_slice(&result.1[..result.0]);
        Ok(result.0)
    }

    #[cfg(target_os = "wasi")]
    {
        use std::io::{Read, Seek, SeekFrom};
        let mut file_handle = file_handles
            .get_mut(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let saved = file_handle.file.stream_position()?;
        file_handle.file.seek(SeekFrom::Start(offset))?;
        let n = file_handle.file.read(buf)?;
        file_handle.file.seek(SeekFrom::Start(saved))?;
        Ok(n)
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = (file_handles, fd, buf, offset);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

#[cfg_attr(
    all(target_family = "wasm", not(target_os = "wasi")),
    expect(clippy::unused_async)
)]
pub(crate) async fn write_at(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    buf: &[u8],
    offset: u64,
) -> std::io::Result<usize> {
    #[cfg(not(target_family = "wasm"))]
    {
        let file_handle = file_handles
            .get(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let std_file = file_handle.file.try_clone().await?.into_std().await;
        drop(file_handle);
        let buf_clone = buf.to_vec();
        tokio::task::spawn_blocking(move || {
            #[cfg(target_family = "unix")]
            {
                use std::os::unix::fs::FileExt;
                std_file.write_at(&buf_clone, offset)
            }
            #[cfg(target_family = "windows")]
            {
                use std::io::{Seek, SeekFrom};
                use std::os::windows::fs::FileExt;
                let mut std_file = std_file;
                let saved_pos = std_file.stream_position()?;
                let n = std_file.seek_write(&buf_clone, offset)?;
                std_file.seek(SeekFrom::Start(saved_pos))?;
                Ok(n)
            }
        })
        .await
        .map_err(std::io::Error::other)?
    }

    #[cfg(target_os = "wasi")]
    {
        use std::io::{Seek, SeekFrom, Write};
        let mut file_handle = file_handles
            .get_mut(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let saved = file_handle.file.stream_position()?;
        file_handle.file.seek(SeekFrom::Start(offset))?;
        let n = file_handle.file.write(buf)?;
        file_handle.file.seek(SeekFrom::Start(saved))?;
        Ok(n)
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = (file_handles, fd, buf, offset);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

#[cfg_attr(
    all(target_family = "wasm", not(target_os = "wasi")),
    expect(clippy::unused_async)
)]
pub(crate) async fn readv(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    #[cfg_attr(
        all(target_family = "wasm", not(target_os = "wasi")),
        expect(unused_mut)
    )]
    mut chunks: Vec<Vec<u8>>,
) -> std::io::Result<(usize, Vec<Vec<u8>>)> {
    #[cfg(not(target_family = "wasm"))]
    {
        let file_handle = file_handles
            .get(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let std_file = file_handle.file.try_clone().await?.into_std().await;
        drop(file_handle);
        tokio::task::spawn_blocking(move || {
            use std::io::Read;
            let mut std_file = std_file;
            let mut io_slices: Vec<std::io::IoSliceMut<'_>> = chunks
                .iter_mut()
                .map(|c| std::io::IoSliceMut::new(c))
                .collect();
            let n = std_file.read_vectored(&mut io_slices)?;
            Ok((n, chunks))
        })
        .await
        .map_err(std::io::Error::other)?
    }

    #[cfg(target_os = "wasi")]
    {
        use std::io::Read;
        let mut file_handle = file_handles
            .get_mut(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let mut io_slices: Vec<std::io::IoSliceMut<'_>> = chunks
            .iter_mut()
            .map(|c| std::io::IoSliceMut::new(c))
            .collect();
        let n = file_handle.file.read_vectored(&mut io_slices)?;
        Ok((n, chunks))
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = (file_handles, fd, chunks);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

#[cfg_attr(
    all(target_family = "wasm", not(target_os = "wasi")),
    expect(clippy::unused_async)
)]
pub(crate) async fn writev(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    chunks: Vec<Vec<u8>>,
) -> std::io::Result<usize> {
    #[cfg(not(target_family = "wasm"))]
    {
        let file_handle = file_handles
            .get(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let std_file = file_handle.file.try_clone().await?.into_std().await;
        drop(file_handle);
        tokio::task::spawn_blocking(move || {
            use std::io::Write;
            let mut std_file = std_file;
            let io_slices: Vec<std::io::IoSlice<'_>> =
                chunks.iter().map(|c| std::io::IoSlice::new(c)).collect();
            std_file.write_vectored(&io_slices)
        })
        .await
        .map_err(std::io::Error::other)?
    }

    #[cfg(target_os = "wasi")]
    {
        use std::io::Write;
        let mut file_handle = file_handles
            .get_mut(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let io_slices: Vec<std::io::IoSlice<'_>> =
            chunks.iter().map(|c| std::io::IoSlice::new(c)).collect();
        file_handle.file.write_vectored(&io_slices)
    }

    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    {
        let _ = (file_handles, fd, chunks);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "file I/O not supported on this platform",
        ))
    }
}

#[cfg_attr(target_family = "wasm", expect(clippy::unused_async))]
pub(crate) async fn lock(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    shared: bool,
    blocking: bool,
) -> std::io::Result<i32> {
    #[cfg(not(target_family = "wasm"))]
    {
        let file_handle = file_handles
            .get(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let std_file = file_handle.file.try_clone().await?.into_std().await;
        drop(file_handle);
        tokio::task::spawn_blocking(move || {
            if shared {
                if blocking {
                    std_file.lock_shared().map(|()| 0i32)
                } else {
                    match std_file.try_lock_shared() {
                        Ok(()) => Ok(0),
                        Err(_) => Ok(-1),
                    }
                }
            } else if blocking {
                std_file.lock().map(|()| 0i32)
            } else {
                match std_file.try_lock() {
                    Ok(()) => Ok(0),
                    Err(_) => Ok(-1),
                }
            }
        })
        .await
        .map_err(std::io::Error::other)?
    }

    #[cfg(target_family = "wasm")]
    {
        let _ = (file_handles, fd, shared, blocking);
        Ok(0)
    }
}

#[cfg_attr(target_family = "wasm", expect(clippy::unused_async))]
pub(crate) async fn unlock(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
) -> std::io::Result<()> {
    #[cfg(not(target_family = "wasm"))]
    {
        let file_handle = file_handles
            .get(&fd)
            .await
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
        let std_file = file_handle.file.try_clone().await?.into_std().await;
        drop(file_handle);
        tokio::task::spawn_blocking(move || std_file.unlock())
            .await
            .map_err(std::io::Error::other)?
    }

    #[cfg(target_family = "wasm")]
    {
        let _ = (file_handles, fd);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::VM;

    async fn test_vm() -> (impl VM, impl std::ops::Deref) {
        let (vm, _thread) = crate::test::thread().await.expect("thread");
        let vm_ref = vm;
        (vm_ref.clone(), vm_ref)
    }

    fn temp_path(name: &str) -> std::path::PathBuf {
        std::env::current_dir().expect("current dir").join(name)
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_open_read_write_close() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_open_rw.tmp");
        let _ = std::fs::remove_file(&path);

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            0o644,
        )
        .await
        .expect("open");

        let data = b"managed_files test";
        let written = write(fh, fd, data).await.expect("write");
        assert_eq!(written, data.len());

        seek(fh, fd, std::io::SeekFrom::Start(0))
            .await
            .expect("seek");

        let mut buf = vec![0u8; data.len()];
        let n = read(fh, fd, &mut buf).await.expect("read");
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_write_all() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_write_all.tmp");
        let _ = std::fs::remove_file(&path);

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            0o644,
        )
        .await
        .expect("open");

        let data = b"write_all test data";
        write_all(fh, fd, data).await.expect("write_all");

        seek(fh, fd, std::io::SeekFrom::Start(0))
            .await
            .expect("seek");
        let mut buf = vec![0u8; data.len()];
        let n = read(fh, fd, &mut buf).await.expect("read");
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    async fn test_write_all_bad_fd() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        let result = write_all(fh, 999_999, b"data").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_read_bad_fd() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        let mut buf = [0u8; 16];
        let result = read(fh, 999_999, &mut buf).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_write_bad_fd() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        let result = write(fh, 999_999, b"data").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_close_nonexistent_fd() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        // Closing a non-existent fd should not panic
        close(fh, 999_999).await;
    }

    #[tokio::test]
    async fn test_metadata_bad_fd() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        let result = metadata(fh, 999_999).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_metadata_success() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_metadata.tmp");
        std::fs::write(&path, b"metadata test").unwrap();

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(fh, rm, path.to_str().unwrap(), 0, 0)
            .await
            .expect("open");

        let meta = metadata(fh, fd).await.expect("metadata");
        assert!(meta.len() > 0);

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    async fn test_seek_bad_fd() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        let result = seek(fh, 999_999, std::io::SeekFrom::Start(0)).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_sync_all_bad_fd() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        let result = sync_all(fh, 999_999).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_sync_data_bad_fd() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        let result = sync_data(fh, 999_999).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_set_len_bad_fd() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        let result = set_len(fh, 999_999, 100).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_set_len_success() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_set_len.tmp");
        let _ = std::fs::remove_file(&path);

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            0o644,
        )
        .await
        .expect("open");

        set_len(fh, fd, 42).await.expect("set_len");
        let meta = metadata(fh, fd).await.expect("metadata");
        assert_eq!(meta.len(), 42);

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_sync_all_success() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_sync_all.tmp");
        let _ = std::fs::remove_file(&path);

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            0o644,
        )
        .await
        .expect("open");

        write_all(fh, fd, b"sync test").await.expect("write_all");
        sync_all(fh, fd).await.expect("sync_all");

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_sync_data_success() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_sync_data.tmp");
        let _ = std::fs::remove_file(&path);

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            0o644,
        )
        .await
        .expect("open");

        write_all(fh, fd, b"sync data test")
            .await
            .expect("write_all");
        sync_data(fh, fd).await.expect("sync_data");

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_try_clone() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_try_clone.tmp");
        std::fs::write(&path, b"clone test data").unwrap();

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(fh, rm, path.to_str().unwrap(), 0, 0)
            .await
            .expect("open");

        let cloned_fd = try_clone(fh, rm, fd).await.expect("try_clone");
        assert_ne!(fd, cloned_fd);

        let mut buf = vec![0u8; 15];
        let n = read(fh, cloned_fd, &mut buf).await.expect("read cloned");
        assert_eq!(n, 15);
        assert_eq!(&buf, b"clone test data");

        close(fh, fd).await;
        close(fh, cloned_fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_read_at_write_at() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_read_write_at.tmp");
        let _ = std::fs::remove_file(&path);

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            0o644,
        )
        .await
        .expect("open");

        // Write at offset 5
        let data = b"hello";
        let n = write_at(fh, fd, data, 5).await.expect("write_at");
        assert_eq!(n, data.len());

        // Read at offset 5
        let mut buf = vec![0u8; 5];
        let n = read_at(fh, fd, &mut buf, 5).await.expect("read_at");
        assert_eq!(n, 5);
        assert_eq!(&buf, b"hello");

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_readv_writev() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_readv_writev.tmp");
        let _ = std::fs::remove_file(&path);

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_CREAT | libc::O_RDWR | libc::O_TRUNC,
            0o644,
        )
        .await
        .expect("open");

        let chunks = vec![b"hello ".to_vec(), b"world".to_vec()];
        let n = writev(fh, fd, chunks).await.expect("writev");
        assert_eq!(n, 11);

        seek(fh, fd, std::io::SeekFrom::Start(0))
            .await
            .expect("seek");

        let bufs = vec![vec![0u8; 6], vec![0u8; 5]];
        let (n, filled) = readv(fh, fd, bufs).await.expect("readv");
        assert_eq!(n, 11);
        assert_eq!(&filled[0], b"hello ");
        assert_eq!(&filled[1], b"world");

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_lock_unlock() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_lock.tmp");
        std::fs::write(&path, b"lock test").unwrap();

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(fh, rm, path.to_str().unwrap(), libc::O_RDWR, 0)
            .await
            .expect("open");

        let result = lock(fh, fd, false, true).await.expect("lock");
        assert_eq!(result, 0);

        unlock(fh, fd).await.expect("unlock");

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_lock_shared() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_lock_shared.tmp");
        std::fs::write(&path, b"shared lock test").unwrap();

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(fh, rm, path.to_str().unwrap(), 0, 0)
            .await
            .expect("open");

        let result = lock(fh, fd, true, true).await.expect("lock shared");
        assert_eq!(result, 0);

        unlock(fh, fd).await.expect("unlock");

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_open_exclusive() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_open_excl.tmp");
        let _ = std::fs::remove_file(&path);

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_CREAT | libc::O_EXCL | libc::O_WRONLY,
            0o644,
        )
        .await
        .expect("open exclusive");

        // Opening again with O_EXCL should fail
        let result = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_CREAT | libc::O_EXCL | libc::O_WRONLY,
            0o644,
        )
        .await;
        assert!(result.is_err());

        close(fh, fd).await;
        std::fs::remove_file(&path).ok();
    }

    #[tokio::test]
    async fn test_open_nonexistent() {
        let (vm, _hold) = test_vm().await;
        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let result = open(fh, rm, "/nonexistent_path_xyz123/file.tmp", 0, 0).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_open_append() {
        let (vm, _hold) = test_vm().await;
        let path = temp_path("_test_mf_append.tmp");
        std::fs::write(&path, b"initial").unwrap();

        let fh = vm.file_handles();
        let rm = vm.resource_manager();
        let fd = open(
            fh,
            rm,
            path.to_str().unwrap(),
            libc::O_WRONLY | libc::O_APPEND,
            0,
        )
        .await
        .expect("open append");

        write_all(fh, fd, b" appended").await.expect("write");
        close(fh, fd).await;

        let content = std::fs::read(&path).unwrap();
        assert_eq!(content, b"initial appended");

        std::fs::remove_file(&path).ok();
    }
}
