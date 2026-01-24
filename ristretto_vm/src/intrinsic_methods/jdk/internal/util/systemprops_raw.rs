use crate::Error::InternalError;
use crate::Result;
use crate::intrinsic_methods::properties;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::collections::HashMap;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/util/SystemProps$Raw.platformProperties()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn platform_properties(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let string_array_class = thread.class("[Ljava/lang/String;").await?;
    let system_properties = &mut properties::system(&thread).await?;
    let java_version = vm.java_major_version();

    // VM properties must be returned in a specific order as they are accessed by array index.
    let mut properties: Vec<Value> = Vec::new();
    push_property(system_properties, &mut properties, "user.country")?;
    push_property(system_properties, &mut properties, "user.language")?;
    push_property(system_properties, &mut properties, "user.script")?;
    push_property(system_properties, &mut properties, "user.variant")?;
    if java_version < JAVA_25.java() {
        push_property(system_properties, &mut properties, "native.encoding")?;
    }
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
    if java_version >= JAVA_25.java() {
        push_property(system_properties, &mut properties, "native.encoding")?;
    }
    push_property(system_properties, &mut properties, "os.arch")?;
    push_property(system_properties, &mut properties, "os.name")?;
    push_property(system_properties, &mut properties, "os.version")?;
    push_property(system_properties, &mut properties, "path.separator")?;
    push_property(system_properties, &mut properties, "socksNonProxyHosts")?;
    push_property(system_properties, &mut properties, "socksProxyHost")?;
    push_property(system_properties, &mut properties, "socksProxyPort")?;
    if java_version >= JAVA_21.java() {
        push_property(system_properties, &mut properties, "stderr.encoding")?;
        if java_version >= JAVA_25.java() {
            push_property(system_properties, &mut properties, "stdin.encoding")?;
        }
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
    if java_version <= JAVA_17.java() {
        push_property(system_properties, &mut properties, "sun.stderr.encoding")?;
        push_property(system_properties, &mut properties, "sun.stdout.encoding")?;
    }
    push_property(system_properties, &mut properties, "user.dir")?;
    push_property(system_properties, &mut properties, "user.home")?;
    push_property(system_properties, &mut properties, "user.name")?;

    let result = Value::try_from((string_array_class, properties))?;
    Ok(Some(result))
}

fn push_property(
    system_properties: &mut HashMap<&str, Value>,
    properties: &mut Vec<Value>,
    property_name: &str,
) -> Result<()> {
    let Some(value) = system_properties.remove(property_name) else {
        return Err(InternalError(format!(
            "Property not found: {property_name}"
        )));
    };
    properties.push(value);
    Ok(())
}

#[intrinsic_method(
    "jdk/internal/util/SystemProps$Raw.vmProperties()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn vm_properties(
    thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
        java_home.to_string_lossy().to_string(),
    );
    system_properties.insert("java.class.path".to_string(), class_path);
    system_properties.insert(
        "java.vm.specification.name".to_string(),
        "Java Virtual Machine Specification".to_string(),
    );
    system_properties.insert("java.vm.version".to_string(), vm_version.to_string());
    system_properties.insert("java.vm.name".to_string(), vm_name);

    let mut properties: Vec<Value> = Vec::new();
    for (key, value) in system_properties {
        let key = key.to_object(&thread).await?;
        let value = value.to_object(&thread).await?;
        properties.push(key);
        properties.push(value);
    }

    let string_array_class = thread.class("[Ljava/lang/String;").await?;
    let result = Value::try_from((string_array_class, properties))?;
    Ok(Some(result))
}
