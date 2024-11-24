use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.sctp.SctpNet`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/sctp/SctpNet";
    registry.register(
        class_name,
        "bindx",
        "(I[Ljava/net/InetAddress;IIZZ)V",
        bindx,
    );
    registry.register(class_name, "branch0", "(II)I", branch_0);
    registry.register(class_name, "close0", "(I)V", close_0);
    registry.register(
        class_name,
        "connect0",
        "(ILjava/net/InetAddress;I)I",
        connect_0,
    );
    registry.register(
        class_name,
        "getInitMsgOption0",
        "(I[I)V",
        get_init_msg_option_0,
    );
    registry.register(class_name, "getIntOption0", "(II)I", get_int_option_0);
    registry.register(
        class_name,
        "getLocalAddresses0",
        "(I)[Ljava/net/SocketAddress;",
        get_local_addresses_0,
    );
    registry.register(
        class_name,
        "getPrimAddrOption0",
        "(II)Ljava/net/SocketAddress;",
        get_prim_addr_option_0,
    );
    registry.register(
        class_name,
        "getRemoteAddresses0",
        "(II)[Ljava/net/SocketAddress;",
        get_remote_addresses_0,
    );
    registry.register(class_name, "init", "()V", init);
    registry.register(class_name, "listen0", "(II)V", listen_0);
    registry.register(class_name, "preClose0", "(I)V", pre_close_0);
    registry.register(
        class_name,
        "setInitMsgOption0",
        "(III)V",
        set_init_msg_option_0,
    );
    registry.register(class_name, "setIntOption0", "(III)V", set_int_option_0);
    registry.register(
        class_name,
        "setPeerPrimAddrOption0",
        "(IILjava/net/InetAddress;IZ)V",
        set_peer_prim_addr_option_0,
    );
    registry.register(
        class_name,
        "setPrimAddrOption0",
        "(IILjava/net/InetAddress;I)V",
        set_prim_addr_option_0,
    );
    registry.register(class_name, "shutdown0", "(II)V", shutdown_0);
    registry.register(class_name, "socket0", "(Z)I", socket_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn bindx(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn branch_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn connect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_init_msg_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_int_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_local_addresses_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_prim_addr_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_remote_addresses_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn listen_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn pre_close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_init_msg_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_int_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_peer_prim_addr_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_prim_addr_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn shutdown_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn socket_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
