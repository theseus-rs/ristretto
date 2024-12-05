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

#[async_recursion(?Send)]
async fn bindx(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V")
}

#[async_recursion(?Send)]
async fn branch_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.branch0(II)I")
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.close0(I)V")
}

#[async_recursion(?Send)]
async fn connect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.connect0(ILjava/net/InetAddress;I)I")
}

#[async_recursion(?Send)]
async fn get_init_msg_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getInitMsgOption0(I[I)V")
}

#[async_recursion(?Send)]
async fn get_int_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getIntOption0(II)I")
}

#[async_recursion(?Send)]
async fn get_local_addresses_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;")
}

#[async_recursion(?Send)]
async fn get_prim_addr_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;")
}

#[async_recursion(?Send)]
async fn get_remote_addresses_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn listen_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.listen0(II)V")
}

#[async_recursion(?Send)]
async fn pre_close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.preClose0(I)V")
}

#[async_recursion(?Send)]
async fn set_init_msg_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.setInitMsgOption0(III)V")
}

#[async_recursion(?Send)]
async fn set_int_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.setIntOption0(III)V")
}

#[async_recursion(?Send)]
async fn set_peer_prim_addr_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V")
}

#[async_recursion(?Send)]
async fn set_prim_addr_option_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V")
}

#[async_recursion(?Send)]
async fn shutdown_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.shutdown0(II)V")
}

#[async_recursion(?Send)]
async fn socket_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.sctp.SctpNet.socket0(Z)I")
}
