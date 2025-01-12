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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/nio/ch/sctp/SctpNet";
        assert!(registry
            .method(class_name, "bindx", "(I[Ljava/net/InetAddress;IIZZ)V")
            .is_some());
        assert!(registry.method(class_name, "branch0", "(II)I").is_some());
        assert!(registry.method(class_name, "close0", "(I)V").is_some());
        assert!(registry
            .method(class_name, "connect0", "(ILjava/net/InetAddress;I)I")
            .is_some());
        assert!(registry
            .method(class_name, "getInitMsgOption0", "(I[I)V")
            .is_some());
        assert!(registry
            .method(class_name, "getIntOption0", "(II)I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getLocalAddresses0",
                "(I)[Ljava/net/SocketAddress;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getPrimAddrOption0",
                "(II)Ljava/net/SocketAddress;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getRemoteAddresses0",
                "(II)[Ljava/net/SocketAddress;"
            )
            .is_some());
        assert!(registry.method(class_name, "init", "()V").is_some());
        assert!(registry.method(class_name, "listen0", "(II)V").is_some());
        assert!(registry.method(class_name, "preClose0", "(I)V").is_some());
        assert!(registry
            .method(class_name, "setInitMsgOption0", "(III)V")
            .is_some());
        assert!(registry
            .method(class_name, "setIntOption0", "(III)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "setPeerPrimAddrOption0",
                "(IILjava/net/InetAddress;IZ)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "setPrimAddrOption0",
                "(IILjava/net/InetAddress;I)V"
            )
            .is_some());
        assert!(registry.method(class_name, "shutdown0", "(II)V").is_some());
        assert!(registry.method(class_name, "socket0", "(Z)I").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V")]
    async fn test_bindx() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bindx(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.branch0(II)I")]
    async fn test_branch_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = branch_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.close0(I)V")]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.connect0(ILjava/net/InetAddress;I)I")]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.getInitMsgOption0(I[I)V")]
    async fn test_get_init_msg_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_init_msg_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.getIntOption0(II)I")]
    async fn test_get_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.sctp.SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;"
    )]
    async fn test_get_local_addresses_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_local_addresses_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.sctp.SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;"
    )]
    async fn test_get_prim_addr_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_prim_addr_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.sctp.SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;"
    )]
    async fn test_get_remote_addresses_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_remote_addresses_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.listen0(II)V")]
    async fn test_listen_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = listen_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.preClose0(I)V")]
    async fn test_pre_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pre_close_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.setInitMsgOption0(III)V")]
    async fn test_set_init_msg_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_init_msg_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.setIntOption0(III)V")]
    async fn test_set_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_int_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.sctp.SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V"
    )]
    async fn test_set_peer_prim_addr_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_peer_prim_addr_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.sctp.SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V"
    )]
    async fn test_set_prim_addr_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_prim_addr_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.shutdown0(II)V")]
    async fn test_shutdown_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = shutdown_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.sctp.SctpNet.socket0(Z)I")]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_0(thread, Arguments::default()).await;
    }
}
