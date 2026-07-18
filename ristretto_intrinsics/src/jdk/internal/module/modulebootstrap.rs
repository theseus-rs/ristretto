use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::attributes::RequiresFlags;
use ristretto_classloader::module::ModuleDescriptor;
use ristretto_classloader::module::{Exports, Opens, Provides, Requires, ResolvedConfiguration};
use ristretto_classloader::{Class, Object, Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::module_access::ModuleAccess;
use ristretto_types::{DefinedModule, Thread, VM};
use ristretto_types::{Parameters, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

/// Boots the module system and returns the boot layer.
///
/// This method initializes the Java Platform Module System (JPMS) and creates the boot layer
/// containing the system modules (java.base, etc.) and any application modules specified
/// on the command line.
///
/// The boot layer is created by:
/// 1. Reading the resolved module configuration from the VM's module system
/// 2. Creating Java `ModuleDescriptor` objects directly (bypassing Builder API for speed)
/// 3. Creating java.lang.Module objects for each module
/// 4. Building a `ModuleLayer` with all modules, Configuration, and parent layers
/// 5. Registering all modules and their read/export/open edges with the VM
#[intrinsic_method(
    "jdk/internal/module/ModuleBootstrap.boot()Ljava/lang/ModuleLayer;",
    Any
)]
#[async_method]
pub async fn boot<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let module_system = vm.module_system();
    let resolved_config = module_system.resolved_configuration_arc();

    if resolved_config.is_empty() {
        debug!("No resolved modules; creating empty boot layer");
        return create_empty_layer(&thread).await;
    }

    debug!("Creating boot layer with {} modules", resolved_config.len());

    let layer = create_populated_layer(&thread, &resolved_config).await?;
    Ok(Some(layer))
}

/// Creates a fully populated boot layer with all resolved modules.
///
/// Uses direct object construction (`Object::new` + `set_value_unchecked`) instead of the
/// Java Builder API to avoid bootstrap circularity and improve performance.
async fn create_populated_layer<T: Thread + 'static>(
    thread: &Arc<T>,
    resolved_config: &ResolvedConfiguration,
) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    // Pre-load classes needed for direct construction
    let descriptor_class = thread.class("java/lang/module/ModuleDescriptor").await?;
    let requires_class = thread
        .class("java/lang/module/ModuleDescriptor$Requires")
        .await?;
    let exports_class = thread
        .class("java/lang/module/ModuleDescriptor$Exports")
        .await?;
    let opens_class = thread
        .class("java/lang/module/ModuleDescriptor$Opens")
        .await?;
    let provides_class = thread
        .class("java/lang/module/ModuleDescriptor$Provides")
        .await?;

    // Create shared immutable empty set via Collections.emptySet() for reuse.
    // Using an immutable set (instead of a mutable HashSet) ensures that if any Java
    // code attempts to mutate it, it will get an UnsupportedOperationException rather
    // than silently corrupting all ModuleDescriptor objects that share the reference.
    let empty_set = thread
        .invoke(
            "java.util.Collections",
            "emptySet()Ljava/util/Set;",
            &[] as &[Value],
        )
        .await?
        .unwrap_or(Value::Object(None));

    // Create Java ModuleDescriptor objects via direct field construction
    let mut java_descriptors: HashMap<String, Value> = HashMap::new();
    let mut version_cache: HashMap<String, Value> = HashMap::new();
    for rm in resolved_config.modules() {
        let desc = rm.descriptor();
        let java_desc = create_module_descriptor_direct(
            thread,
            desc,
            &descriptor_class,
            &requires_class,
            &exports_class,
            &opens_class,
            &provides_class,
            &empty_set,
            &mut version_cache,
        )
        .await?;
        java_descriptors.insert(desc.name.clone(), java_desc);
    }

    debug!("Created {} module descriptors", java_descriptors.len());

    // Create the ModuleLayer shell (fields set later to break circular ref)
    let layer_class = thread.class("java/lang/ModuleLayer").await?;
    let layer_object = Object::new(layer_class.clone())?;
    let layer_value = Value::from_object(gc, layer_object);

    // Create Module objects and register with the VM
    let module_class = thread.class("java/lang/Module").await?;
    let (name_to_module, module_map) = create_module_objects(
        thread,
        resolved_config,
        &module_class,
        &java_descriptors,
        &layer_value,
    )
    .await?;
    debug!("Created {} module objects", module_map.len());

    // Populate reads, exportedPackages, and openPackages on Module objects
    populate_module_edges(thread, resolved_config, &module_class, &module_map).await?;
    debug!("Populated module edges");

    // Create Configuration and set ModuleLayer fields
    let config_value =
        create_configuration_object(thread, resolved_config, &java_descriptors).await?;
    debug!("Created module configuration");
    set_layer_fields(
        thread,
        &layer_value,
        &config_value,
        &name_to_module,
        &module_map,
    )
    .await?;

    // Register ServicesCatalog for ServiceLoader support
    register_services_catalog_on_layer(thread, resolved_config, &module_map, &layer_value).await;

    // Update primitive class modules to point to java.base
    update_primitive_class_modules(thread, &module_map).await;
    update_loaded_class_modules(thread, &module_map).await;

    debug!("Boot layer created with {} modules", resolved_config.len());

    Ok(layer_value)
}

/// Creates `java.lang.Module` objects for each resolved module and registers them with the VM.
///
/// Returns the `nameToModule` `HashMap` (for the `ModuleLayer`) and a Rust-side map of
/// module name -> Module Value for use in subsequent phases.
async fn create_module_objects<T: Thread + 'static>(
    thread: &Arc<T>,
    resolved_config: &ResolvedConfiguration,
    module_class: &Arc<Class>,
    java_descriptors: &HashMap<String, Value>,
    layer_value: &Value,
) -> Result<(Value, HashMap<String, Value>)> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let mut module_map: HashMap<String, Value> = HashMap::new();
    let mut name_entries = Vec::with_capacity(resolved_config.len());

    for rm in resolved_config.modules() {
        let desc = rm.descriptor();
        let mut module_object = Object::new(module_class.clone())?;

        let name_str = thread.intern_string(&desc.name).await?;
        module_object.set_value_unchecked("name", name_str.clone())?;
        module_object.set_value_unchecked("layer", layer_value.clone())?;
        // Boot-loaded modules have a null class loader (bootstrap loader).
        module_object.set_value_unchecked("loader", Value::Object(None))?;
        if let Some(java_desc) = java_descriptors.get(&desc.name) {
            module_object.set_value_unchecked("descriptor", java_desc.clone())?;
        }

        let module_value = Value::from_object(gc, module_object);

        name_entries.push((string_hash_code(&desc.name), name_str, module_value.clone()));

        // Register with the VM module system
        let mut defined = DefinedModule::new(desc.name.clone(), desc.is_open());
        defined.version.clone_from(&desc.version);
        // Normalize packages to dotted format (java.lang) at storage time so all
        // lookups use a single canonical format and avoid redundant conversions.
        defined.packages = desc.packages.iter().map(|p| p.replace('/', ".")).collect();
        defined.module_object = Some(module_value.clone());
        vm.module_system().define_module(defined);

        module_map.insert(desc.name.clone(), module_value);
    }

    let name_to_module = create_hashed_map(thread, name_entries).await?;
    Ok((name_to_module, module_map))
}

