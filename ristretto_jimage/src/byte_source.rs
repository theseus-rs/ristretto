use crate::Error::{InvalidIndex, PoisonedLock};
use crate::Result;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Bound;
use std::path::Path;
use std::sync::Mutex;

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
    pub(crate) fn get_bytes<R>(&'_ self, range: R) -> Result<Cow<'_, [u8]>>
    where
        R: std::ops::RangeBounds<usize>,
    {
        let len = self.len()?;
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start.checked_add(1).ok_or(InvalidIndex(start))?,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end.checked_add(1).ok_or(InvalidIndex(end))?,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => len,
        };

        if start > end || end > len {
            return Err(InvalidIndex(start));
        }

        match self {
            ByteSource::Bytes(bytes) => Ok(Cow::Borrowed(&bytes[start..end])),
            ByteSource::File(file) => {
                let file = &mut file
                    .lock()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                file.seek(SeekFrom::Start(u64::try_from(start)?))?;
                let mut buffer = vec![0u8; end - start];
                file.read_exact(&mut buffer)?;
                Ok(Cow::Owned(buffer))
            }
            #[cfg(not(target_family = "wasm"))]
            ByteSource::MemoryMap(mmap) => Ok(Cow::Borrowed(&mmap[start..end])),
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
                let bytes_slice = &bytes[start_index..];
                let length = memchr::memchr(0, bytes_slice).ok_or(InvalidIndex(start_index))?;
                let end_index = start_index + length;
                Ok(Cow::Borrowed(&bytes[start_index..end_index]))
            }
            ByteSource::File(file) => {
                let file = &mut file
                    .lock()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                file.seek(SeekFrom::Start(start_index as u64))?;
                let mut buffer = Vec::new();
                let mut read_buffer = [0u8; 64];

                loop {
                    let bytes_read = file.read(&mut read_buffer)?;
                    if bytes_read == 0 {
                        break;
                    }

                    if let Some(null_position) = memchr::memchr(0, &read_buffer[..bytes_read]) {
                        buffer.extend_from_slice(&read_buffer[..null_position]);
                        return Ok(Cow::Owned(buffer));
                    }

                    buffer.extend_from_slice(&read_buffer[..bytes_read]);
                }

                Err(InvalidIndex(start_index))
            }
            #[cfg(not(target_family = "wasm"))]
            ByteSource::MemoryMap(mmap) => {
                let bytes_slice = &mmap[start_index..];
                let length = memchr::memchr(0, bytes_slice).ok_or(InvalidIndex(start_index))?;
                let end_index = start_index + length;
                Ok(Cow::Borrowed(&mmap[start_index..end_index]))
            }
        }
    }

    /// Returns the length of the underlying byte source.
    pub(crate) fn len(&self) -> Result<usize> {
        let length = match self {
            ByteSource::Bytes(bytes) => bytes.len(),
            ByteSource::File(file) => {
                let file = &mut file
                    .lock()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                file.metadata()
                    .map(|metadata| usize::try_from(metadata.len()))
                    .unwrap_or(usize::try_from(0))?
            }
            #[cfg(not(target_family = "wasm"))]
            ByteSource::MemoryMap(mmap) => mmap.len(),
        };
        Ok(length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_byte_source_type() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
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
    fn test_byte_source_bytes_get_bytes() -> Result<()> {
        let data = b"Hello, world!".to_vec();
        let byte_source = ByteSource::Bytes(data);
        assert_eq!(13, byte_source.len()?);
        let bytes = byte_source.get_bytes(0..5)?;
        assert_eq!(&*bytes, b"Hello");
        Ok(())
    }

    #[test]
    fn test_byte_source_file_get_bytes() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
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
        let mut temp_file = NamedTempFile::new()?;
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
        Ok(())
    }

    #[test]
    fn test_byte_source_file_get_bytes_to_null() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        let data = b"Hello\0world!";
        temp_file.write_all(data)?;
        temp_file.flush()?;

        let file = File::open(temp_file.path())?;
        let byte_source = ByteSource::File(Mutex::new(file));
        let bytes = byte_source.get_bytes_to_null(0)?;
        assert_eq!(&*bytes, b"Hello");
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_byte_source_mmap_get_bytes_to_null() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        let data = b"Hello\0world!";
        temp_file.write_all(data)?;
        temp_file.flush()?;

        let byte_source = ByteSource::from(temp_file.path())?;
        let bytes = byte_source.get_bytes_to_null(0)?;
        assert_eq!(&*bytes, b"Hello");
        Ok(())
    }
}
