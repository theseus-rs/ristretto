use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/jar/JarFile";

/// Register all native methods for `java.util.jar.JarFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getMetaInfEntryNames",
        "()[Ljava/lang/String;",
        get_meta_inf_entry_names,
    );
}

#[async_recursion(?Send)]
async fn get_meta_inf_entry_names(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.jar.JarFile.getMetaInfEntryNames()[Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.jar.JarFile.getMetaInfEntryNames()[Ljava/lang/String;"
    )]
    async fn test_get_meta_inf_entry_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_meta_inf_entry_names(thread, Parameters::default()).await;
    }
}
