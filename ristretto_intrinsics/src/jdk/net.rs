#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub mod linuxsocketoptions;
#[cfg(target_os = "macos")]
pub mod macosxsocketoptions;
#[cfg(target_family = "windows")]
pub mod windowssocketoptions;
