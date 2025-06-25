use crate::Error::InternalError;
use crate::Result;
use crate::intrinsic_methods::java::lang::object::object_hash_code;
use crate::intrinsic_methods::properties;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_classfile::{
    ClassAccessFlags, ClassFile, ConstantPool, JAVA_8, JAVA_11, JAVA_17, MethodAccessFlags,
};
use ristretto_classloader::{Class, ConcurrentVec, Object, Reference, Value};
use ristretto_macros::intrinsic_method;
use std::cmp::min;
use std::env::consts::OS;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

fn arraycopy_vec<T: Clone + Debug + PartialEq>(
    source: &ConcurrentVec<T>,
    source_position: usize,
    destination: &ConcurrentVec<T>,
    mut destination_position: usize,
    length: usize,
) -> Result<()> {
    // TODO: optimize this logic to avoid the need for looping
    let max_length = min(source_position + length, source.len()?);
    for i in source_position..max_length {
        let Some(value) = source.get(i)? else {
            return Err(InternalError("invalid source array index".to_string()));
        };
        destination.set(destination_position, value)?;
        destination_position += 1;
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
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let hash_code = match parameters.pop_reference()? {
        Some(object) => object_hash_code(&object),
        None => 0,
    };
    Ok(Some(Value::Int(hash_code)))
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
    let vm = thread.vm()?;
    let properties_class = thread.class("java.util.Properties").await?;
    let set_property_method = properties_class.try_get_method(
        "setProperty",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
    )?;
    let system_properties = &mut properties::system(&thread).await?;

    for (key, value) in system_properties.iter() {
        let key = key.to_object(&vm).await?;
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
    let Some(Reference::Object(object)) = parameters.pop_reference()? else {
        return Err(InternalError("parameter must be an object".to_string()));
    };
    let library_name: String = object.try_into()?;
    let library_file_name = match OS {
        "macos" => format!("lib{library_name}.dylib"),
        "windows" => format!("{library_name}.dll"),
        _ => format!("lib{library_name}.so"),
    };
    let vm = thread.vm()?;
    let library_name = library_file_name.to_object(&vm).await?;
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
    let java_lang_ref_access =
        Value::Object(Some(Reference::Object(Object::new(java_lang_ref_access)?)));
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
    let this_class = constant_pool.add_class("ristretto.internal.access.JavaLangRefAccess")?;
    let interface_class = constant_pool.add_class(format!("{package_name}/JavaLangRefAccess"))?;
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

    let java_lang_ref_access = Class::from(class_file)?;
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
    let in_field = system.static_field("in")?;
    in_field.unsafe_set_value(Value::Object(input_stream))?;
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
    let out_field = system.static_field("out")?;
    out_field.unsafe_set_value(Value::Object(print_stream))?;
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
    let err_field = system.static_field("err")?;
    err_field.unsafe_set_value(Value::Object(print_stream))?;
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
        let time = result.unwrap_or(Value::Long(0)).to_long()?;
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
}