/// Populates `reads`, `exportedPackages`, and `openPackages` fields on Module objects.
///
/// These fields are checked by `Module.isExported()` and `Module.isOpen()` in Java code.
async fn populate_module_edges<T: Thread + 'static>(
    thread: &Arc<T>,
    resolved_config: &ResolvedConfiguration,
    module_class: &Arc<Class>,
    module_map: &HashMap<String, Value>,
) -> Result<()> {
    // Populate reads sets
    for rm in resolved_config.modules() {
        if let Some(module_value) = module_map.get(rm.name()) {
            let reads = rm
                .reads()
                .iter()
                .filter_map(|read_name| module_map.get(read_name))
                .collect::<Vec<_>>();
            let reads_set = create_java_set_from_values(thread, reads.into_iter()).await?;
            let mut module_ref = module_value.as_object_mut()?;
            module_ref.set_value_unchecked("reads", reads_set)?;
        }
    }
    debug!("Populated module reads");

    // Read Module.EVERYONE_MODULE sentinel. Filter out null object values so that
    // unqualified exports/opens are not silently broken if the field is uninitialised.
    let everyone_module = module_class
        .static_value("EVERYONE_MODULE")
        .ok()
        .filter(|v| !matches!(v, Value::Object(None)));
    if everyone_module.is_none() {
        warn!(
            "Module.EVERYONE_MODULE is null; unqualified exports/opens in Java-side Module API may return incorrect results"
        );
    }

    // Populate exportedPackages and openPackages maps
    for rm in resolved_config.modules() {
        if let Some(module_value) = module_map.get(rm.name()) {
            let desc = rm.descriptor();
            populate_exported_packages(
                thread,
                module_value,
                desc,
                module_map,
                everyone_module.as_ref(),
            )
            .await?;
            populate_open_packages(
                thread,
                module_value,
                desc,
                module_map,
                everyone_module.as_ref(),
            )
            .await?;
        }
    }

    Ok(())
}

/// Populates the `exportedPackages` field on a single Module object.
///
/// Always creates and sets the map even when there are no exports so that
/// `Module.isExported()` never encounters a null field.
async fn populate_exported_packages<T: Thread + 'static>(
    thread: &Arc<T>,
    module_value: &Value,
    desc: &ModuleDescriptor,
    module_map: &HashMap<String, Value>,
    everyone_module: Option<&Value>,
) -> Result<()> {
    let mut entries = Vec::with_capacity(desc.exports.len());
    for export in &desc.exports {
        let package_name = to_dotted(&export.package);
        let package = thread.intern_string(&package_name).await?;
        let target_set =
            build_target_set(thread, export.targets.as_ref(), module_map, everyone_module).await?;
        entries.push((string_hash_code(&package_name), package, target_set));
    }
    let exports_map = create_hashed_map(thread, entries).await?;
    let mut module_ref = module_value.as_object_mut()?;
    module_ref.set_value_unchecked("exportedPackages", exports_map)?;
    Ok(())
}

/// Populates the `openPackages` field on a single Module object.
///
/// Always creates and sets the map even when there are no opens so that
/// `Module.isOpen()` never encounters a null field.
async fn populate_open_packages<T: Thread + 'static>(
    thread: &Arc<T>,
    module_value: &Value,
    desc: &ModuleDescriptor,
    module_map: &HashMap<String, Value>,
    everyone_module: Option<&Value>,
) -> Result<()> {
    let mut entries = Vec::with_capacity(desc.opens.len());
    for open in &desc.opens {
        let package_name = to_dotted(&open.package);
        let package = thread.intern_string(&package_name).await?;
        let target_set =
            build_target_set(thread, open.targets.as_ref(), module_map, everyone_module).await?;
        entries.push((string_hash_code(&package_name), package, target_set));
    }
    let opens_map = create_hashed_map(thread, entries).await?;
    let mut module_ref = module_value.as_object_mut()?;
    module_ref.set_value_unchecked("openPackages", opens_map)?;
    Ok(())
}

/// Builds a `HashSet<Module>` of target modules for an export or opens directive.
///
/// For unqualified directives (`targets` is `None`), adds `EVERYONE_MODULE` to indicate
/// the package is exported/opened to all modules per JVM spec §5.4.4.
/// For qualified directives, adds each named target module found in the module map.
async fn build_target_set<T: Thread + 'static>(
    thread: &Arc<T>,
    targets: Option<&Vec<String>>,
    module_map: &HashMap<String, Value>,
    everyone_module: Option<&Value>,
) -> Result<Value> {
    let values = match targets {
        None => {
            // Unqualified: add EVERYONE_MODULE sentinel.
            everyone_module.into_iter().collect::<Vec<_>>()
        }
        Some(targets) => targets
            .iter()
            .filter_map(|target| module_map.get(target))
            .collect(),
    };
    create_java_set_from_values(thread, values.into_iter()).await
}

/// Creates the `java.lang.module.Configuration` Java object with `ResolvedModule` entries.
async fn create_configuration_object<T: Thread + 'static>(
    thread: &Arc<T>,
    resolved_config: &ResolvedConfiguration,
    java_descriptors: &HashMap<String, Value>,
) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let config_class = thread.class("java/lang/module/Configuration").await?;
    let config_object = Object::new(config_class)?;
    let config_value = Value::from_object(gc, config_object);

    // Build ResolvedModule objects and the nameToModule map
    let (config_name_to_module, resolved_module_map) =
        create_resolved_modules(thread, resolved_config, java_descriptors, &config_value).await?;

    // Boot layer configuration's parent is Configuration.empty().
    let empty_config = thread
        .invoke(
            "java.lang.module.Configuration",
            "empty()Ljava/lang/module/Configuration;",
            &[] as &[Value],
        )
        .await?;
    let config_parents = create_array_list(thread, empty_config.into_iter().collect()).await?;

    // Create modules set and graph for Configuration
    let config_modules_set =
        create_java_set_from_values(thread, resolved_module_map.values()).await?;
    let config_graph =
        create_configuration_graph(thread, resolved_config, &resolved_module_map).await?;

    {
        let mut config_ref = config_value.as_object_mut()?;
        config_ref.set_value_unchecked("parents", config_parents)?;
        config_ref.set_value_unchecked("nameToModule", config_name_to_module)?;
        config_ref.set_value_unchecked("modules", config_modules_set)?;
        config_ref.set_value_unchecked("graph", config_graph)?;
    }

    Ok(config_value)
}

