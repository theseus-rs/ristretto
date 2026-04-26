//! Filesystem access with platform-appropriate async backends.
//!
//! On non-wasm targets these helpers delegate to `tokio::fs`. On wasm targets `tokio::fs` is not
//! available, so the helpers fall back to `std::fs` wrapped in async functions. The public API is
//! intentionally narrow so the call sites can `await` the same operations regardless of target.

#![allow(clippy::unused_async)]

use std::io;
use std::path::Path;

#[cfg(not(target_family = "wasm"))]
pub use tokio::fs::{OpenOptions, create_dir, metadata, remove_dir, remove_file, rename};

#[cfg(target_family = "wasm")]
#[cfg_attr(target_os = "unknown", expect(unused_imports))]
pub use wasm_impl::{OpenOptions, create_dir, metadata, remove_dir, remove_file, rename};

#[cfg(target_family = "wasm")]
mod wasm_impl {
    use std::io;
    use std::path::Path;

    pub async fn create_dir(path: impl AsRef<Path>) -> io::Result<()> {
        std::fs::create_dir(path)
    }

    pub async fn metadata(path: impl AsRef<Path>) -> io::Result<std::fs::Metadata> {
        std::fs::metadata(path)
    }

    pub async fn remove_dir(path: impl AsRef<Path>) -> io::Result<()> {
        std::fs::remove_dir(path)
    }

    pub async fn remove_file(path: impl AsRef<Path>) -> io::Result<()> {
        std::fs::remove_file(path)
    }

    pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<()> {
        std::fs::rename(from, to)
    }

    /// Async-friendly wrapper around [`std::fs::OpenOptions`].
    #[derive(Debug)]
    pub struct OpenOptions(std::fs::OpenOptions);

    impl OpenOptions {
        #[must_use]
        pub fn new() -> Self {
            Self(std::fs::OpenOptions::new())
        }

        pub fn write(&mut self, write: bool) -> &mut Self {
            self.0.write(write);
            self
        }

        pub fn create_new(&mut self, create_new: bool) -> &mut Self {
            self.0.create_new(create_new);
            self
        }

        pub async fn open(&self, path: impl AsRef<Path>) -> io::Result<std::fs::File> {
            self.0.open(path)
        }
    }
}

/// Sets file permissions, mirroring `tokio::fs::set_permissions`/`std::fs::set_permissions`.
#[cfg(not(target_family = "wasm"))]
pub async fn set_permissions(path: impl AsRef<Path>, perm: std::fs::Permissions) -> io::Result<()> {
    tokio::fs::set_permissions(path, perm).await
}

#[cfg(target_family = "wasm")]
pub async fn set_permissions(path: impl AsRef<Path>, perm: std::fs::Permissions) -> io::Result<()> {
    std::fs::set_permissions(path, perm)
}

/// Reads the entries of a directory and returns the [`std::ffi::OsString`] file names.
///
/// This abstracts over `tokio::fs::ReadDir` (which exposes `next_entry().await`) and
/// `std::fs::ReadDir` (which is a synchronous iterator) by returning an eagerly collected list of
/// names. Entries that fail to read are silently skipped, matching the behavior expected by the
/// `java.io.File#list` intrinsic.
#[cfg(not(target_family = "wasm"))]
pub async fn read_dir_names(path: impl AsRef<Path>) -> io::Result<Vec<std::ffi::OsString>> {
    let mut directory = tokio::fs::read_dir(path).await?;
    let mut names = Vec::new();
    while let Ok(Some(entry)) = directory.next_entry().await {
        names.push(entry.file_name());
    }
    Ok(names)
}

#[cfg(target_family = "wasm")]
pub async fn read_dir_names(path: impl AsRef<Path>) -> io::Result<Vec<std::ffi::OsString>> {
    let directory = std::fs::read_dir(path)?;
    let names = directory.flatten().map(|entry| entry.file_name()).collect();
    Ok(names)
}
