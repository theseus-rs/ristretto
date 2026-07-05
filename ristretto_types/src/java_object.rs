use crate::Error::InternalError;
use crate::Result;
use crate::Thread;
use crate::VM;
use crate::module_access::ModuleAccess;
use ristretto_classfile::{JAVA_8, JAVA_17, JAVA_25, JavaStr};
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
        Box::pin(async move {
            let s: &str = self;
            let class = thread.class("java.lang.String").await?;
            let mut object = Object::new(class)?;

            let vm = thread.vm()?;
            let collector = vm.garbage_collector();
            let java_class_file_version = vm.java_class_file_version();
            let array = if java_class_file_version <= &JAVA_8 {
                let chars = s.encode_utf16().collect::<Vec<u16>>();
                Value::new_object(collector, Reference::CharArray(chars.into()))
            } else {
                if java_class_file_version >= &JAVA_17
                    && let Err(error) = object.set_value("hashIsZero", Value::Int(0))
                {
                    return Err(error.into());
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
        Box::pin(async move { self.as_str().to_object(thread).await })
    }
}

impl<T: Thread> JavaObject<T> for &JavaStr {
    fn to_object<'a>(&'a self, thread: &'a T) -> crate::BoxFuture<'a, Result<Value>> {
        // Fast path: if the MUTF-8 bytes are also valid UTF-8, delegate to &str (zero-copy)
        if let Some(s) = self.as_str() {
            return Box::pin(async move { s.to_object(thread).await });
        }

        // Slow path: decode MUTF-8 -> UTF-16 directly, preserving lone surrogates
        let utf16 = self.to_utf16().unwrap_or_default();

        Box::pin(async move {
            let class = thread.class("java.lang.String").await?;
            let mut object = Object::new(class)?;

            let vm = thread.vm()?;
            let collector = vm.garbage_collector();
            let java_class_file_version = vm.java_class_file_version();
            let array = if java_class_file_version <= &JAVA_8 {
                Value::new_object(collector, Reference::CharArray(utf16.into()))
            } else {
                if java_class_file_version >= &JAVA_17
                    && let Err(error) = object.set_value("hashIsZero", Value::Int(0))
                {
                    return Err(error.into());
                }

                let use_latin1 = utf16.iter().all(|&c| c <= 0xFF);
                #[expect(clippy::cast_possible_truncation)]
                let (coder, bytes): (i32, Vec<i8>) = if use_latin1 {
                    (0, utf16.iter().map(|&c| c as i8).collect())
                } else {
                    let utf16_bytes: Vec<u8> = utf16.iter().flat_map(|c| c.to_ne_bytes()).collect();
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
                    let mut loader_obj = class_loader_object.as_object_mut()?;
                    let _ = loader_obj.set_value_unchecked("unnamedModule", module.clone());
                }

                let mut object_mut = object.as_object_mut()?;
                object_mut.set_value_unchecked("module", module)?;
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
            vm.module_system().set_boot_unnamed_module(module.clone());
            let mut object_mut = object.as_object_mut()?;
            object_mut.set_value_unchecked("module", module)?;
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
            let mut loader_obj = class_loader_object.as_object_mut()?;
            let _ = loader_obj.set_value_unchecked("unnamedModule", module.clone());
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

/// Build a `ProtectionDomain` Java object from a code source URL string.
fn build_protection_domain<'a, T: Thread + 'static>(
    thread: &'a T,
    url_str: &'a str,
) -> crate::BoxFuture<'a, Result<Value>> {
    Box::pin(async move {
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
    })
}

/// Build the constructor descriptor and parameters for creating a Class object.
fn build_class_constructor_params<'a, T: Thread + 'static>(
    thread: &'a T,
    class: &'a Arc<Class>,
    java_version: &'a ristretto_classfile::Version,
    class_loader_object: Value,
    component_type_object: Value,
    module: Value,
) -> crate::BoxFuture<'a, Result<(&'static str, Vec<Value>, Value)>> {
    Box::pin(async move {
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
    })
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
                let component_type = class.component_type().unwrap_or_default();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17, JAVA_25, JavaString};
    use ristretto_classloader::ClassPath;

    fn class_with_loader(name: &str, class_loader: &Arc<ClassLoader>) -> Result<Arc<Class>> {
        let class_file = test_utils::class_file(name, &[], None)?;
        let class = Class::from(Some(Arc::downgrade(class_loader)), class_file)?;
        Ok(class)
    }

    fn class_object_with_null_module<T: Thread + 'static>(
        thread: &T,
    ) -> crate::BoxFuture<'_, Result<Value>> {
        Box::pin(async move {
            thread
                .object("java.lang.Class", "Ljava/lang/ClassLoader;", &[])
                .await
        })
    }

    #[tokio::test]
    async fn test_primitive_and_value_to_object() -> Result<()> {
        let vm = test_utils::MockVm::new(JAVA_17);
        let thread = test_utils::MockThread::new(vm);

        assert_eq!(true.to_object(&*thread).await?, Value::Int(1));
        assert_eq!('*'.to_object(&*thread).await?, Value::Int(42));
        assert_eq!(42i8.to_object(&*thread).await?, Value::Int(42));
        assert_eq!(42u8.to_object(&*thread).await?, Value::Int(42));
        assert_eq!(42i16.to_object(&*thread).await?, Value::Int(42));
        assert_eq!(42u16.to_object(&*thread).await?, Value::Int(42));
        assert_eq!(42i32.to_object(&*thread).await?, Value::Int(42));
        assert_eq!(42u32.to_object(&*thread).await?, Value::Int(42));
        assert_eq!(42i64.to_object(&*thread).await?, Value::Long(42));
        assert_eq!(42u64.to_object(&*thread).await?, Value::Long(42));
        assert_eq!(42isize.to_object(&*thread).await?, Value::Long(42));
        assert_eq!(42usize.to_object(&*thread).await?, Value::Long(42));
        assert_eq!(42.5f32.to_object(&*thread).await?, Value::Float(42.5));
        assert_eq!(42.5f64.to_object(&*thread).await?, Value::Double(42.5));

        assert_eq!(Value::Int(1).to_object(&*thread).await?, Value::Int(1));
        assert_eq!(Value::Long(2).to_object(&*thread).await?, Value::Long(2));
        assert_eq!(
            Value::Float(3.0).to_object(&*thread).await?,
            Value::Float(3.0)
        );
        assert_eq!(
            Value::Double(4.0).to_object(&*thread).await?,
            Value::Double(4.0)
        );
        assert_eq!(
            Value::Object(None).to_object(&*thread).await?,
            Value::Object(None)
        );
        assert!(Value::Unused.to_object(&*thread).await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_string_and_java_str_to_object_versions() -> Result<()> {
        let java8_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_8));
        assert!("latin".to_object(&*java8_thread).await?.is_object());

        let java17_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_17));
        assert!("latin".to_object(&*java17_thread).await?.is_object());
        assert!("\u{0100}".to_object(&*java17_thread).await?.is_object());
        assert!(
            "owned"
                .to_string()
                .to_object(&*java17_thread)
                .await?
                .is_object()
        );

        let java11_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_11));
        assert!("latin".to_object(&*java11_thread).await?.is_object());

        let fast_java_str = JavaStr::try_from_str("fast")?;
        assert!(fast_java_str.to_object(&*java17_thread).await?.is_object());

        let slow_latin1 = JavaString::from("a\u{0}b");
        assert!(
            slow_latin1
                .as_java_str()
                .to_object(&*java17_thread)
                .await?
                .is_object()
        );

        let slow_utf16 = JavaString::from("\u{1f600}");
        assert!(
            slow_utf16
                .as_java_str()
                .to_object(&*java17_thread)
                .await?
                .is_object()
        );
        assert!(
            slow_utf16
                .as_java_str()
                .to_object(&*java8_thread)
                .await?
                .is_object()
        );
        assert!(
            slow_latin1
                .as_java_str()
                .to_object(&*java11_thread)
                .await?
                .is_object()
        );

        let bad_vm = test_utils::MockVm::new(JAVA_17);
        let bad_thread = test_utils::MockThread::new(bad_vm);
        let bad_string_class = test_utils::class(
            "java/lang/String",
            &[("value", "[B"), ("hash", "I"), ("coder", "I")],
        )
        .expect("bad string class");
        bad_thread.register_class(bad_string_class).await?;
        assert!("bad".to_object(&*bad_thread).await.is_err());
        assert!(
            slow_latin1
                .as_java_str()
                .to_object(&*bad_thread)
                .await
                .is_err()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_class_loader_to_object_paths() -> Result<()> {
        let java17_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_17));
        let cached_loader = ClassLoader::new("cached", ClassPath::new(Vec::new()));
        cached_loader.set_object(Some(Value::Int(99))).await;
        assert_eq!(
            cached_loader.to_object(&*java17_thread).await?,
            Value::Int(99)
        );

        let bootstrap_loader = ClassLoader::new("bootstrap", ClassPath::new(Vec::new()));
        assert_eq!(
            bootstrap_loader.to_object(&*java17_thread).await?,
            Value::Object(None)
        );

        let java8_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_8));
        let java8_loader = ClassLoader::new("app", ClassPath::new(Vec::new()));
        assert_eq!(
            java8_loader.to_object(&*java8_thread).await?,
            Value::Object(None)
        );

        let parent_loader = ClassLoader::new("parent", ClassPath::new(Vec::new()));
        let child_loader = ClassLoader::new("child", ClassPath::new(Vec::new()));
        child_loader.set_parent(Some(parent_loader)).await;
        assert!(child_loader.to_object(&*java17_thread).await?.is_object());

        let no_parent_loader = ClassLoader::new("standalone", ClassPath::new(Vec::new()));
        assert!(
            no_parent_loader
                .to_object(&*java17_thread)
                .await?
                .is_object()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_class_to_object_constructor_versions_and_modules() -> Result<()> {
        let java8_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_8));
        let java8_class = test_utils::class("example/Java8Class", &[])?;
        assert!(java8_class.to_object(&*java8_thread).await?.is_object());

        let java17_vm = test_utils::MockVm::new(JAVA_17);
        let java17_thread = test_utils::MockThread::new(java17_vm.clone());
        let package_module = java17_vm.object_value("java/lang/Module")?;
        java17_vm
            .module_system()
            .set_module_for_package("example", package_module);
        let mapped_module_class = test_utils::class("example/MappedModuleClass", &[])?;
        assert!(
            mapped_module_class
                .to_object(&*java17_thread)
                .await?
                .is_object()
        );

        let boot_module = java17_vm.object_value("java/lang/Module")?;
        java17_vm
            .module_system()
            .set_boot_unnamed_module(boot_module);
        let boot_module_class = test_utils::class("other/BootModuleClass", &[])?;
        assert!(
            boot_module_class
                .to_object(&*java17_thread)
                .await?
                .is_object()
        );

        let fresh_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_17));
        let creates_boot_module_class = test_utils::class("fresh/BootModuleClass", &[])?;
        assert!(
            creates_boot_module_class
                .to_object(&*fresh_thread)
                .await?
                .is_object()
        );

        let loader = ClassLoader::new("loader", ClassPath::new(Vec::new()));
        let loaded_class = class_with_loader("loaded/WithLoader", &loader)?;
        assert!(loaded_class.to_object(&*java17_thread).await?.is_object());
        let second_loaded_class = class_with_loader("loaded/SecondWithLoader", &loader)?;
        assert!(
            second_loaded_class
                .to_object(&*java17_thread)
                .await?
                .is_object()
        );

        let java25_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_25));
        let with_code_source = test_utils::class_with_code_source(
            "example/WithCodeSource",
            &[],
            "file:/tmp/example.jar",
        )
        .expect("class with code source");
        assert!(
            with_code_source
                .to_object(&*java25_thread)
                .await?
                .is_object()
        );
        let without_code_source = test_utils::class("example/WithoutCodeSource", &[])?;
        assert!(
            without_code_source
                .to_object(&*java25_thread)
                .await?
                .is_object()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_cached_class_module_updates() -> Result<()> {
        let java8_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_8));
        let java8_class = test_utils::class("cached/Java8", &[])?;
        java8_class.set_object(Some(class_object_with_null_module(&*java8_thread).await?))?;
        assert!(java8_class.to_object(&*java8_thread).await?.is_object());

        let loader_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_17));
        let loader = ClassLoader::new("loader", ClassPath::new(Vec::new()));
        let loaded_class = class_with_loader("cached/Loaded", &loader)?;
        let cached_object = class_object_with_null_module(&*loader_thread).await?;
        loaded_class.set_object(Some(cached_object))?;
        assert!(loaded_class.to_object(&*loader_thread).await?.is_object());
        let second_loaded_class = class_with_loader("cached/SecondLoaded", &loader)?;
        let second_cached_object = class_object_with_null_module(&*loader_thread).await?;
        second_loaded_class.set_object(Some(second_cached_object))?;
        assert!(
            second_loaded_class
                .to_object(&*loader_thread)
                .await?
                .is_object()
        );
        let bootstrap_loader = ClassLoader::new("bootstrap", ClassPath::new(Vec::new()));
        let bootstrap_loaded_class =
            class_with_loader("cached/BootstrapLoaded", &bootstrap_loader)?;
        let bootstrap_cached_object = class_object_with_null_module(&*loader_thread).await?;
        bootstrap_loaded_class.set_object(Some(bootstrap_cached_object))?;
        assert!(
            bootstrap_loaded_class
                .to_object(&*loader_thread)
                .await?
                .is_object()
        );

        let bad_cached_class = test_utils::class("bad/Cached", &[])?;
        bad_cached_class.set_object(Some(Value::Int(0)))?;
        assert!(bad_cached_class.to_object(&*loader_thread).await.is_err());

        let mapped_vm = test_utils::MockVm::new(JAVA_17);
        let mapped_thread = test_utils::MockThread::new(mapped_vm.clone());
        let module = mapped_vm.object_value("java/lang/Module")?;
        mapped_vm
            .module_system()
            .set_module_for_package("cached", module);
        let mapped_class = test_utils::class("cached/Mapped", &[])?;
        mapped_class.set_object(Some(class_object_with_null_module(&*mapped_thread).await?))?;
        assert!(mapped_class.to_object(&*mapped_thread).await?.is_object());

        let boot_vm = test_utils::MockVm::new(JAVA_17);
        let boot_thread = test_utils::MockThread::new(boot_vm.clone());
        let boot_module = boot_vm.object_value("java/lang/Module")?;
        boot_vm.module_system().set_boot_unnamed_module(boot_module);
        let boot_class = test_utils::class("boot/Cached", &[])?;
        boot_class.set_object(Some(class_object_with_null_module(&*boot_thread).await?))?;
        assert!(boot_class.to_object(&*boot_thread).await?.is_object());

        let fresh_thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_17));
        let fresh_class = test_utils::class("fresh/Cached", &[])?;
        fresh_class.set_object(Some(class_object_with_null_module(&*fresh_thread).await?))?;
        assert!(fresh_class.to_object(&*fresh_thread).await?.is_object());

        let already_set_class = test_utils::class("already/Set", &[])?;
        let already_set_object = class_object_with_null_module(&*fresh_thread).await?;
        {
            let mut object = already_set_object.as_object_mut()?;
            let module = fresh_thread
                .vm()?
                .object("java/lang/Module", "()V", &[])
                .await?;
            object.set_value_unchecked("module", module)?;
        }
        already_set_class.set_object(Some(already_set_object))?;
        assert!(
            already_set_class
                .to_object(&*fresh_thread)
                .await?
                .is_object()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_array_class_to_object_sets_component_type() -> Result<()> {
        let thread = test_utils::MockThread::new(test_utils::MockVm::new(JAVA_17));
        let array_class = test_utils::class("[I", &[])?;
        assert!(array_class.to_object(&*thread).await?.is_object());
        Ok(())
    }
}
