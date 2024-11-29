use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.net.ExtendedOptionsImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/net/ExtendedOptionsImpl";
    registry.register(class_name, "flowSupported", "()Z", flow_supported);
    registry.register(
        class_name,
        "getFlowOption",
        "(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V",
        get_flow_option,
    );
    registry.register(
        class_name,
        "getTcpKeepAliveIntvl",
        "(Ljava/io/FileDescriptor;)I",
        get_tcp_keep_alive_intvl,
    );
    registry.register(
        class_name,
        "getTcpKeepAliveProbes",
        "(Ljava/io/FileDescriptor;)I",
        get_tcp_keep_alive_probes,
    );
    registry.register(
        class_name,
        "getTcpKeepAliveTime",
        "(Ljava/io/FileDescriptor;)I",
        get_tcp_keep_alive_time,
    );
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "keepAliveOptionsSupported",
        "()Z",
        keep_alive_options_supported,
    );
    registry.register(
        class_name,
        "setFlowOption",
        "(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V",
        set_flow_option,
    );
    registry.register(
        class_name,
        "setTcpKeepAliveIntvl",
        "(Ljava/io/FileDescriptor;I)V",
        set_tcp_keep_alive_intvl,
    );
    registry.register(
        class_name,
        "setTcpKeepAliveProbes",
        "(Ljava/io/FileDescriptor;I)V",
        set_tcp_keep_alive_probes,
    );
    registry.register(
        class_name,
        "setTcpKeepAliveTime",
        "(Ljava/io/FileDescriptor;I)V",
        set_tcp_keep_alive_time,
    );
}

#[async_recursion(?Send)]
async fn flow_supported(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_flow_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_intvl(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_probes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn keep_alive_options_supported(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_flow_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_intvl(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_probes(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
