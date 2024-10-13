use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::properties;
use crate::native_methods::registry::MethodRegistry;
use crate::Error::RuntimeError;
use crate::Result;
use indexmap::IndexMap;
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{ClassFile, MethodAccessFlags};
use ristretto_classloader::{Class, ConcurrentVec, Method, Object, Reference, Value};
use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Register all native methods for java.lang.System
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
        "initProperties",
        "(Ljava/util/Properties;)Ljava/util/Properties;",
        init_properties,
    );
    registry.register(class_name, "nanoTime", "()J", nano_time);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(class_name, "setIn0", "(Ljava/io/InputStream;)V", set_in_0);
    registry.register(class_name, "setOut0", "(Ljava/io/PrintStream;)V", set_out_0);
    registry.register(class_name, "setErr0", "(Ljava/io/PrintStream;)V", set_err_0);
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
            return Err(RuntimeError("invalid source array index".to_string()));
        };
        destination.set(destination_position, value)?;
        destination_position += 1;
    }
    Ok(())
}

fn arraycopy(_call_stack: &Arc<CallStack>, mut arguments: Arguments) -> Result<Option<Value>> {
    let length = arguments.pop_int()?;
    let destination_position = arguments.pop_int()?;
    let Some(destination) = arguments.pop_object()? else {
        return Err(RuntimeError("destination must be an object".to_string()));
    };
    let source_position = arguments.pop_int()?;
    let Some(source) = arguments.pop_object()? else {
        return Err(RuntimeError("source must be an object".to_string()));
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
            return Err(RuntimeError(
                "source and destination must be arrays of the same type".to_string(),
            ))
        }
    };
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn allow_security_manager(
    _call_stack: &Arc<CallStack>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[expect(clippy::needless_pass_by_value)]
fn current_time_millis(
    _call_stack: &Arc<CallStack>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|error| RuntimeError(error.to_string()))?;
    let time = i64::try_from(duration.as_millis())?;
    Ok(Some(Value::Long(time)))
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn gc(_call_stack: &Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

/// Mechanism for initializing properties for Java versions <= 8
fn init_properties(call_stack: &Arc<CallStack>, mut arguments: Arguments) -> Result<Option<Value>> {
    let properties = arguments.pop_object()?;
    let _system_properties = &mut properties::system(call_stack)?;
    // TODO: add system properties to the properties object
    Ok(Some(Value::Object(properties)))
}

#[expect(clippy::needless_pass_by_value)]
fn nano_time(_call_stack: &Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|error| RuntimeError(error.to_string()))?;
    let time = i64::try_from(duration.as_nanos())?;
    Ok(Some(Value::Long(time)))
}

#[expect(clippy::needless_pass_by_value)]
fn register_natives(call_stack: &Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
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
    let vm = call_stack.vm()?;
    let system = vm.class(call_stack, "java/lang/System")?;
    let set_properties = system.try_get_method("setProperties", "(Ljava/util/Properties;)V")?;
    vm.invoke(&system, &set_properties, vec![Value::Object(None)])?;

    // TODO: remove this once threading is implemented
    let jdk_java_lang_ref_access = vm.class(call_stack, "jdk/internal/access/JavaLangRefAccess")?;
    let interfaces = vec![jdk_java_lang_ref_access];
    let mut methods = HashMap::new();
    let start_threads = Method::new(
        MethodAccessFlags::PUBLIC,
        "startThreads",
        "()V",
        0,
        1,
        vec![Instruction::Return],
        vec![],
    )?;
    methods.insert(
        format!("{}:{}", start_threads.name(), start_threads.descriptor()),
        Arc::new(start_threads),
    );
    let java_lang_ref_access = Arc::new(Class::new(
        "ristretto/internal/access/JavaLangRefAccess".to_string(),
        None,
        ClassFile::default(),
        None,
        interfaces,
        IndexMap::new(),
        methods,
    ));
    vm.register_class(java_lang_ref_access.clone())?;
    let java_lang_ref_access =
        Value::Object(Some(Reference::Object(Object::new(java_lang_ref_access)?)));
    let shared_secrets = vm.class(call_stack, "jdk/internal/access/SharedSecrets")?;
    let set_java_lang_ref_access = shared_secrets.try_get_method(
        "setJavaLangRefAccess",
        "(Ljdk/internal/access/JavaLangRefAccess;)V",
    )?;
    vm.invoke(
        &shared_secrets,
        &set_java_lang_ref_access,
        vec![java_lang_ref_access],
    )?;

    Ok(None)
}

fn set_in_0(call_stack: &Arc<CallStack>, mut arguments: Arguments) -> Result<Option<Value>> {
    let input_stream = arguments.pop_object()?;
    let vm = call_stack.vm()?;
    let system = vm.class(call_stack, "java/lang/System")?;
    let in_field = system.static_field("in")?;
    in_field.unsafe_set_value(Value::Object(input_stream))?;
    Ok(None)
}

fn set_out_0(call_stack: &Arc<CallStack>, mut arguments: Arguments) -> Result<Option<Value>> {
    let print_stream = arguments.pop_object()?;
    let vm = call_stack.vm()?;
    let system = vm.class(call_stack, "java/lang/System")?;
    let out_field = system.static_field("out")?;
    out_field.unsafe_set_value(Value::Object(print_stream))?;
    Ok(None)
}

fn set_err_0(call_stack: &Arc<CallStack>, mut arguments: Arguments) -> Result<Option<Value>> {
    let print_stream = arguments.pop_object()?;
    let vm = call_stack.vm()?;
    let system = vm.class(call_stack, "java/lang/System")?;
    let err_field = system.static_field("err")?;
    err_field.unsafe_set_value(Value::Object(print_stream))?;
    Ok(None)
}
