use crate::Error::InternalError;
use crate::Result;
use crate::thread::Thread;
use ristretto_classfile::{JAVA_8, JAVA_17, JAVA_25, JavaStr};
use ristretto_classloader::{Class, ClassLoader, Object, Reference, Value};
use std::sync::Arc;

/// Trait for converting a Rust value to a Java object.  Converts to objects of the primitive
/// wrapper, classes, and strings.
pub trait JavaObject {
    #[expect(async_fn_in_trait)]
    async fn to_object(&self, thread: &Thread) -> Result<Value>;
}

impl JavaObject for bool {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke(
                "java.lang.Boolean",
                "valueOf(Z)Ljava/lang/Boolean;",
                &[value],
            )
            .await?;
        Ok(result)
    }
}

impl JavaObject for char {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke(
                "java.lang.Character",
                "valueOf(C)Ljava/lang/Character;",
                &[value],
            )
            .await?;
        Ok(result)
    }
}

impl JavaObject for i8 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Byte", "valueOf(B)Ljava/lang/Byte;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for u8 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value: i8 = zerocopy::transmute!(*self);
        value.to_object(thread).await
    }
}

impl JavaObject for i16 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Short", "valueOf(S)Ljava/lang/Short;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for u16 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value: i16 = zerocopy::transmute!(*self);
        value.to_object(thread).await
    }
}

impl JavaObject for i32 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke(
                "java.lang.Integer",
                "valueOf(I)Ljava/lang/Integer;",
                &[value],
            )
            .await?;
        Ok(result)
    }
}

impl JavaObject for u32 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value: i32 = zerocopy::transmute!(*self);
        value.to_object(thread).await
    }
}

impl JavaObject for i64 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Long", "valueOf(J)Ljava/lang/Long;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for u64 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value: i64 = zerocopy::transmute!(*self);
        value.to_object(thread).await
    }
}

impl JavaObject for isize {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = *self as i64;
        value.to_object(thread).await
    }
}

impl JavaObject for usize {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = *self as u64;
        value.to_object(thread).await
    }
}

impl JavaObject for f32 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Float", "valueOf(F)Ljava/lang/Float;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for f64 {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = Value::from(*self);
        let result = thread
            .try_invoke("java.lang.Double", "valueOf(D)Ljava/lang/Double;", &[value])
            .await?;
        Ok(result)
    }
}

impl JavaObject for &str {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let class = thread.class("java.lang.String").await?;
        let mut object = Object::new(class)?;

        let vm = thread.vm()?;
        let collector = vm.garbage_collector();
        let java_class_file_version = vm.java_class_file_version();
        let array = if java_class_file_version <= &JAVA_8 {
            // Java 8 and below: store as UTF-16 char array
            let chars = self.encode_utf16().collect::<Vec<u16>>();
            Value::new_object(collector, Reference::CharArray(chars.into()))
        } else {
            if java_class_file_version >= &JAVA_17 {
                object.set_value("hashIsZero", Value::Int(0))?;
            }

            // Determine coder and value
            let use_latin1 = self.chars().all(|c| (c as u32) <= 0xFF);
            let (coder, bytes): (i32, Vec<i8>) = if use_latin1 {
                // All chars fit in Latin1; truncating to i8 is intentional: values 0x80..=0xFF
                // become negative in Rust, matching Java's signed byte[] representation.
                (0, self.chars().map(|c| c as i8).collect())
            } else {
                // Must use UTF-16
                let mut bytes = Vec::with_capacity(self.len() * 2);
                for c in self.encode_utf16() {
                    let b = c.to_ne_bytes();
                    bytes.push(b[0].cast_signed());
                    bytes.push(b[1].cast_signed());
                }
                (1, bytes)
            };

            object.set_value("coder", Value::Int(coder))?;
            Value::new_object(collector, Reference::from(bytes))
        };

        object.set_value("value", array)?;
        object.set_value("hash", Value::Int(0))?;

        let value = Value::from_object(collector, object);
        Ok(value)
    }
}

impl JavaObject for &JavaStr {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        // Fast path: if the MUTF-8 bytes are also valid UTF-8, delegate to &str
        if let Some(s) = self.as_str() {
            return s.to_object(thread).await;
        }

        // Slow path: decode MUTF-8 -> UTF-16 directly, preserving lone surrogates
        let utf16 = self
            .to_utf16()
            .map_err(|e| InternalError(format!("Failed to decode MUTF-8 to UTF-16: {e}")))?;

        let class = thread.class("java.lang.String").await?;
        let mut object = Object::new(class)?;

