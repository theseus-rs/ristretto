use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `sun.nio.ch.Net`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/Net";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(
            class_name,
            "isReusePortAvailable0",
            "()Z",
            is_reuse_port_available_0,
        );
    }

    if java_version >= JAVA_17 {
        registry.register(
            class_name,
            "accept",
            "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I",
            accept,
        );
        registry.register(
            class_name,
            "available",
            "(Ljava/io/FileDescriptor;)I",
            available,
        );
        registry.register(
            class_name,
            "canUseIPv6OptionsWithIPv4LocalAddress0",
            "()Z",
            can_use_ipv6_options_with_ipv4_local_address_0,
        );
        registry.register(
            class_name,
            "discardOOB",
            "(Ljava/io/FileDescriptor;)Z",
            discard_oob,
        );
        registry.register(
            class_name,
            "pollConnect",
            "(Ljava/io/FileDescriptor;J)Z",
            poll_connect,
        );
        registry.register(
            class_name,
            "sendOOB",
            "(Ljava/io/FileDescriptor;B)I",
            send_oob,
        );
        registry.register(
            class_name,
            "shouldSetBothIPv4AndIPv6Options0",
            "()Z",
            should_set_both_ipv4_and_ipv6_options_0,
        );
    }

    registry.register(
        class_name,
        "bind0",
        "(Ljava/io/FileDescriptor;ZZLjava/net/InetAddress;I)V",
        bind_0,
    );
    registry.register(
        class_name,
        "blockOrUnblock4",
        "(ZLjava/io/FileDescriptor;III)I",
        block_or_unblock_4,
    );
    registry.register(
        class_name,
        "blockOrUnblock6",
        "(ZLjava/io/FileDescriptor;[BI[B)I",
        block_or_unblock_6,
    );
    registry.register(
        class_name,
        "canIPv6SocketJoinIPv4Group0",
        "()Z",
        can_ipv6_socket_join_ipv4_group_0,
    );
    registry.register(
        class_name,
        "canJoin6WithIPv4Group0",
        "()Z",
        can_join_6_with_ipv4_group_0,
    );
    registry.register(
        class_name,
        "connect0",
        "(ZLjava/io/FileDescriptor;Ljava/net/InetAddress;I)I",
        connect_0,
    );
    registry.register(
        class_name,
        "getIntOption0",
        "(Ljava/io/FileDescriptor;ZII)I",
        get_int_option_0,
    );
    registry.register(
        class_name,
        "getInterface4",
        "(Ljava/io/FileDescriptor;)I",
        get_interface_4,
    );
    registry.register(
        class_name,
        "getInterface6",
        "(Ljava/io/FileDescriptor;)I",
        get_interface_6,
    );
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "isExclusiveBindAvailable",
        "()I",
        is_exclusive_bind_available,
    );
    registry.register(class_name, "isIPv6Available0", "()Z", is_ipv6_available_0);
    registry.register(
        class_name,
        "joinOrDrop4",
        "(ZLjava/io/FileDescriptor;III)I",
        join_or_drop_4,
    );
    registry.register(
        class_name,
        "joinOrDrop6",
        "(ZLjava/io/FileDescriptor;[BI[B)I",
        join_or_drop_6,
    );
    registry.register(class_name, "listen", "(Ljava/io/FileDescriptor;I)V", listen);
    registry.register(
        class_name,
        "localInetAddress",
        "(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;",
        local_inet_address,
    );
    registry.register(
        class_name,
        "localPort",
        "(Ljava/io/FileDescriptor;)I",
        local_port,
    );
    registry.register(class_name, "poll", "(Ljava/io/FileDescriptor;IJ)I", poll);
    registry.register(class_name, "pollconnValue", "()S", pollconn_value);
    registry.register(class_name, "pollerrValue", "()S", pollerr_value);
    registry.register(class_name, "pollhupValue", "()S", pollhup_value);
    registry.register(class_name, "pollinValue", "()S", pollin_value);
    registry.register(class_name, "pollnvalValue", "()S", pollnval_value);
    registry.register(class_name, "polloutValue", "()S", pollout_value);
    registry.register(
        class_name,
        "remoteInetAddress",
        "(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;",
        remote_inet_address,
    );
    registry.register(
        class_name,
        "remotePort",
        "(Ljava/io/FileDescriptor;)I",
        remote_port,
    );
    registry.register(
        class_name,
        "setIntOption0",
        "(Ljava/io/FileDescriptor;ZIIIZ)V",
        set_int_option_0,
    );
    registry.register(
        class_name,
        "setInterface4",
        "(Ljava/io/FileDescriptor;I)V",
        set_interface_4,
    );
    registry.register(
        class_name,
        "setInterface6",
        "(Ljava/io/FileDescriptor;I)V",
        set_interface_6,
    );
    registry.register(
        class_name,
        "shutdown",
        "(Ljava/io/FileDescriptor;I)V",
        shutdown,
    );
    registry.register(class_name, "socket0", "(ZZZZ)I", socket_0);
}

