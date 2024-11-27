use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_18: Version = Version::Java18 { minor: 0 };
const JAVA_19: Version = Version::Java19 { minor: 0 };

/// Register all native methods for `java.net.InetAddress`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/InetAddress";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_18 {
        registry.register(class_name, "isIPv4Available", "()Z", is_i_pv_4_available);
    }
    if java_version >= JAVA_19 {
        registry.register(class_name, "isIPv6Supported", "()Z", is_i_pv_6_supported);
    }

    registry.register(class_name, "init", "()V", init);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_i_pv_4_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_i_pv_6_supported(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
