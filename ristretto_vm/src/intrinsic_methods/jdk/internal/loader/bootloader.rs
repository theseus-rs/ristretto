use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::{JavaObject, Result};
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{ClassLoader, Value};
use ristretto_macros::intrinsic_method;
use std::collections::HashSet;
use std::sync::Arc;

/// Get the boot class loader for the current thread.
async fn boot_class_loader(thread: &Arc<Thread>) -> Result<Arc<ClassLoader>> {
    let vm = thread.vm()?;
    let class_loader = vm.class_loader().read().await.clone();
    let mut current_class_loader = class_loader;
    while let Some(parent) = current_class_loader.parent().await {
        current_class_loader = parent.clone();
    }
    Ok(current_class_loader)
}

#[intrinsic_method(
    "jdk/internal/loader/BootLoader.getSystemPackageLocation(Ljava/lang/String;)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_system_package_location(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let package_name = parameters.pop()?.as_string()?;
    let boot_class_loader = boot_class_loader(&thread).await?;
    let package_path = package_name.replace('.', "/");

    for class_path_entry in boot_class_loader.class_path().iter() {
        let class_names = class_path_entry.class_names().await?;
        for class_name in class_names {
            if class_name.starts_with(&package_path) && class_name.contains('/') {
                let class_package = class_name.rsplit_once('/').map_or("", |x| x.0);
                if class_package == package_path {
                    let class_path_entry_name =
                        class_path_entry.name().to_string_lossy().to_string();
                    let location = class_path_entry_name.to_object(&thread).await?;
                    return Ok(Some(location));
                }
            }
        }
    }

    // Package not found in system class loader
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "jdk/internal/loader/BootLoader.getSystemPackageNames()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_system_package_names(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let boot_class_loader = boot_class_loader(&thread).await?;
    let class_path = boot_class_loader.class_path();
    let mut package_names = HashSet::new();

    for class_path_entry in class_path.iter() {
        let class_names = class_path_entry.class_names().await?;
        for class_name in class_names {
            if let Some(last_slash_index) = class_name.rfind('/') {
                let package_path = &class_name[..last_slash_index];
                let package_name = package_path.replace('/', ".");
                package_names.insert(package_name);
            }
        }
    }

    let mut package_names: Vec<String> = package_names.into_iter().collect();
    package_names.sort();

    let mut string_objects = Vec::with_capacity(package_names.len());
    for package_name in package_names {
        let string_object = package_name.to_object(&thread).await?;
        string_objects.push(string_object);
    }

    let string_class = thread.class("java.lang.String").await?;
    let package_names = Value::try_from((string_class, string_objects))?;
    Ok(Some(package_names))
}

#[intrinsic_method(
    "jdk/internal/loader/BootLoader.setBootLoaderUnnamedModule0(Ljava/lang/Module;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_boot_loader_unnamed_module_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let module = parameters.pop()?;
    let vm = thread.vm()?;
    vm.module_system().set_boot_unnamed_module(module);
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_system_package_location() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let package_name = "java.lang".to_object(&thread).await?;
        let parameters = Parameters::new(vec![package_name]);
        let result = get_system_package_location(thread, parameters).await?;
        let location = result.expect("location").as_string()?;
        assert!(location.ends_with("java.base.jmod") || location.ends_with("modules"));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_system_package_names() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_system_package_names(thread, Parameters::default()).await?;
        let package_name_objects: Vec<Value> = result.expect("package names").try_into()?;
        let mut package_names = Vec::new();

        for package_name_object in package_name_objects {
            let package_name = package_name_object.as_string()?;
            package_names.push(package_name);
        }

        assert!(package_names.contains(&"java.lang".to_string()));
        assert!(package_names.len() > 250);
        Ok(())
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
