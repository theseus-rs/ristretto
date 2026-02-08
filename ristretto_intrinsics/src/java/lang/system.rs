use crate::java::lang::object::hash_code;
use crate::properties;
use parking_lot::RwLock;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
use ristretto_gc::Gc;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError::IllegalArgumentException;
use ristretto_types::JavaObject;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
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
    if Gc::ptr_eq(source, destination) {
        let mut array = source.write();
        arraycopy_within_helper(&mut array, source_position, destination_position, length)
    } else {
        let source = source.read();
        let mut destination = destination.write();
        arraycopy_helper(
            &source,
            source_position,
            &mut destination,
            destination_position,
            length,
        )
    }
}

fn arraycopy_helper<T: Clone>(
    source: &[T],
    source_position: usize,
    destination: &mut [T],
    destination_position: usize,
    length: usize,
) -> Result<()> {
    if source_position + length > source.len() {
        return Err(
            IllegalArgumentException("source array index out of bounds".to_string()).into(),
        );
    }
    if destination_position + length > destination.len() {
        return Err(
            IllegalArgumentException("destination array index out of bounds".to_string()).into(),
        );
    }
    destination[destination_position..destination_position + length]
        .clone_from_slice(&source[source_position..source_position + length]);
    Ok(())
}

fn arraycopy_within_helper<T: Clone>(
    array: &mut [T],
    source_position: usize,
    destination_position: usize,
    length: usize,
) -> Result<()> {
    if source_position + length > array.len() {
        return Err(
            IllegalArgumentException("source array index out of bounds".to_string()).into(),
        );
    }
    if destination_position + length > array.len() {
        return Err(
            IllegalArgumentException("destination array index out of bounds".to_string()).into(),
        );
    }
    let temp = array[source_position..source_position + length].to_vec();
    array[destination_position..destination_position + length].clone_from_slice(&temp);
    Ok(())
}

#[intrinsic_method(
    "java/lang/System.arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V",
    Any
)]
#[async_method]
pub async fn arraycopy<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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

    if Gc::ptr_eq(&source, &destination) {
        let mut guard = source.write();
        match &mut *guard {
            Reference::BooleanArray(array) | Reference::ByteArray(array) => {
                arraycopy_within_helper(array, source_position, destination_position, length)?;
            }
            Reference::CharArray(array) => {
                arraycopy_within_helper(array, source_position, destination_position, length)?;
            }
            Reference::ShortArray(array) => {
                arraycopy_within_helper(array, source_position, destination_position, length)?;
            }
            Reference::IntArray(array) => {
                arraycopy_within_helper(array, source_position, destination_position, length)?;
            }
            Reference::LongArray(array) => {
                arraycopy_within_helper(array, source_position, destination_position, length)?;
            }
            Reference::FloatArray(array) => {
                arraycopy_within_helper(array, source_position, destination_position, length)?;
            }
            Reference::DoubleArray(array) => {
                arraycopy_within_helper(array, source_position, destination_position, length)?;
            }
            Reference::Array(array) => {
                arraycopy_within_helper(
                    &mut array.elements,
                    source_position,
                    destination_position,
                    length,
                )?;
            }
            Reference::Object(_) => {
                return Err(InternalError("source must be an array".to_string()));
            }
        }
    } else {
        let source_guard = source.read();
        let mut destination_guard = destination.write();
        match (&*source_guard, &mut *destination_guard) {
            (Reference::BooleanArray(src), Reference::BooleanArray(dst))
            | (Reference::ByteArray(src), Reference::ByteArray(dst)) => {
                arraycopy_helper(src, source_position, dst, destination_position, length)?;
            }
            (Reference::CharArray(src), Reference::CharArray(dst)) => {
                arraycopy_helper(src, source_position, dst, destination_position, length)?;
            }
            (Reference::ShortArray(src), Reference::ShortArray(dst)) => {
                arraycopy_helper(src, source_position, dst, destination_position, length)?;
            }
            (Reference::IntArray(src), Reference::IntArray(dst)) => {
                arraycopy_helper(src, source_position, dst, destination_position, length)?;
            }
            (Reference::LongArray(src), Reference::LongArray(dst)) => {
                arraycopy_helper(src, source_position, dst, destination_position, length)?;
            }
            (Reference::FloatArray(src), Reference::FloatArray(dst)) => {
                arraycopy_helper(src, source_position, dst, destination_position, length)?;
            }
            (Reference::DoubleArray(src), Reference::DoubleArray(dst)) => {
                arraycopy_helper(src, source_position, dst, destination_position, length)?;
            }
            (Reference::Array(src), Reference::Array(dst)) => {
                arraycopy_helper(
                    &src.elements,
                    source_position,
                    &mut dst.elements,
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
    }

    Ok(None)
}

#[intrinsic_method("java/lang/System.allowSecurityManager()Z", Any)]
#[async_method]
pub async fn allow_security_manager<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("java/lang/System.currentTimeMillis()J", Any)]
#[async_method]
pub async fn current_time_millis<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn get_security_manager<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn identity_hash_code<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    hash_code(thread, parameters).await
}

