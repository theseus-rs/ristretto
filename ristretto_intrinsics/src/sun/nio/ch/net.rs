use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/nio/ch/Net.accept(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn accept<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.nio.ch.Net.accept(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I"
    )
}

#[intrinsic_method(
    "sun/nio/ch/Net.available(Ljava/io/FileDescriptor;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn available<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.available(Ljava/io/FileDescriptor;)I")
}

#[intrinsic_method(
    "sun/nio/ch/Net.bind0(Ljava/io/FileDescriptor;ZZLjava/net/InetAddress;I)V",
    Any
)]
#[async_method]
pub async fn bind_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.bind0(Ljava/io/FileDescriptor;ZZLjava/net/InetAddress;I)V")
}

#[intrinsic_method("sun/nio/ch/Net.blockOrUnblock4(ZLjava/io/FileDescriptor;III)I", Any)]
#[async_method]
pub async fn block_or_unblock_4<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.blockOrUnblock4(ZLjava/io/FileDescriptor;III)I")
}

#[intrinsic_method("sun/nio/ch/Net.blockOrUnblock6(ZLjava/io/FileDescriptor;[BI[B)I", Any)]
#[async_method]
pub async fn block_or_unblock_6<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.blockOrUnblock6(ZLjava/io/FileDescriptor;[BI[B)I")
}

#[intrinsic_method("sun/nio/ch/Net.canIPv6SocketJoinIPv4Group0()Z", Any)]
#[async_method]
pub async fn can_ipv6_socket_join_ipv4_group_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.canIPv6SocketJoinIPv4Group0()Z")
}

#[intrinsic_method("sun/nio/ch/Net.canJoin6WithIPv4Group0()Z", Any)]
#[async_method]
pub async fn can_join_6_with_ipv4_group_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.canJoin6WithIPv4Group0()Z")
}

#[intrinsic_method(
    "sun/nio/ch/Net.canUseIPv6OptionsWithIPv4LocalAddress0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn can_use_ipv6_options_with_ipv4_local_address_0<
    T: ristretto_types::Thread + 'static,
>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.canUseIPv6OptionsWithIPv4LocalAddress0()Z")
}

#[intrinsic_method(
    "sun/nio/ch/Net.connect0(ZLjava/io/FileDescriptor;Ljava/net/InetAddress;I)I",
    Any
)]
#[async_method]
pub async fn connect_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.connect0(ZLjava/io/FileDescriptor;Ljava/net/InetAddress;I)I")
}

#[intrinsic_method(
    "sun/nio/ch/Net.discardOOB(Ljava/io/FileDescriptor;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn discard_oob<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.discardOOB(Ljava/io/FileDescriptor;)Z")
}

#[intrinsic_method("sun/nio/ch/Net.getIntOption0(Ljava/io/FileDescriptor;ZII)I", Any)]
#[async_method]
pub async fn get_int_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.getIntOption0(Ljava/io/FileDescriptor;ZII)I")
}

#[intrinsic_method("sun/nio/ch/Net.getInterface4(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn get_interface_4<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.getInterface4(Ljava/io/FileDescriptor;)I")
}

#[intrinsic_method("sun/nio/ch/Net.getInterface6(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn get_interface_6<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.getInterface6(Ljava/io/FileDescriptor;)I")
}

#[intrinsic_method("sun/nio/ch/Net.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/Net.isExclusiveBindAvailable()I", Any)]
#[async_method]
pub async fn is_exclusive_bind_available<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.isExclusiveBindAvailable()I")
}

#[intrinsic_method("sun/nio/ch/Net.isIPv6Available0()Z", Any)]
#[async_method]
pub async fn is_ipv6_available_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.isIPv6Available0()Z")
}

#[intrinsic_method("sun/nio/ch/Net.isReusePortAvailable0()Z", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn is_reuse_port_available_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.isReusePortAvailable0()Z")
}

#[intrinsic_method("sun/nio/ch/Net.joinOrDrop4(ZLjava/io/FileDescriptor;III)I", Any)]
#[async_method]
pub async fn join_or_drop_4<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.joinOrDrop4(ZLjava/io/FileDescriptor;III)I")
}