/// Creates `ResolvedModule` Java objects and the `nameToModule` map for `Configuration`.
async fn create_resolved_modules<T: Thread + 'static>(
    thread: &Arc<T>,
    resolved_config: &ResolvedConfiguration,
    java_descriptors: &HashMap<String, Value>,
    config_value: &Value,
) -> Result<(Value, HashMap<String, Value>)> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let mref_class = match thread
        .class("jdk/internal/module/ModuleReferenceImpl")
        .await
    {
        Ok(class) => Some(class),
        Err(_) => thread.class("java/lang/module/ModuleReference").await.ok(),
    };
    let resolved_module_class = thread.class("java/lang/module/ResolvedModule").await.ok();

    let mut resolved_module_map: HashMap<String, Value> = HashMap::new();
    let mut name_entries = Vec::with_capacity(resolved_config.len());

    if let (Some(mref_cls), Some(rm_cls)) = (mref_class.as_ref(), resolved_module_class.as_ref()) {
        for rm in resolved_config.modules() {
            let desc = rm.descriptor();

            // Create ModuleReference directly. ModuleReference is abstract in OpenJDK,
            // but Object::new does not enforce abstract-class checks. This is intentional:
            // we only need the `descriptor` field (accessed via the final `descriptor()`
            // method), and the abstract `open()` method is never called during bootstrap.
            let mut mref_object = Object::new(mref_cls.clone())?;
            if let Some(java_desc) = java_descriptors.get(&desc.name) {
                mref_object.set_value_unchecked("descriptor", java_desc.clone())?;
            }
            // JDK 17+ caches a descriptor-derived hash in ModuleReferenceImpl. Populate it
            // directly so Configuration graph construction does not interpret hashCode for
            // every reference. JDK 11 uses a different modifier hash and stays on the Java path.
            if vm.java_major_version() >= 17
                && mref_cls.name() == "jdk/internal/module/ModuleReferenceImpl"
            {
                let hash = module_descriptor_hash_code(desc)
                    .wrapping_mul(43)
                    .wrapping_mul(43);
                let hash = match hash {
                    0 => -1,
                    hash => hash,
                };
                mref_object.set_value_unchecked("hash", Value::Int(hash))?;
            }
            let mref_value = Value::from_object(gc, mref_object);

            let mut rm_object = Object::new(rm_cls.clone())?;
            rm_object.set_value_unchecked("cf", config_value.clone())?;
            rm_object.set_value_unchecked("mref", mref_value)?;
            let rm_value = Value::from_object(gc, rm_object);

            resolved_module_map.insert(desc.name.clone(), rm_value.clone());

            let name_str = thread.intern_string(&desc.name).await?;
            name_entries.push((string_hash_code(&desc.name), name_str, rm_value));
        }
    }

    let config_name_to_module = create_hashed_map(thread, name_entries).await?;
    Ok((config_name_to_module, resolved_module_map))
}

/// Creates a `HashSet` Java object from an iterator of Values.
async fn create_java_set_from_values<'a, T: Thread + 'static>(
    thread: &Arc<T>,
    values: impl ExactSizeIterator<Item = &'a Value>,
) -> Result<Value> {
    let mut hashed_values = Vec::with_capacity(values.len());
    for value in values {
        hashed_values.push((value_hash_code(thread, value).await?, value.clone()));
    }
    create_hashed_set(thread, hashed_values).await
}

/// Creates the graph map for `Configuration` (`Map<ResolvedModule, Set<ResolvedModule>>`).
async fn create_configuration_graph<T: Thread + 'static>(
    thread: &Arc<T>,
    resolved_config: &ResolvedConfiguration,
    resolved_module_map: &HashMap<String, Value>,
) -> Result<Value> {
    let mut entries = Vec::with_capacity(resolved_module_map.len());
    for rm in resolved_config.modules() {
        if let Some(rm_value) = resolved_module_map.get(rm.name()) {
            let reads = rm
                .reads()
                .iter()
                .filter_map(|read_name| resolved_module_map.get(read_name))
                .collect::<Vec<_>>();
            let reads_set = create_java_set_from_values(thread, reads.into_iter()).await?;
            entries.push((
                value_hash_code(thread, rm_value).await?,
                rm_value.clone(),
                reads_set,
            ));
        }
    }
    create_hashed_map(thread, entries).await
}

/// Sets fields on the `ModuleLayer` Java object.
async fn set_layer_fields<T: Thread + 'static>(
    thread: &Arc<T>,
    layer_value: &Value,
    config_value: &Value,
    name_to_module: &Value,
    module_map: &HashMap<String, Value>,
) -> Result<()> {
    // Create modules set for ModuleLayer (Set<Module>)
    let layer_modules_set = create_java_set_from_values(thread, module_map.values()).await?;

    // Boot layer's parent is ModuleLayer.empty().
    // Invoke the public ModuleLayer.empty() method which both triggers <clinit>
    // and returns the singleton, rather than reading the private static field directly.
    let empty_layer = thread
        .invoke(
            "java.lang.ModuleLayer",
            "empty()Ljava/lang/ModuleLayer;",
            &[] as &[Value],
        )
        .await?;
    let parents_list = create_array_list(thread, empty_layer.into_iter().collect()).await?;
    {
        let mut layer_ref = layer_value.as_object_mut()?;
        layer_ref.set_value_unchecked("cf", config_value.clone())?;
        layer_ref.set_value_unchecked("parents", parents_list)?;
        layer_ref.set_value_unchecked("nameToModule", name_to_module.clone())?;
        layer_ref.set_value_unchecked("modules", layer_modules_set)?;
    }

    Ok(())
}