#[intrinsic_method(
    "java/lang/System.initProperties(Ljava/util/Properties;)Ljava/util/Properties;",
    LessThanOrEqual(JAVA_11)
)]
/// Mechanism for initializing properties for Java versions <= 11
#[async_method]
pub async fn init_properties<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
#[async_method]
pub async fn map_library_name<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
#[async_method]
pub async fn nano_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
#[async_method]
pub async fn register_natives<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;

    if vm.java_major_version() <= JAVA_8.java() {
        thread
            .invoke("java.lang.System", "setJavaLangAccess()V", &[] as &[Value])
            .await?;
    }

    if vm.java_major_version() <= JAVA_17.java() {
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

    Ok(None)
}

#[intrinsic_method("java/lang/System.setIn0(Ljava/io/InputStream;)V", Any)]
#[async_method]
pub async fn set_in_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let input_stream = parameters.pop_reference()?;
    let system = thread.class("java.lang.System").await?;
    let value = Value::Object(input_stream);
    system.set_static_value_unchecked("in", value)?;
    Ok(None)
}

#[intrinsic_method("java/lang/System.setOut0(Ljava/io/PrintStream;)V", Any)]
#[async_method]
pub async fn set_out_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let print_stream = parameters.pop_reference()?;
    let system = thread.class("java.lang.System").await?;
    let value = Value::Object(print_stream);
    system.set_static_value_unchecked("out", value)?;
    Ok(None)
}

#[intrinsic_method("java/lang/System.setErr0(Ljava/io/PrintStream;)V", Any)]
#[async_method]
pub async fn set_err_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
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
#[async_method]
pub async fn set_security_manager<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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

    use ristretto_gc::GarbageCollector;
    use ristretto_types::Error::JavaError;

    #[test]
    fn test_arraycopy_vec_basic_copy() -> Result<()> {
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0, 0, 0]));

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
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0, 0, 0]));

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
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![10, 20, 30]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0]));

        arraycopy_vec(&source, 0, &destination, 0, 3)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&10));
        assert_eq!(destination.get(1), Some(&20));
        assert_eq!(destination.get(2), Some(&30));
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_zero_length() -> Result<()> {
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0]));

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
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![42]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0]));

        arraycopy_vec(&source, 0, &destination, 0, 1)?;
        let destination = destination.read();

        assert_eq!(destination.first(), Some(&42));
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_copy_to_end() -> Result<()> {
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0, 0, 0]));

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
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(
            &GarbageCollector::new(),
            RwLock::new(vec![10, 11, 12, 13, 14]),
        );

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
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0, 0, 0]));

        // Try to copy 3 elements starting from position 2 (would need source[2,3,4] but only have [0,1,2])
        let result = arraycopy_vec(&source, 2, &destination, 0, 2);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.clone().contains("source array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_destination_bounds_error() {
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3, 4, 5]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0]));

        // Try to copy 3 elements to position 1 (would need dest[1,2,3] but only have [0,1,2])
        let result = arraycopy_vec(&source, 0, &destination, 1, 3);
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.clone().contains("destination array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_source_position_out_of_bounds() {
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0]));

        // Try to start copying from position 3 (doesn't exist)
        let result = arraycopy_vec(&source, 3, &destination, 0, 1);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.clone().contains("source array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_destination_position_out_of_bounds() {
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0]));

        // Try to start copying to position 3 (doesn't exist)
        let result = arraycopy_vec(&source, 0, &destination, 3, 1);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.clone().contains("destination array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_exact_boundary() -> Result<()> {
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1, 2, 3]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0, 0, 0]));

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
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(Vec::<i32>::new()));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(Vec::<i32>::new()));

        // Copying 0 elements from empty arrays should work
        arraycopy_vec(&source, 0, &destination, 0, 0)?;
        Ok(())
    }

    #[test]
    fn test_arraycopy_vec_empty_source_error() {
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(Vec::<i32>::new()));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(vec![0]));

        // Try to copy from empty array
        let result = arraycopy_vec(&source, 0, &destination, 0, 1);
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.clone().contains("source array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_empty_destination_error() {
        let source = Gc::new(&GarbageCollector::new(), RwLock::new(vec![1]));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(Vec::<i32>::new()));

        // Try to copy to empty array
        let result = arraycopy_vec(&source, 0, &destination, 0, 1);
        assert!(matches!(
            result,
            Err(JavaError(IllegalArgumentException(message))) if message.clone().contains("destination array index out of bounds")
        ));
    }

    #[test]
    fn test_arraycopy_vec_string_copy() -> Result<()> {
        let source = Gc::new(
            &GarbageCollector::new(),
            RwLock::new(vec![
                "hello".to_string(),
                "world".to_string(),
                "test".to_string(),
            ]),
        );
        let destination = Gc::new(
            &GarbageCollector::new(),
            RwLock::new(vec![String::new(), String::new(), String::new()]),
        );

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

        let source = Gc::new(&GarbageCollector::new(), RwLock::new(source_data));
        let destination = Gc::new(&GarbageCollector::new(), RwLock::new(dest_data));

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
        let array = Gc::new(
            &GarbageCollector::new(),
            RwLock::new(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j']),
        );

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
        let array = Gc::new(
            &GarbageCollector::new(),
            RwLock::new(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']),
        );

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
        let array = Gc::new(
            &GarbageCollector::new(),
            RwLock::new(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        );

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
