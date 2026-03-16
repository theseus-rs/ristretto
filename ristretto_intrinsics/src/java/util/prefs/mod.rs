#[cfg(not(target_os = "windows"))]
pub mod filesystempreferences;
#[cfg(target_os = "macos")]
pub mod macosxpreferencesfile;
