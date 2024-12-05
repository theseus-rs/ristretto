use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CFRetainedResource`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CFRetainedResource";
    registry.register(class_name, "nativeCFRelease", "(JZ)V", native_cf_release);
}

#[async_recursion(?Send)]
async fn native_cf_release(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CFRetainedResource.nativeCFRelease(JZ)V")
}