/// Registers modules in `ServicesCatalog` for `ServiceLoader` support.
///
/// Builds the `ServicesCatalog` entirely from Rust to avoid triggering
/// `ClassLoaderValue`/`AbstractClassLoaderValue` static initialization (which causes
/// `IncompatibleClassChangeError` during phase 2).
async fn register_services_catalog_on_layer<T: Thread + 'static>(
    thread: &Arc<T>,
    resolved_config: &ResolvedConfiguration,
    module_map: &HashMap<String, Value>,
    layer_value: &Value,
) {
    let result: Result<()> = async {
        let vm = thread.vm()?;
        let gc = vm.garbage_collector();

        // Create ServicesCatalog object directly (bypassing constructor and clinit)
        let catalog_class = thread.class("jdk/internal/module/ServicesCatalog").await?;
        let mut catalog_object = Object::new(catalog_class)?;

        // Create ConcurrentHashMap(32) for the map field
        let chm_value = thread
            .object(
                "java/util/concurrent/ConcurrentHashMap",
                "I",
                &[Value::Int(32)],
            )
            .await?;
        catalog_object.set_value_unchecked("map", chm_value.clone())?;

        let catalog_value = Value::from_object(gc, catalog_object);

        // Build service entries: Map<String, List<ServiceProvider>>
        let sp_class = thread
            .class("jdk/internal/module/ServicesCatalog$ServiceProvider")
            .await?;

        let mut providers_by_service: HashMap<String, Vec<Value>> = HashMap::new();
        for (name, module_value) in module_map {
            if let Some(rm) = resolved_config.get(name) {
                for provides in &rm.descriptor().provides {
                    let service = to_dotted(&provides.service);
                    let service_providers = providers_by_service.entry(service).or_default();
                    for provider_name in &provides.implementations {
                        let mut sp_obj = Object::new(sp_class.clone())?;
                        sp_obj.set_value_unchecked("module", module_value.clone())?;
                        let pn_str = thread.intern_string(&to_dotted(provider_name)).await?;
                        sp_obj.set_value_unchecked("providerName", pn_str)?;
                        let sp_val = Value::from_object(gc, sp_obj);
                        service_providers.push(sp_val);
                    }
                }
            }
        }

        for (service, providers) in providers_by_service {
            let service_key = thread.intern_string(&service).await?;
            let provider_list = create_copy_on_write_array_list(thread, providers).await?;
            thread
                .invoke(
                    "java.util.concurrent.ConcurrentHashMap",
                    "put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    &[chm_value.clone(), service_key, provider_list],
                )
                .await?;
        }

        // Set the catalog on the boot ModuleLayer
        let mut layer_ref = layer_value.as_object_mut()?;
        layer_ref.set_value_unchecked("servicesCatalog", catalog_value)?;
        Ok(())
    }
    .await;

    if let Err(e) = result {
        debug!("ServicesCatalog registration failed (non-fatal): {e}");
    }
}

/// Updates the module field on every loaded boot-loader Class object whose package
/// belongs to a now-defined named module. Classes loaded during early bootstrap
/// (before `ModuleBootstrap.boot()` ran) had their module field assigned to the
/// boot loader's unnamed module. After `defineModule0` populates the canonical
/// `Module` instances, those Class objects must be re-pointed at the canonical
/// `Module` so that JDK identity comparisons such as
/// `callerModule == declaringModule` in
/// `java.lang.reflect.AccessibleObject.checkCanSetAccessible` succeed for
/// system-to-system reflection (e.g. `Class.getEnumConstantsShared` calling
/// `Method.setAccessible(true)` on an enum's `values()` method).
async fn update_loaded_class_modules<T: Thread + 'static>(
    thread: &Arc<T>,
    module_map: &HashMap<String, Value>,
) {
    let Ok(vm) = thread.vm() else { return };
    let mut class_loader = vm.class_loader().read().await.clone();
    while let Some(parent) = class_loader.parent().await {
        class_loader = parent.clone();
    }
    let classes = class_loader.loaded_classes().await;
    for class in classes {
        if class.is_primitive() {
            continue;
        }
        let Ok(Some(class_obj)) = class.object() else {
            continue;
        };
        let Ok(obj_ref) = class_obj.as_object_ref() else {
            continue;
        };
        let Ok(current_module) = obj_ref.value("module") else {
            continue;
        };
        drop(obj_ref);
        let package = ristretto_classloader::ClassLoader::package_from_class_name(class.name());
        let Some(name) = vm
            .module_system()
            .resolved_configuration()
            .find_module_for_package(package)
        else {
            continue;
        };
        let Some(canonical) = module_map.get(name) else {
            continue;
        };
        let same = !current_module.is_null()
            && match (&current_module, canonical) {
                (Value::Object(Some(a)), Value::Object(Some(b))) => ristretto_gc::Gc::ptr_eq(a, b),
                _ => false,
            };
        if same {
            continue;
        }
        if let Ok(mut obj_mut) = class_obj.as_object_mut() {
            let _ = obj_mut.set_value_unchecked("module", canonical.clone());
        }
    }
}

/// Updates primitive class modules to point to `java.base`.
///
/// Primitive classes (`int`, `boolean`, etc.) are created during early bootstrap before
/// `ModuleBootstrap.boot()`. Their Class objects get assigned to the unnamed module.
/// Since `int.class` compiles to `getstatic Integer.TYPE`, the module field is never
/// updated by the normal class-loading path. This fixes them up after boot.
async fn update_primitive_class_modules<T: Thread + 'static>(
    thread: &Arc<T>,
    module_map: &HashMap<String, Value>,
) {
    if let Some(java_base_value) = module_map.get("java.base") {
        let primitives = [
            "boolean", "byte", "char", "double", "float", "int", "long", "short", "void",
        ];
        for prim_name in &primitives {
            if let Ok(prim_class) = thread.class(prim_name).await
                && let Ok(Some(class_obj)) = prim_class.object()
                && let Ok(mut obj_mut) = class_obj.as_object_mut()
            {
                let _ = obj_mut.set_value_unchecked("module", java_base_value.clone());
            }
        }
    }
}

/// Creates an empty boot layer (fallback when no modules are resolved).
async fn create_empty_layer<T: Thread + 'static>(thread: &Arc<T>) -> Result<Option<Value>> {
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

