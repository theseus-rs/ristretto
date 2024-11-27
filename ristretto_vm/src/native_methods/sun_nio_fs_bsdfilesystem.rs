use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.fs.BsdFileSystem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/fs/BsdFileSystem";
    registry.register(class_name, "directCopy0", "(IIJ)I", direct_copy_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn direct_copy_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
