use crate::Error::InternalError;
use crate::JavaError::IllegalArgumentException;
use crate::Result;
use crate::intrinsic_methods::java::lang::object::hash_code;
use crate::intrinsic_methods::properties;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use parking_lot::RwLock;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, ConstantPool, JAVA_8, JAVA_11, JAVA_17, MethodAccessFlags,
};
use ristretto_classloader::{Class, Object, Reference, Value};
use ristretto_gc::Gc;
use ristretto_macros::intrinsic_method;
use std::env::consts::OS;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

fn arraycopy_vec<T: Clone + Debug + PartialEq + Send + Sync>(
    source: &Gc<RwLock<Vec<T>>>,
    source_position: usize,
    destination: &Gc<RwLock<Vec<T>>>,
    destination_position: usize,
    length: usize,
) -> Result<()> {
    // Early return for zero-length copies
    if length == 0 {
        return Ok(());
    }

    // Validate bounds before copying; get lengths once to avoid multiple lock acquisitions
    let source_len = {
        let source = source.read();
        source.len()
    };
    let destination_len = {
        let destination = destination.read();
        destination.len()
    };

    if source_position + length > source_len {
        return Err(
            IllegalArgumentException("source array index out of bounds".to_string()).into(),
        );
    }
    if destination_position + length > destination_len {
        return Err(
            IllegalArgumentException("destination array index out of bounds".to_string()).into(),
        );
    }

    // Check if source and destination are the same array
    if source.ptr_eq(destination) {
        // Same array; need to handle overlapping regions
        let mut array = destination.write();

        // Handle overlapping regions correctly
        let regions_overlap = (source_position < destination_position
            && source_position + length > destination_position)
            || (destination_position < source_position
                && destination_position + length > source_position);

        if regions_overlap && destination_position > source_position {
            // Copy backwards to avoid overwriting source elements
            for i in (0..length).rev() {
                let value = array[source_position + i].clone();
                array[destination_position + i] = value;
            }
        } else if regions_overlap {
            // Copy forwards for left shift or other overlapping cases
            for i in 0..length {
                let value = array[source_position + i].clone();
                array[destination_position + i] = value;
            }
        } else {
            // Non-overlapping regions in same array; use slice operations for better performance
            let (src_start, src_end) = (source_position, source_position + length);
            let (dst_start, dst_end) = (destination_position, destination_position + length);

            // Create a temporary vector to hold the source elements
            let temp: Vec<T> = array[src_start..src_end].to_vec();
            array[dst_start..dst_end].clone_from_slice(&temp);
        }
    } else {
        // Different arrays; can optimize with bulk operations
        let source = source.read();
        let mut destination = destination.write();

        // Use slice operations for maximum efficiency
        let source_slice = &source[source_position..source_position + length];
        let destination_slice =
            &mut destination[destination_position..destination_position + length];

        destination_slice.clone_from_slice(source_slice);
    }

    Ok(())
}

#[intrinsic_method(
    "java/lang/System.arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn arraycopy(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let length = parameters.pop_int()?;
    let destination_position = parameters.pop_int()?;
    let Some(destination) = parameters.pop_reference()? else {
        return Err(InternalError("destination must be an object".to_string()));
    };
    let source_position = parameters.pop_int()?;
    let Some(source) = parameters.pop_reference()? else {
        return Err(InternalError("source must be an object".to_string()));
    };

    let source_position = usize::try_from(source_position)?;
    let destination_position = usize::try_from(destination_position)?;
    let length = usize::try_from(length)?;

    match (source, destination) {
        (Reference::ByteArray(source), Reference::ByteArray(destination)) => {
            arraycopy_vec(
                &source,
                source_position,
                &destination,
                destination_position,
                length,
            )?;
        }
        (Reference::CharArray(source), Reference::CharArray(destination)) => {
            arraycopy_vec(
                &source,
                source_position,
                &destination,
                destination_position,
                length,
            )?;
        }
        (Reference::DoubleArray(source), Reference::DoubleArray(destination)) => {
            arraycopy_vec(
                &source,
                source_position,
                &destination,
                destination_position,
                length,
            )?;
        }
        (Reference::FloatArray(source), Reference::FloatArray(destination)) => {
            arraycopy_vec(
                &source,
                source_position,
                &destination,
                destination_position,
                length,
            )?;
        }
        (Reference::IntArray(source), Reference::IntArray(destination)) => {
            arraycopy_vec(
                &source,
                source_position,
                &destination,
                destination_position,
                length,
            )?;
        }
        (Reference::LongArray(source), Reference::LongArray(destination)) => {
            arraycopy_vec(
                &source,
                source_position,
                &destination,
                destination_position,
                length,
            )?;
        }
        (Reference::ShortArray(source), Reference::ShortArray(destination)) => {
            arraycopy_vec(
                &source,
                source_position,
                &destination,
                destination_position,
                length,
            )?;
        }
        (Reference::Array(source), Reference::Array(destination)) => {
            arraycopy_vec(
                &source.elements,
                source_position,
                &destination.elements,
                destination_position,
                length,
            )?;
        }
        _ => {
            return Err(InternalError(
                "source and destination must be arrays of the same type".to_string(),
            ));
        }
    }
    Ok(None)
}