/// Creates a Java `ModuleDescriptor` object by directly constructing the object and
/// setting its fields, bypassing the Builder API to avoid bootstrap circularity
/// and thousands of Java method invocations.
#[expect(clippy::too_many_arguments)]
#[expect(clippy::too_many_lines)]
async fn create_module_descriptor_direct<T: Thread + 'static>(
    thread: &Arc<T>,
    desc: &ModuleDescriptor,
    descriptor_class: &Arc<Class>,
    requires_class: &Arc<Class>,
    exports_class: &Arc<Class>,
    opens_class: &Arc<Class>,
    provides_class: &Arc<Class>,
    empty_set: &Value,
    version_cache: &mut HashMap<String, Value>,
) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let mut md_object = Object::new(descriptor_class.clone())?;

    // Set name
    let name_val = thread.intern_string(&desc.name).await?;
    md_object.set_value_unchecked("name", name_val)?;

    // Set open and automatic flags
    if let Err(e) = md_object.set_value_unchecked("open", Value::Int(i32::from(desc.is_open()))) {
        debug!("Failed to set ModuleDescriptor.open: {e}");
    }
    if let Err(e) = md_object.set_value_unchecked("automatic", Value::Int(0)) {
        debug!("Failed to set ModuleDescriptor.automatic: {e}");
    }

    // Set modifiers (empty set for normal modules)
    if let Err(e) = md_object.set_value_unchecked("modifiers", empty_set.clone()) {
        debug!("Failed to set ModuleDescriptor.modifiers: {e}");
    }

    // Set packages (convert '/' to '.')
    if desc.packages.is_empty() {
        md_object.set_value_unchecked("packages", empty_set.clone())?;
    } else {
        let pkg_set = create_string_set_dotted(thread, &desc.packages).await?;
        md_object.set_value_unchecked("packages", pkg_set)?;
    }

    // Set requires
    if desc.requires.is_empty() {
        md_object.set_value_unchecked("requires", empty_set.clone())?;
    } else {
        let mut values = Vec::with_capacity(desc.requires.len());
        for req in &desc.requires {
            let req_obj =
                create_requires_direct(thread, req, requires_class, empty_set, version_cache)
                    .await?;
            values.push((requires_hash_code(req), req_obj));
        }
        let req_set = if vm.java_major_version() >= 17 {
            create_hashed_set(thread, values).await?
        } else {
            let values = values
                .into_iter()
                .map(|(_, value)| value)
                .collect::<Vec<_>>();
            create_java_set_from_values(thread, values.iter()).await?
        };
        md_object.set_value_unchecked("requires", req_set)?;
    }

    // Set exports (convert package names '/' to '.')
    if desc.exports.is_empty() {
        md_object.set_value_unchecked("exports", empty_set.clone())?;
    } else {
        let mut values = Vec::with_capacity(desc.exports.len());
        for export in &desc.exports {
            let exp_obj = create_exports_direct(thread, export, exports_class, empty_set).await?;
            values.push((exports_hash_code(export), exp_obj));
        }
        let exp_set = if vm.java_major_version() >= 17 {
            create_hashed_set(thread, values).await?
        } else {
            let values = values
                .into_iter()
                .map(|(_, value)| value)
                .collect::<Vec<_>>();
            create_java_set_from_values(thread, values.iter()).await?
        };
        md_object.set_value_unchecked("exports", exp_set)?;
    }

    // Set opens (convert package names '/' to '.')
    if desc.opens.is_empty() {
        md_object.set_value_unchecked("opens", empty_set.clone())?;
    } else {
        let mut values = Vec::with_capacity(desc.opens.len());
        for open in &desc.opens {
            let open_obj = create_opens_direct(thread, open, opens_class, empty_set).await?;
            values.push((opens_hash_code(open), open_obj));
        }
        let opens_set = if vm.java_major_version() >= 17 {
            create_hashed_set(thread, values).await?
        } else {
            let values = values
                .into_iter()
                .map(|(_, value)| value)
                .collect::<Vec<_>>();
            create_java_set_from_values(thread, values.iter()).await?
        };
        md_object.set_value_unchecked("opens", opens_set)?;
    }

    // Set uses (convert class names '/' to '.')
    if desc.uses.is_empty() {
        md_object.set_value_unchecked("uses", empty_set.clone())?;
    } else {
        let uses_set = {
            let dotted: std::collections::BTreeSet<String> =
                desc.uses.iter().map(|s| to_dotted(s)).collect();
            create_string_set_dotted(thread, &dotted).await?
        };
        md_object.set_value_unchecked("uses", uses_set)?;
    }

    // Set provides (Set<Provides>)
    if desc.provides.is_empty() {
        md_object.set_value_unchecked("provides", empty_set.clone())?;
    } else {
        let mut values = Vec::with_capacity(desc.provides.len());
        for prov in &desc.provides {
            let prov_obj = create_provides_direct(thread, prov, provides_class).await?;
            values.push((provides_hash_code(prov), prov_obj));
        }
        let provides_set = if vm.java_major_version() >= 17 {
            create_hashed_set(thread, values).await?
        } else {
            let values = values
                .into_iter()
                .map(|(_, value)| value)
                .collect::<Vec<_>>();
            create_java_set_from_values(thread, values.iter()).await?
        };
        md_object.set_value_unchecked("provides", provides_set)?;
    }

    // Set version
    if let Some(ref ver) = desc.version {
        let ver_string = ver.clone();
        let ver_str = thread.intern_string(&ver_string).await?;
        if let Err(e) = md_object.set_value_unchecked("rawVersionString", ver_str) {
            debug!("Failed to set ModuleDescriptor.rawVersionString: {e}");
        }
        // Use cached Version object or create one via Version.parse()
        let version_obj = if let Some(cached) = version_cache.get(&ver_string) {
            cached.clone()
        } else {
            let ver_str_for_parse = thread.intern_string(&ver_string).await?;
            let parsed = thread
                .invoke(
                    "java.lang.module.ModuleDescriptor$Version",
                    "parse(Ljava/lang/String;)Ljava/lang/module/ModuleDescriptor$Version;",
                    &[ver_str_for_parse],
                )
                .await?;
            if let Some(ref v) = parsed {
                version_cache.insert(ver_string, v.clone());
            }
            parsed.unwrap_or(Value::Object(None))
        };
        if !version_obj.is_null()
            && let Err(e) = md_object.set_value_unchecked("version", version_obj)
        {
            debug!("Failed to set ModuleDescriptor.version: {e}");
        }
    }

    // Set main class (convert '/' to '.')
    if let Some(ref mc) = desc.main_class {
        let mc_str = thread.intern_string(&to_dotted(mc)).await?;
        if let Err(e) = md_object.set_value_unchecked("mainClass", mc_str) {
            debug!("Failed to set ModuleDescriptor.mainClass: {e}");
        }
    }

    // ModuleDescriptor uses this exact cached hash scheme on JDK 17 and later.
    // JDK 11 hashes enum modifier sets by identity, so it must compute the value in Java.
    if vm.java_major_version() >= 17 {
        md_object.set_value_unchecked("hash", Value::Int(module_descriptor_hash_code(desc)))?;
    }

    Ok(Value::from_object(gc, md_object))
}

