use crate::Error::InternalError;
use crate::Result;
use crate::Thread;
use crate::VM;
use crate::module_access::ModuleAccess;
use ristretto_classfile::{JAVA_8, JAVA_17, JAVA_25};
use ristretto_classloader::{Class, ClassLoader, Object, Reference, Value};
use std::sync::Arc;

/// Trait for converting a Rust value to a Java object. Converts to objects of the primitive
/// wrapper, classes, and strings.
pub trait JavaObject<T: Thread> {
    /// Convert this value to a Java object.
    ///
    /// # Errors
    /// Returns an error if the conversion fails.
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>>;
}

impl<T: Thread> JavaObject<T> for bool {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = Value::from(*self);
        Box::pin(async move {
            let result = thread
                .try_invoke(
                    "java.lang.Boolean",
                    "valueOf(Z)Ljava/lang/Boolean;",
                    &[value],
                )
                .await?;
            Ok(result)
        })
    }
}

impl<T: Thread> JavaObject<T> for char {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = Value::from(*self);
        Box::pin(async move {
            let result = thread
                .try_invoke(
                    "java.lang.Character",
                    "valueOf(C)Ljava/lang/Character;",
                    &[value],
                )
                .await?;
            Ok(result)
        })
    }
}

impl<T: Thread> JavaObject<T> for i8 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = Value::from(*self);
        Box::pin(async move {
            let result = thread
                .try_invoke("java.lang.Byte", "valueOf(B)Ljava/lang/Byte;", &[value])
                .await?;
            Ok(result)
        })
    }
}

impl<T: Thread> JavaObject<T> for u8 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value: i8 = zerocopy::transmute!(*self);
        Box::pin(async move { value.to_object(thread).await })
    }
}

impl<T: Thread> JavaObject<T> for i16 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = Value::from(*self);
        Box::pin(async move {
            let result = thread
                .try_invoke("java.lang.Short", "valueOf(S)Ljava/lang/Short;", &[value])
                .await?;
            Ok(result)
        })
    }
}

impl<T: Thread> JavaObject<T> for u16 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value: i16 = zerocopy::transmute!(*self);
        Box::pin(async move { value.to_object(thread).await })
    }
}

impl<T: Thread> JavaObject<T> for i32 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = Value::from(*self);
        Box::pin(async move {
            let result = thread
                .try_invoke(
                    "java.lang.Integer",
                    "valueOf(I)Ljava/lang/Integer;",
                    &[value],
                )
                .await?;
            Ok(result)
        })
    }
}

impl<T: Thread> JavaObject<T> for u32 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value: i32 = zerocopy::transmute!(*self);
        Box::pin(async move { value.to_object(thread).await })
    }
}

impl<T: Thread> JavaObject<T> for i64 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = Value::from(*self);
        Box::pin(async move {
            let result = thread
                .try_invoke("java.lang.Long", "valueOf(J)Ljava/lang/Long;", &[value])
                .await?;
            Ok(result)
        })
    }
}

impl<T: Thread> JavaObject<T> for u64 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value: i64 = zerocopy::transmute!(*self);
        Box::pin(async move { value.to_object(thread).await })
    }
}

impl<T: Thread> JavaObject<T> for isize {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = *self as i64;
        Box::pin(async move { value.to_object(thread).await })
    }
}

impl<T: Thread> JavaObject<T> for usize {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = *self as u64;
        Box::pin(async move { value.to_object(thread).await })
    }
}

impl<T: Thread> JavaObject<T> for f32 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = Value::from(*self);
        Box::pin(async move {
            let result = thread
                .try_invoke("java.lang.Float", "valueOf(F)Ljava/lang/Float;", &[value])
                .await?;
            Ok(result)
        })
    }
}

impl<T: Thread> JavaObject<T> for f64 {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let value = Value::from(*self);
        Box::pin(async move {
            let result = thread
                .try_invoke("java.lang.Double", "valueOf(D)Ljava/lang/Double;", &[value])
                .await?;
            Ok(result)
        })
    }
}

impl<T: Thread> JavaObject<T> for &str {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let s = self.to_string();
        Box::pin(async move {
            let class = thread.class("java.lang.String").await?;
            let mut object = Object::new(class)?;

            let vm = thread.vm()?;
            let collector = vm.garbage_collector();
            let java_class_file_version = vm.java_class_file_version();
            let array = if java_class_file_version <= &JAVA_8 {
                let chars = s.encode_utf16().collect::<Vec<u16>>();
                Value::new_object(collector, Reference::CharArray(chars.into()))
            } else {
                if java_class_file_version >= &JAVA_17 {
                    object.set_value("hashIsZero", Value::Int(0))?;
                }

                let use_latin1 = s.chars().all(|c| (c as u32) <= 0xFF);
                let (coder, bytes): (i32, Vec<i8>) = if use_latin1 {
                    (0, s.chars().map(|c| c as i8).collect())
                } else {
                    let utf16_bytes: Vec<u8> =
                        s.encode_utf16().flat_map(u16::to_ne_bytes).collect();
                    let signed_bytes: &[i8] = zerocopy::transmute_ref!(utf16_bytes.as_slice());
                    (1, signed_bytes.to_vec())
                };

                object.set_value("coder", Value::Int(coder))?;
                Value::new_object(collector, Reference::from(bytes))
            };

            object.set_value("value", array)?;
            object.set_value("hash", Value::Int(0))?;

            let value = Value::from_object(collector, object);
            Ok(value)
        })
    }
}