#[intrinsic_method("java/lang/System.allowSecurityManager()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn allow_security_manager(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("java/lang/System.currentTimeMillis()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn current_time_millis(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|error| InternalError(error.to_string()))?;
    let time = i64::try_from(duration.as_millis())?;
    Ok(Some(Value::Long(time)))
}

#[intrinsic_method(
    "java/lang/System.getSecurityManager()Ljava/lang/SecurityManager;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_security_manager(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // The SecurityManager is not supported in Ristretto.
    //
    // NOTE: This is not a native method in any version of Java.  This is here to prevent the JVM
    // from initializing the SecurityManager class in System.initPhase1() prior to the module layer
    // being initialized in System.initPhase2(). This is necessary because the SecurityManager
    // class is loaded when System.getProperty() is called, which in turn calls this method and
    // attempts to initialize the field class.
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/lang/System.identityHashCode(Ljava/lang/Object;)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn identity_hash_code(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    hash_code(thread, parameters).await
}

#[intrinsic_method(
    "java/lang/System.initProperties(Ljava/util/Properties;)Ljava/util/Properties;",
    LessThanOrEqual(JAVA_11)
)]
/// Mechanism for initializing properties for Java versions <= 11
#[async_recursion(?Send)]
pub(crate) async fn init_properties(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let properties = parameters.pop()?;
    let properties_class = thread.class("java.util.Properties").await?;
    let set_property_method = properties_class.try_get_method(
        "setProperty",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
    )?;
    let system_properties = &mut properties::system(&thread).await?;

    for (key, value) in system_properties.iter() {
        let key = key.to_object(&thread).await?;
        let value = value.clone();
        let parameters = vec![properties.clone(), key, value];
        thread
            .execute(&properties_class, &set_property_method, &parameters)
            .await?;
    }
    Ok(Some(properties))
}

#[intrinsic_method(
    "java/lang/System.mapLibraryName(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn map_library_name(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let library_name = object.as_string()?;
    let library_file_name = match OS {
        "macos" => format!("lib{library_name}.dylib"),
        "windows" => format!("{library_name}.dll"),
        _ => format!("lib{library_name}.so"),
    };
    let library_name = library_file_name.to_object(&thread).await?;
    Ok(Some(library_name))
}

#[intrinsic_method("java/lang/System.nanoTime()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn nano_time(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|error| InternalError(error.to_string()))?;
    let time = i64::try_from(duration.as_nanos())?;
    Ok(Some(Value::Long(time)))
}

#[intrinsic_method("java/lang/System.registerNatives()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    if vm.java_major_version() <= JAVA_8.java() {
        thread
            .invoke("java.lang.System", "setJavaLangAccess()V", &[] as &[Value])
            .await?;
        return Ok(None);
    }

    if vm.java_major_version() == JAVA_17.java() {
        // Force the initialization of the system properties; this is required because no security
        // manager is installed and when System::initPhase1() is called, the resulting call chain:
        //
        // System::initPhase1()
        //   System::setJavaLangAccess()
        //     SharedSecrets::<clinit>()
        //       MethodHandles::<clinit>()
        //         MethodHandleStatics::<clinit>()
        //           GetPropertyAction.privilegedGetProperties()
        //             System::getProperties()
        //
        // will eventually call System::getProperty() which fails if this is not initialized.
        thread
            .invoke(
                "java.lang.System",
                "setProperties(Ljava/util/Properties;)V",
                &[Value::Object(None)],
            )
            .await?;
    }

    let java_major_version = vm.java_major_version();
    let package_name = if java_major_version <= JAVA_8.java() {
        "sun/misc"
    } else if java_major_version <= JAVA_11.java() {
        "jdk/internal/misc"
    } else {
        "jdk/internal/access"
    };
    let java_lang_ref_access = java_lang_ref_access_class(&thread, package_name).await?;
    let java_lang_ref_access = Value::from(Object::new(java_lang_ref_access)?);
    let shared_secrets_class = format!("{package_name}/SharedSecrets");
    thread
        .invoke(
            &shared_secrets_class,
            format!("setJavaLangRefAccess(L{package_name}/JavaLangRefAccess;)V"),
            &[java_lang_ref_access],
        )
        .await?;

    Ok(None)
}

