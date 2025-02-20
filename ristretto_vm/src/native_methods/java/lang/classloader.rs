use crate::JavaError::{ClassFormatError, IndexOutOfBoundsException, NoClassDefFoundError};
use crate::java_object::JavaObject;
use crate::native_methods::registry::{JAVA_8, JAVA_11, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::{Result, VM};
use async_recursion::async_recursion;
use ristretto_classfile::ClassFile;
use ristretto_classloader::{Class, Object, Reference, Value};
use std::io::Cursor;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/ClassLoader";

/// Register all native methods for `java.lang.ClassLoader`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "defineClass0",
            "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;)Ljava/lang/Class;",
            define_class_0,
        );
        registry.register(CLASS_NAME, "defineClass1", "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_1);
        registry.register(CLASS_NAME, "defineClass2", "(Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_2);
        registry.register(
            CLASS_NAME,
            "resolveClass0",
            "(Ljava/lang/Class;)V",
            resolve_class_0,
        );
    } else {
        registry.register(CLASS_NAME, "defineClass1", "(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_1);
        registry.register(CLASS_NAME, "defineClass2", "(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_2);
    }

    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "findBuiltinLib",
            "(Ljava/lang/String;)Ljava/lang/String;",
            find_builtin_lib,
        );
    }

    if registry.java_major_version() > JAVA_11 {
        registry.register(
            CLASS_NAME,
            "defineClass0",
            "(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BIILjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;",
            define_class_0,
        );
    }

    registry.register(
        CLASS_NAME,
        "findBootstrapClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        find_bootstrap_class,
    );
    registry.register(
        CLASS_NAME,
        "findLoadedClass0",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        find_loaded_class_0,
    );
    registry.register(
        CLASS_NAME,
        "initSystemClassLoader",
        "()Ljava/lang/ClassLoader;",
        init_system_class_loader,
    );
    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
    registry.register(
        CLASS_NAME,
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

    let class = Class::from(class_file)?;
    let class = class.to_object(vm).await?;
    let class: Object = class.try_into()?;
    Ok(class)
}

#[async_recursion(?Send)]
async fn define_class_0(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let vm = thread.vm()?;

    let class = if vm.java_major_version() <= JAVA_8 {
        let _protection_domain = parameters.pop_object()?;
        let length = parameters.pop_int()?;
        let offset = parameters.pop_int()?;
        let bytes: Vec<u8> = parameters.pop()?.try_into()?;
        let class = class_object_from_bytes(&vm, None, &bytes, offset, length).await?;
        if let Some(expected_class_name) = parameters.pop_reference()? {
            let expected_class_name: String = expected_class_name.try_into()?;
            let class_name = class.class().name();
            if class_name != expected_class_name {
                return Err(NoClassDefFoundError(class_name.to_string()).into());
            }
        }
        class
    } else {
        let _class_data = parameters.pop_reference()?;
        let _flags = parameters.pop_int()?;
        let _initialize = parameters.pop_bool()?;
        let _protection_domain = parameters.pop_object()?;
        let length = parameters.pop_int()?;
        let offset = parameters.pop_int()?;
        let bytes: Vec<u8> = parameters.pop()?.try_into()?;
        let _name: String = parameters.pop()?.try_into()?;
        let _lookup: Arc<Class> = parameters.pop()?.try_into()?;
        let class = class_object_from_bytes(&vm, None, &bytes, offset, length).await?;
        let class_loader = parameters.pop()?;
        class.set_value("classLoader", class_loader)?;
        class
    };

    Ok(Some(Value::from(class)))
}

#[async_recursion(?Send)]
async fn define_class_1(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let source_file = parameters.pop_reference()?;
    let _protection_domain = parameters.pop_object()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let bytes: Vec<u8> = parameters.pop()?.try_into()?;
    let vm = thread.vm()?;
    let class = class_object_from_bytes(&vm, source_file, &bytes, offset, length).await?;

    if vm.java_major_version() <= JAVA_8 {
        if let Some(expected_class_name) = parameters.pop_reference()? {
            let expected_class_name: String = expected_class_name.try_into()?;
            let class_name = class.class().name();
            if class_name != expected_class_name {
                return Err(NoClassDefFoundError(class_name.to_string()).into());
            }
        }
    } else {
        let class_loader = parameters.pop()?;
        class.set_value("classLoader", class_loader)?;
    }

    Ok(Some(Value::from(class)))
}

#[async_recursion(?Send)]
async fn define_class_2(thread: Arc<Thread>, mut parameters: Parameters) -> Result<Option<Value>> {
    let source_file = parameters.pop_reference()?;
    let _protection_domain = parameters.pop_object()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let byte_buffer = parameters.pop_object()?;
    let buffer: Vec<u8> = byte_buffer.value("hb")?.try_into()?;
    let buffer_offset = byte_buffer.value("offset")?.try_into()?;
    let bytes: Vec<u8> = buffer.into_iter().skip(buffer_offset).collect();
    let vm = thread.vm()?;
    let class = class_object_from_bytes(&vm, source_file, &bytes, offset, length).await?;

    if vm.java_major_version() <= JAVA_8 {
        if let Some(expected_class_name) = parameters.pop_reference()? {
            let expected_class_name: String = expected_class_name.try_into()?;
            let class_name = class.class().name();
            if class_name != expected_class_name {
                return Err(NoClassDefFoundError(class_name.to_string()).into());
            }
        }
    } else {
        let class_loader = parameters.pop()?;
        class.set_value("classLoader", class_loader)?;
    }

    Ok(Some(Value::from(class)))
}

#[async_recursion(?Send)]
async fn find_bootstrap_class(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let class_name: String = parameters.pop()?.try_into()?;
    let vm = thread.vm()?;
    let Ok(class) = vm.class(class_name).await else {
        return Ok(Some(Value::Object(None)));
    };
    let class = class.to_object(&vm).await?;
    Ok(Some(class))
}

#[async_recursion(?Send)]
async fn find_builtin_lib(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    // Ristretto has not built-in libraries; all native methods are implemented in Rust
    Ok(Some(Value::Object(None)))
}

#[async_recursion(?Send)]
async fn find_loaded_class_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let class_name: String = parameters.pop()?.try_into()?;
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Ristretto initializes the system class loader in the VM
    Ok(Some(Value::Object(None)))
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn resolve_class_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    // Ristretto resolves classes when they are loaded
    Ok(None)
}

#[async_recursion(?Send)]
async fn retrieve_directives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: implement the `retrieveDirectives` native method
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_builtin_lib() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = find_builtin_lib(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_init_system_class_loader() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_system_class_loader(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_resolve_class_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = resolve_class_0(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_retrieve_directives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = retrieve_directives(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
