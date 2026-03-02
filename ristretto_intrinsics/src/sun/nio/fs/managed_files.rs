use parking_lot::RwLock;
use ristretto_types::handles::{HandleManager, NioFile};
use std::collections::HashMap;
#[cfg(target_family = "wasm")]
use std::io::{Read, Seek, Write};
use std::sync::atomic::{AtomicI32, Ordering};
#[cfg(not(target_family = "wasm"))]
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

/// Counter for generating unique Windows file handles.
static WINDOWS_HANDLE_COUNTER: AtomicI32 = AtomicI32::new(0x1000);

/// Map from file handle/fd to its filesystem path (for `GetFinalPathNameByHandle` emulation).
static FILE_PATHS: std::sync::LazyLock<RwLock<HashMap<i32, String>>> =
    std::sync::LazyLock::new(|| RwLock::new(HashMap::new()));

pub(crate) async fn open(
    nio_file_handles: &HandleManager<i32, NioFile>,
    fd: i32,
    path: &str,
    flags: i32,
    _mode: i32,
) -> std::io::Result<i32> {
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
    #[cfg(target_family = "wasm")]
    let file = open_options.open(path)?;

    nio_file_handles
        .insert(fd, file)
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(fd)
}

pub(crate) async fn read(
    nio_file_handles: &HandleManager<i32, NioFile>,
    fd: i32,
    buf: &mut [u8],
) -> std::io::Result<usize> {
    let mut file = nio_file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file.read(buf).await
    }

    #[cfg(target_family = "wasm")]
    {
        file.read(buf)
    }
}

pub(crate) async fn write(
    nio_file_handles: &HandleManager<i32, NioFile>,
    fd: i32,
    buf: &[u8],
) -> std::io::Result<usize> {
    let mut file = nio_file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file.write(buf).await
    }

    #[cfg(target_family = "wasm")]
    {
        file.write(buf)
    }
}

pub(crate) async fn close(nio_file_handles: &HandleManager<i32, NioFile>, fd: i32) {
    nio_file_handles.remove(&fd).await;
    FILE_PATHS.write().remove(&fd);
}

pub(crate) async fn metadata(
    nio_file_handles: &HandleManager<i32, NioFile>,
    fd: i32,
) -> std::io::Result<std::fs::Metadata> {
    let file = nio_file_handles
        .get(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file.metadata().await
    }

    #[cfg(target_family = "wasm")]
    {
        file.metadata()
    }
}

pub(crate) async fn seek(
    nio_file_handles: &HandleManager<i32, NioFile>,
    fd: i32,
    pos: std::io::SeekFrom,
) -> std::io::Result<u64> {
    let mut file = nio_file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;

    #[cfg(not(target_family = "wasm"))]
    {
        file.seek(pos).await
    }

    #[cfg(target_family = "wasm")]
    {
        file.seek(pos)
    }
}

/// Get the file size for the given file descriptor.
pub(crate) async fn file_size(
    nio_file_handles: &HandleManager<i32, NioFile>,
    fd: i32,
) -> std::io::Result<i64> {
    let meta = metadata(nio_file_handles, fd).await?;
    #[expect(clippy::cast_possible_wrap)]
    Ok(meta.len() as i64)
}

/// Open a file Windows-style (for `CreateFile` emulation) and return a handle.
pub(crate) async fn open_windows(
    nio_file_handles: &HandleManager<i32, NioFile>,
    path: &str,
) -> std::io::Result<i32> {
    let fd = WINDOWS_HANDLE_COUNTER.fetch_add(1, Ordering::Relaxed);

    #[cfg(not(target_family = "wasm"))]
    let file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(path)
        .await
        .or_else(|_| {
            // Fall back to read-only if read-write fails
            std::fs::OpenOptions::new()
                .read(true)
                .open(path)
                .map(tokio::fs::File::from_std)
        })?;

    #[cfg(target_family = "wasm")]
    let file = std::fs::OpenOptions::new().read(true).open(path)?;

    nio_file_handles
        .insert(fd, file)
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;

    FILE_PATHS.write().insert(fd, path.to_string());
    Ok(fd)
}

/// Get the filesystem path associated with a handle (for `GetFinalPathNameByHandle` emulation).
pub(crate) fn get_path(_nio_file_handles: &HandleManager<i32, NioFile>, fd: i32) -> String {
    FILE_PATHS.read().get(&fd).cloned().unwrap_or_default()
}
