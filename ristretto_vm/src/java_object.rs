use crate::Error::InternalError;
use crate::Result;
use crate::thread::Thread;
use ristretto_classfile::{JAVA_8, JAVA_17, JAVA_25};
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
        #[expect(clippy::cast_possible_wrap)]
        let value = *self as i8;
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
        #[expect(clippy::cast_possible_wrap)]
        let value = *self as i16;
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
        #[expect(clippy::cast_possible_wrap)]
        let value = *self as i32;
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
        #[expect(clippy::cast_possible_wrap)]
        let value = *self as i64;
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
        let java_class_file_version = vm.java_class_file_version();
        let array = if java_class_file_version <= &JAVA_8 {
            // Java 8 and below: store as UTF-16 char array
            let chars = self.encode_utf16().collect::<Vec<u16>>();
            Value::from(Reference::CharArray(chars.into()))
        } else {
            if java_class_file_version >= &JAVA_17 {
                object.set_value("hashIsZero", Value::Int(0))?;
            }

            // Determine coder and value
            let use_latin1 = self.chars().all(|c| (c as u32) <= 0xFF);
            let (coder, bytes): (i32, Vec<i8>) = if use_latin1 {
                // All chars fit in Latin1
                (0, self.chars().map(|c| c as i8).collect())
            } else {
                #[expect(clippy::cast_possible_wrap)]
                // Must use UTF-16
                (
                    1,
                    self.encode_utf16()
                        .flat_map(u16::to_ne_bytes)
                        .map(|b| b as i8)
                        .collect(),
                )
            };

            object.set_value("coder", Value::Int(coder))?;
            Value::from(bytes)
        };

        object.set_value("value", array)?;
        object.set_value("hash", Value::Int(0))?;

        let value = Value::from(object);
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
        let builtin_class_loader = thread
            .object(
                "jdk.internal.loader.BuiltinClassLoader",
                "Ljava/lang/String;Ljdk/internal/loader/BuiltinClassLoader;Ljdk/internal/loader/URLClassPath;",
                &[name, parent_class_loader, url_class_path],
            )
            .await?;
        class_loader
            .set_object(Some(builtin_class_loader.clone()))
            .await;
        builtin_class_loader
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
        class,
        java_version,
        class_loader_object,
        component_type_object,
        module,
    );

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

    class.set_object(Some(object_value.clone()))?;
    Ok(object_value)
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
        return Ok(());
    }

    if let Some(class_loader) = class.class_loader()? {
        let class_loader_object = Box::pin(to_class_loader_object(thread, &class_loader)).await?;
        if !class_loader_object.is_null() {
            let module = thread
                .try_invoke(
                    "java.lang.ClassLoader",
                    "getUnnamedModule()Ljava/lang/Module;",
                    std::slice::from_ref(&class_loader_object),
                )
                .await?;
            if !module.is_null() {
                let mut object_mut = object.as_object_mut()?;
                object_mut.set_value_unchecked("module", module)?;
            }
        }
    } else {
        // For bootstrap classes, try named module first, then unnamed
        let package = ClassLoader::package_from_class_name(class.name());
        let module = vm
            .module_system()
            .get_module_for_package(package)
            .or_else(|| vm.module_system().boot_unnamed_module());
        if let Some(module) = module {
            let mut object_mut = object.as_object_mut()?;
            object_mut.set_value_unchecked("module", module)?;
        }
    }

    Ok(())
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
        let module = vm
            .module_system()
            .get_module_for_package(package)
            .or_else(|| vm.module_system().boot_unnamed_module())
            .unwrap_or(Value::Object(None));
        return Ok(module);
    }

    thread
        .try_invoke(
            "java.lang.ClassLoader",
            "getUnnamedModule()Ljava/lang/Module;",
            std::slice::from_ref(class_loader_object),
        )
        .await
}

/// Build the constructor descriptor and parameters for creating a Class object based on Java version.
///
/// # Returns
///
/// A tuple containing the constructor descriptor, parameters, and module value.
fn build_class_constructor_params(
    class: &Arc<Class>,
    java_version: &ristretto_classfile::Version,
    class_loader_object: Value,
    component_type_object: Value,
    module: Value,
) -> (&'static str, Vec<Value>, Value) {
    if *java_version <= JAVA_8 {
        (
            "Ljava/lang/ClassLoader;",
            vec![class_loader_object],
            Value::Object(None),
        )
    } else if *java_version < JAVA_25 {
        (
            "Ljava/lang/ClassLoader;Ljava/lang/Class;",
            vec![class_loader_object, component_type_object],
            module,
        )
    } else {
        let modifiers = Value::from(class.class_file().access_flags.bits());
        let protected_domain = Value::Object(None);
        let primitive = Value::from(class.is_primitive());
        (
            "Ljava/lang/ClassLoader;Ljava/lang/Class;CLjava/security/ProtectionDomain;Z",
            vec![
                class_loader_object,
                component_type_object,
                modifiers,
                protected_domain,
                primitive,
            ],
            module,
        )
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
