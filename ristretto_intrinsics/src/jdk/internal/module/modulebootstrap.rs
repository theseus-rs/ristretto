use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::module::ModuleDescriptor;
use ristretto_classloader::{Class, Object, Value};
use ristretto_gc::GarbageCollector;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{ModuleAccess, Parameters, Result, VM};
use std::sync::Arc;

/// Creates a Java `ModuleDescriptor$Provides` object from a Rust Provides struct.
async fn create_java_provides<T: ristretto_types::Thread + 'static>(
    thread: &Arc<T>,
    provides_class: &Arc<Class>,
    service: &str,
    implementations: &[String],
    gc: &Arc<GarbageCollector>,
) -> Result<Value> {
    let mut provides_obj = Object::new(provides_class.clone())?;

    // Set service field (uses dot-separated class name)
    let service_name = service.replace('/', ".");
    let service_value = thread.intern_string(&service_name).await?;
    provides_obj.set_value_unchecked("service", service_value)?;

    // Create providers list (List<String>)
    let providers_list = thread
        .object("java/util/ArrayList", "", &[] as &[Value])
        .await?;
    for impl_class in implementations {
        let impl_name = impl_class.replace('/', ".");
        let impl_value = thread.intern_string(&impl_name).await?;
        thread
            .invoke(
                "java/util/ArrayList",
                "add(Ljava/lang/Object;)Z",
                &[providers_list.clone(), impl_value],
            )
            .await?;
    }
    provides_obj.set_value_unchecked("providers", providers_list)?;

    Ok(Value::from_object(gc, provides_obj))
}

/// Creates a Java `ModuleDescriptor` object from a Rust `ModuleDescriptor`.
async fn create_java_module_descriptor<T: ristretto_types::Thread + 'static>(
    thread: &Arc<T>,
    descriptor_class: &Arc<Class>,
    provides_class: &Arc<Class>,
    descriptor: &ModuleDescriptor,
    empty_set: &Value,
    gc: &Arc<GarbageCollector>,
) -> Result<Value> {
    let mut desc_obj = Object::new(descriptor_class.clone())?;

    // Set module name
    let name = thread.intern_string(&descriptor.name).await?;
    desc_obj.set_value_unchecked("name", name)?;

    // Set open flag - mark all modules as open so their packages are accessible
    // to the unnamed module (classpath applications). This enables ServiceLoader
    // to discover providers via Module.isExported() checks.
    desc_obj.set_value_unchecked("open", Value::from(true))?;

    // Set empty collections for fields that ServiceLoader doesn't need
    desc_obj.set_value_unchecked("requires", empty_set.clone())?;
    desc_obj.set_value_unchecked("exports", empty_set.clone())?;
    desc_obj.set_value_unchecked("opens", empty_set.clone())?;
    desc_obj.set_value_unchecked("modifiers", empty_set.clone())?;

    // Set packages
    if descriptor.packages.is_empty() {
        desc_obj.set_value_unchecked("packages", empty_set.clone())?;
    } else {
        let packages_set = thread
            .object("java/util/HashSet", "", &[] as &[Value])
            .await?;
        for package in &descriptor.packages {
            let pkg_name = package.replace('/', ".");
            let pkg_value = thread.intern_string(&pkg_name).await?;
            thread
                .invoke(
                    "java/util/HashSet",
                    "add(Ljava/lang/Object;)Z",
                    &[packages_set.clone(), pkg_value],
                )
                .await?;
        }
        desc_obj.set_value_unchecked("packages", packages_set)?;
    }

    // Set uses
    if descriptor.uses.is_empty() {
        desc_obj.set_value_unchecked("uses", empty_set.clone())?;
    } else {
        let uses_set = thread
            .object("java/util/HashSet", "", &[] as &[Value])
            .await?;
        for service in &descriptor.uses {
            let svc_name = service.replace('/', ".");
            let svc_value = thread.intern_string(&svc_name).await?;
            thread
                .invoke(
                    "java/util/HashSet",
                    "add(Ljava/lang/Object;)Z",
                    &[uses_set.clone(), svc_value],
                )
                .await?;
        }
        desc_obj.set_value_unchecked("uses", uses_set)?;
    }

    // Set provides
    if descriptor.provides.is_empty() {
        desc_obj.set_value_unchecked("provides", empty_set.clone())?;
    } else {
        let provides_set = thread
            .object("java/util/HashSet", "", &[] as &[Value])
            .await?;
        for provides in &descriptor.provides {
            let provides_value = create_java_provides(
                thread,
                provides_class,
                &provides.service,
                &provides.implementations,
                gc,
            )
            .await?;
            thread
                .invoke(
                    "java/util/HashSet",
                    "add(Ljava/lang/Object;)Z",
                    &[provides_set.clone(), provides_value],
                )
                .await?;
        }
        desc_obj.set_value_unchecked("provides", provides_set)?;
    }

    Ok(Value::from_object(gc, desc_obj))
}

