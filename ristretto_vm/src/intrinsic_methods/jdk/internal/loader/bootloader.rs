use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/loader/BootLoader.getSystemPackageLocation(Ljava/lang/String;)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_system_package_location(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.loader.BootLoader.getSystemPackageLocation(Ljava/lang/String;)Ljava/lang/String;"
    )
}

#[intrinsic_method(
    "jdk/internal/loader/BootLoader.getSystemPackageNames()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_system_package_names(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.loader.BootLoader.getSystemPackageNames()[Ljava/lang/String;")
}

#[intrinsic_method(
    "jdk/internal/loader/BootLoader.setBootLoaderUnnamedModule0(Ljava/lang/Module;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_boot_loader_unnamed_module_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _object = parameters.pop_reference()?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.loader.BootLoader.getSystemPackageLocation(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_get_system_package_location() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_package_location(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.loader.BootLoader.getSystemPackageNames()[Ljava/lang/String;"
    )]
    async fn test_get_system_package_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_package_names(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_set_boot_loader_unnamed_module_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let parameters = Parameters::new(vec![Value::Object(None)]);
        let result = set_boot_loader_unnamed_module_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
