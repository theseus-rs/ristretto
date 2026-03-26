use ahash::AHashSet;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{JavaObject, Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/Package.getSystemPackage0(Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_system_package_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let package_name = parameters.pop()?.as_string()?;
    let vm = thread.vm()?;
    let class_loader = vm.class_loader().read().await.clone();

    // Walk to the boot class loader
    let mut current = class_loader;
    while let Some(parent) = current.parent().await {
        current = parent.clone();
    }
    let boot_class_loader = current;

    // The package name is in internal format (e.g. "java/lang")
    let package_path = &package_name;

    for class_path_entry in boot_class_loader.class_path().iter() {
        let class_names = class_path_entry.class_names().await?;
        for class_name in class_names {
            if !class_name.starts_with(package_path.as_str()) && class_name.contains('/') {
                continue;
            }

            let class_package = class_name.rsplit_once('/').map_or("", |x| x.0);
            if class_package != package_path.as_str() {
                continue;
            }

            let class_path_entry_name = class_path_entry.name().to_string_lossy().to_string();
            let location = class_path_entry_name.to_object(&thread).await?;
            return Ok(Some(location));
        }
    }

    // Package not found
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "java/lang/Package.getSystemPackages0()[Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_system_packages_0<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let class_loader = vm.class_loader().read().await.clone();

    // Walk to the boot class loader
    let mut current = class_loader;
    while let Some(parent) = current.parent().await {
        current = parent.clone();
    }
    let boot_class_loader = current;

    let mut package_names = AHashSet::default();

    for class_path_entry in boot_class_loader.class_path().iter() {
        let class_names = class_path_entry.class_names().await?;
        for class_name in class_names {
            let Some(last_slash_index) = class_name.rfind('/') else {
                continue;
            };

            let package_path = &class_name[..last_slash_index];
            package_names.insert(package_path.to_string());
        }
    }

    let mut package_names: Vec<String> = package_names.into_iter().collect();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn test_get_system_package_0() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let package_name = "java/lang".to_object(&thread).await?;
        let parameters = Parameters::new(vec![package_name]);
        let result = get_system_package_0(thread, parameters).await?;
        let location = result.expect("location").as_string()?;
        assert!(
            Path::new(&location)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("jar"))
                || location.ends_with("rt.jar")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_system_package_0_not_found() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let package_name = "com/nonexistent/package".to_object(&thread).await?;
        let parameters = Parameters::new(vec![package_name]);
        let result = get_system_package_0(thread, parameters).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_system_packages_0() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = get_system_packages_0(thread, Parameters::default()).await?;
        let package_name_objects: Vec<Value> = result.expect("package names").try_into()?;
        let mut package_names = Vec::new();

        for package_name_object in package_name_objects {
            let package_name = package_name_object.as_string()?;
            package_names.push(package_name);
        }

        assert!(package_names.contains(&"java/lang".to_string()));
        assert!(!package_names.is_empty());
        Ok(())
    }
}
