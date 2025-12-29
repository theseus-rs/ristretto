use crate::Result;
use crate::module_system::{ALL_UNNAMED, DefinedModule};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::collections::HashSet;
use std::sync::Arc;

/// Helper function to get the module name from a Module object.
/// Returns None if the module is null (representing the unnamed module).
fn get_module_name(module_value: &Value) -> Result<Option<String>> {
    if module_value.is_null() {
        return Ok(None);
    }
    let module_ref = module_value.as_object_ref()?;
    let name_value = module_ref.value("name")?;
    if name_value.is_null() {
        Ok(None)
    } else {
        Ok(Some(name_value.as_string()?))
    }
}

/// Adds an export from the source module to the target module.
///
/// This is the native implementation of `Module.addExports0(Module, String, Module)`.
/// It updates the module's exports to allow the target module to access the specified package.
#[intrinsic_method(
    "java/lang/Module.addExports0(Ljava/lang/Module;Ljava/lang/String;Ljava/lang/Module;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn add_exports_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Parameters: Module from, String pn, Module to
    // These are static method parameters in reverse order
    let to_module = parameters.pop()?;
    let package_name = parameters.pop()?;
    let from_module = parameters.pop()?;

    // Get the module names
    let from_name = get_module_name(&from_module)?;
    let to_name = get_module_name(&to_module)?;
    let package = if package_name.is_null() {
        None
    } else {
        Some(package_name.as_string()?)
    };

    // Update the module system if we have valid module and package names
    if let (Some(from), Some(pkg)) = (from_name, package) {
        let vm = thread.vm()?;
        let target = to_name.as_deref();
        vm.module_system().add_export(&from, &pkg, target);
    }

    Ok(None)
}

/// Adds an unqualified export from the source module.
///
/// This is the native implementation of `Module.addExportsToAll0(Module, String)`.
/// It updates the module's exports to allow all modules to access the specified package.
#[intrinsic_method(
    "java/lang/Module.addExportsToAll0(Ljava/lang/Module;Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn add_exports_to_all_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Parameters: Module from, String pn
    let package_name = parameters.pop()?;
    let from_module = parameters.pop()?;

    let from_name = get_module_name(&from_module)?;
    let package = if package_name.is_null() {
        None
    } else {
        Some(package_name.as_string()?)
    };

    // Update the module system to add an unqualified export
    if let (Some(from), Some(pkg)) = (from_name, package) {
        let vm = thread.vm()?;
        vm.module_system().add_export_to_all(&from, &pkg);
    }

    Ok(None)
}

/// Adds an export from the source module to all unnamed modules.
///
/// This is the native implementation of `Module.addExportsToAllUnnamed0(Module, String)`.
/// It updates the module's exports to allow the unnamed module (classpath code) to access
/// the specified package.
#[intrinsic_method(
    "java/lang/Module.addExportsToAllUnnamed0(Ljava/lang/Module;Ljava/lang/String;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn add_exports_to_all_unnamed_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Parameters: Module from, String pn
    let package_name = parameters.pop()?;
    let from_module = parameters.pop()?;

    let from_name = get_module_name(&from_module)?;
    let package = if package_name.is_null() {
        None
    } else {
        Some(package_name.as_string()?)
    };

    // Update the module system to add an export to ALL-UNNAMED
    if let (Some(from), Some(pkg)) = (from_name, package) {
        let vm = thread.vm()?;
        vm.module_system().add_export_to_all_unnamed(&from, &pkg);
    }

    Ok(None)
}

/// Adds a read edge from the source module to the target module.
///
/// This is the native implementation of `Module.addReads0(Module, Module)`.
/// After this call, `from_module` can read `to_module`.
#[intrinsic_method(
    "java/lang/Module.addReads0(Ljava/lang/Module;Ljava/lang/Module;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn add_reads_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Parameters: Module from, Module to
    let to_module = parameters.pop()?;
    let from_module = parameters.pop()?;

    let from_name = get_module_name(&from_module)?;
    let to_name = get_module_name(&to_module)?;

    // Update the module system to add a read edge
    if let Some(from) = from_name {
        let vm = thread.vm()?;
        // Use ALL-UNNAMED if target is the unnamed module
        let target = to_name.as_deref().unwrap_or(ALL_UNNAMED);
        vm.module_system().add_read(&from, target);
    }

    Ok(None)
}

/// Helper to extract packages from a String[] or Object[] array.
fn extract_packages(packages_value: &Value) -> Result<HashSet<String>> {
    let mut packages = HashSet::new();
    if packages_value.is_null() {
        return Ok(packages);
    }

    let (_class, elements) = packages_value.as_class_vec_ref()?;

    for element in elements.iter() {
        if !element.is_null() {
            let pkg_name = element.as_string()?;
            packages.insert(pkg_name);
        }
    }

    Ok(packages)
}

