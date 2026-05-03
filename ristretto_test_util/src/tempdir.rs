//! Temporary-directory helpers for tests.
//!
//! Centralizes the wasi `tempfile::env::override_temp_dir("/tmp")` setup that
//! used to be duplicated as `Once`-guarded blocks across multiple crates, and
//! exposes thin wrappers that ensure the override is installed before the
//! `tempfile` API is used.

/// Initialize the temporary directory override required for tests on
/// `target_os = "wasi"`. On all other targets this is a no-op.
///
/// `tempfile::env::override_temp_dir` is invoked under a `Once` guard so
/// repeated calls from any number of test entry points are safe and
/// idempotent.
#[inline]
pub fn init_wasi_tempdir() {
    #[cfg(target_os = "wasi")]
    {
        use std::sync::Once;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            let _ = tempfile::env::override_temp_dir(std::path::Path::new("/tmp"));
        });
    }
}

/// Create a new `NamedTempFile`, ensuring the wasi tempdir override is in
/// place first.
///
/// # Errors
///
/// Propagates any error returned by [`tempfile::NamedTempFile::new`].
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub fn new_named_tempfile() -> std::io::Result<tempfile::NamedTempFile> {
    init_wasi_tempdir();
    tempfile::NamedTempFile::new()
}

/// Create a new `TempDir`, ensuring the wasi tempdir override is in place
/// first.
///
/// # Errors
///
/// Propagates any error returned by [`tempfile::TempDir::new`].
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub fn new_tempdir() -> std::io::Result<tempfile::TempDir> {
    init_wasi_tempdir();
    tempfile::TempDir::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_wasi_tempdir_is_idempotent() {
        // Safe to call any number of times from any thread.
        init_wasi_tempdir();
        init_wasi_tempdir();
        init_wasi_tempdir();
    }

    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    #[test]
    fn new_named_tempfile_succeeds() {
        let file = new_named_tempfile().expect("create NamedTempFile");
        assert!(file.path().exists());
    }

    #[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
    #[test]
    fn new_tempdir_succeeds() {
        let dir = new_tempdir().expect("create TempDir");
        assert!(dir.path().exists());
    }
}
