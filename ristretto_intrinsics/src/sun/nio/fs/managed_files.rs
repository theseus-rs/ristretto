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
    _mode: i32,
) -> std::io::Result<i64> {
    // O_RDONLY=0, O_WRONLY=1, O_RDWR=2
    let access_mode = flags & 0x3;
    let create = flags & 0x40 != 0;
    let truncate = flags & 0x200 != 0;
    let append = flags & 0x400 != 0;

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
    if create {
        open_options.create(true);
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
        fd = raw_file_descriptor(&path.to_string(), resource_manager)
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
    #[cfg_attr(target_family = "wasm", allow(unused_mut))]
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
    #[cfg_attr(target_family = "wasm", allow(unused_mut))]
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

pub(crate) async fn close(file_handles: &HandleManager<i64, FileHandle>, fd: i64) {
    file_handles.remove(&fd).await;
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
    #[cfg_attr(target_family = "wasm", allow(unused_mut))]
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

pub(crate) async fn readv(
    file_handles: &HandleManager<i64, FileHandle>,
    fd: i64,
    #[cfg_attr(target_family = "wasm", allow(unused_mut))] mut chunks: Vec<Vec<u8>>,
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