/// Boots the module system and returns the boot layer.
///
/// This method initializes the Java Platform Module System (JPMS) and creates the boot layer
/// containing the system modules (java.base, etc.) and any application modules specified
/// on the command line.
///
/// The boot layer is populated with:
/// - A `Module` object for each resolved module (with name, descriptor, and layer)
/// - A `ModuleDescriptor` for each module (with provides, uses, and packages)
/// - A `Configuration` wrapping the resolved module graph
/// - A `ModuleLayer` containing all modules
///
/// This enables `ServiceLoader` to discover service providers through module descriptors'
/// `provides` directives, which is critical for APIs like `javax.sound` that rely on
/// service provider discovery.
#[intrinsic_method(
    "jdk/internal/module/ModuleBootstrap.boot()Ljava/lang/ModuleLayer;",
    Any
)]
#[async_method]
#[expect(clippy::too_many_lines)]
pub async fn boot<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector().clone();

    // Get resolved module configuration from the class loader
    let config_opt = {
        let class_loader_lock = vm.class_loader();
        let class_loader = class_loader_lock.read().await;
        class_loader.module_configuration().await
    };

    // Load Java classes needed for module objects
    let module_class = thread.class("java/lang/Module").await?;
    let descriptor_class = thread.class("java/lang/module/ModuleDescriptor").await?;
    let provides_class = thread
        .class("java/lang/module/ModuleDescriptor$Provides")
        .await?;
    let layer_class = thread.class("java/lang/ModuleLayer").await?;

    // Create shared empty set for descriptor fields
    let empty_set = thread
        .invoke(
            "java/util/Collections",
            "emptySet()Ljava/util/Set;",
            &[] as &[Value],
        )
        .await?
        .unwrap_or(Value::Object(None));

    // Create Configuration (empty shell; the Rust-side ResolvedConfiguration handles resolution)
    let configuration = thread
        .object("java/lang/module/Configuration", "", &[] as &[Value])
        .await?;

    // Create empty parents list for the boot layer
    let empty_parents = thread
        .invoke(
            "java/util/Collections",
            "emptyList()Ljava/util/List;",
            &[] as &[Value],
        )
        .await?
        .unwrap_or(Value::Object(None));

    // Create modules set and name-to-module map for the layer
    let modules_set = thread
        .object("java/util/HashSet", "", &[] as &[Value])
        .await?;
    let name_to_module = thread
        .object("java/util/HashMap", "", &[] as &[Value])
        .await?;

    // Track module values so we can set the layer reference after creating the layer
    let mut module_values = Vec::new();

    if let Some(resolved_config) = config_opt {
        for resolved_module in resolved_config.modules() {
            let descriptor = resolved_module.descriptor();

            // Create Java ModuleDescriptor with provides, uses, and packages
            let java_descriptor = create_java_module_descriptor(
                &thread,
                &descriptor_class,
                &provides_class,
                descriptor,
                &empty_set,
                &gc,
            )
            .await?;

            // Create Java Module object
            let mut module_obj = Object::new(module_class.clone())?;
            let name_value = thread.intern_string(&descriptor.name).await?;
            module_obj.set_value_unchecked("name", name_value.clone())?;
            module_obj.set_value_unchecked("descriptor", java_descriptor)?;
            // loader = null (bootstrap class loader)
            // Grant native access to all system modules to avoid restricted method warnings
            module_obj.set_value_unchecked("enableNativeAccess", Value::from(true))?;

            let module_value = Value::from_object(&gc, module_obj);

            // Add to modules set
            thread
                .invoke(
                    "java/util/HashSet",
                    "add(Ljava/lang/Object;)Z",
                    &[modules_set.clone(), module_value.clone()],
                )
                .await?;

            // Add to name-to-module map
            thread
                .invoke(
                    "java/util/HashMap",
                    "put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    &[name_to_module.clone(), name_value, module_value.clone()],
                )
                .await?;

            // Register module with the VM's module system so that get_module_for_package() can
            // resolve class-to-module associations. This is essential for ServiceLoader to discover
            // providers.
            let mut defined_module =
                ristretto_types::DefinedModule::new(descriptor.name.clone(), true);
            defined_module.packages = descriptor
                .packages
                .iter()
                .map(|p| p.replace('/', "."))
                .collect();
            defined_module.module_object = Some(module_value.clone());
            vm.module_system().define_module(defined_module);

            module_values.push(module_value);
        }
    }

    // Create the boot ModuleLayer by directly setting fields
    // (bypassing the constructor which would create an empty map for boot layers)
    let mut layer_obj = Object::new(layer_class)?;
    layer_obj.set_value_unchecked("cf", configuration)?;
    layer_obj.set_value_unchecked("parents", empty_parents)?;
    layer_obj.set_value_unchecked("nameToModule", name_to_module)?;
    layer_obj.set_value_unchecked("modules", modules_set)?;
    let layer_value = Value::from_object(&gc, layer_obj);

    // Set the layer reference on each Module object
    for module_value in &module_values {
        let mut module_ref = module_value.as_object_mut()?;
        module_ref.set_value_unchecked("layer", layer_value.clone())?;
    }

    // Store the catalog for the boot class loader
    let boot_loader = thread
        .invoke(
            "jdk/internal/loader/ClassLoaders",
            "bootLoader()Ljdk/internal/loader/BuiltinClassLoader;",
            &[] as &[Value],
        )
        .await?
        .unwrap_or(Value::Object(None));

    // Get the existing catalog (or create one) for the boot class loader
    let catalog = thread
        .invoke(
            "jdk/internal/module/ServicesCatalog",
            "getServicesCatalog(Ljava/lang/ClassLoader;)Ljdk/internal/module/ServicesCatalog;",
            &[boot_loader],
        )
        .await?
        .unwrap_or(Value::Object(None));

    for module_value in &module_values {
        thread
            .invoke(
                "jdk/internal/module/ServicesCatalog",
                "register(Ljava/lang/Module;)V",
                &[catalog.clone(), module_value.clone()],
            )
            .await?;
    }

    // Set the servicesCatalog on the boot layer so that ServiceLoader's
    // ModuleServicesLookupIterator can discover providers via the layer.
    {
        let mut layer_ref = layer_value.as_object_mut()?;
        layer_ref.set_value_unchecked("servicesCatalog", catalog)?;
    }

    Ok(Some(layer_value))
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