        let vm = thread.vm()?;
        let collector = vm.garbage_collector();
        let java_class_file_version = vm.java_class_file_version();
        let array = if java_class_file_version <= &JAVA_8 {
            // Java 8 and below: store as UTF-16 char array
            Value::new_object(collector, Reference::CharArray(utf16.into()))
        } else {
            if java_class_file_version >= &JAVA_17 {
                object.set_value("hashIsZero", Value::Int(0))?;
            }

            // Determine coder and value
            let use_latin1 = utf16.iter().all(|&c| c <= 0xFF);
            #[expect(clippy::cast_possible_truncation)]
            let (coder, bytes): (i32, Vec<i8>) = if use_latin1 {
                // Truncating u16 to i8 is intentional: values 0x80..=0xFF become negative in
                // Rust, matching Java's signed byte[] representation for Latin-1 strings.
                (0, utf16.iter().map(|&c| c as i8).collect())
            } else {
                let mut bytes = Vec::with_capacity(utf16.len() * 2);
                for &c in &utf16 {
                    let b = c.to_ne_bytes();
                    bytes.push(b[0].cast_signed());
                    bytes.push(b[1].cast_signed());
                }
                (1, bytes)
            };

            object.set_value("coder", Value::Int(coder))?;
            Value::new_object(collector, Reference::from(bytes))
        };

        object.set_value("value", array)?;
        object.set_value("hash", Value::Int(0))?;

        let value = Value::from_object(collector, object);
        Ok(value)
    }
}

impl JavaObject for String {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let value = self.as_str();
        value.to_object(thread).await
    }
}

impl JavaObject for Value {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        match self {
            Value::Int(value) => value.to_object(thread).await,
            Value::Long(value) => value.to_object(thread).await,
            Value::Float(value) => value.to_object(thread).await,
            Value::Double(value) => value.to_object(thread).await,
            Value::Object(_) => Ok(self.clone()),
            Value::Unused => Err(InternalError(
                "Cannot convert unused value to object".to_string(),
            )),
        }
    }
}

async fn to_class_loader_object(thread: &Thread, class_loader: &Arc<ClassLoader>) -> Result<Value> {
    if let Some(object) = class_loader.object().await {
        return Ok(object);
    }

    let name = class_loader.name();
    if name == "bootstrap" {
        let builtin_class_loader = Value::Object(None);
        class_loader
            .set_object(Some(builtin_class_loader.clone()))
            .await;
        return Ok(builtin_class_loader);
    }

    let vm = thread.vm()?;
    let builtin_class_loader = if *vm.java_class_file_version() == JAVA_8 {
        let builtin_class_loader = Value::Object(None);
        class_loader
            .set_object(Some(builtin_class_loader.clone()))
            .await;
        builtin_class_loader
    } else {
        let name: Value = name.to_object(thread).await?;
        let parent_class_loader = match class_loader.parent().await {
            Some(parent_class_loader) => Box::pin(parent_class_loader.to_object(thread)).await?,
            None => Value::Object(None),
        };
        let class_path = class_loader.class_path().to_string();
        let class_path_object: Value = class_path.to_object(thread).await?;

        let url_class_path = thread
            .object(
                "jdk.internal.loader.URLClassPath",
                "Ljava/lang/String;Z",
                &[class_path_object, Value::from(false)],
            )
            .await?;

        // For the system/app class loader, create a ClassLoaders$AppClassLoader instance
        // to match the JDK behavior where the app class loader is AppClassLoader
        let loader_class_name = if class_loader.name() == "system" {
            "jdk.internal.loader.ClassLoaders$AppClassLoader"
        } else {
            "jdk.internal.loader.BuiltinClassLoader"
        };
        let loader_class = thread.class(loader_class_name).await?;
        let builtin_class = thread
            .class("jdk.internal.loader.BuiltinClassLoader")
            .await?;
        let init_descriptor = "(Ljava/lang/String;Ljdk/internal/loader/BuiltinClassLoader;Ljdk/internal/loader/URLClassPath;)V";
        let Some(init_method) = builtin_class.method("<init>", init_descriptor) else {
            return Err(InternalError(format!(
                "No constructor found: BuiltinClassLoader.<init>{init_descriptor}"
            )));
        };
        let gc = vm.garbage_collector();
        let object = Value::new_object(gc, Reference::Object(Object::new(loader_class)?));
        let parameters = [object.clone(), name, parent_class_loader, url_class_path];
        Box::pin(thread.execute(&builtin_class, &init_method, &parameters)).await?;

        class_loader.set_object(Some(object.clone())).await;
        object
    };

    Ok(builtin_class_loader)
}