/// Defines a module (Java 11 version with String[] packages).
///
/// This is the native implementation of `Module.defineModule0` for Java 11.
/// It registers the module with the VM and sets up its initial state.
#[intrinsic_method(
    "java/lang/Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/String;)V",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_module_0_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Parameters: Module module, boolean isOpen, String version, String location, String[] packages
    let packages_value = parameters.pop()?; // String[]
    let location_value = parameters.pop()?; // String
    let version_value = parameters.pop()?; // String
    let is_open = parameters.pop_bool()?; // boolean
    let module = parameters.pop()?; // Module

    let module_name = get_module_name(&module)?;

    if let Some(name) = module_name {
        let version = if version_value.is_null() {
            None
        } else {
            Some(version_value.as_string()?)
        };

        let location = if location_value.is_null() {
            None
        } else {
            Some(location_value.as_string()?)
        };

        let packages = extract_packages(&packages_value)?;

        let mut defined_module = DefinedModule::new(name.clone(), is_open);
        defined_module.version = version;
        defined_module.location = location;
        defined_module.packages = packages;

        let vm = thread.vm()?;
        vm.module_system().define_module(defined_module);
    }

    Ok(None)
}

/// Defines a module (Java 12+ version with Object[] packages).
///
/// This is the native implementation of `Module.defineModule0` for Java 12+.
/// The packages array contains String objects (changed from String[] for consistency).
#[intrinsic_method(
    "java/lang/Module.defineModule0(Ljava/lang/Module;ZLjava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_module_0_1(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Parameters: Module module, boolean isOpen, String version, String location, Object[] packages
    let packages_value = parameters.pop()?; // Object[]
    let location_value = parameters.pop()?; // String
    let version_value = parameters.pop()?; // String
    let is_open = parameters.pop_bool()?; // boolean
    let module = parameters.pop()?; // Module

    let module_name = get_module_name(&module)?;

    if let Some(name) = module_name {
        let version = if version_value.is_null() {
            None
        } else {
            Some(version_value.as_string()?)
        };

        let location = if location_value.is_null() {
            None
        } else {
            Some(location_value.as_string()?)
        };

        let packages = extract_packages(&packages_value)?;

        let mut defined_module = DefinedModule::new(name.clone(), is_open);
        defined_module.version = version;
        defined_module.location = location;
        defined_module.packages = packages;

        let vm = thread.vm()?;
        vm.module_system().define_module(defined_module);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_exports_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        // Push parameters in reverse order (from, pn, to)
        parameters.push(Value::Object(None)); // from module (null = unnamed)
        parameters.push(Value::Object(None)); // package name (null)
        parameters.push(Value::Object(None)); // to module (null = unnamed)
        let result = add_exports_0(thread, parameters).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_add_exports_to_all_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        // Push parameters in reverse order (from, pn)
        parameters.push(Value::Object(None)); // from module (null = unnamed)
        parameters.push(Value::Object(None)); // package name (null)
        let result = add_exports_to_all_0(thread, parameters).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_add_exports_to_all_unnamed_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        // Push parameters in reverse order (from, pn)
        parameters.push(Value::Object(None)); // from module (null = unnamed)
        parameters.push(Value::Object(None)); // package name (null)
        let result = add_exports_to_all_unnamed_0(thread, parameters).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_add_reads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        // Push parameters in reverse order (from, to)
        parameters.push(Value::Object(None)); // from module (null = unnamed)
        parameters.push(Value::Object(None)); // to module (null = unnamed)
        let result = add_reads_0(thread, parameters).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_define_module_0_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        // Push parameters in reverse order: module, isOpen, version, location, packages
        parameters.push(Value::Object(None)); // module
        parameters.push(Value::Int(0)); // isOpen = false
        parameters.push(Value::Object(None)); // version
        parameters.push(Value::Object(None)); // location
        parameters.push(Value::Object(None)); // packages
        let result = define_module_0_0(thread, parameters).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_define_module_0_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let mut parameters = Parameters::default();
        // Push parameters in reverse order: module, isOpen, version, location, packages
        parameters.push(Value::Object(None)); // module
        parameters.push(Value::Int(0)); // isOpen = false
        parameters.push(Value::Object(None)); // version
        parameters.push(Value::Object(None)); // location
        parameters.push(Value::Object(None)); // packages
        let result = define_module_0_1(thread, parameters).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_module_name_null() {
        let result = get_module_name(&Value::Object(None));
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
