//! This module defines handle types for operating system resources.

mod file;
mod manager;
mod member;
mod thread;

pub use file::{FileHandle, FileModeFlags};
pub use manager::HandleManager;
pub use member::MemberHandle;
pub use thread::ThreadHandle;