impl JavaObject for Arc<ClassLoader> {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        to_class_loader_object(thread, self).await
    }
}

/// Convert a Class to a Class object.
///
/// # Errors
///
/// An error will be returned if the conversion fails.
async fn to_class_object(thread: &Thread, class: &Arc<Class>) -> Result<Value> {
    let vm = thread.vm()?;
    let java_version = vm.java_class_file_version();

    // Return cached object if available, updating module if necessary
    if let Some(object) = class.object()? {
        if *java_version > JAVA_8 {
            update_cached_class_module(thread, class, &object).await?;
        }
        return Ok(object);
    }

    // Build class name and get class loader object
    let class_name = class.name().replace('/', ".");
    let name = class_name.to_object(thread).await?;
    let class_loader_object = match class.class_loader()? {
        Some(class_loader) => Box::pin(to_class_loader_object(thread, &class_loader)).await?,
        None => Value::Object(None),
    };

    // Get component type and module
    let component_type_object = get_component_type_object(thread, class).await?;
    let module = get_class_module(thread, class, &class_loader_object).await?;

    // Build constructor parameters based on Java version
    let (descriptor, parameters, module) = build_class_constructor_params(
        thread,
        class,
        java_version,
        class_loader_object,
        component_type_object,
        module,
    )
    .await?;

    // Create the Class object
    let object_value = thread
        .object("java.lang.Class", descriptor, &parameters)
        .await?;

    {
        let mut object = object_value.as_object_mut()?;
        object.set_value("name", name)?;

        if !matches!(module, Value::Object(None)) {
            object.set_value_unchecked("module", module)?;
        }
    }

    let result = class.set_object_if_absent(object_value)?;
    Ok(result)
}

/// Update the module field on a cached Class object for Java 9+.
/// This handles the case where the Class object was created before the module system was
/// fully initialized.
///
/// # Errors
///
/// An error will be returned if the module cannot be updated.
async fn update_cached_class_module(
    thread: &Thread,
    class: &Arc<Class>,
    object: &Value,
) -> Result<()> {
    let vm = thread.vm()?;
    let current_module = object.as_object_ref()?.value("module")?;
    if !current_module.is_null() {
        // Check if this is an unnamed module that should be upgraded to a named module.
        // During early bootstrap, classes get assigned to the unnamed module before
        // ModuleBootstrap.boot() defines the named modules.
        let is_unnamed = current_module
            .as_object_ref()
            .map(|obj| obj.value("name").map(|v| v.is_null()).unwrap_or(true))
            .unwrap_or(true);
        if !is_unnamed {
            return Ok(());
        }
        // If the unnamed module belongs to a non-bootstrap class loader (loader != null),
        // it was correctly set by defineClass and should not be overwritten.
        let has_real_loader = current_module
            .as_object_ref()
            .map(|obj| obj.value("loader").map(|v| !v.is_null()).unwrap_or(false))
            .unwrap_or(false);
        if has_real_loader {
            return Ok(());
        }
        // Fall through to check if a named module is now available
    }

    // Try to upgrade to a named module first (bootstrap classes loaded from system modules)
    let package = ClassLoader::package_from_class_name(class.name());

    // Primitive types (int, boolean, etc.) have no package but belong to java.base
    if package.is_empty()
        && class.is_primitive()
        && let Some(module) = vm.module_system().get_module_for_package("java/lang")
    {
        let mut object_mut = object.as_object_mut()?;
        object_mut.set_value_unchecked("module", module)?;
        return Ok(());
    }

    if let Some(module) = vm.module_system().get_module_for_package(package) {
        let mut object_mut = object.as_object_mut()?;
        object_mut.set_value_unchecked("module", module)?;
        return Ok(());
    }

    if let Some(class_loader) = class.class_loader()? {
        let class_loader_object = Box::pin(to_class_loader_object(thread, &class_loader)).await?;
        if !class_loader_object.is_null() {
            let mut module = thread
                .try_invoke(
                    "java.lang.ClassLoader",
                    "getUnnamedModule()Ljava/lang/Module;",
                    std::slice::from_ref(&class_loader_object),
                )
                .await?;

            // If the unnamed module is null, create one
            if module.is_null() {
                module = create_unnamed_module(thread, &class_loader_object).await?;
                // Store it in the class loader's unnamedModule field
                if !module.is_null() {
                    let mut loader_obj = class_loader_object.as_object_mut()?;
                    let _ = loader_obj.set_value_unchecked("unnamedModule", module.clone());
                }
            }

            if !module.is_null() {
                let mut object_mut = object.as_object_mut()?;
                object_mut.set_value_unchecked("module", module)?;
            }
        }
    } else {
        // Fall back to the boot loader's unnamed module
        if let Some(module) = vm.module_system().boot_unnamed_module() {
            let mut object_mut = object.as_object_mut()?;
            object_mut.set_value_unchecked("module", module)?;
            return Ok(());
        }

        // If no boot unnamed module exists yet, create one
        let module = create_boot_unnamed_module(thread).await?;
        if !module.is_null() {
            vm.module_system().set_boot_unnamed_module(module.clone());
            let mut object_mut = object.as_object_mut()?;
            object_mut.set_value_unchecked("module", module)?;
        }
    }

    Ok(())
}