/// Creates a `ModuleDescriptor$Requires` object directly.
async fn create_requires_direct<T: Thread + 'static>(
    thread: &Arc<T>,
    req: &Requires,
    requires_class: &Arc<Class>,
    empty_set: &Value,
    version_cache: &mut HashMap<String, Value>,
) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let mut obj = Object::new(requires_class.clone())?;

    let name_val = thread.intern_string(&req.name).await?;
    obj.set_value_unchecked("name", name_val)?;

    // Set modifier set
    if req.flags.is_empty() {
        obj.set_value_unchecked("mods", empty_set.clone())?;
    } else {
        let mods = create_requires_modifier_set(thread, req.flags).await?;
        obj.set_value_unchecked("mods", mods)?;
    }

    // Set compiled version
    if let Some(ref ver) = req.version {
        let ver_string = ver.clone();
        let ver_str = thread.intern_string(&ver_string).await?;
        if let Err(e) = obj.set_value_unchecked("rawCompiledVersion", ver_str) {
            debug!("Failed to set Requires.rawCompiledVersion: {e}");
        }
        // Use cached Version object or create one via Version.parse()
        let version_obj = if let Some(cached) = version_cache.get(&ver_string) {
            cached.clone()
        } else {
            let ver_str_for_parse = thread.intern_string(&ver_string).await?;
            let parsed = thread
                .invoke(
                    "java.lang.module.ModuleDescriptor$Version",
                    "parse(Ljava/lang/String;)Ljava/lang/module/ModuleDescriptor$Version;",
                    &[ver_str_for_parse],
                )
                .await?;
            if let Some(ref v) = parsed {
                version_cache.insert(ver_string, v.clone());
            }
            parsed.unwrap_or(Value::Object(None))
        };
        if !version_obj.is_null()
            && let Err(e) = obj.set_value_unchecked("compiledVersion", version_obj)
        {
            debug!("Failed to set Requires.compiledVersion: {e}");
        }
    }

    Ok(Value::from_object(gc, obj))
}

/// Creates a `ModuleDescriptor$Exports` object directly.
async fn create_exports_direct<T: Thread + 'static>(
    thread: &Arc<T>,
    export: &Exports,
    exports_class: &Arc<Class>,
    empty_set: &Value,
) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let mut obj = Object::new(exports_class.clone())?;

    let source_val = thread.intern_string(&to_dotted(&export.package)).await?;
    obj.set_value_unchecked("source", source_val)?;

    // Set modifiers (empty for now)
    obj.set_value_unchecked("mods", empty_set.clone())?;

    // Set targets
    match &export.targets {
        None => {
            obj.set_value_unchecked("targets", empty_set.clone())?;
        }
        Some(targets) => {
            let target_set = create_string_set_from_vec(thread, targets).await?;
            obj.set_value_unchecked("targets", target_set)?;
        }
    }

    Ok(Value::from_object(gc, obj))
}

/// Creates a `ModuleDescriptor$Opens` object directly.
async fn create_opens_direct<T: Thread + 'static>(
    thread: &Arc<T>,
    open: &Opens,
    opens_class: &Arc<Class>,
    empty_set: &Value,
) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let mut obj = Object::new(opens_class.clone())?;

    let source_val = thread.intern_string(&to_dotted(&open.package)).await?;
    obj.set_value_unchecked("source", source_val)?;

    // Set modifiers (empty for now)
    obj.set_value_unchecked("mods", empty_set.clone())?;

    // Set targets
    match &open.targets {
        None => {
            obj.set_value_unchecked("targets", empty_set.clone())?;
        }
        Some(targets) => {
            let target_set = create_string_set_from_vec(thread, targets).await?;
            obj.set_value_unchecked("targets", target_set)?;
        }
    }

    Ok(Value::from_object(gc, obj))
}

/// Creates a `ModuleDescriptor$Provides` object directly.
async fn create_provides_direct<T: Thread + 'static>(
    thread: &Arc<T>,
    prov: &Provides,
    provides_class: &Arc<Class>,
) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let mut obj = Object::new(provides_class.clone())?;

    let svc_val = thread.intern_string(&to_dotted(&prov.service)).await?;
    obj.set_value_unchecked("service", svc_val)?;

    // Create providers list
    let mut providers = Vec::with_capacity(prov.implementations.len());
    for impl_class in &prov.implementations {
        let impl_str = thread.intern_string(&to_dotted(impl_class)).await?;
        providers.push(impl_str);
    }
    let providers_list = create_array_list(thread, providers).await?;
    obj.set_value_unchecked("providers", providers_list)?;

    Ok(Value::from_object(gc, obj))
}

/// Creates a `HashSet` containing Requires.Modifier enum values from flags.
async fn create_requires_modifier_set<T: Thread + 'static>(
    thread: &Arc<T>,
    flags: RequiresFlags,
) -> Result<Value> {
    let mod_class = thread
        .class("java/lang/module/ModuleDescriptor$Requires$Modifier")
        .await?;
    let mut modifiers = Vec::with_capacity(4);

    if flags.contains(RequiresFlags::TRANSITIVE) {
        let val = mod_class.static_value("TRANSITIVE")?;
        modifiers.push((identity_hash_code(&val)?, val));
    }
    if flags.contains(RequiresFlags::STATIC_PHASE) {
        let val = mod_class.static_value("STATIC")?;
        modifiers.push((identity_hash_code(&val)?, val));
    }
    if flags.contains(RequiresFlags::SYNTHETIC) {
        let val = mod_class.static_value("SYNTHETIC")?;
        modifiers.push((identity_hash_code(&val)?, val));
    }
    if flags.contains(RequiresFlags::MANDATED) {
        let val = mod_class.static_value("MANDATED")?;
        modifiers.push((identity_hash_code(&val)?, val));
    }

    create_hashed_set(thread, modifiers).await
}

