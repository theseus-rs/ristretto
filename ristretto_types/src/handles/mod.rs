//! This module defines handle types for operating system resources.

mod file;
mod manager;
mod member;
mod socket;
mod thread;

pub use file::{FileHandle, FileModeFlags};
pub use manager::HandleManager;
pub use member::MemberHandle;
#[cfg(not(target_family = "wasm"))]
pub use socket::{SocketHandle, SocketType};
pub use thread::ThreadHandle;
