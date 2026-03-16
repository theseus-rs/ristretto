pub mod ntsystem;
#[cfg(not(target_os = "windows"))]
pub mod unixsystem;