/// Creates a `HashSet<String>` from a `BTreeSet<String>` (strings already in dotted format).
async fn create_string_set_dotted<T: Thread + 'static>(
    thread: &Arc<T>,
    items: &std::collections::BTreeSet<String>,
) -> Result<Value> {
    let mut values = Vec::with_capacity(items.len());
    for item in items {
        let item = to_dotted(item);
        let value = thread.intern_string(&item).await?;
        values.push((string_hash_code(&item), value));
    }
    create_hashed_set(thread, values).await
}

/// Creates a `HashSet<String>` from a `Vec<String>`.
async fn create_string_set_from_vec<T: Thread + 'static>(
    thread: &Arc<T>,
    items: &[String],
) -> Result<Value> {
    let mut values = Vec::with_capacity(items.len());
    for item in items {
        let value = thread.intern_string(item).await?;
        values.push((string_hash_code(item), value));
    }
    create_hashed_set(thread, values).await
}

async fn create_array_list<T: Thread + 'static>(
    thread: &Arc<T>,
    values: Vec<Value>,
) -> Result<Value> {
    let size = i32::try_from(values.len())?;
    let array_class = thread.class("[Ljava/lang/Object;").await?;
    let list_class = thread.class("java/util/ArrayList").await?;
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();
    let elements = Value::new_object(gc, Reference::try_from((array_class, values))?);
    let mut list = Object::new(list_class)?;
    list.set_value_unchecked("elementData", elements)?;
    list.set_value_unchecked("size", Value::Int(size))?;
    Ok(Value::from_object(gc, list))
}

async fn create_copy_on_write_array_list<T: Thread + 'static>(
    thread: &Arc<T>,
    values: Vec<Value>,
) -> Result<Value> {
    let array_class = thread.class("[Ljava/lang/Object;").await?;
    let list_class = thread
        .class("java/util/concurrent/CopyOnWriteArrayList")
        .await?;
    let lock_class = thread.class("java/lang/Object").await?;
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();
    let elements = Value::new_object(gc, Reference::try_from((array_class, values))?);
    let lock = Value::from_object(gc, Object::new(lock_class)?);
    let mut list = Object::new(list_class)?;
    list.set_value_unchecked("lock", lock)?;
    list.set_value_unchecked("array", elements)?;
    Ok(Value::from_object(gc, list))
}

async fn create_hashed_set<T: Thread + 'static>(
    thread: &Arc<T>,
    values: Vec<(i32, Value)>,
) -> Result<Value> {
    let hash_set_class = thread.class("java/util/HashSet").await?;
    let present = hash_set_class.static_value("PRESENT")?;
    let entries = values
        .into_iter()
        .map(|(hash, key)| (hash, key, present.clone()))
        .collect();
    let map = create_hashed_map(thread, entries).await?;

    let vm = thread.vm()?;
    let mut set = Object::new(hash_set_class)?;
    set.set_value_unchecked("map", map)?;
    Ok(Value::from_object(vm.garbage_collector(), set))
}

async fn create_hashed_map<T: Thread + 'static>(
    thread: &Arc<T>,
    entries: Vec<(i32, Value, Value)>,
) -> Result<Value> {
    let map_class = thread.class("java/util/HashMap").await?;
    let vm = thread.vm()?;
    let mut map = Object::new(map_class)?;
    map.set_value_unchecked("loadFactor", Value::Float(0.75))?;
    let map = Value::from_object(vm.garbage_collector(), map);
    populate_hashed_map(thread, &map, entries).await?;
    Ok(map)
}

async fn populate_hashed_map<T: Thread + 'static>(
    thread: &Arc<T>,
    map: &Value,
    entries: Vec<(i32, Value, Value)>,
) -> Result<()> {
    let size = entries.len();
    if entries.is_empty() {
        return Ok(());
    }

    let node_class = thread.class("java/util/HashMap$Node").await?;
    let node_array_class = thread.class("[Ljava/util/HashMap$Node;").await?;
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let minimum_capacity = size.saturating_mul(4).div_ceil(3).max(1);
    let capacity = minimum_capacity
        .checked_next_power_of_two()
        .unwrap_or(1 << 30)
        .min(1 << 30);
    let mut buckets = vec![Value::Object(None); capacity];

    for (hash, key, value) in entries {
        let hash = spread_hash(hash);
        let bucket_index = usize::try_from(hash.cast_unsigned())? & (capacity - 1);
        let bucket = buckets.get_mut(bucket_index).ok_or_else(|| {
            ristretto_types::Error::InternalError("invalid hash bucket".to_string())
        })?;
        let next = std::mem::replace(bucket, Value::Object(None));
        let mut node = Object::new(node_class.clone())?;
        node.set_value_unchecked("hash", Value::Int(hash))?;
        node.set_value_unchecked("key", key)?;
        node.set_value_unchecked("value", value)?;
        node.set_value_unchecked("next", next)?;
        *bucket = Value::from_object(gc, node);
    }

    let table = Reference::try_from((node_array_class, buckets))?;
    let table = Value::new_object(gc, table);
    let mut map_object = map.as_object_mut()?;
    map_object.set_value_unchecked("table", table)?;
    map_object.set_value_unchecked("size", Value::Int(i32::try_from(size)?))?;
    map_object.set_value_unchecked("modCount", Value::Int(0))?;
    map_object.set_value_unchecked(
        "threshold",
        Value::Int(i32::try_from(capacity.saturating_mul(3) / 4)?),
    )?;
    drop(map_object);
    Ok(())
}

fn string_hash_code(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, character| {
        hash.wrapping_mul(31).wrapping_add(i32::from(character))
    })
}

fn combine_hash_code(hash: i32, value: i32) -> i32 {
    hash.wrapping_mul(43).wrapping_add(value)
}

fn string_set_hash_code<'a>(values: impl IntoIterator<Item = &'a String>) -> i32 {
    values.into_iter().fold(0_i32, |hash, value| {
        hash.wrapping_add(string_hash_code(value))
    })
}

fn dotted_string_set_hash_code<'a>(values: impl IntoIterator<Item = &'a String>) -> i32 {
    values.into_iter().fold(0_i32, |hash, value| {
        hash.wrapping_add(string_hash_code(&to_dotted(value)))
    })
}

fn requires_modifiers_hash_code(flags: RequiresFlags) -> i32 {
    let mut hash = 0_i32;
    if flags.contains(RequiresFlags::TRANSITIVE) {
        hash = hash.wrapping_add(string_hash_code("TRANSITIVE"));
    }
    if flags.contains(RequiresFlags::STATIC_PHASE) {
        hash = hash.wrapping_add(string_hash_code("STATIC"));
    }
    if flags.contains(RequiresFlags::SYNTHETIC) {
        hash = hash.wrapping_add(string_hash_code("SYNTHETIC"));
    }
    if flags.contains(RequiresFlags::MANDATED) {
        hash = hash.wrapping_add(string_hash_code("MANDATED"));
    }
    hash
}

