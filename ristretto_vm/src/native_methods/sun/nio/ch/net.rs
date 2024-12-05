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
