use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.jar.JarFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/jar/JarFile";
    registry.register(
        class_name,
        "getMetaInfEntryNames",
        "()[Ljava/lang/String;",
        get_meta_inf_entry_names,
    );
}

#[async_recursion(?Send)]
async fn get_meta_inf_entry_names(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
