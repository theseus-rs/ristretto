use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/module/ModuleBootstrap.boot()Ljava/lang/ModuleLayer;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn boot(thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    // TODO: correctly populate the ModuleLayer
    let configuration = thread
        .object("java/lang/module/Configuration", "", &[] as &[Value])
        .await?;

    let empty_list = thread
        .object("java/util/ArrayList", "", &[] as &[Value])
        .await?;
    let module_layer = thread
        .object(
            "java/lang/ModuleLayer",
            "Ljava/lang/module/Configuration;Ljava/util/List;Ljava/util/function/Function;",
            &[configuration, empty_list, Value::Object(None)],
        )
        .await?;
    Ok(Some(module_layer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_boot() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = boot(thread, Parameters::default())
            .await?
            .expect("module layer");
        let object = result.as_object_ref()?;
        let class = object.class();
        assert_eq!("java/lang/ModuleLayer", class.name());
        Ok(())
    }
}