fn requires_hash_code(requires: &Requires) -> i32 {
    let mut hash = combine_hash_code(
        string_hash_code(&requires.name),
        requires_modifiers_hash_code(requires.flags),
    );
    if let Some(version) = &requires.version {
        let version_hash = string_hash_code(version);
        hash = combine_hash_code(hash, version_hash);
        hash = combine_hash_code(hash, version_hash);
    }
    hash
}

fn exports_hash_code(exports: &Exports) -> i32 {
    let source_hash = string_hash_code(&to_dotted(&exports.package));
    let targets_hash = exports.targets.as_ref().map_or(0, string_set_hash_code);
    combine_hash_code(source_hash, targets_hash)
}

fn opens_hash_code(opens: &Opens) -> i32 {
    let source_hash = string_hash_code(&to_dotted(&opens.package));
    let targets_hash = opens.targets.as_ref().map_or(0, string_set_hash_code);
    combine_hash_code(source_hash, targets_hash)
}

fn provides_hash_code(provides: &Provides) -> i32 {
    let providers_hash = provides
        .implementations
        .iter()
        .fold(1_i32, |hash, provider| {
            hash.wrapping_mul(31)
                .wrapping_add(string_hash_code(&to_dotted(provider)))
        });
    combine_hash_code(
        string_hash_code(&to_dotted(&provides.service)),
        providers_hash,
    )
}

fn module_descriptor_hash_code(descriptor: &ModuleDescriptor) -> i32 {
    let requires_hash = descriptor.requires.iter().fold(0_i32, |hash, requires| {
        hash.wrapping_add(requires_hash_code(requires))
    });
    let exports_hash = descriptor.exports.iter().fold(0_i32, |hash, exports| {
        hash.wrapping_add(exports_hash_code(exports))
    });
    let opens_hash = descriptor.opens.iter().fold(0_i32, |hash, opens| {
        hash.wrapping_add(opens_hash_code(opens))
    });
    let provides_hash = descriptor.provides.iter().fold(0_i32, |hash, provides| {
        hash.wrapping_add(provides_hash_code(provides))
    });
    let version_hash = descriptor.version.as_deref().map_or(0, string_hash_code);
    let main_class_hash = descriptor
        .main_class
        .as_deref()
        .map_or(0, |main_class| string_hash_code(&to_dotted(main_class)));

    let mut hash = string_hash_code(&descriptor.name);
    hash = combine_hash_code(hash, 0); // modifiers
    hash = combine_hash_code(hash, requires_hash);
    hash = combine_hash_code(hash, dotted_string_set_hash_code(&descriptor.packages));
    hash = combine_hash_code(hash, exports_hash);
    hash = combine_hash_code(hash, opens_hash);
    hash = combine_hash_code(hash, dotted_string_set_hash_code(&descriptor.uses));
    hash = combine_hash_code(hash, provides_hash);
    hash = combine_hash_code(hash, version_hash);
    hash = combine_hash_code(hash, version_hash); // rawVersionString
    hash = combine_hash_code(hash, main_class_hash);
    if hash == 0 { -1 } else { hash }
}

async fn value_hash_code<T: Thread + 'static>(thread: &Arc<T>, value: &Value) -> Result<i32> {
    let mut class = value.as_object_ref()?.class().clone();
    let class_name = loop {
        if class.try_get_method("hashCode", "()I").is_ok() {
            break class.name().to_string();
        }
        class = class.parent()?.ok_or_else(|| {
            ristretto_types::Error::InternalError("hashCode method not found".to_string())
        })?;
    };

    if class_name == "java/lang/Object" {
        return identity_hash_code(value);
    }

    let result = match thread
        .invoke(&class_name, "hashCode()I", std::slice::from_ref(value))
        .await
    {
        Ok(result) => result,
        Err(error) => {
            debug!("Failed to compute hashCode for {class_name}: {error:?}");
            return Err(error);
        }
    }
    .ok_or_else(|| {
        ristretto_types::Error::InternalError("hashCode returned no value".to_string())
    })?;
    Ok(result.as_i32()?)
}

fn identity_hash_code(value: &Value) -> Result<i32> {
    let hash_code = value.as_reference()?.hash_code() as u64;
    #[expect(clippy::cast_possible_truncation)]
    let hash_code = (hash_code ^ (hash_code >> 32)) as u32;
    Ok(zerocopy::transmute!(hash_code))
}

fn spread_hash(hash: i32) -> i32 {
    let hash = u32::from_ne_bytes(hash.to_ne_bytes());
    let hash = hash ^ (hash >> 16);
    i32::from_ne_bytes(hash.to_ne_bytes())
}

/// Converts JVM internal name format ('/' separated) to Java dotted format ('.' separated).
fn to_dotted(name: &str) -> String {
    name.replace('/', ".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_boot() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = boot(Arc::clone(&thread), Parameters::default())
            .await?
            .expect("module layer");
        let object = result.as_object_ref()?;
        let class = object.class();
        assert_eq!("java/lang/ModuleLayer", class.name());
        drop(object);

        let direct_hash = value_hash_code(&thread, &result).await?;
        let invoked_hash = thread
            .invoke(
                "java.lang.Object",
                "hashCode()I",
                std::slice::from_ref(&result),
            )
            .await?
            .expect("hash code")
            .as_i32()?;
        assert_eq!(invoked_hash, direct_hash);

        let vm = thread.vm()?;
        let resolved_configuration = vm.module_system().resolved_configuration_arc();
        for resolved_module in resolved_configuration.modules() {
            let descriptor = vm
                .module_system()
                .get_module(resolved_module.name())
                .and_then(|module| module.module_object)
                .expect("module")
                .as_object_ref()?
                .value("descriptor")?
                .clone();
            descriptor
                .as_object_mut()?
                .set_value_unchecked("hash", Value::Int(0))?;
            let java_hash = thread
                .invoke(
                    "java.lang.module.ModuleDescriptor",
                    "hashCode()I",
                    std::slice::from_ref(&descriptor),
                )
                .await?
                .expect("descriptor hash")
                .as_i32()?;
            assert_eq!(
                module_descriptor_hash_code(resolved_module.descriptor()),
                java_hash,
                "{} descriptor hash",
                resolved_module.name()
            );
        }
        Ok(())
    }
}
