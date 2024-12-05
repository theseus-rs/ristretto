use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.prefs.FileSystemPreferences`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/prefs/FileSystemPreferences";
    registry.register(class_name, "chmod", "(Ljava/lang/String;I)I", chmod);
    registry.register(
        class_name,
        "lockFile0",
        "(Ljava/lang/String;IZ)[I",
        lock_file_0,
    );
    registry.register(class_name, "unlockFile0", "(I)I", unlock_file_0);
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