#[async_recursion(?Send)]
async fn accept(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.accept(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I")
}

#[async_recursion(?Send)]
async fn available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.available(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn bind_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.bind0(Ljava/io/FileDescriptor;ZZLjava/net/InetAddress;I)V")
}

#[async_recursion(?Send)]
async fn block_or_unblock_4(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.blockOrUnblock4(ZLjava/io/FileDescriptor;III)I")
}

#[async_recursion(?Send)]
async fn block_or_unblock_6(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.blockOrUnblock6(ZLjava/io/FileDescriptor;[BI[B)I")
}

#[async_recursion(?Send)]
async fn can_ipv6_socket_join_ipv4_group_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.canIPv6SocketJoinIPv4Group0()Z")
}

#[async_recursion(?Send)]
async fn can_join_6_with_ipv4_group_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.canJoin6WithIPv4Group0()Z")
}

#[async_recursion(?Send)]
async fn can_use_ipv6_options_with_ipv4_local_address_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.canUseIPv6OptionsWithIPv4LocalAddress0()Z")
}

#[async_recursion(?Send)]
async fn connect_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.connect0(ZLjava/io/FileDescriptor;Ljava/net/InetAddress;I)I")
}

#[async_recursion(?Send)]
async fn discard_oob(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.discardOOB(Ljava/io/FileDescriptor;)Z")
}

#[async_recursion(?Send)]
async fn get_int_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.getIntOption0(Ljava/io/FileDescriptor;ZII)I")
}

#[async_recursion(?Send)]
async fn get_interface_4(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.getInterface4(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn get_interface_6(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.getInterface6(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_exclusive_bind_available(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.isExclusiveBindAvailable()I")
}

#[async_recursion(?Send)]
async fn is_ipv6_available_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.isIPv6Available0()Z")
}

#[async_recursion(?Send)]
async fn is_reuse_port_available_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.isReusePortAvailable0()Z")
}

#[async_recursion(?Send)]
async fn join_or_drop_4(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.joinOrDrop4(ZLjava/io/FileDescriptor;III)I")
}

#[async_recursion(?Send)]
async fn join_or_drop_6(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.joinOrDrop6(ZLjava/io/FileDescriptor;[BI[B)I")
}

#[async_recursion(?Send)]
async fn listen(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.listen(Ljava/io/FileDescriptor;I)V")
}

#[async_recursion(?Send)]
async fn local_inet_address(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.localInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;")
}

#[async_recursion(?Send)]
async fn local_port(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.localPort(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn poll(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.poll(Ljava/io/FileDescriptor;IJ)I")
}

#[async_recursion(?Send)]
async fn poll_connect(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollConnect(Ljava/io/FileDescriptor;J)Z")
}

#[async_recursion(?Send)]
async fn pollconn_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollconnValue()S")
}

#[async_recursion(?Send)]
async fn pollerr_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollerrValue()S")
}

#[async_recursion(?Send)]
async fn pollhup_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollhupValue()S")
}

#[async_recursion(?Send)]
async fn pollin_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollinValue()S")
}

#[async_recursion(?Send)]
async fn pollnval_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollnvalValue()S")
}

#[async_recursion(?Send)]
async fn pollout_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.polloutValue()S")
}

#[async_recursion(?Send)]
async fn remote_inet_address(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.remoteInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;")
}

#[async_recursion(?Send)]
async fn remote_port(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.remotePort(Ljava/io/FileDescriptor;)I")
}

#[async_recursion(?Send)]
async fn send_oob(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.sendOOB(Ljava/io/FileDescriptor;B)I")
}