/// Create an unnamed module for the boot class loader.
async fn create_boot_unnamed_module(thread: &Thread) -> Result<Value> {
    // Create a new Module object for an unnamed module.
    // The unnamed module has:
    // - layer = null
    // - name = null
    // - loader = null (boot class loader)
    // - descriptor = null
    let module_class = thread.class("java/lang/Module").await?;
    let module_object = Object::new(module_class)?;
    // All fields remain null/default which is correct for an unnamed boot module

    let vm = thread.vm()?;
    Ok(Value::from_object(vm.garbage_collector(), module_object))
}

/// Get the module for a class based on Java version and class loader.
///
/// # Errors
///
/// An error will be returned if the module cannot be retrieved.
async fn get_class_module(
    thread: &Thread,
    class: &Arc<Class>,
    class_loader_object: &Value,
) -> Result<Value> {
    let vm = thread.vm()?;
    let java_version = vm.java_class_file_version();

    if *java_version <= JAVA_8 {
        return Ok(Value::Object(None));
    }

    if class_loader_object.is_null() {
        // For bootstrap-loaded classes, try to find the correct named module first (e.g., java.base
        // for java.lang.String). If not found, fall back to the boot loader's unnamed module.
        let package = ClassLoader::package_from_class_name(class.name());

        // Primitive types (int, boolean, etc.) have no package but belong to java.base
        if package.is_empty()
            && class.is_primitive()
            && let Some(module) = vm.module_system().get_module_for_package("java/lang")
        {
            return Ok(module);
        }

        // First, try to get the module from the defined modules (from defineModule0)
        // This should have a complete Module object with descriptor
        if let Some(module) = vm.module_system().get_module_for_package(package) {
            return Ok(module);
        }

        // Fall back to the boot loader's unnamed module
        // Note: VM level checks in invoke0 enforce JPMS for system modules
        if let Some(module) = vm.module_system().boot_unnamed_module() {
            return Ok(module);
        }

        // If no boot unnamed module exists yet, create one
        let module = create_boot_unnamed_module(thread).await?;
        if !module.is_null() {
            vm.module_system().set_boot_unnamed_module(module.clone());
        }
        return Ok(module);
    }

    // Try to get the unnamed module from the class loader
    let module = thread
        .try_invoke(
            "java.lang.ClassLoader",
            "getUnnamedModule()Ljava/lang/Module;",
            std::slice::from_ref(class_loader_object),
        )
        .await?;

    // If the unnamed module is null, we need to create one.
    // This can happen if the ClassLoader was not fully initialized through
    // the normal JDK initialization path.
    if module.is_null() {
        // Create an unnamed module for this class loader
        let module = create_unnamed_module(thread, class_loader_object).await?;
        // Store it in the class loader's unnamedModule field
        if !module.is_null() {
            let mut loader_obj = class_loader_object.as_object_mut()?;
            // Try to set the field; it might be final, so we use set_value_unchecked
            let _ = loader_obj.set_value_unchecked("unnamedModule", module.clone());
        }
        return Ok(module);
    }

    Ok(module)
}

/// Create an unnamed module for the given class loader.
async fn create_unnamed_module(thread: &Thread, class_loader_object: &Value) -> Result<Value> {
    // Create a new Module object for an unnamed module.
    // The unnamed module has:
    // - layer = null
    // - name = null
    // - loader = the class loader
    // - descriptor = null
    //
    // We can't call the Module(ClassLoader) constructor directly as it's package-private.
    // Instead, we create the object and set the fields directly.
    let module_class = thread.class("java/lang/Module").await?;
    let mut module_object = Object::new(module_class)?;

    // Set the loader field to the class loader
    module_object.set_value_unchecked("loader", class_loader_object.clone())?;
    // layer and descriptor remain null (default)
    // name remains null (unnamed module)

    let vm = thread.vm()?;
    Ok(Value::from_object(vm.garbage_collector(), module_object))
}

