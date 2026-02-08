use bitflags::bitflags;

bitflags! {
    /// File Mode Flags
    ///
    /// This enum represents the various modes in which a `File` can be opened.
    ///
    /// # References
    ///
    /// - [RandomAccessFile (modes)](https://docs.oracle.com/en/java/javase/25/docs/api/java.base/java/io/RandomAccessFile.html#%3Cinit%3E(java.io.File,java.lang.String))
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct FileModeFlags: u16 {
        const READ_ONLY = 1;
        const READ_WRITE = 2;
        const SYNCHRONIZE_ALL = 4;
        const SYNCHRONIZE_DATA = 8;
        const TEMPORARY = 16;
    }
}

/// Represents a handle to a file.
#[derive(Debug)]
pub struct FileHandle {
    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    pub file: String,
    #[cfg(target_os = "wasi")]
    pub file: std::fs::File,
    #[cfg(not(target_family = "wasm"))]
    pub file: tokio::fs::File,
    pub append: bool,
    pub mode: FileModeFlags,
}

#[cfg(target_os = "wasi")]
impl From<(std::fs::File, FileModeFlags)> for FileHandle {
    fn from((file, mode): (std::fs::File, FileModeFlags)) -> Self {
        FileHandle {
            file,
            append: false,
            mode,
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<(tokio::fs::File, FileModeFlags)> for FileHandle {
    fn from((file, mode): (tokio::fs::File, FileModeFlags)) -> Self {
        FileHandle {
            file,
            append: false,
            mode,
        }
    }
}

#[cfg(target_os = "wasi")]
impl From<(std::fs::File, bool)> for FileHandle {
    fn from((file, append): (std::fs::File, bool)) -> Self {
        FileHandle {
            file,
            append,
            mode: FileModeFlags::empty(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<(tokio::fs::File, bool)> for FileHandle {
    fn from((file, append): (tokio::fs::File, bool)) -> Self {
        FileHandle {
            file,
            append,
            mode: FileModeFlags::empty(),
        }
    }
}

#[cfg(target_os = "wasi")]
impl TryInto<std::fs::File> for FileHandle {
    type Error = crate::Error;

    fn try_into(self) -> Result<std::fs::File, Self::Error> {
        let FileHandle { file, .. } = self;
        Ok(file)
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryInto<tokio::fs::File> for FileHandle {
    type Error = crate::Error;

    fn try_into(self) -> Result<tokio::fs::File, Self::Error> {
        let FileHandle { file, .. } = self;
        Ok(file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use tempfile::NamedTempFile;
    use tokio::fs::File;

    #[tokio::test]
    async fn test_file_handle_from_file_and_append() -> Result<()> {
        let tmp = NamedTempFile::new().expect("temp file");
        let file = File::create(tmp.path()).await.expect("create");
        let file_handle: FileHandle = (file, false).into();
        assert!(!file_handle.append);
        assert_eq!(file_handle.mode, FileModeFlags::empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_file_handle_from_file_and_append_true() -> Result<()> {
        let tmp = NamedTempFile::new().expect("temp file");
        let file = File::create(tmp.path()).await.expect("create");
        let file_handle: FileHandle = (file, true).into();
        assert!(file_handle.append);
        assert_eq!(file_handle.mode, FileModeFlags::empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_file_handle_from_file_and_mode() -> Result<()> {
        let tmp = NamedTempFile::new().expect("temp file");
        let file = File::create(tmp.path()).await.expect("create");
        let expected_mode = FileModeFlags::READ_WRITE;
        let file_handle: FileHandle = (file, expected_mode).into();
        assert!(!file_handle.append);
        assert_eq!(file_handle.mode, expected_mode);
        Ok(())
    }

    #[tokio::test]
    async fn test_file_handle_from_file_and_mode_read_only() -> Result<()> {
        let tmp = NamedTempFile::new().expect("temp file");
        let file = File::create(tmp.path()).await.expect("create");
        let file_handle: FileHandle = (file, FileModeFlags::READ_ONLY).into();
        assert_eq!(file_handle.mode, FileModeFlags::READ_ONLY);
        Ok(())
    }

    #[tokio::test]
    async fn test_file_handle_try_into_file() -> Result<()> {
        let tmp = NamedTempFile::new().expect("temp file");
        let file = File::create(tmp.path()).await.expect("create");
        let file_handle: FileHandle = (file, false).into();
        let extracted_file: File = file_handle.try_into()?;
        assert!(extracted_file.metadata().await.is_ok());
        Ok(())
    }

    #[test]
    fn test_file_mode_flags_values() {
        assert_eq!(FileModeFlags::READ_ONLY.bits(), 1);
        assert_eq!(FileModeFlags::READ_WRITE.bits(), 2);
        assert_eq!(FileModeFlags::SYNCHRONIZE_ALL.bits(), 4);
        assert_eq!(FileModeFlags::SYNCHRONIZE_DATA.bits(), 8);
        assert_eq!(FileModeFlags::TEMPORARY.bits(), 16);
    }

    #[test]
    fn test_file_mode_flags_empty() {
        let flags = FileModeFlags::empty();
        assert!(!flags.contains(FileModeFlags::READ_ONLY));
        assert!(!flags.contains(FileModeFlags::READ_WRITE));
    }

    #[test]
    fn test_file_mode_flags_combine() {
        let flags = FileModeFlags::READ_WRITE | FileModeFlags::SYNCHRONIZE_ALL;
        assert!(flags.contains(FileModeFlags::READ_WRITE));
        assert!(flags.contains(FileModeFlags::SYNCHRONIZE_ALL));
        assert!(!flags.contains(FileModeFlags::READ_ONLY));
    }

    #[test]
    fn test_file_mode_flags_clone() {
        let flags = FileModeFlags::READ_ONLY;
        let cloned = flags;
        assert_eq!(flags, cloned);
    }

    #[test]
    fn test_file_mode_flags_debug() {
        let flags = FileModeFlags::READ_ONLY;
        let debug = format!("{flags:?}");
        assert!(debug.contains("READ_ONLY"));
    }
}
