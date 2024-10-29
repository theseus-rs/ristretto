use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use bitflags::bitflags;
use ristretto_classloader::{ConcurrentVec, Reference, Value};
use std::sync::Arc;

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct SupportsFlags: i32 {
        /// Supports openat and other *at calls.
        const OPENAT = 1 << 1;  // syscalls
        /// Supports futimes or futimesat
        const FUTIMES = 1 << 2;
        /// Supports futimens
        const FUTIMENS = 1 << 3;
        /// Supports lutimes
        const LUTIMES = 1 << 4;
        /// Supports extended attributes
        const XATTR = 1 << 5;
        /// Supports file birth (creation) time attribute
        const BIRTHTIME = 1 << 16; // other features
    }
}

/// Register all native methods for sun.nio.fs.UnixNativeDispatcher.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/fs/UnixNativeDispatcher";
    registry.register(class_name, "getcwd", "()[B", get_cwd);
    registry.register(class_name, "init", "()I", init);
}

#[expect(clippy::cast_possible_wrap)]
#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_cwd(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let current_dir_path =
        std::env::current_dir().map_err(|error| InternalError(format!("getcwd: {error}")))?;
    let current_dir_str = current_dir_path.to_string_lossy();
    let current_dir = current_dir_str
        .as_bytes()
        .to_vec()
        .iter()
        .map(|&b| b as i8)
        .collect();
    let current_dir_vec = ConcurrentVec::from(current_dir);
    let current_dir_bytes = Reference::ByteArray(current_dir_vec);
    Ok(Some(Value::Object(Some(current_dir_bytes))))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let capabilities = SupportsFlags::empty();
    // TODO: Implement the actual capabilities check
    let capabilities = capabilities.bits();
    Ok(Some(Value::Int(capabilities)))
}
