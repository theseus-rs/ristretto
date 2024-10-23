use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use ristretto_classloader::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for jdk.internal.loader.BootLoader.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/loader/BootLoader";
    registry.register(
        class_name,
        "setBootLoaderUnnamedModule0",
        "(Ljava/lang/Module;)V",
        set_boot_loader_unnamed_module_0,
    );
}

#[expect(clippy::needless_pass_by_value)]
fn set_boot_loader_unnamed_module_0(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let _object = arguments.pop_object()?;
        Ok(None)
    })
}
