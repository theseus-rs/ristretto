use bitflags::bitflags;

bitflags! {
    /// File Mode Flags
    ///
    /// This enum represents the various modes in which a `File` can be opened.
    ///
    /// # References
    ///
    /// - [RandomAccessFile (modes)](https://docs.oracle.com/en/java/javase/24/docs/api/java.base/java/io/RandomAccessFile.html#%3Cinit%3E(java.io.File,java.lang.String))
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) struct FileModeFlags: u16 {
        const READ_ONLY = 1;
        const READ_WRITE = 2;
        const SYNCHRONIZE_ALL = 4;
        const SYNCHRONIZE_DATA = 8;
        const TEMPORARY = 16;
    }
}

/// Represents a handle to an operating system resource, such as a file or socket.
#[derive(Debug)]
pub(crate) enum Handle {
    #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
    File {
        file: String,
        append: bool,
        mode: FileModeFlags,
    },
    #[cfg(target_os = "wasi")]
    File {
        file: std::fs::File,
        append: bool,
        mode: FileModeFlags,
    },
    #[cfg(not(target_family = "wasm"))]
    File {
        file: tokio::fs::File,
        append: bool,
        mode: FileModeFlags,
    },
}

#[cfg(target_os = "wasi")]
impl From<(std::fs::File, FileModeFlags)> for Handle {
    fn from((file, mode): (std::fs::File, FileModeFlags)) -> Self {
        Handle::File {
            file,
            append: false,
            mode,
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<(tokio::fs::File, FileModeFlags)> for Handle {
    fn from((file, mode): (tokio::fs::File, FileModeFlags)) -> Self {
        Handle::File {
            file,
            append: false,
            mode,
        }
    }
}

#[cfg(target_os = "wasi")]
impl From<(std::fs::File, bool)> for Handle {
    fn from((file, append): (std::fs::File, bool)) -> Self {
        Handle::File {
            file,
            append,
            mode: FileModeFlags::empty(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<(tokio::fs::File, bool)> for Handle {
    fn from((file, append): (tokio::fs::File, bool)) -> Self {
        Handle::File {
            file,
            append,
            mode: FileModeFlags::empty(),
        }
    }
}

#[cfg(target_os = "wasi")]
impl TryInto<std::fs::File> for Handle {
    type Error = crate::Error;

    fn try_into(self) -> Result<std::fs::File, Self::Error> {
        match self {
            Handle::File { file, .. } => Ok(file),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryInto<tokio::fs::File> for Handle {
    type Error = crate::Error;

    fn try_into(self) -> Result<tokio::fs::File, Self::Error> {
        match self {
            Handle::File { file, .. } => Ok(file),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use tokio::fs::{File, remove_file};

    #[tokio::test]
    async fn test_handle_from_file_and_append() -> Result<()> {
        let file_name = "test_handle_from_file_and_append.txt";
        let file = File::create(file_name).await?;
        let handle: Handle = (file, false).into();
        assert!(matches!(
            handle,
            Handle::File {
                file: _,
                append: false,
                mode,
            } if mode == FileModeFlags::empty()
        ));
        remove_file(file_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_from_file_and_mode() -> Result<()> {
        let file_name = "test_handle_from_file_and_mode.txt";
        let file = File::create(file_name).await?;
        let expected_mode = FileModeFlags::READ_WRITE;
        let handle: Handle = (file, expected_mode).into();
        assert!(matches!(
            handle,
            Handle::File {
                file: _,
                append: false,
                mode,
            } if mode == expected_mode
        ));
        remove_file(file_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_try_into_file() -> Result<()> {
        let file_name = "test_handle_try_into_file.txt";
        let file = File::create(file_name).await?;
        let handle: Handle = (file, false).into();
        let extracted_file: File = handle.try_into()?;
        assert!(extracted_file.metadata().await.is_ok());
        remove_file(file_name).await?;
        Ok(())
    }
}
