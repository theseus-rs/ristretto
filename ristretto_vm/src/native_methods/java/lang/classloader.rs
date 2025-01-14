use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::JavaError::{ClassFormatError, IndexOutOfBoundsException, NoClassDefFoundError};
use crate::{Result, VM};
use async_recursion::async_recursion;
use ristretto_classfile::{ClassFile, Version};
use ristretto_classloader::{Class, Object, Reference, Value};
use std::io::Cursor;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `java.lang.ClassLoader`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ClassLoader";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_8 {
        registry.register(
            class_name,
            "defineClass0",
            "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;)Ljava/lang/Class;",
            define_class_0,
        );
        registry.register(class_name, "defineClass1", "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_1);
        registry.register(class_name, "defineClass2", "(Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_2);
        registry.register(
            class_name,
            "resolveClass0",
            "(Ljava/lang/Class;)V",
            resolve_class_0,
        );
    } else {
        registry.register(class_name, "defineClass1", "(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_1);
        registry.register(class_name, "defineClass2", "(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_2);
    }

    if java_version <= JAVA_11 {
        registry.register(
            class_name,
            "findBuiltinLib",
            "(Ljava/lang/String;)Ljava/lang/String;",
            find_builtin_lib,
        );
    }

    registry.register(
        class_name,
        "findBootstrapClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        find_bootstrap_class,
    );
    registry.register(
        class_name,
        "findLoadedClass0",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        find_loaded_class_0,
    );
    registry.register(
        class_name,
        "initSystemClassLoader",
        "()Ljava/lang/ClassLoader;",
        init_system_class_loader,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "retrieveDirectives",
        "()Ljava/lang/AssertionStatusDirectives;",
        retrieve_directives,
    );
}

/// Create a `java.lang.Class` object from a byte array.
/// This method is used by the `defineClass0`, `defineClass1`, and `defineClass2` native methods.
/// The `defineClass0` method is used by Java 8 and earlier versions.
async fn class_object_from_bytes(
    vm: &Arc<VM>,
    source_file: Option<Reference>,
    bytes: &[u8],
    offset: i32,
    length: i32,
) -> Result<Object> {
    let bytes_length = i32::try_from(bytes.len())?;
    if offset < 0 || length < 0 || offset + length > bytes_length {
        return Err(IndexOutOfBoundsException {
            index: offset,
            size: bytes_length,
        }
        .into());
    }
    let offset = usize::try_from(offset)?;
    let length = usize::try_from(length)?;
    let bytes = bytes[offset..offset + length].to_vec();
    let mut bytes = Cursor::new(bytes);
    let class_file = match ClassFile::from_bytes(&mut bytes) {
        Ok(class_file) => class_file,
        Err(error) => {
            return Err(ClassFormatError(error.to_string()).into());
        }
    };
    if let Err(error) = class_file.verify() {
        return Err(ClassFormatError(error.to_string()).into());
    }

    if let Some(source_file) = source_file {
        let _source_file: String = source_file.try_into()?;
        // TODO: implement setting the source file
    }

    let class = Arc::new(Class::from(class_file)?);
    let class = class.to_object(vm).await?;
    let class: Object = class.try_into()?;
    Ok(class)
}

#[async_recursion(?Send)]
async fn define_class_0(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _protection_domain = arguments.pop_object()?;
    let length = arguments.pop_int()?;
    let offset = arguments.pop_int()?;
    let bytes: Vec<u8> = arguments.pop()?.try_into()?;
    let vm = thread.vm()?;
    let class = class_object_from_bytes(&vm, None, &bytes, offset, length).await?;

    if vm.java_class_file_version() <= &JAVA_8 {
        if let Some(expected_class_name) = arguments.pop_reference()? {
            let expected_class_name: String = expected_class_name.try_into()?;
            let class_name = class.class().name();
            if class_name != expected_class_name {
                return Err(NoClassDefFoundError(class_name.to_string()).into());
            }
        }
    } else {
        let class_loader = arguments.pop()?;
        class.set_value("classLoader", class_loader)?;
    }

    Ok(Some(Value::from(class)))
}

#[async_recursion(?Send)]
async fn define_class_1(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let source_file = arguments.pop_reference()?;
    let _protection_domain = arguments.pop_object()?;
    let length = arguments.pop_int()?;
    let offset = arguments.pop_int()?;
    let bytes: Vec<u8> = arguments.pop()?.try_into()?;
    let vm = thread.vm()?;
    let class = class_object_from_bytes(&vm, source_file, &bytes, offset, length).await?;

    if vm.java_class_file_version() <= &JAVA_8 {
        if let Some(expected_class_name) = arguments.pop_reference()? {
            let expected_class_name: String = expected_class_name.try_into()?;
            let class_name = class.class().name();
            if class_name != expected_class_name {
                return Err(NoClassDefFoundError(class_name.to_string()).into());
            }
        }
    } else {
        let class_loader = arguments.pop()?;
        class.set_value("classLoader", class_loader)?;
    }

    Ok(Some(Value::from(class)))
}

#[async_recursion(?Send)]
async fn define_class_2(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let source_file = arguments.pop_reference()?;
    let _protection_domain = arguments.pop_object()?;
    let length = arguments.pop_int()?;
    let offset = arguments.pop_int()?;
    let byte_buffer = arguments.pop_object()?;
    let buffer: Vec<u8> = byte_buffer.value("hb")?.try_into()?;
    let buffer_offset = byte_buffer.value("offset")?.try_into()?;
    let bytes: Vec<u8> = buffer.into_iter().skip(buffer_offset).collect();
    let vm = thread.vm()?;
    let class = class_object_from_bytes(&vm, source_file, &bytes, offset, length).await?;

    if vm.java_class_file_version() <= &JAVA_8 {
        if let Some(expected_class_name) = arguments.pop_reference()? {
            let expected_class_name: String = expected_class_name.try_into()?;
            let class_name = class.class().name();
            if class_name != expected_class_name {
                return Err(NoClassDefFoundError(class_name.to_string()).into());
            }
        }
    } else {
        let class_loader = arguments.pop()?;
        class.set_value("classLoader", class_loader)?;
    }

    Ok(Some(Value::from(class)))
}

#[async_recursion(?Send)]
async fn find_bootstrap_class(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let class_name: String = arguments.pop()?.try_into()?;
    let vm = thread.vm()?;
    let Ok(class) = vm.class(class_name).await else {
        return Ok(Some(Value::Object(None)));
    };
    let class = class.to_object(&vm).await?;
    Ok(Some(class))
}

#[async_recursion(?Send)]
async fn find_builtin_lib(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // Ristretto has not built-in libraries; all native methods are implemented in Rust
    Ok(Some(Value::Object(None)))
}

#[async_recursion(?Send)]
async fn find_loaded_class_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let class_name: String = arguments.pop()?.try_into()?;
    let vm = thread.vm()?;
    let Ok(class) = vm.class(class_name).await else {
        return Ok(Some(Value::Object(None)));
    };
    let class = class.to_object(&vm).await?;
    Ok(Some(class))
}

#[async_recursion(?Send)]
async fn init_system_class_loader(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    // Ristretto initializes the system class loader in the VM
    Ok(Some(Value::Object(None)))
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn resolve_class_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // Ristretto resolves classes when they are loaded
    Ok(None)
}

#[async_recursion(?Send)]
async fn retrieve_directives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: implement the `retrieveDirectives` native method
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java9 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/ClassLoader";
        assert!(registry
            .method(
                class_name,
                "defineClass1",
                "(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "defineClass2",
                "(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "findBuiltinLib",
                "(Ljava/lang/String;)Ljava/lang/String;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "findBootstrapClass",
                "(Ljava/lang/String;)Ljava/lang/Class;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "findLoadedClass0",
                "(Ljava/lang/String;)Ljava/lang/Class;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "initSystemClassLoader",
                "()Ljava/lang/ClassLoader;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "registerNatives", "()V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "retrieveDirectives",
                "()Ljava/lang/AssertionStatusDirectives;"
            )
            .is_some());
    }

    #[test]
    fn test_register_java_8() {
        let mut registry = MethodRegistry::new(&Version::Java8 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/lang/ClassLoader";
        assert!(registry
            .method(
                class_name,
                "defineClass0",
                "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;)Ljava/lang/Class;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "defineClass1",
                "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "defineClass2",
                "(Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "resolveClass0", "(Ljava/lang/Class;)V")
            .is_some());
    }

    #[tokio::test]
    async fn test_find_builtin_lib() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = find_builtin_lib(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_init_system_class_loader() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_system_class_loader(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_resolve_class_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = resolve_class_0(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_retrieve_directives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = retrieve_directives(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