impl<T: Thread> JavaObject<T> for String {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let s = self.clone();
        Box::pin(async move { s.as_str().to_object(thread).await })
    }
}

impl<T: Thread> JavaObject<T> for Value {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let val = self.clone();
        Box::pin(async move {
            match &val {
                Value::Int(value) => value.to_object(thread).await,
                Value::Long(value) => value.to_object(thread).await,
                Value::Float(value) => value.to_object(thread).await,
                Value::Double(value) => value.to_object(thread).await,
                Value::Object(_) => Ok(val),
                Value::Unused => Err(InternalError(
                    "Cannot convert unused value to object".to_string(),
                )),
            }
        })
    }
}

fn to_class_loader_object<'a, T: Thread + 'static>(
    thread: &'a T,
    class_loader: &'a Arc<ClassLoader>,
) -> crate::BoxFuture<'a, Result<Value>> {
    Box::pin(async move {
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
                Some(parent_class_loader) => {
                    to_class_loader_object(thread, &parent_class_loader).await?
                }
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
    })
}

impl<T: Thread + 'static> JavaObject<T> for Arc<ClassLoader> {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        Box::pin(async move { to_class_loader_object(thread, self).await })
    }
}

/// Convert a Class to a Class object.
fn to_class_object<'a, T: Thread + 'static>(
    thread: &'a T,
    class: &'a Arc<Class>,
) -> crate::BoxFuture<'a, Result<Value>> {
    Box::pin(async move {
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
            Some(class_loader) => to_class_loader_object(thread, &class_loader).await?,
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
    })
}

/// Update the module field on a cached Class object for Java 9+.
fn update_cached_class_module<'a, T: Thread + 'static>(
    thread: &'a T,
    class: &'a Arc<Class>,
    object: &'a Value,
) -> crate::BoxFuture<'a, Result<()>> {
    Box::pin(async move {
        let vm = thread.vm()?;
        let current_module = object.as_object_ref()?.value("module")?;
        if !current_module.is_null() {
            return Ok(());
        }

        if let Some(class_loader) = class.class_loader()? {
            let class_loader_object = to_class_loader_object(thread, &class_loader).await?;
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
            // For bootstrap classes, try named module first, then unnamed
            let package = ClassLoader::package_from_class_name(class.name());

            // First, try to get the module from the defined modules (from defineModule0)
            if let Some(module) = vm.module_system().get_module_for_package(package) {
                let mut object_mut = object.as_object_mut()?;
                object_mut.set_value_unchecked("module", module)?;
                return Ok(());
            }

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
    })
}

/// Create an unnamed module for the boot class loader.
fn create_boot_unnamed_module<T: Thread + 'static>(
    thread: &T,
) -> crate::BoxFuture<'_, Result<Value>> {
    Box::pin(async move {
        let module_class = thread.class("java/lang/Module").await?;
        let module_object = Object::new(module_class)?;

        let vm = thread.vm()?;
        Ok(Value::from_object(vm.garbage_collector(), module_object))
    })
}

/// Get the module for a class based on Java version and class loader.
fn get_class_module<'a, T: Thread + 'static>(
    thread: &'a T,
    class: &'a Arc<Class>,
    class_loader_object: &'a Value,
) -> crate::BoxFuture<'a, Result<Value>> {
    Box::pin(async move {
        let vm = thread.vm()?;
        let java_version = vm.java_class_file_version();

        if *java_version <= JAVA_8 {
            return Ok(Value::Object(None));
        }

        if class_loader_object.is_null() {
            let package = ClassLoader::package_from_class_name(class.name());

            if let Some(module) = vm.module_system().get_module_for_package(package) {
                return Ok(module);
            }

            if let Some(module) = vm.module_system().boot_unnamed_module() {
                return Ok(module);
            }

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

        if module.is_null() {
            let module = create_unnamed_module(thread, class_loader_object).await?;
            if !module.is_null() {
                let mut loader_obj = class_loader_object.as_object_mut()?;
                let _ = loader_obj.set_value_unchecked("unnamedModule", module.clone());
            }
            return Ok(module);
        }

        Ok(module)
    })
}

/// Create an unnamed module for the given class loader.
fn create_unnamed_module<'a, T: Thread + 'static>(
    thread: &'a T,
    class_loader_object: &'a Value,
) -> crate::BoxFuture<'a, Result<Value>> {
    Box::pin(async move {
        let module_class = thread.class("java/lang/Module").await?;
        let mut module_object = Object::new(module_class)?;
        module_object.set_value_unchecked("loader", class_loader_object.clone())?;

        let vm = thread.vm()?;
        Ok(Value::from_object(vm.garbage_collector(), module_object))
    })
}

/// Build the constructor descriptor and parameters for creating a Class object.
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
fn get_component_type_object<'a, T: Thread + 'static>(
    thread: &'a T,
    class: &'a Arc<Class>,
) -> crate::BoxFuture<'a, Result<Value>> {
    Box::pin(async move {
        if let Some(component_type) = class.component_type() {
            let component_type_class = thread.class(component_type).await?;
            to_class_object(thread, &component_type_class).await
        } else {
            Ok(Value::Object(None))
        }
    })
}

impl<T: Thread + 'static> JavaObject<T> for Arc<Class> {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        let class = self.clone();
        Box::pin(async move {
            let class_object = to_class_object(thread, &class).await?;

            let vm = thread.vm()?;
            if *vm.java_class_file_version() > JAVA_8 && class.is_array() {
                let Some(component_type) = class.component_type() else {
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
        })
    }
}
