use ahash::AHashSet;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{ClassLoader, Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::ModuleAccess;
use ristretto_types::Parameters;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{JavaObject, Result};
use std::sync::Arc;

/// Get the boot class loader for the current thread.
async fn boot_class_loader<T: Thread + 'static>(thread: &Arc<T>) -> Result<Arc<ClassLoader>> {
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
#[async_method]
pub async fn get_system_package_location<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let package_name = parameters.pop()?.as_string()?;
    let vm = thread.vm()?;

    // Use resolved module configuration to find which module contains this package
    let resolved_config = vm.module_system().resolved_configuration();
    let pkg_internal = package_name.replace('.', "/");
    if let Some(module_name) = resolved_config.find_module_for_package(&pkg_internal) {
        let location = format!("jrt:/{module_name}").to_object(&thread).await?;
        return Ok(Some(location));
    }

    // Package not found
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "jdk/internal/loader/BootLoader.getSystemPackageNames()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_system_package_names<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;

    // Return packages from classes that have actually been loaded by the boot class loader.
    // This matches JDK behavior where BootLoader tracks packages from defineClass calls;
    // packages are only included once a class from that package has been loaded.
    let boot_class_loader = boot_class_loader(&thread).await?;
    let loaded_classes = boot_class_loader.loaded_classes().await;
    let mut package_set = AHashSet::default();

    for class in &loaded_classes {
        let name = class.name();
        // Skip array classes; their names start with '[' and should not be treated as packages
        if name.starts_with('[') {
            continue;
        }
        if let Some(last_slash) = name.rfind('/') {
            let package = &name[..last_slash];
            package_set.insert(package.replace('/', "."));
        }
    }

    let mut package_names: Vec<String> = package_set.into_iter().collect();
    package_names.sort();

    let mut string_objects = Vec::with_capacity(package_names.len());
    for package_name in package_names {
        let string_object = package_name.to_object(&thread).await?;
        string_objects.push(string_object);
    }

    let string_class = thread.class("[Ljava/lang/String;").await?;
    let reference = Reference::try_from((string_class, string_objects))?;
    let package_names = Value::new_object(vm.garbage_collector(), reference);
    Ok(Some(package_names))
}

#[intrinsic_method(
    "jdk/internal/loader/BootLoader.setBootLoaderUnnamedModule0(Ljava/lang/Module;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_boot_loader_unnamed_module_0<T: Thread + 'static>(
    thread: Arc<T>,
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
        assert!(location.starts_with("jrt:/"));
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
        // The count depends on how many classes the boot loader has loaded in this test context
        assert!(!package_names.is_empty());
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
