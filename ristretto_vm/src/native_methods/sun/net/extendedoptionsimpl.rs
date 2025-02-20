use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/net/ExtendedOptionsImpl";

/// Register all native methods for `sun.net.ExtendedOptionsImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "flowSupported", "()Z", flow_supported);
    registry.register(
        CLASS_NAME,
        "getFlowOption",
        "(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V",
        get_flow_option,
    );
    registry.register(
        CLASS_NAME,
        "getTcpKeepAliveIntvl",
        "(Ljava/io/FileDescriptor;)I",
        get_tcp_keep_alive_intvl,
    );
    registry.register(
        CLASS_NAME,
        "getTcpKeepAliveProbes",
        "(Ljava/io/FileDescriptor;)I",
        get_tcp_keep_alive_probes,
    );
    registry.register(
        CLASS_NAME,
        "getTcpKeepAliveTime",
        "(Ljava/io/FileDescriptor;)I",
        get_tcp_keep_alive_time,
    );
    registry.register(CLASS_NAME, "init", "()V", init);
    registry.register(
        CLASS_NAME,
        "keepAliveOptionsSupported",
        "()Z",
        keep_alive_options_supported,
    );
    registry.register(
        CLASS_NAME,
        "setFlowOption",
        "(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V",
        set_flow_option,
    );
    registry.register(
        CLASS_NAME,
        "setTcpKeepAliveIntvl",
        "(Ljava/io/FileDescriptor;I)V",
        set_tcp_keep_alive_intvl,
    );
    registry.register(
        CLASS_NAME,
        "setTcpKeepAliveProbes",
        "(Ljava/io/FileDescriptor;I)V",
        set_tcp_keep_alive_probes,
    );
    registry.register(
        CLASS_NAME,
        "setTcpKeepAliveTime",
        "(Ljava/io/FileDescriptor;I)V",
        set_tcp_keep_alive_time,
    );
}

#[async_recursion(?Send)]
async fn flow_supported(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.flowSupported()Z")
}

#[async_recursion(?Send)]
async fn get_flow_option(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.net.ExtendedOptionsImpl.getFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V"
    )
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_intvl(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.getTcpKeepAliveIntvl(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_probes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.getTcpKeepAliveProbes(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_time(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.getTcpKeepAliveTime(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn keep_alive_options_supported(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.keepAliveOptionsSupported()Z")
}

#[async_recursion(?Send)]
async fn set_flow_option(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.net.ExtendedOptionsImpl.setFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V"
    )
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_intvl(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.setTcpKeepAliveIntvl(Ljava/io/FileDescriptor;I)V")
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_probes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.setTcpKeepAliveProbes(Ljava/io/FileDescriptor;I)V")
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_time(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.setTcpKeepAliveTime(Ljava/io/FileDescriptor;I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.net.ExtendedOptionsImpl.flowSupported()Z")]
    async fn test_flow_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flow_supported(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.getFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V"
    )]
    async fn test_get_flow_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_flow_option(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.getTcpKeepAliveIntvl(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_get_tcp_keep_alive_intvl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_intvl(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.getTcpKeepAliveProbes(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_get_tcp_keep_alive_probes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_probes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.getTcpKeepAliveTime(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_get_tcp_keep_alive_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.keepAliveOptionsSupported()Z"
    )]
    async fn test_keep_alive_options_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = keep_alive_options_supported(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.setFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V"
    )]
    async fn test_set_flow_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_flow_option(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.setTcpKeepAliveIntvl(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_set_tcp_keep_alive_intvl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_intvl(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.setTcpKeepAliveProbes(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_set_tcp_keep_alive_probes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_probes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.setTcpKeepAliveTime(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_set_tcp_keep_alive_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_time(thread, Parameters::default()).await;
    }
}