#[async_recursion(?Send)]
async fn set_int_option_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.setIntOption0(Ljava/io/FileDescriptor;ZIIIZ)V")
}

#[async_recursion(?Send)]
async fn set_interface_4(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.setInterface4(Ljava/io/FileDescriptor;I)V")
}

#[async_recursion(?Send)]
async fn set_interface_6(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.setInterface6(Ljava/io/FileDescriptor;I)V")
}

#[async_recursion(?Send)]
async fn should_set_both_ipv4_and_ipv6_options_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.shouldSetBothIPv4AndIPv6Options0()Z")
}

#[async_recursion(?Send)]
async fn shutdown(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.shutdown(Ljava/io/FileDescriptor;I)V")
}

#[async_recursion(?Send)]
async fn socket_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.socket0(ZZZZ)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java17 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/nio/ch/Net";
        assert!(registry
            .method(class_name, "isReusePortAvailable0", "()Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "accept",
                "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I"
            )
            .is_some());
        assert!(registry
            .method(class_name, "available", "(Ljava/io/FileDescriptor;)I")
            .is_some());
        assert!(registry
            .method(class_name, "canUseIPv6OptionsWithIPv4LocalAddress0", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "discardOOB", "(Ljava/io/FileDescriptor;)Z")
            .is_some());
        assert!(registry
            .method(class_name, "pollConnect", "(Ljava/io/FileDescriptor;J)Z")
            .is_some());
        assert!(registry
            .method(class_name, "sendOOB", "(Ljava/io/FileDescriptor;B)I")
            .is_some());
        assert!(registry
            .method(class_name, "shouldSetBothIPv4AndIPv6Options0", "()Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "bind0",
                "(Ljava/io/FileDescriptor;ZZLjava/net/InetAddress;I)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "blockOrUnblock4",
                "(ZLjava/io/FileDescriptor;III)I"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "blockOrUnblock6",
                "(ZLjava/io/FileDescriptor;[BI[B)I"
            )
            .is_some());
        assert!(registry
            .method(class_name, "canIPv6SocketJoinIPv4Group0", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "canJoin6WithIPv4Group0", "()Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "connect0",
                "(ZLjava/io/FileDescriptor;Ljava/net/InetAddress;I)I"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getIntOption0",
                "(Ljava/io/FileDescriptor;ZII)I"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getInterface4", "(Ljava/io/FileDescriptor;)I")
            .is_some());
        assert!(registry
            .method(class_name, "getInterface6", "(Ljava/io/FileDescriptor;)I")
            .is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "isExclusiveBindAvailable", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "isIPv6Available0", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "joinOrDrop4", "(ZLjava/io/FileDescriptor;III)I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "joinOrDrop6",
                "(ZLjava/io/FileDescriptor;[BI[B)I"
            )
            .is_some());
        assert!(registry
            .method(class_name, "listen", "(Ljava/io/FileDescriptor;I)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "localInetAddress",
                "(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "localPort", "(Ljava/io/FileDescriptor;)I")
            .is_some());
        assert!(registry
            .method(class_name, "poll", "(Ljava/io/FileDescriptor;IJ)I")
            .is_some());
        assert!(registry
            .method(class_name, "pollconnValue", "()S")
            .is_some());
        assert!(registry.method(class_name, "pollerrValue", "()S").is_some());
        assert!(registry.method(class_name, "pollhupValue", "()S").is_some());
        assert!(registry.method(class_name, "pollinValue", "()S").is_some());
        assert!(registry
            .method(class_name, "pollnvalValue", "()S")
            .is_some());
        assert!(registry.method(class_name, "polloutValue", "()S").is_some());
        assert!(registry
            .method(
                class_name,
                "remoteInetAddress",
                "(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "remotePort", "(Ljava/io/FileDescriptor;)I")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "setIntOption0",
                "(Ljava/io/FileDescriptor;ZIIIZ)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "setInterface4", "(Ljava/io/FileDescriptor;I)V")
            .is_some());
        assert!(registry
            .method(class_name, "setInterface6", "(Ljava/io/FileDescriptor;I)V")
            .is_some());
        assert!(registry
            .method(class_name, "shutdown", "(Ljava/io/FileDescriptor;I)V")
            .is_some());
        assert!(registry.method(class_name, "socket0", "(ZZZZ)I").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.Net.accept(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I"
    )]
    async fn test_accept() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = accept(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.available(Ljava/io/FileDescriptor;)I")]
    async fn test_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = available(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.Net.bind0(Ljava/io/FileDescriptor;ZZLjava/net/InetAddress;I)V"
    )]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bind_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.blockOrUnblock4(ZLjava/io/FileDescriptor;III)I")]
    async fn test_block_or_unblock_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = block_or_unblock_4(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.blockOrUnblock6(ZLjava/io/FileDescriptor;[BI[B)I")]
    async fn test_block_or_unblock_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = block_or_unblock_6(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.canIPv6SocketJoinIPv4Group0()Z")]
    async fn test_can_ipv6_socket_join_ipv4_group_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_ipv6_socket_join_ipv4_group_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.canJoin6WithIPv4Group0()Z")]
    async fn test_can_join_6_with_ipv4_group_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_join_6_with_ipv4_group_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.canUseIPv6OptionsWithIPv4LocalAddress0()Z")]
    async fn test_can_use_ipv6_options_with_ipv4_local_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_use_ipv6_options_with_ipv4_local_address_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.Net.connect0(ZLjava/io/FileDescriptor;Ljava/net/InetAddress;I)I"
    )]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.discardOOB(Ljava/io/FileDescriptor;)Z")]
    async fn test_discard_oob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = discard_oob(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.getIntOption0(Ljava/io/FileDescriptor;ZII)I")]
    async fn test_get_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.getInterface4(Ljava/io/FileDescriptor;)I")]
    async fn test_get_interface_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_interface_4(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.getInterface6(Ljava/io/FileDescriptor;)I")]
    async fn test_get_interface_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_interface_6(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.isExclusiveBindAvailable()I")]
    async fn test_is_exclusive_bind_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_exclusive_bind_available(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.isIPv6Available0()Z")]
    async fn test_is_ipv6_available_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_ipv6_available_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.isReusePortAvailable0()Z")]
    async fn test_is_reuse_port_available_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_reuse_port_available_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.joinOrDrop4(ZLjava/io/FileDescriptor;III)I")]
    async fn test_join_or_drop_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = join_or_drop_4(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.joinOrDrop6(ZLjava/io/FileDescriptor;[BI[B)I")]
    async fn test_join_or_drop_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = join_or_drop_6(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.listen(Ljava/io/FileDescriptor;I)V")]
    async fn test_listen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = listen(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.Net.localInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;"
    )]
    async fn test_local_inet_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = local_inet_address(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.localPort(Ljava/io/FileDescriptor;)I")]
    async fn test_local_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = local_port(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.poll(Ljava/io/FileDescriptor;IJ)I")]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.pollConnect(Ljava/io/FileDescriptor;J)Z")]
    async fn test_poll_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll_connect(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.pollconnValue()S")]
    async fn test_pollconn_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollconn_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.pollerrValue()S")]
    async fn test_pollerr_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollerr_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.pollhupValue()S")]
    async fn test_pollhup_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollhup_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.pollinValue()S")]
    async fn test_pollin_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollin_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.pollnvalValue()S")]
    async fn test_pollnval_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollnval_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.polloutValue()S")]
    async fn test_pollout_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollout_value(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.Net.remoteInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;"
    )]
    async fn test_remote_inet_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remote_inet_address(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.remotePort(Ljava/io/FileDescriptor;)I")]
    async fn test_remote_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remote_port(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.sendOOB(Ljava/io/FileDescriptor;B)I")]
    async fn test_send_oob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_oob(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.setIntOption0(Ljava/io/FileDescriptor;ZIIIZ)V")]
    async fn test_set_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_int_option_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.setInterface4(Ljava/io/FileDescriptor;I)V")]
    async fn test_set_interface_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_interface_4(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.setInterface6(Ljava/io/FileDescriptor;I)V")]
    async fn test_set_interface_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_interface_6(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.shouldSetBothIPv4AndIPv6Options0()Z")]
    async fn test_should_set_both_ipv4_and_ipv6_options_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = should_set_both_ipv4_and_ipv6_options_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.shutdown(Ljava/io/FileDescriptor;I)V")]
    async fn test_shutdown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = shutdown(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.Net.socket0(ZZZZ)I")]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_0(thread, Arguments::default()).await;
    }
}