/// Create a class for `<package>.JavaLangRefAccess` to bootstrap the VM startup process.
pub(crate) async fn java_lang_ref_access_class(
    thread: &Arc<Thread>,
    package_name: &str,
) -> Result<Arc<Class>> {
    let vm = thread.vm()?;
    let java_class_file_version = vm.java_class_file_version();
    let mut constant_pool = ConstantPool::default();
    let this_class = constant_pool.add_class("ristretto/internal/access/JavaLangRefAccess")?;
    let interface_name = format!("{package_name}/JavaLangRefAccess");
    let interface_class = constant_pool.add_class(&interface_name)?;
    let code_index = constant_pool.add_utf8("Code")?;
    let start_threads_index = constant_pool.add_utf8("startThreads")?;
    let start_threads_descriptor_index = constant_pool.add_utf8("()V")?;

    let start_threads_method_attributes = vec![Attribute::Code {
        name_index: code_index,
        max_stack: 0,
        max_locals: 1,
        code: vec![Instruction::Return],
        exception_table: Vec::new(),
        attributes: Vec::new(),
    }];
    let start_threads_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC,
        name_index: start_threads_index,
        descriptor_index: start_threads_descriptor_index,
        attributes: start_threads_method_attributes,
    };
    let class_file = ClassFile {
        version: java_class_file_version.clone(),
        access_flags: ClassAccessFlags::PUBLIC,
        constant_pool,
        this_class,
        methods: vec![start_threads_method],
        interfaces: vec![interface_class],
        ..Default::default()
    };

    let java_lang_ref_access = Class::from(None, class_file)?;
    let interface = thread.class(&interface_name).await?;
    java_lang_ref_access.set_interfaces(vec![interface])?;
    thread.register_class(java_lang_ref_access.clone()).await?;
    Ok(java_lang_ref_access)
}

#[intrinsic_method("java/lang/System.setIn0(Ljava/io/InputStream;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_in_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let input_stream = parameters.pop_reference()?;
    let system = thread.class("java.lang.System").await?;
    let value = Value::Object(input_stream);
    system.set_static_value_unchecked("in", value)?;
    Ok(None)
}

#[intrinsic_method("java/lang/System.setOut0(Ljava/io/PrintStream;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_out_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let print_stream = parameters.pop_reference()?;
    let system = thread.class("java.lang.System").await?;
    let value = Value::Object(print_stream);
    system.set_static_value_unchecked("out", value)?;
    Ok(None)
}

#[intrinsic_method("java/lang/System.setErr0(Ljava/io/PrintStream;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_err_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let print_stream = parameters.pop_reference()?;
    let system = thread.class("java.lang.System").await?;
    let value = Value::Object(print_stream);
    system.set_static_value_unchecked("err", value)?;
    Ok(None)
}

