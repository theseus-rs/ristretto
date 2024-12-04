use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.Inet4AddressImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/Inet4AddressImpl";
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
    registry.register(class_name, "isReachable0", "([BI[BI)Z", is_reachable_0);
    registry.register(
        class_name,
        "lookupAllHostAddr",
        "(Ljava/lang/String;)[Ljava/net/InetAddress;",
        lookup_all_host_addr,
    );
}

#[async_recursion(?Send)]
async fn get_host_by_addr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_local_host_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_reachable_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn lookup_all_host_addr(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
