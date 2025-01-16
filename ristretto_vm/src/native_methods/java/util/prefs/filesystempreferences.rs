use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/prefs/FileSystemPreferences";

/// Register all native methods for `java.util.prefs.FileSystemPreferences`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "chmod", "(Ljava/lang/String;I)I", chmod);
    registry.register(
        CLASS_NAME,
        "lockFile0",
        "(Ljava/lang/String;IZ)[I",
        lock_file_0,
    );
    registry.register(CLASS_NAME, "unlockFile0", "(I)I", unlock_file_0);
}

#[async_recursion(?Send)]
async fn chmod(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.prefs.FileSystemPreferences.chmod(Ljava/lang/String;I)I")
}

#[async_recursion(?Send)]
async fn lock_file_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.prefs.FileSystemPreferences.lockFile0(Ljava/lang/String;IZ)[I")
}

#[async_recursion(?Send)]
async fn unlock_file_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.util.prefs.FileSystemPreferences.unlockFile0(I)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.FileSystemPreferences.chmod(Ljava/lang/String;I)I"
    )]
    async fn test_chmod() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = chmod(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.FileSystemPreferences.lockFile0(Ljava/lang/String;IZ)[I"
    )]
    async fn test_lock_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = lock_file_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.FileSystemPreferences.unlockFile0(I)I"
    )]
    async fn test_unlock_file_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unlock_file_0(thread, Arguments::default()).await;
    }
}
