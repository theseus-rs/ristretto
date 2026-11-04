use crate::{Result, Thread, VM};
use ahash::AHashMap;
use ristretto_classfile::{JAVA_8, JavaStr};
use ristretto_classloader::{Class, ClassLoader, ClassLoaderType, ClassPath, Reference, Value};
use ristretto_gc::sync::RwLock;
use ristretto_gc::{GarbageCollector, Gc, GcRootGuard};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Native metadata associated with Java class and class-loader mirrors.
///
/// Mirrors and loaders are keyed by object identity, with initiated classes scoped to each
/// loader. Like Java allocations, these associations live until VM teardown. Keeping loaders
/// alive also preserves classes' weak references to their defining loaders.
#[derive(Debug, Default)]
pub struct ClassRegistry {
    mirrors: RwLock<AHashMap<usize, Arc<Class>>>,
    loaders: Mutex<AHashMap<usize, Arc<ClassLoader>>>,
    initiated_classes: RwLock<AHashMap<i64, InitiatingLoaderClasses>>,
}

#[derive(Debug)]
struct InitiatingLoaderClasses {
    // Root the loader as well as its mirrors so pointer keys cannot be reused by GC.
    _loader: GcRootGuard<RwLock<Reference>>,
    classes: AHashMap<String, GcRootGuard<RwLock<Reference>>>,
}

impl ClassRegistry {
    /// Get the registry owned by this VM.
    ///
    /// # Errors
    /// Returns an error if the VM resource cannot be accessed.
    pub fn for_vm(vm: &impl VM) -> Result<Arc<Self>> {
        vm.resource_manager().get_or_init(Self::default)
    }

    /// Associate a class with its canonical Java mirror.
    pub fn register_mirror(&self, mirror: &Value, class: Arc<Class>) {
        if let Value::Object(Some(reference)) = mirror {
            self.mirrors
                .write()
                .insert(reference.as_ptr() as usize, class);
        }
    }

    /// Recover the exact definition represented by a Java mirror.
    pub fn class(&self, mirror: &Value) -> Option<Arc<Class>> {
        let Value::Object(Some(reference)) = mirror else {
            return None;
        };
        self.mirrors
            .read()
            .get(&(reference.as_ptr() as usize))
            .cloned()
    }

    /// Find a successful resolution by initiating loader and binary class name.
    /// This only reads recorded associations; it never loads or initializes a class.
    pub fn initiated_class(&self, loader: &Value, name: &str) -> Option<Value> {
        let Value::Object(Some(loader)) = loader else {
            return None;
        };
        self.initiated_classes
            .read()
            .get(&loader.as_ptr_i64())?
            .classes
            .get(name)
            .map(|class| Value::Object(Some(class.clone_gc())))
    }

    /// Record a successful resolution using its binary class name before initialization.
    /// The initiating loader can differ from the defining loader, including a sibling.
    pub fn register_initiated_class(
        &self,
        collector: &Arc<GarbageCollector>,
        loader: &Value,
        name: &str,
        class: &Value,
    ) {
        let (Value::Object(Some(loader)), Value::Object(Some(class))) = (loader, class) else {
            return;
        };
        self.initiated_classes
            .write()
            .entry(loader.as_ptr_i64())
            .or_insert_with(|| InitiatingLoaderClasses {
                _loader: collector.create_root_guard(loader.clone()),
                classes: AHashMap::default(),
            })
            .classes
            .entry(name.to_string())
            .or_insert_with(|| collector.create_root_guard(class.clone()));
    }

    /// Find or create the native metadata for a Java defining loader.
    ///
    /// # Errors
    /// Returns an error if the loader's parent or VM metadata cannot be read.
    pub async fn loader<T: Thread>(thread: &T, java_loader: &Value) -> Result<Arc<ClassLoader>> {
        let vm = thread.vm()?;
        let root = vm.class_loader().read().await.clone();
        if let Value::Object(Some(expected)) = java_loader
            && let Some(Value::Object(Some(actual))) = root.object().await
            && Gc::ptr_eq(expected, &actual)
        {
            return Ok(root);
        }
        let builtin_type = if java_loader.is_null() {
            Some(ClassLoaderType::Bootstrap)
        } else {
            let (holder, field) = if *vm.java_class_file_version() <= JAVA_8 {
                ("java/lang/ClassLoader", "scl")
            } else {
                ("jdk/internal/loader/ClassLoaders", "APP_LOADER")
            };
            // Do not initialize a loader while defining a class.
            let app_loader = root
                .find_loaded(JavaStr::try_from_str(holder)?)
                .await
                .and_then(|class| class.static_value(field).ok());
            matches!(
                (java_loader, app_loader),
                (Value::Object(Some(expected)), Some(Value::Object(Some(actual))))
                    if Gc::ptr_eq(expected, &actual)
            )
            .then_some(ClassLoaderType::System)
        };
        let mut current = Some(root);
        while let Some(loader) = current {
            if builtin_type.is_some() && loader.loader_type() == builtin_type {
                if !java_loader.is_null() {
                    loader.set_object(Some(java_loader.clone())).await;
                }
                return Ok(loader);
            }
            if let (Value::Object(Some(expected)), Some(Value::Object(Some(actual)))) =
                (java_loader, loader.object().await)
                && Gc::ptr_eq(expected, &actual)
            {
                return Ok(loader);
            }
            current = loader.parent().await;
        }

        let Value::Object(Some(reference)) = java_loader else {
            return Err(crate::Error::InternalError(
                "missing bootstrap class loader".to_string(),
            ));
        };
        let id = reference.as_ptr() as usize;
        let registry = Self::for_vm(&*vm)?;
        if let Some(loader) = registry.loaders.lock().await.get(&id).cloned() {
            return Ok(loader);
        }
        let declaration = thread
            .load_and_link_class(JavaStr::try_from_str("java/lang/ClassLoader")?)
            .await?;
        let (name, parent) = {
            let object = java_loader.as_object_ref()?;
            (
                object.class().name().to_string(),
                object.value_in_class(&declaration, "parent")?,
            )
        };
        let parent = Box::pin(Self::loader(thread, &parent)).await?;
        // Recheck after resolving the parent; two threads can define through the same loader.
        let mut loaders = registry.loaders.lock().await;
        if let Some(loader) = loaders.get(&id) {
            return Ok(loader.clone());
        }
        let loader = ClassLoader::new(name, ClassPath::new(Vec::new()));
        loader.set_parent(Some(parent)).await;
        loader.set_object(Some(java_loader.clone())).await;
        loaders.insert(id, loader.clone());
        Ok(loader)
    }
}