#[intrinsic_method(
    "java/lang/System.setSecurityManager(Ljava/lang/SecurityManager;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn set_security_manager(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // The SecurityManager is not supported in Ristretto.
    Err(InternalError(
        "SecurityManager is not supported".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::JavaError;

    #[test]
    fn test_arraycopy_vec_basic_copy() -> Result<()> {
        let source = Gc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0, 0, 0]));

        arraycopy_vec(&source, 0, &destination, 0, 3)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&1));
        assert_eq!(destination.get(1), Some(&2));
        assert_eq!(destination.get(2), Some(&3));
        assert_eq!(destination.get(3), Some(&0)); // Unchanged
        assert_eq!(destination.get(4), Some(&0)); // Unchanged
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_offset_copy() -> Result<()> {
        let source = Gc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0, 0, 0]));

        // Copy from source[1..3] to destination[2..4]
        arraycopy_vec(&source, 1, &destination, 2, 2)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&0)); // Unchanged
        assert_eq!(destination.get(1), Some(&0)); // Unchanged
        assert_eq!(destination.get(2), Some(&2)); // source[1]
        assert_eq!(destination.get(3), Some(&3)); // source[2]
        assert_eq!(destination.get(4), Some(&0)); // Unchanged
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_full_array_copy() -> Result<()> {
        let source = Gc::new(RwLock::new(vec![10, 20, 30]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0]));

        arraycopy_vec(&source, 0, &destination, 0, 3)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&10));
        assert_eq!(destination.get(1), Some(&20));
        assert_eq!(destination.get(2), Some(&30));
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_zero_length() -> Result<()> {
        let source = Gc::new(RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0]));

        arraycopy_vec(&source, 0, &destination, 0, 0)?;
        let destination = destination.read();

        // Nothing should be copied
        assert_eq!(destination.first(), Some(&0));
        assert_eq!(destination.get(1), Some(&0));
        assert_eq!(destination.get(2), Some(&0));
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_single_element() -> Result<()> {
        let source = Gc::new(RwLock::new(vec![42]));
        let destination = Gc::new(RwLock::new(vec![0]));

        arraycopy_vec(&source, 0, &destination, 0, 1)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&42));
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_copy_to_end() -> Result<()> {
        let source = Gc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0, 0, 0]));

        // Copy last 2 elements of source to last 2 positions of destination
        arraycopy_vec(&source, 3, &destination, 3, 2)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&0)); // Unchanged
        assert_eq!(destination.get(1), Some(&0)); // Unchanged
        assert_eq!(destination.get(2), Some(&0)); // Unchanged
        assert_eq!(destination.get(3), Some(&4)); // source[3]
        assert_eq!(destination.get(4), Some(&5)); // source[4]
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_overlapping_arrays_different_objects() -> Result<()> {
        let source = Gc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(RwLock::new(vec![10, 11, 12, 13, 14]));

        // This should work fine since they're different arrays
        arraycopy_vec(&source, 1, &destination, 0, 3)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&2)); // source[1]
        assert_eq!(destination.get(1), Some(&3)); // source[2]
        assert_eq!(destination.get(2), Some(&4)); // source[3]
        assert_eq!(destination.get(3), Some(&13)); // Unchanged
        assert_eq!(destination.get(4), Some(&14)); // Unchanged
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_source_bounds_error() {
        let source = Gc::new(RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0, 0, 0]));

        // Try to copy 3 elements starting from position 2 (would need source[2,3,4] but only have [0,1,2])
        let result = arraycopy_vec(&source, 2, &destination, 0, 2);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.to_string().contains("source array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_destination_bounds_error() {
        let source = Gc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0]));

        // Try to copy 3 elements to position 1 (would need dest[1,2,3] but only have [0,1,2])
        let result = arraycopy_vec(&source, 0, &destination, 1, 3);
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.to_string().contains("destination array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_source_position_out_of_bounds() {
        let source = Gc::new(RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0]));

        // Try to start copying from position 3 (doesn't exist)
        let result = arraycopy_vec(&source, 3, &destination, 0, 1);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.to_string().contains("source array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_destination_position_out_of_bounds() {
        let source = Gc::new(RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0]));

        // Try to start copying to position 3 (doesn't exist)
        let result = arraycopy_vec(&source, 0, &destination, 3, 1);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.to_string().contains("destination array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_exact_boundary() -> Result<()> {
        let source = Gc::new(RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(RwLock::new(vec![0, 0, 0]));

        // Copy exactly to the boundary - should work
        arraycopy_vec(&source, 0, &destination, 0, 3)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&1));
        assert_eq!(destination.get(1), Some(&2));
        assert_eq!(destination.get(2), Some(&3));
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_empty_arrays() -> Result<()> {
        let source = Gc::new(RwLock::new(Vec::<i32>::new()));
        let destination = Gc::new(RwLock::new(Vec::<i32>::new()));

        // Copying 0 elements from empty arrays should work
        arraycopy_vec(&source, 0, &destination, 0, 0)?;
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_empty_source_error() {
        let source = Gc::new(RwLock::new(Vec::<i32>::new()));
        let destination = Gc::new(RwLock::new(vec![0]));

        // Try to copy from empty array
        let result = arraycopy_vec(&source, 0, &destination, 0, 1);
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.to_string().contains("source array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_empty_destination_error() {
        let source = Gc::new(RwLock::new(vec![1]));
        let destination = Gc::new(RwLock::new(Vec::<i32>::new()));

        // Try to copy to empty array
        let result = arraycopy_vec(&source, 0, &destination, 0, 1);
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.to_string().contains("destination array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_string_copy() -> Result<()> {
        let source = Gc::new(RwLock::new(vec![
            "hello".to_string(),
            "world".to_string(),
            "test".to_string(),
        ]));
        let destination = Gc::new(RwLock::new(vec![
            String::new(),
            String::new(),
            String::new(),
        ]));

        arraycopy_vec(&source, 0, &destination, 0, 2)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&"hello".to_string()));
        assert_eq!(destination.get(1), Some(&"world".to_string()));
        assert_eq!(destination.get(2), Some(&String::new())); // Unchanged
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_large_copy() -> Result<()> {
        let source_data: Vec<i32> = (0..1000).collect();
        let dest_data: Vec<i32> = vec![0; 1000];

        let source = Gc::new(RwLock::new(source_data));
        let destination = Gc::new(RwLock::new(dest_data));

        arraycopy_vec(&source, 100, &destination, 200, 500)?;
        let destination = destination.read();

        // Verify first few and last few elements
        assert_eq!(destination.get(200), Some(&100));
        assert_eq!(destination.get(201), Some(&101));
        assert_eq!(destination.get(699), Some(&599)); // 200 + 500 - 1

        // Verify boundaries weren't affected
        assert_eq!(destination.get(199), Some(&0)); // Before copy region
        assert_eq!(destination.get(700), Some(&0)); // After copy region
        Ok(())
    }

    #[tokio::test]
    async fn test_allow_security_manager() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = allow_security_manager(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::from(false)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_security_manager() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_security_manager(thread, Parameters::default()).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_nano_time() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = nano_time(thread, Parameters::default()).await?;
        let time = result.unwrap_or(Value::Long(0)).as_i64()?;
        assert!(time > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_security_manager() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = set_security_manager(thread, Parameters::default()).await;
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_overlapping_same_array_forward_shift() -> Result<()> {
        // Test shifting elements to the right within the same array (like StringBuilder insert)
        let array = Gc::new(RwLock::new(vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        ]));

        // Shift "cdef" to the right by 2 positions to make room for insertion
        // This simulates what StringBuilder does when inserting text
        arraycopy_vec(&array, 2, &array, 4, 4)?;
        let array = array.read();

        assert_eq!(array.first(), Some(&'a')); // Unchanged
        assert_eq!(array.get(1), Some(&'b')); // Unchanged
        assert_eq!(array.get(2), Some(&'c')); // Original position
        assert_eq!(array.get(3), Some(&'d')); // Original position
        assert_eq!(array.get(4), Some(&'c')); // Shifted from position 2
        assert_eq!(array.get(5), Some(&'d')); // Shifted from position 3
        assert_eq!(array.get(6), Some(&'e')); // Shifted from position 4
        assert_eq!(array.get(7), Some(&'f')); // Shifted from position 5
        assert_eq!(array.get(8), Some(&'i')); // Unchanged
        assert_eq!(array.get(9), Some(&'j')); // Unchanged
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_overlapping_same_array_backward_shift() -> Result<()> {
        // Test shifting elements to the left within the same array
        let array = Gc::new(RwLock::new(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']));

        // Shift "cdefgh" to the left by 1 position
        arraycopy_vec(&array, 2, &array, 1, 6)?;
        let array = array.read();

        assert_eq!(array.first(), Some(&'a')); // Unchanged
        assert_eq!(array.get(1), Some(&'c')); // Shifted from position 2
        assert_eq!(array.get(2), Some(&'d')); // Shifted from position 3
        assert_eq!(array.get(3), Some(&'e')); // Shifted from position 4
        assert_eq!(array.get(4), Some(&'f')); // Shifted from position 5
        assert_eq!(array.get(5), Some(&'g')); // Shifted from position 6
        assert_eq!(array.get(6), Some(&'h')); // Shifted from position 7
        assert_eq!(array.get(7), Some(&'h')); // Original value still here
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_non_overlapping_same_array() -> Result<()> {
        // Test copying within the same array but with non-overlapping regions
        let array = Gc::new(RwLock::new(vec![1, 2, 3, 4, 5, 6, 7, 8]));

        // Copy first 3 elements to positions 5-7 (no overlap)
        arraycopy_vec(&array, 0, &array, 5, 3)?;
        let array = array.read();

        assert_eq!(array.first(), Some(&1)); // Unchanged
        assert_eq!(array.get(1), Some(&2)); // Unchanged
        assert_eq!(array.get(2), Some(&3)); // Unchanged
        assert_eq!(array.get(3), Some(&4)); // Unchanged
        assert_eq!(array.get(4), Some(&5)); // Unchanged
        assert_eq!(array.get(5), Some(&1)); // Copied from position 0
        assert_eq!(array.get(6), Some(&2)); // Copied from position 1
        assert_eq!(array.get(7), Some(&3)); // Copied from position 2
        Ok(())
    }
}
