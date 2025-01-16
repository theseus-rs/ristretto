use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::properties;
use crate::native_methods::registry::{MethodRegistry, JAVA_19};
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{ConcurrentVec, Reference, Value};
use std::collections::HashMap;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/util/SystemProps$Raw";

/// Register all native methods for `jdk.internal.util.SystemProps$Raw`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "platformProperties",
        "()[Ljava/lang/String;",
        platform_properties,
    );
    registry.register(
        CLASS_NAME,
        "vmProperties",
        "()[Ljava/lang/String;",
        vm_properties,
    );
}

#[async_recursion(?Send)]
async fn platform_properties(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let string_array_class = thread.class("[Ljava/lang/String;").await?;
    let system_properties = &mut properties::system(&thread).await?;
    let java_version = vm.java_major_version();

    // VM properties must be returned in a specific order as they are accessed by array index.
    let mut properties: Vec<Option<Reference>> = Vec::new();
    push_property(system_properties, &mut properties, "user.country")?;
    push_property(system_properties, &mut properties, "user.language")?;
    push_property(system_properties, &mut properties, "user.script")?;
    push_property(system_properties, &mut properties, "user.variant")?;
    push_property(system_properties, &mut properties, "native.encoding")?;
    push_property(system_properties, &mut properties, "file.separator")?;
    push_property(system_properties, &mut properties, "format.country")?;
    push_property(system_properties, &mut properties, "format.language")?;
    push_property(system_properties, &mut properties, "format.script")?;
    push_property(system_properties, &mut properties, "format.variant")?;
    push_property(system_properties, &mut properties, "ftp.nonProxyHosts")?;
    push_property(system_properties, &mut properties, "ftp.proxyHost")?;
    push_property(system_properties, &mut properties, "ftp.proxyPort")?;
    push_property(system_properties, &mut properties, "http.nonProxyHosts")?;
    push_property(system_properties, &mut properties, "http.proxyHost")?;
    push_property(system_properties, &mut properties, "http.proxyPort")?;
    push_property(system_properties, &mut properties, "https.proxyHost")?;
    push_property(system_properties, &mut properties, "https.proxyPort")?;
    push_property(system_properties, &mut properties, "java.io.tmpdir")?;
    push_property(system_properties, &mut properties, "line.separator")?;
    push_property(system_properties, &mut properties, "os.arch")?;
    push_property(system_properties, &mut properties, "os.name")?;
    push_property(system_properties, &mut properties, "os.version")?;
    push_property(system_properties, &mut properties, "path.separator")?;
    push_property(system_properties, &mut properties, "socksNonProxyHosts")?;
    push_property(system_properties, &mut properties, "socksProxyHost")?;
    push_property(system_properties, &mut properties, "socksProxyPort")?;
    if java_version >= JAVA_19 {
        push_property(system_properties, &mut properties, "stderr.encoding")?;
        push_property(system_properties, &mut properties, "stdout.encoding")?;
    }
    push_property(system_properties, &mut properties, "sun.arch.abi")?;
    push_property(system_properties, &mut properties, "sun.arch.data.model")?;
    push_property(system_properties, &mut properties, "sun.cpu.endian")?;
    push_property(system_properties, &mut properties, "sun.cpu.isalist")?;
    push_property(
        system_properties,
        &mut properties,
        "sun.io.unicode.encoding",
    )?;
    push_property(system_properties, &mut properties, "sun.jnu.encoding")?;
    push_property(system_properties, &mut properties, "sun.os.patch.level")?;
    if java_version < JAVA_19 {
        push_property(system_properties, &mut properties, "sun.stderr.encoding")?;
        push_property(system_properties, &mut properties, "sun.stdout.encoding")?;
    }
    push_property(system_properties, &mut properties, "user.dir")?;
    push_property(system_properties, &mut properties, "user.home")?;
    push_property(system_properties, &mut properties, "user.name")?;

    let properties = ConcurrentVec::from(properties);
    let result = Value::Object(Some(Reference::Array(string_array_class, properties)));
    Ok(Some(result))
}

fn push_property(
    system_properties: &mut HashMap<&str, Value>,
    properties: &mut Vec<Option<Reference>>,
    property_name: &str,
) -> Result<()> {
    let Some(Value::Object(value)) = system_properties.remove(property_name) else {
        return Err(InternalError(format!(
            "Property not found: {property_name}"
        )));
    };
    properties.push(value);
    Ok(())
}

#[async_recursion(?Send)]
async fn vm_properties(thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let java_home = vm.java_home();
    let class_path = vm.configuration().class_path().to_string();
    let vm_version = env!("CARGO_PKG_VERSION");
    let java_version = vm.java_version();
    let architecture_bits = usize::BITS;
    let vm_name =
        format!("ristretto {vm_version} (Java {java_version}) {architecture_bits}-bit VM");
    let mut system_properties = vm.system_properties().clone();
    system_properties.insert(
        "java.home".to_string(),
        java_home.to_string_lossy().as_ref().to_string(),
    );
    system_properties.insert("java.class.path".to_string(), class_path);
    system_properties.insert(
        "java.vm.specification.name".to_string(),
        "Java Virtual Machine Specification".to_string(),
    );
    system_properties.insert("java.vm.version".to_string(), vm_version.to_string());
    system_properties.insert("java.vm.name".to_string(), vm_name);

    let mut properties: Vec<Option<Reference>> = Vec::new();
    for (key, value) in system_properties {
        let Value::Object(key) = key.to_object(&vm).await? else {
            return Err(InternalError(format!(
                "Unable to convert key to string: {key}"
            )));
        };
        properties.push(key);
        let Value::Object(value) = value.to_object(&vm).await? else {
            return Err(InternalError(format!(
                "Unable to convert value to string: {value}"
            )));
        };
        properties.push(value);
    }

    let string_array_class = thread.class("[Ljava/lang/String;").await?;
    let properties = ConcurrentVec::from(properties);
    let result = Value::Object(Some(Reference::Array(string_array_class, properties)));
    Ok(Some(result))
}
