use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `java.net.Inet6AddressImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/Inet6AddressImpl";
    let java_version = registry.java_version();

    if java_version <= &JAVA_17 {
        registry.register(
            class_name,
            "lookupAllHostAddr",
            "(Ljava/lang/String;)[Ljava/net/InetAddress;",
            lookup_all_host_addr,
        );
    } else {
        registry.register(
            class_name,
            "lookupAllHostAddr",
            "(Ljava/lang/String;I)[Ljava/net/InetAddress;",
            lookup_all_host_addr,
        );
    }

    registry.register(
        class_name,
        "getHostByAddr",
        "([B)Ljava/lang/String;",
        get_host_by_addr,
    );
    registry.register(
        class_name,
        "getLocalHostName",
        "()Ljava/lang/String;",
        get_local_host_name,
    );
    registry.register(class_name, "isReachable0", "([BII[BII)Z", is_reachable_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_host_by_addr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_local_host_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_reachable_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn lookup_all_host_addr(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
