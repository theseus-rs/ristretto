use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.fs.UnixCopyFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/fs/UnixCopyFile";
    registry.register(class_name, "transfer", "(IIJ)V", transfer);
}

#[async_recursion(?Send)]
async fn transfer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.fs.UnixCopyFile.transfer(IIJ)V");
}
