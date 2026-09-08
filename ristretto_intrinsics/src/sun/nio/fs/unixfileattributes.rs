use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::{Parameters, Result, Thread};
use std::sync::Arc;

/// WASI metadata has no portable device/inode identity. Returning no file key lets
/// Java use path-based caching instead of treating every file as device 0/inode 0.
#[intrinsic_method("sun/nio/fs/UnixFileAttributes.fileKey()Lsun/nio/fs/UnixFileKey;", Any)]
#[async_method]
pub async fn file_key<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}
