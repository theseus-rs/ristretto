use crate::JavaError::{ClassFormatError, IndexOutOfBoundsException, NoClassDefFoundError};
use crate::Result;
use crate::intrinsic_methods::java::lang::class::get_class;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use parking_lot::RwLock;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classfile::{ClassFile, JAVA_8, JAVA_11};
use ristretto_classloader::{Class, Reference, Value};
use ristretto_gc::Gc;
use ristretto_macros::intrinsic_method;
use std::io::Cursor;
use std::sync::Arc;
use zerocopy::transmute_ref;

/// Create a `java.lang.Class` object from a byte array.
/// This method is used by the `defineClass0`, `defineClass1`, and `defineClass2` native methods.
/// The `defineClass0` method is used by Java 8 and earlier versions.
async fn class_object_from_bytes(
    thread: &Arc<Thread>,
    source_file: Option<Gc<RwLock<Reference>>>,
    bytes: &[u8],
    offset: i32,
    length: i32,
) -> Result<Value> {
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
        let _source_file = source_file.read().as_string()?;
        // TODO: implement setting the source file
    }

    let class = Class::from(None, class_file)?;
    let class = class.to_object(thread).await?;
    Ok(class)
}

#[intrinsic_method(
    "java/lang/ClassLoader.defineClass0(Ljava/lang/String;[BIILjava/security/ProtectionDomain;)Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_class_0_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _protection_domain = parameters.pop()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let bytes = parameters.pop()?;
    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        bytes.to_vec()
    };
    let class = class_object_from_bytes(&thread, None, &bytes, offset, length).await?;
    if let Some(expected_class_name) = parameters.pop_reference()? {
        let expected_class_name = expected_class_name.read().as_string()?;
        let class = class.as_object_ref()?;
        let class_name = class.class().name();
        if class_name != expected_class_name {
            return Err(NoClassDefFoundError(class_name.to_string()).into());
        }
    }
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/ClassLoader.defineClass1(Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_class_1_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source_file = parameters.pop_reference()?;
    let _protection_domain = parameters.pop()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let bytes = parameters.pop()?;
    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        bytes.to_vec()
    };
    let class = class_object_from_bytes(&thread, source_file, &bytes, offset, length).await?;
    if let Some(expected_class_name) = parameters.pop_reference()? {
        let expected_class_name = expected_class_name.read().as_string()?;
        let class_object = class.as_object_ref()?;
        let class_name = class_object.class().name();
        if class_name != expected_class_name {
            return Err(NoClassDefFoundError(class_name.to_string()).into());
        }
    }
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/ClassLoader.defineClass2(Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_class_2_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source_file = parameters.pop_reference()?;
    let _protection_domain = parameters.pop()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let byte_buffer = parameters.pop()?;
    let (buffer, buffer_offset) = {
        let byte_buffer = byte_buffer.as_object_ref()?;
        let buffer = byte_buffer.value("hb")?;
        let buffer_offset = byte_buffer.value("offset")?.as_usize()?;
        (buffer, buffer_offset)
    };
    let buffer = {
        let buffer = buffer.as_byte_vec_ref()?;
        let buffer: &[u8] = transmute_ref!(&*buffer);
        buffer.to_vec()
    };

    let bytes: Vec<u8> = buffer.iter().copied().skip(buffer_offset).collect();
    let class = class_object_from_bytes(&thread, source_file, &bytes, offset, length).await?;
    if let Some(expected_class_name) = parameters.pop_reference()? {
        let expected_class_name = expected_class_name.read().as_string()?;
        let class_object = class.as_object_ref()?;
        let class_name = class_object.class().name();
        if class_name != expected_class_name {
            return Err(NoClassDefFoundError(class_name.to_string()).into());
        }
    }
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/ClassLoader.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/Class;Ljava/lang/String;[BIILjava/security/ProtectionDomain;ZILjava/lang/Object;)Ljava/lang/Class;",
    GreaterThan(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_class_0_1(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _class_data = parameters.pop_reference()?;
    let _flags = parameters.pop_int()?;
    let _initialize = parameters.pop_bool()?;
    let _protection_domain = parameters.pop()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let bytes = parameters.pop()?;
    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        bytes.to_vec()
    };
    let _name = parameters.pop()?.as_string()?;
    let lookup = parameters.pop()?;
    let _lookup = get_class(&thread, &lookup).await?;
    let class = class_object_from_bytes(&thread, None, &bytes, offset, length).await?;
    let class_loader = parameters.pop()?;
    {
        let mut class_object = class.as_object_mut()?;
        class_object.set_value("classLoader", class_loader)?;
    }
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/ClassLoader.defineClass1(Ljava/lang/ClassLoader;Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_class_1_1(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source_file = parameters.pop_reference()?;
    let _protection_domain = parameters.pop()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let bytes = parameters.pop()?;
    let bytes = {
        let bytes = bytes.as_byte_vec_ref()?;
        let bytes: &[u8] = transmute_ref!(&*bytes);
        bytes.to_vec()
    };
    let class = class_object_from_bytes(&thread, source_file, &bytes, offset, length).await?;
    let class_loader = parameters.pop()?;
    {
        let mut class_object = class.as_object_mut()?;
        class_object.set_value("classLoader", class_loader)?;
    }
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/ClassLoader.defineClass2(Ljava/lang/ClassLoader;Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn define_class_2_1(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let source_file = parameters.pop_reference()?;
    let _protection_domain = parameters.pop()?;
    let length = parameters.pop_int()?;
    let offset = parameters.pop_int()?;
    let byte_buffer = parameters.pop()?;
    let (buffer, buffer_offset) = {
        let byte_buffer = byte_buffer.as_object_ref()?;
        let buffer = byte_buffer.value("hb")?;
        let buffer_offset = byte_buffer.value("offset")?.as_usize()?;
        (buffer, buffer_offset)
    };
    let buffer = {
        let buffer = buffer.as_byte_vec_ref()?;
        let buffer: &[u8] = transmute_ref!(&*buffer);
        buffer.to_vec()
    };

    let bytes: Vec<u8> = buffer.iter().copied().skip(buffer_offset).collect();
    let class = class_object_from_bytes(&thread, source_file, &bytes, offset, length).await?;
    let class_loader = parameters.pop()?;
    {
        let mut class_object = class.as_object_mut()?;
        class_object.set_value("classLoader", class_loader)?;
    }
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/ClassLoader.findBootstrapClass(Ljava/lang/String;)Ljava/lang/Class;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn find_bootstrap_class(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let class_name = parameters.pop()?.as_string()?;
    let Ok(class) = thread.class(class_name).await else {
        return Ok(Some(Value::Object(None)));
    };
    let class = class.to_object(&thread).await?;
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/ClassLoader.findBuiltinLib(Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn find_builtin_lib(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Ristretto has not built-in libraries; all native methods are implemented in Rust
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "java/lang/ClassLoader.findLoadedClass0(Ljava/lang/String;)Ljava/lang/Class;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn find_loaded_class_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let class_name = parameters.pop()?.as_string()?;
    let Ok(class) = thread.class(class_name).await else {
        return Ok(Some(Value::Object(None)));
    };
    let class = class.to_object(&thread).await?;
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/ClassLoader.initSystemClassLoader()Ljava/lang/ClassLoader;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn init_system_class_loader(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Ristretto initializes the system class loader in the VM
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/lang/ClassLoader.registerNatives()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/ClassLoader.resolveClass0(Ljava/lang/Class;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn resolve_class_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Ristretto resolves classes when they are loaded
    Ok(None)
}

#[intrinsic_method(
    "java/lang/ClassLoader.retrieveDirectives()Ljava/lang/AssertionStatusDirectives;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn retrieve_directives(
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
