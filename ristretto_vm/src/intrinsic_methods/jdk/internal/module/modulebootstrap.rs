use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/module/ModuleBootstrap";

/// Register all intrinsic methods for `jdk.internal.module.ModuleBootstrap`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "boot", "()Ljava/lang/ModuleLayer;", boot);
}

#[async_recursion(?Send)]
async fn boot(thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    // TODO: correctly populate the ModuleLayer
    let empty_list = thread
        .object("java/util/ArrayList", "", Vec::<Value>::new())
        .await?;
    let module_layer = thread
        .object(
            "java/lang/ModuleLayer",
            "Ljava/lang/module/Configuration;Ljava/util/List;Ljava/util/function/Function;",
            vec![Value::Object(None), empty_list, Value::Object(None)],
        )
        .await?;
    Ok(Some(module_layer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Object;

    #[tokio::test]
    async fn test_boot() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = boot(thread, Parameters::default())
            .await?
            .expect("module layer");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/ModuleLayer", class.name());
        Ok(())
    }
}
