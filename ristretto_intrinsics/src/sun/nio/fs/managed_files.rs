use ristretto_types::handles::HandleManager;
use std::io::{Read, Seek, Write};

pub(crate) async fn open(
    nio_file_handles: &HandleManager<i32, std::fs::File>,
    fd: i32,
    path: &str,
    flags: i32,
    _mode: i32,
) -> std::io::Result<i32> {
    let mut open_options = std::fs::OpenOptions::new();
    // O_RDONLY=0, O_WRONLY=1, O_RDWR=2
    let access_mode = flags & 0x3;
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
    if flags & 0x40 != 0 {
        open_options.create(true);
    }
    if flags & 0x200 != 0 {
        open_options.truncate(true);
    }
    if flags & 0x400 != 0 {
        open_options.append(true);
    }

    let file = open_options.open(path)?;
    nio_file_handles
        .insert(fd, file)
        .await
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(fd)
}

pub(crate) async fn read(
    nio_file_handles: &HandleManager<i32, std::fs::File>,
    fd: i32,
    buf: &mut [u8],
) -> std::io::Result<usize> {
    let mut file = nio_file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
    file.read(buf)
}

pub(crate) async fn write(
    nio_file_handles: &HandleManager<i32, std::fs::File>,
    fd: i32,
    buf: &[u8],
) -> std::io::Result<usize> {
    let mut file = nio_file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
    file.write(buf)
}

pub(crate) async fn close(nio_file_handles: &HandleManager<i32, std::fs::File>, fd: i32) {
    nio_file_handles.remove(&fd).await;
}

pub(crate) async fn metadata(
    nio_file_handles: &HandleManager<i32, std::fs::File>,
    fd: i32,
) -> std::io::Result<std::fs::Metadata> {
    let file = nio_file_handles
        .get(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
    file.metadata()
}

pub(crate) async fn seek(
    nio_file_handles: &HandleManager<i32, std::fs::File>,
    fd: i32,
    pos: std::io::SeekFrom,
) -> std::io::Result<u64> {
    let mut file = nio_file_handles
        .get_mut(&fd)
        .await
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "bad fd"))?;
    file.seek(pos)
}