#[intrinsic_method("sun/nio/ch/Net.joinOrDrop6(ZLjava/io/FileDescriptor;[BI[B)I", Any)]
#[async_method]
pub async fn join_or_drop_6<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.joinOrDrop6(ZLjava/io/FileDescriptor;[BI[B)I")
}

#[intrinsic_method("sun/nio/ch/Net.listen(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn listen<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.listen(Ljava/io/FileDescriptor;I)V")
}

#[intrinsic_method(
    "sun/nio/ch/Net.localInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;",
    Any
)]
#[async_method]
pub async fn local_inet_address<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.localInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;")
}

#[intrinsic_method("sun/nio/ch/Net.localPort(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn local_port<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.localPort(Ljava/io/FileDescriptor;)I")
}

#[intrinsic_method("sun/nio/ch/Net.poll(Ljava/io/FileDescriptor;IJ)I", Any)]
#[async_method]
pub async fn poll<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.poll(Ljava/io/FileDescriptor;IJ)I")
}

#[intrinsic_method(
    "sun/nio/ch/Net.pollConnect(Ljava/io/FileDescriptor;J)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn poll_connect<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollConnect(Ljava/io/FileDescriptor;J)Z")
}

#[intrinsic_method("sun/nio/ch/Net.pollconnValue()S", Any)]
#[async_method]
pub async fn pollconn_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollconnValue()S")
}

#[intrinsic_method("sun/nio/ch/Net.pollerrValue()S", Any)]
#[async_method]
pub async fn pollerr_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollerrValue()S")
}

#[intrinsic_method("sun/nio/ch/Net.pollhupValue()S", Any)]
#[async_method]
pub async fn pollhup_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollhupValue()S")
}

#[intrinsic_method("sun/nio/ch/Net.pollinValue()S", Any)]
#[async_method]
pub async fn pollin_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollinValue()S")
}

#[intrinsic_method("sun/nio/ch/Net.pollnvalValue()S", Any)]
#[async_method]
pub async fn pollnval_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.pollnvalValue()S")
}

#[intrinsic_method("sun/nio/ch/Net.polloutValue()S", Any)]
#[async_method]
pub async fn pollout_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.polloutValue()S")
}

#[intrinsic_method(
    "sun/nio/ch/Net.remoteInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;",
    Any
)]
#[async_method]
pub async fn remote_inet_address<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.remoteInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;")
}

#[intrinsic_method("sun/nio/ch/Net.remotePort(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn remote_port<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.remotePort(Ljava/io/FileDescriptor;)I")
}

#[intrinsic_method(
    "sun/nio/ch/Net.sendOOB(Ljava/io/FileDescriptor;B)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn send_oob<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.sendOOB(Ljava/io/FileDescriptor;B)I")
}

#[intrinsic_method("sun/nio/ch/Net.setIntOption0(Ljava/io/FileDescriptor;ZIIIZ)V", Any)]
#[async_method]
pub async fn set_int_option_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.setIntOption0(Ljava/io/FileDescriptor;ZIIIZ)V")
}

#[intrinsic_method("sun/nio/ch/Net.setInterface4(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn set_interface_4<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.setInterface4(Ljava/io/FileDescriptor;I)V")
}

#[intrinsic_method("sun/nio/ch/Net.setInterface6(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn set_interface_6<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.setInterface6(Ljava/io/FileDescriptor;I)V")
}

#[intrinsic_method(
    "sun/nio/ch/Net.shouldSetBothIPv4AndIPv6Options0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn should_set_both_ipv4_and_ipv6_options_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.shouldSetBothIPv4AndIPv6Options0()Z")
}

#[intrinsic_method("sun/nio/ch/Net.shutdown(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn shutdown<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.shutdown(Ljava/io/FileDescriptor;I)V")
}

#[intrinsic_method("sun/nio/ch/Net.socket0(ZZZZ)I", Any)]
#[async_method]
pub async fn socket_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.socket0(ZZZZ)I")
}

