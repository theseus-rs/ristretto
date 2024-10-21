use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::properties;
use crate::native_methods::registry::MethodRegistry;
use crate::Error::InternalError;
use crate::Result;
use ristretto_classfile::Version;
use ristretto_classloader::{ConcurrentVec, Reference, Value};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

const JAVA_19: Version = Version::Java19 { minor: 0 };

/// Register all native methods for jdk.internal.util.SystemProps$Raw.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/util/SystemProps$Raw";
    registry.register(
        class_name,
        "platformProperties",
        "()[Ljava/lang/String;",
        platform_properties,
    );
    registry.register(
        class_name,
        "vmProperties",
        "()[Ljava/lang/String;",
        vm_properties,
    );
}

#[expect(clippy::needless_pass_by_value)]
fn platform_properties(
    call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let vm = call_stack.vm()?;
        let string_array_class = vm.load_class(&call_stack, "[Ljava/lang/String;").await?;
        let system_properties = &mut properties::system(call_stack).await?;
        let java_version = vm.java_class_file_version();

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
        if java_version >= &JAVA_19 {
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
        if java_version < &JAVA_19 {
            push_property(system_properties, &mut properties, "sun.stderr.encoding")?;
            push_property(system_properties, &mut properties, "sun.stdout.encoding")?;
        }
        push_property(system_properties, &mut properties, "user.dir")?;
        push_property(system_properties, &mut properties, "user.home")?;
        push_property(system_properties, &mut properties, "user.name")?;

        let properties = ConcurrentVec::from(properties);
        let result = Value::Object(Some(Reference::Array(string_array_class, properties)));
        Ok(Some(result))
    })
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

#[expect(clippy::needless_pass_by_value)]
fn vm_properties(
    call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let vm = call_stack.vm()?;
        let java_home = vm.java_home();
        let string_array_class = vm.load_class(&call_stack, "[Ljava/lang/String;").await?;
        let mut system_properties = vm.system_properties().clone();
        system_properties.insert(
            "java.home".to_string(),
            java_home.to_string_lossy().to_string(),
        );

        let mut properties: Vec<Option<Reference>> = Vec::new();
        for (key, value) in system_properties {
            let Value::Object(key) = vm.to_string_value(&call_stack, &key).await? else {
                return Err(InternalError(format!(
                    "Unable to convert key to string: {key}"
                )));
            };
            properties.push(key);
            let Value::Object(value) = vm.to_string_value(&call_stack, value.as_str()).await?
            else {
                return Err(InternalError(format!(
                    "Unable to convert value to string: {value}"
                )));
            };
            properties.push(value);
        }

        let properties = ConcurrentVec::from(properties);
        let result = Value::Object(Some(Reference::Array(string_array_class, properties)));
        Ok(Some(result))
    })
}