/// Build a `ProtectionDomain` Java object from a code source URL string.
///
/// Creates a `ProtectionDomain(CodeSource(URL(url_str), null), null)` object hierarchy.
///
/// # Errors
///
/// An error will be returned if the Java objects cannot be created.
async fn build_protection_domain(thread: &Thread, url_str: &str) -> Result<Value> {
    let url_string = url_str.to_object(thread).await?;
    let url = thread
        .object("java/net/URL", "Ljava/lang/String;", &[url_string])
        .await?;
    let code_source = thread
        .object(
            "java/security/CodeSource",
            "Ljava/net/URL;[Ljava/security/cert/Certificate;",
            &[url, Value::Object(None)],
        )
        .await?;
    thread
        .object(
            "java/security/ProtectionDomain",
            "Ljava/security/CodeSource;Ljava/security/PermissionCollection;",
            &[code_source, Value::Object(None)],
        )
        .await
}

/// Build the constructor descriptor and parameters for creating a Class object based on Java version.
///
/// # Returns
///
/// A tuple containing the constructor descriptor, parameters, and module value.
async fn build_class_constructor_params(
    thread: &Thread,
    class: &Arc<Class>,
    java_version: &ristretto_classfile::Version,
    class_loader_object: Value,
    component_type_object: Value,
    module: Value,
) -> Result<(&'static str, Vec<Value>, Value)> {
    if *java_version <= JAVA_8 {
        Ok((
            "Ljava/lang/ClassLoader;",
            vec![class_loader_object],
            Value::Object(None),
        ))
    } else if *java_version < JAVA_25 {
        Ok((
            "Ljava/lang/ClassLoader;Ljava/lang/Class;",
            vec![class_loader_object, component_type_object],
            module,
        ))
    } else {
        let modifiers = Value::from(class.class_file().access_flags.bits());
        let protection_domain = match class.class_file().code_source_url.as_deref() {
            Some(url_str) => build_protection_domain(thread, url_str).await?,
            None => Value::Object(None),
        };
        let primitive = Value::from(class.is_primitive());
        Ok((
            "Ljava/lang/ClassLoader;Ljava/lang/Class;CLjava/security/ProtectionDomain;Z",
            vec![
                class_loader_object,
                component_type_object,
                modifiers,
                protection_domain,
                primitive,
            ],
            module,
        ))
    }
}

/// Get the component type object for an array class.
///
/// # Errors
///
/// An error will be returned if the component type cannot be converted to a Class object.
async fn get_component_type_object(thread: &Thread, class: &Arc<Class>) -> Result<Value> {
    if let Some(component_type) = class.component_type() {
        let component_type_class = thread.class(component_type).await?;
        Box::pin(to_class_object(thread, &component_type_class)).await
    } else {
        Ok(Value::Object(None))
    }
}

impl JavaObject for Arc<Class> {
    async fn to_object(&self, thread: &Thread) -> Result<Value> {
        let class_object = to_class_object(thread, self).await?;

        let vm = thread.vm()?;
        if *vm.java_class_file_version() > JAVA_8 && self.is_array() {
            let Some(component_type) = self.component_type() else {
                return Err(InternalError(
                    "array class missing component type".to_string(),
                ));
            };
            let component_type_class = thread.class(component_type).await?;
            let component_type_object = to_class_object(thread, &component_type_class).await?;
            {
                let mut object = class_object.as_object_mut()?;
                object.set_value("componentType", component_type_object)?;
            }
        }

        Ok(class_object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bool_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = true;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_bool()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_char_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = '*';
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_char()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_i8_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42i8;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_i8()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_u8_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42u8;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_u8()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_i16_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42i16;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_i16()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_u16_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42u16;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_u16()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_i32_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42i32;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_i32()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_u32_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42u32;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_u32()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_i64_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42i64;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_i64()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_u64_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42u64;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_u64()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_isize_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42isize;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_isize()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_usize_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42usize;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_usize()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_f32_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42.1f32;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_f32()?;
        let value = value - original_value;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_f64_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = 42.1f64;
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_f64()?;
        let value = value - original_value;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_str_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = "foo";
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_string()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_string_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = "foo".to_string();
        let value: Value = original_value.to_object(&thread).await?;
        let value = value.as_string()?;
        assert_eq!(original_value, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_class_to_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let original_value = thread.class("[I").await?;
        let value: Value = original_value.to_object(&thread).await?;
        let object = value.as_object_ref()?;
        let class = object.class();
        assert_eq!("java/lang/Class", class.name());
        Ok(())
    }
}
