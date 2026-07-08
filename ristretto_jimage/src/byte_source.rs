use crate::Error::{InvalidIndex, PoisonedLock};
use crate::Result;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Range;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

/// A source of bytes, which can be an in-memory byte array, a file, or a memory-mapped file.
#[derive(Debug)]
pub(crate) enum ByteSource {
    /// In-memory byte array
    Bytes(Vec<u8>),
    /// File with mutex for thread-safe access
    File(Mutex<File>),
    /// Memory-mapped file (not available in WebAssembly)
    #[cfg(not(target_family = "wasm"))]
    MemoryMap(memmap2::Mmap),
}

impl ByteSource {
    /// Creates a new `ByteSource` from the given file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or memory-mapped.
    pub fn from(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path)?;

        #[cfg(not(target_family = "wasm"))]
        {
            // Safety: This is safe because:
            // 1. The file is opened in read-only mode.
            // 2. The contents of the file are not modified through the memory map.
            #[expect(unsafe_code)]
            if let Ok(mmap) = unsafe { memmap2::Mmap::map(&file) } {
                return Ok(ByteSource::MemoryMap(mmap));
            }
        }

        Ok(ByteSource::File(Mutex::new(file)))
    }

    /// Reads a range of bytes from the underlying byte source.
    ///
    /// # Errors
    ///
    /// Returns an error if the read operation fails or if the range is out of bounds.
    pub(crate) fn get_bytes(&'_ self, range: Range<usize>) -> Result<Cow<'_, [u8]>> {
        let len = self.len()?;
        if range.start > range.end || range.end > len {
            return Err(InvalidIndex(range.start));
        }

        self.get_bytes_range(range.start, range.end)
    }

    fn get_bytes_range(&'_ self, start: usize, end: usize) -> Result<Cow<'_, [u8]>> {
        match self {
            ByteSource::Bytes(bytes) => {
                let bytes = bytes.get(start..end).ok_or(InvalidIndex(start))?;
                Ok(Cow::Borrowed(bytes))
            }
            ByteSource::File(file) => {
                let file = &mut lock_file(file)?;
                file.seek(SeekFrom::Start(start as u64))?;
                let mut buffer = vec![0u8; end - start];
                file.read_exact(&mut buffer)?;
                Ok(Cow::Owned(buffer))
            }
            #[cfg(not(target_family = "wasm"))]
            ByteSource::MemoryMap(mmap) => {
                let bytes = mmap.get(start..end).ok_or(InvalidIndex(start))?;
                Ok(Cow::Borrowed(bytes))
            }
        }
    }

    /// Reads bytes from the underlying byte source starting at `start` until a null byte (0)
    /// is encountered. This is useful for reading null-terminated strings.
    ///
    /// # Errors
    ///
    /// Returns an error if the read operation fails or if no null byte is found.
    pub(crate) fn get_bytes_to_null(&'_ self, start_index: usize) -> Result<Cow<'_, [u8]>> {
        match self {
            ByteSource::Bytes(bytes) => {
                let bytes_slice = bytes.get(start_index..).ok_or(InvalidIndex(start_index))?;
                let length = memchr::memchr(0, bytes_slice).ok_or(InvalidIndex(start_index))?;
                let end_index = start_index + length;
                let bytes = bytes
                    .get(start_index..end_index)
                    .ok_or(InvalidIndex(start_index))?;
                Ok(Cow::Borrowed(bytes))
            }
            ByteSource::File(file) => {
                let file = &mut lock_file(file)?;
                file.seek(SeekFrom::Start(start_index as u64))?;
                let mut buffer = Vec::new();
                let mut read_buffer = [0u8; 64];

                loop {
                    let bytes_read = file.read(&mut read_buffer)?;
                    if bytes_read == 0 {
                        break;
                    }

                    let read_bytes = read_buffer
                        .get(..bytes_read)
                        .ok_or(InvalidIndex(start_index))?;
                    if let Some(null_position) = memchr::memchr(0, read_bytes) {
                        let prefix = read_bytes
                            .get(..null_position)
                            .ok_or(InvalidIndex(start_index))?;
                        buffer.extend_from_slice(prefix);
                        return Ok(Cow::Owned(buffer));
                    }

                    buffer.extend_from_slice(read_bytes);
                }

                Err(InvalidIndex(start_index))
            }
            #[cfg(not(target_family = "wasm"))]
            ByteSource::MemoryMap(mmap) => {
                let bytes_slice = mmap.get(start_index..).ok_or(InvalidIndex(start_index))?;
                let length = memchr::memchr(0, bytes_slice).ok_or(InvalidIndex(start_index))?;
                let end_index = start_index + length;
                let bytes = mmap
                    .get(start_index..end_index)
                    .ok_or(InvalidIndex(start_index))?;
                Ok(Cow::Borrowed(bytes))
            }
        }
    }

    /// Returns the length of the underlying byte source.
    pub(crate) fn len(&self) -> Result<usize> {
        let length = match self {
            ByteSource::Bytes(bytes) => bytes.len(),
            ByteSource::File(file) => {
                let file = &mut lock_file(file)?;
                #[cfg(target_pointer_width = "64")]
                {
                    file_len(file)
                }
                #[cfg(not(target_pointer_width = "64"))]
                {
                    file_len(file)?
                }
            }
            #[cfg(not(target_family = "wasm"))]
            ByteSource::MemoryMap(mmap) => mmap.len(),
        };
        Ok(length)
    }
}

#[inline]
fn lock_file(file: &Mutex<File>) -> Result<MutexGuard<'_, File>> {
    match file.lock() {
        Ok(file) => Ok(file),
        Err(error) => Err(PoisonedLock(error.to_string())),
    }
}

#[inline]
#[cfg(target_pointer_width = "64")]
fn file_len(file: &File) -> usize {
    let length = file.metadata().map_or(0, |metadata| metadata.len());
    usize::from_ne_bytes(length.to_ne_bytes())
}

#[inline]
#[cfg(not(target_pointer_width = "64"))]
fn file_len(file: &File) -> Result<usize> {
    let length = file.metadata().map_or(0, |metadata| metadata.len());
    Ok(usize::try_from(length)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn make_named_tempfile() -> std::io::Result<NamedTempFile> {
        ristretto_test_util::init_wasi_tempdir();
        NamedTempFile::new()
    }

    #[test]
    fn test_byte_source_type() -> Result<()> {
        let mut temp_file = make_named_tempfile()?;
        let data = b"Hello, world!";
        temp_file.write_all(data)?;
        temp_file.flush()?;

        let byte_source = ByteSource::from(temp_file.path())?;

        #[cfg(target_family = "wasm")]
        assert!(matches!(byte_source, ByteSource::File(_)));
        #[cfg(not(target_family = "wasm"))]
        assert!(matches!(byte_source, ByteSource::MemoryMap(_)));
        Ok(())
    }

    #[test]
    fn test_byte_source_from_missing_file() {
        assert!(matches!(
            ByteSource::from(Path::new("missing.jimage")),
            Err(crate::Error::IoError(_))
        ));
    }

    #[test]
    #[cfg(all(unix, not(target_family = "wasm")))]
    fn test_byte_source_empty_file_falls_back_to_file() -> Result<()> {
        ristretto_test_util::init_wasi_tempdir();
        let temp_dir = tempfile::tempdir()?;
        let byte_source = ByteSource::from(temp_dir.path())?;
        assert!(matches!(byte_source, ByteSource::File(_)));
        Ok(())
    }

    #[test]
    fn test_byte_source_bytes_get_bytes() -> Result<()> {
        let data = b"Hello, world!".to_vec();
        let byte_source = ByteSource::Bytes(data);
        assert_eq!(13, byte_source.len()?);
        let bytes = byte_source.get_bytes(0..5)?;
        assert_eq!(&*bytes, b"Hello");
        let bytes = byte_source.get_bytes(0..13)?;
        assert_eq!(&*bytes, b"Hello, world!");
        let bytes = byte_source.get_bytes(0..5)?;
        assert_eq!(&*bytes, b"Hello");
        let bytes = byte_source.get_bytes(7..13)?;
        assert_eq!(&*bytes, b"world!");
        let bytes = byte_source.get_bytes(0..0)?;
        assert!(bytes.is_empty());
        let invalid_range = Range { start: 6, end: 5 };
        assert!(matches!(
            byte_source.get_bytes(invalid_range),
            Err(InvalidIndex(6))
        ));
        assert!(matches!(byte_source.get_bytes(0..14), Err(InvalidIndex(0))));
        Ok(())
    }

    #[test]
    fn test_byte_source_file_get_bytes() -> Result<()> {
        let mut temp_file = make_named_tempfile()?;
        let data = b"Hello, world!";
        temp_file.write_all(data)?;
        temp_file.flush()?;

        let file = File::open(temp_file.path())?;
        let byte_source = ByteSource::File(Mutex::new(file));
        assert_eq!(13, byte_source.len()?);
        let bytes = byte_source.get_bytes(0..5)?;
        assert_eq!(&*bytes, b"Hello");
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_byte_source_mmap_get_bytes() -> Result<()> {
        let mut temp_file = make_named_tempfile()?;
        let data = b"Hello, world!";
        temp_file.write_all(data)?;
        temp_file.flush()?;

        let byte_source = ByteSource::from(temp_file.path())?;
        assert_eq!(13, byte_source.len()?);
        let bytes = byte_source.get_bytes(0..5)?;
        assert_eq!(&*bytes, b"Hello");
        Ok(())
    }

    #[test]
    fn test_byte_source_bytes_get_bytes_to_null() -> Result<()> {
        let data = b"Hello\0world!".to_vec();
        let byte_source = ByteSource::Bytes(data);
        let bytes = byte_source.get_bytes_to_null(0)?;
        assert_eq!(&*bytes, b"Hello");

        let byte_source = ByteSource::Bytes(b"Hello".to_vec());
        assert!(matches!(
            byte_source.get_bytes_to_null(0),
            Err(InvalidIndex(0))
        ));
        Ok(())
    }

    #[test]
    fn test_byte_source_file_get_bytes_to_null() -> Result<()> {
        let mut temp_file = make_named_tempfile()?;
        let data = b"Hello\0world!";
        temp_file.write_all(data)?;
        temp_file.flush()?;

        let file = File::open(temp_file.path())?;
        let byte_source = ByteSource::File(Mutex::new(file));
        let bytes = byte_source.get_bytes_to_null(0)?;
        assert_eq!(&*bytes, b"Hello");
        Ok(())
    }

    #[test]
    fn test_byte_source_file_get_bytes_to_null_after_multiple_reads() -> Result<()> {
        let mut temp_file = make_named_tempfile()?;
        let mut data = vec![b'a'; 70];
        data.push(0);
        data.extend_from_slice(b"tail");
        temp_file.write_all(&data)?;
        temp_file.flush()?;

        let file = File::open(temp_file.path())?;
        let byte_source = ByteSource::File(Mutex::new(file));
        let bytes = byte_source.get_bytes_to_null(0)?;
        assert_eq!(&*bytes, vec![b'a'; 70].as_slice());
        Ok(())
    }

    #[test]
    fn test_byte_source_file_get_bytes_to_null_without_null() -> Result<()> {
        let mut temp_file = make_named_tempfile()?;
        let data = vec![b'a'; 70];
        temp_file.write_all(&data)?;
        temp_file.flush()?;

        let file = File::open(temp_file.path())?;
        let byte_source = ByteSource::File(Mutex::new(file));
        assert!(matches!(
            byte_source.get_bytes_to_null(0),
            Err(InvalidIndex(0))
        ));
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_byte_source_poisoned_lock() -> Result<()> {
        let mut temp_file = make_named_tempfile()?;
        temp_file.write_all(b"Hello\0world!")?;
        temp_file.flush()?;

        let file = File::open(temp_file.path())?;
        let file = Mutex::new(file);
        let result = std::panic::catch_unwind(|| poison_file_lock(&file));
        assert!(result.is_err());
        let byte_source = ByteSource::File(file);

        assert!(matches!(byte_source.get_bytes(0..5), Err(PoisonedLock(_))));
        assert!(matches!(
            byte_source.get_bytes_range(0, 5),
            Err(PoisonedLock(_))
        ));
        assert!(matches!(
            byte_source.get_bytes_to_null(0),
            Err(PoisonedLock(_))
        ));
        assert!(matches!(byte_source.len(), Err(PoisonedLock(_))));
        Ok(())
    }

    #[test]
    fn test_byte_source_file_get_bytes_range_unexpected_eof() -> Result<()> {
        let mut temp_file = make_named_tempfile()?;
        temp_file.write_all(b"short")?;
        temp_file.flush()?;

        let file = File::open(temp_file.path())?;
        let byte_source = ByteSource::File(Mutex::new(file));
        assert!(matches!(
            byte_source.get_bytes_range(0, 10),
            Err(crate::Error::IoError(error)) if error.kind() == std::io::ErrorKind::UnexpectedEof
        ));
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn test_byte_source_file_rejects_non_seekable_file() -> Result<()> {
        use std::os::fd::OwnedFd;
        use std::os::unix::net::UnixStream;

        let (stream, _peer) = UnixStream::pair()?;
        let owned_fd: OwnedFd = stream.into();
        let file = File::from(owned_fd);
        let byte_source = ByteSource::File(Mutex::new(file));

        assert!(matches!(
            byte_source.get_bytes_range(0, 1),
            Err(crate::Error::IoError(_))
        ));
        assert!(matches!(
            byte_source.get_bytes_to_null(0),
            Err(crate::Error::IoError(_))
        ));
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn test_byte_source_file_get_bytes_to_null_rejects_directory_read() -> Result<()> {
        ristretto_test_util::init_wasi_tempdir();
        let temp_dir = tempfile::tempdir()?;
        let file = File::open(temp_dir.path())?;
        let byte_source = ByteSource::File(Mutex::new(file));

        assert!(matches!(
            byte_source.get_bytes_to_null(0),
            Err(crate::Error::IoError(_))
        ));
        Ok(())
    }

    fn poison_file_lock(file: &Mutex<File>) {
        let _guard = file.lock().expect("lock should be available");
        panic!("poison byte source lock");
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_byte_source_mmap_get_bytes_to_null() -> Result<()> {
        let mut temp_file = make_named_tempfile()?;
        let data = b"Hello\0world!";
        temp_file.write_all(data)?;
        temp_file.flush()?;

        let byte_source = ByteSource::from(temp_file.path())?;
        let bytes = byte_source.get_bytes_to_null(0)?;
        assert_eq!(&*bytes, b"Hello");
        drop(byte_source);

        let mut temp_file = make_named_tempfile()?;
        temp_file.write_all(b"Hello")?;
        temp_file.flush()?;

        let byte_source = ByteSource::from(temp_file.path())?;
        assert!(matches!(
            byte_source.get_bytes_to_null(0),
            Err(InvalidIndex(0))
        ));
        Ok(())
    }
}