#[intrinsic_method(
    "sun/nio/ch/Net.shouldShutdownWriteBeforeClose0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn should_shutdown_write_before_close_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.Net.shouldShutdownWriteBeforeClose0()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.accept(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;[Ljava/net/InetSocketAddress;)I"
    )]
    async fn test_accept() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = accept(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.available(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = available(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.bind0(Ljava/io/FileDescriptor;ZZLjava/net/InetAddress;I)V"
    )]
    async fn test_bind_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bind_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.blockOrUnblock4(ZLjava/io/FileDescriptor;III)I"
    )]
    async fn test_block_or_unblock_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = block_or_unblock_4(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.blockOrUnblock6(ZLjava/io/FileDescriptor;[BI[B)I"
    )]
    async fn test_block_or_unblock_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = block_or_unblock_6(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.canIPv6SocketJoinIPv4Group0()Z")]
    async fn test_can_ipv6_socket_join_ipv4_group_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_ipv6_socket_join_ipv4_group_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.canJoin6WithIPv4Group0()Z")]
    async fn test_can_join_6_with_ipv4_group_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_join_6_with_ipv4_group_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.canUseIPv6OptionsWithIPv4LocalAddress0()Z"
    )]
    async fn test_can_use_ipv6_options_with_ipv4_local_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_use_ipv6_options_with_ipv4_local_address_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.connect0(ZLjava/io/FileDescriptor;Ljava/net/InetAddress;I)I"
    )]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = connect_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.discardOOB(Ljava/io/FileDescriptor;)Z"
    )]
    async fn test_discard_oob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = discard_oob(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.getIntOption0(Ljava/io/FileDescriptor;ZII)I"
    )]
    async fn test_get_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.getInterface4(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_get_interface_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_interface_4(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.getInterface6(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_get_interface_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_interface_6(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.isExclusiveBindAvailable()I")]
    async fn test_is_exclusive_bind_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_exclusive_bind_available(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.isIPv6Available0()Z")]
    async fn test_is_ipv6_available_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_ipv6_available_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.isReusePortAvailable0()Z")]
    async fn test_is_reuse_port_available_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_reuse_port_available_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.joinOrDrop4(ZLjava/io/FileDescriptor;III)I"
    )]
    async fn test_join_or_drop_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = join_or_drop_4(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.joinOrDrop6(ZLjava/io/FileDescriptor;[BI[B)I"
    )]
    async fn test_join_or_drop_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = join_or_drop_6(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.listen(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_listen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = listen(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.localInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;"
    )]
    async fn test_local_inet_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = local_inet_address(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.localPort(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_local_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = local_port(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.poll(Ljava/io/FileDescriptor;IJ)I"
    )]
    async fn test_poll() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.pollConnect(Ljava/io/FileDescriptor;J)Z"
    )]
    async fn test_poll_connect() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = poll_connect(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.pollconnValue()S")]
    async fn test_pollconn_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollconn_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.pollerrValue()S")]
    async fn test_pollerr_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollerr_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.pollhupValue()S")]
    async fn test_pollhup_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollhup_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.pollinValue()S")]
    async fn test_pollin_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollin_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.pollnvalValue()S")]
    async fn test_pollnval_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollnval_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.polloutValue()S")]
    async fn test_pollout_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pollout_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.remoteInetAddress(Ljava/io/FileDescriptor;)Ljava/net/InetAddress;"
    )]
    async fn test_remote_inet_address() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remote_inet_address(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.remotePort(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_remote_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remote_port(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.sendOOB(Ljava/io/FileDescriptor;B)I"
    )]
    async fn test_send_oob() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = send_oob(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.setIntOption0(Ljava/io/FileDescriptor;ZIIIZ)V"
    )]
    async fn test_set_int_option_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_int_option_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.setInterface4(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_set_interface_4() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_interface_4(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.setInterface6(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_set_interface_6() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_interface_6(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.shouldSetBothIPv4AndIPv6Options0()Z"
    )]
    async fn test_should_set_both_ipv4_and_ipv6_options_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = should_set_both_ipv4_and_ipv6_options_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.shutdown(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_shutdown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = shutdown(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.Net.socket0(ZZZZ)I")]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = socket_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.Net.shouldShutdownWriteBeforeClose0()Z"
    )]
    async fn test_should_shutdown_write_before_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = should_shutdown_write_before_close_0(thread, Parameters::default()).await;
    }
}
