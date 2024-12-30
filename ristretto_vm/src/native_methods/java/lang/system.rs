use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::java::lang::object::object_hash_code;
use crate::native_methods::properties;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{ClassFile, MethodAccessFlags, Version};
use ristretto_classloader::{Class, ConcurrentVec, Method, Object, Reference, Value};
use std::cmp::min;
use std::env::consts::OS;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `java.lang.System`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/System";

    registry.register(
        class_name,
        "arraycopy",
        "(Ljava/lang/Object;ILjava/lang/Object;II)V",
        arraycopy,
    );
    registry.register(
        class_name,
        "allowSecurityManager",
        "()Z",
        allow_security_manager,
    );
    registry.register(class_name, "currentTimeMillis", "()J", current_time_millis);
    registry.register(class_name, "gc", "()V", gc);
    registry.register(
        class_name,
        "getSecurityManager",
        "()Ljava/lang/SecurityManager;",
        get_security_manager,
    );
    registry.register(
        class_name,
        "identityHashCode",
        "(Ljava/lang/Object;)I",
        identity_hash_code,
    );
    registry.register(
        class_name,
        "initProperties",
        "(Ljava/util/Properties;)Ljava/util/Properties;",
        init_properties,
    );
    registry.register(
        class_name,
        "mapLibraryName",
        "(Ljava/lang/String;)Ljava/lang/String;",
        map_library_name,
    );
    registry.register(class_name, "nanoTime", "()J", nano_time);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(class_name, "setErr0", "(Ljava/io/PrintStream;)V", set_err_0);
    registry.register(class_name, "setIn0", "(Ljava/io/InputStream;)V", set_in_0);
    registry.register(class_name, "setOut0", "(Ljava/io/PrintStream;)V", set_out_0);
    registry.register(
        class_name,
        "setSecurityManager",
        "(Ljava/lang/SecurityManager;)V",
        set_security_manager,
    );
}

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

#[async_recursion(?Send)]
async fn arraycopy(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let length = arguments.pop_int()?;
    let destination_position = arguments.pop_int()?;
    let Some(destination) = arguments.pop_reference()? else {
        return Err(InternalError("destination must be an object".to_string()));
    };
    let source_position = arguments.pop_int()?;
    let Some(source) = arguments.pop_reference()? else {
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
        (Reference::Array(_, source), Reference::Array(_, destination)) => {
            arraycopy_vec(
                &source,
                source_position,
                &destination,
                destination_position,
                length,
            )?;
        }
        _ => {
            return Err(InternalError(
                "source and destination must be arrays of the same type".to_string(),
            ))
        }
    };
    Ok(None)
}

#[async_recursion(?Send)]
async fn allow_security_manager(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn current_time_millis(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|error| InternalError(error.to_string()))?;
    let time = i64::try_from(duration.as_millis())?;
    Ok(Some(Value::Long(time)))
}

#[async_recursion(?Send)]
async fn gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn get_security_manager(
    _thread: Arc<Thread>,
    _arguments: Arguments,
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

#[async_recursion(?Send)]
async fn identity_hash_code(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let hash_code = match arguments.pop_reference()? {
        Some(object) => object_hash_code(&object),
        None => 0,
    };
    Ok(Some(Value::Int(hash_code)))
}

/// Mechanism for initializing properties for Java versions <= 11
#[async_recursion(?Send)]
async fn init_properties(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let properties = arguments.pop()?;
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
        let arguments = vec![properties.clone(), key, value];
        thread
            .execute(&properties_class, &set_property_method, arguments)
            .await?;
    }
    Ok(Some(properties))
}

#[async_recursion(?Send)]
async fn map_library_name(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_reference()? else {
        return Err(InternalError("argument must be an object".to_string()));
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

#[async_recursion(?Send)]
async fn nano_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|error| InternalError(error.to_string()))?;
    let time = i64::try_from(duration.as_nanos())?;
    Ok(Some(Value::Long(time)))
}

#[async_recursion(?Send)]
async fn register_natives(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    if vm.java_class_file_version() <= &JAVA_8 {
        vm.invoke(
            "java.lang.System",
            "setJavaLangAccess",
            "()V",
            Vec::<Value>::new(),
        )
        .await?;
        return Ok(None);
    }

    if vm.java_class_file_version() == &JAVA_17 {
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
        vm.invoke(
            "java.lang.System",
            "setProperties",
            "(Ljava/util/Properties;)V",
            vec![Value::Object(None)],
        )
        .await?;
    }

    let java_class_file_version = vm.java_class_file_version();
    let package_name = if java_class_file_version <= &JAVA_8 {
        "sun/misc"
    } else if java_class_file_version <= &JAVA_11 {
        "jdk/internal/misc"
    } else {
        "jdk/internal/access"
    };
    let java_lang_ref_access = thread
        .class(format!("{package_name}/JavaLangRefAccess"))
        .await?;
    let interfaces = vec![java_lang_ref_access];
    let mut methods = Vec::new();
    let start_threads = Method::new(
        MethodAccessFlags::PUBLIC,
        "startThreads",
        "()V",
        0,
        1,
        vec![Instruction::Return],
        vec![],
        vec![],
    )?;
    methods.push(start_threads);
    let java_lang_ref_access = Arc::new(Class::new(
        "ristretto.internal.access.JavaLangRefAccess".to_string(),
        None,
        ClassFile::default(),
        None,
        interfaces,
        Vec::new(),
        methods,
    ));
    thread.register_class(java_lang_ref_access.clone()).await?;
    let java_lang_ref_access =
        Value::Object(Some(Reference::Object(Object::new(java_lang_ref_access)?)));
    vm.invoke(
        format!("{package_name}/SharedSecrets"),
        "setJavaLangRefAccess",
        format!("(L{package_name}/JavaLangRefAccess;)V"),
        vec![java_lang_ref_access],
    )
    .await?;

    Ok(None)
}

#[async_recursion(?Send)]
async fn set_in_0(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let input_stream = arguments.pop_reference()?;
    let system = thread.class("java.lang.System").await?;
    let in_field = system.static_field("in")?;
    in_field.unsafe_set_value(Value::Object(input_stream))?;
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_out_0(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let print_stream = arguments.pop_reference()?;
    let system = thread.class("java.lang.System").await?;
    let out_field = system.static_field("out")?;
    out_field.unsafe_set_value(Value::Object(print_stream))?;
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_err_0(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let print_stream = arguments.pop_reference()?;
    let system = thread.class("java.lang.System").await?;
    let err_field = system.static_field("err")?;
    err_field.unsafe_set_value(Value::Object(print_stream))?;
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_security_manager(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    // The SecurityManager is not supported in Ristretto.
    Err(InternalError(
        "SecurityManager is not supported".to_string(),
    ))
}
