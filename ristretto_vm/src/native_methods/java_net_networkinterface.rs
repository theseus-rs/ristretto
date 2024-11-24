use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.NetworkInterface`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/NetworkInterface";
    registry.register(
        class_name,
        "getAll",
        "()[Ljava/net/NetworkInterface;",
        get_all,
    );
    registry.register(
        class_name,
        "getByIndex0",
        "(I)Ljava/net/NetworkInterface;",
        get_by_index_0,
    );
    registry.register(
        class_name,
        "getByInetAddress0",
        "(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;",
        get_by_inet_address_0,
    );
    registry.register(
        class_name,
        "getByName0",
        "(Ljava/lang/String;)Ljava/net/NetworkInterface;",
        get_by_name_0,
    );
    registry.register(class_name, "getMTU0", "(Ljava/lang/String;I)I", get_mtu_0);
    registry.register(
        class_name,
        "getMacAddr0",
        "([BLjava/lang/String;I)[B",
        get_mac_addr_0,
    );
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "isLoopback0",
        "(Ljava/lang/String;I)Z",
        is_loopback_0,
    );
    registry.register(class_name, "isP2P0", "(Ljava/lang/String;I)Z", is_p_2_p_0);
    registry.register(class_name, "isUp0", "(Ljava/lang/String;I)Z", is_up_0);
    registry.register(
        class_name,
        "supportsMulticast0",
        "(Ljava/lang/String;I)Z",
        supports_multicast_0,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_all(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_by_index_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_by_inet_address_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_by_name_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_mtu_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_mac_addr_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_loopback_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_p_2_p_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_up_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn supports_multicast_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
