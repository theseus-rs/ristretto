//! This module defines the `Handle` enum, which represents various types of handles to operating
//! system resources.  Handles can represent files, sockets, or thread that the system manages. The
//! module also provides a `HandleManager` struct that manages a collection of these handles,
//! allowing for insertion, retrieval, and removal of handles by their keys.

mod file;
mod manager;
mod thread;

pub(crate) use file::{FileHandle, FileModeFlags};
pub(crate) use manager::HandleManager;
pub(crate) use thread::ThreadHandle;
