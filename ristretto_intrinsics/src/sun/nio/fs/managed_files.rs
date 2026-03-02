use ristretto_types::handles::{HandleManager, NioFile};
#[cfg(target_family = "wasm")]
use std::io::{Read, Seek, Write};
#[cfg(not(target_family = "wasm"))]
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

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
