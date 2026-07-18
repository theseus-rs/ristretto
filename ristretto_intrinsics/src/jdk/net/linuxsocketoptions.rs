#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
#[cfg(target_os = "linux")]
use ristretto_types::{Error, VM};
use ristretto_types::{Parameters, Result};
#[cfg(target_os = "linux")]
use socket2::{Domain, Protocol, Socket, Type};
#[cfg(target_os = "linux")]
use std::io;
#[cfg(target_os = "linux")]
use std::mem::{size_of, size_of_val};
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
use std::sync::Arc;

#[cfg(not(target_os = "linux"))]
const UNSUPPORTED_PLATFORM: &str = "Linux socket options are not supported on this platform";

#[cfg(target_os = "linux")]
fn socket_option_error(message: &str, error: &io::Error) -> Error {
    if error.raw_os_error() == Some(libc::ENOPROTOOPT) {
        JavaError::UnsupportedOperationException("unsupported socket option".to_string()).into()
    } else {
        JavaError::SocketException(format!("{message}: {error}")).into()
    }
}

#[cfg(target_os = "linux")]
#[expect(unsafe_code)]
fn get_raw_int_socket_option(raw_fd: i32, level: i32, option: i32) -> io::Result<i32> {
    let mut value = 0_i32;
    let mut length = libc::socklen_t::try_from(size_of::<i32>()).unwrap_or_default();
    // SAFETY: `value` and `length` point to initialized, correctly-sized storage and `raw_fd`
    // remains owned by the managed socket handle for the duration of this call.
    let result = unsafe {
        libc::getsockopt(
            raw_fd,
            level,
            option,
            std::ptr::from_mut(&mut value).cast(),
            std::ptr::from_mut(&mut length),
        )
    };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(value)
    }
}

#[cfg(target_os = "linux")]
#[expect(unsafe_code)]
fn set_raw_int_socket_option(raw_fd: i32, level: i32, option: i32, value: i32) -> io::Result<()> {
    let length = libc::socklen_t::try_from(size_of_val(&value)).unwrap_or_default();
    // SAFETY: `value` points to initialized storage of `length` bytes and `raw_fd` remains owned
    // by the managed socket handle for the duration of this call.
    let result = unsafe {
        libc::setsockopt(
            raw_fd,
            level,
            option,
            std::ptr::from_ref(&value).cast(),
            length,
        )
    };
    if result < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(target_os = "linux")]
async fn get_int_socket_option<T: Thread + 'static>(
    thread: &Arc<T>,
    fd: i32,
    level: i32,
    option: i32,
    message: &str,
) -> Result<i32> {
    let vm = thread.vm()?;
    let handle =
        vm.socket_handles().get(&fd).await.ok_or_else(|| {
            socket_option_error(message, &io::Error::from_raw_os_error(libc::EBADF))
        })?;
    get_raw_int_socket_option(handle.socket_type.raw_fd(), level, option)
        .map_err(|error| socket_option_error(message, &error))
}

#[cfg(target_os = "linux")]
async fn set_int_socket_option<T: Thread + 'static>(
    thread: &Arc<T>,
    fd: i32,
    level: i32,
    option: i32,
    value: i32,
    message: &str,
) -> Result<()> {
    let vm = thread.vm()?;
    let handle =
        vm.socket_handles().get(&fd).await.ok_or_else(|| {
            socket_option_error(message, &io::Error::from_raw_os_error(libc::EBADF))
        })?;
    set_raw_int_socket_option(handle.socket_type.raw_fd(), level, option, value)
        .map_err(|error| socket_option_error(message, &error))
}

#[cfg(target_os = "linux")]
#[expect(unsafe_code)]
async fn get_peer_credentials<T: Thread + 'static>(thread: &Arc<T>, fd: i32) -> Result<i64> {
    const MESSAGE: &str = "get SO_PEERCRED failed";

    let vm = thread.vm()?;
    let handle =
        vm.socket_handles().get(&fd).await.ok_or_else(|| {
            socket_option_error(MESSAGE, &io::Error::from_raw_os_error(libc::EBADF))
        })?;
    let mut credentials = libc::ucred {
        pid: 0,
        uid: 0,
        gid: 0,
    };
    let mut length = libc::socklen_t::try_from(size_of::<libc::ucred>()).unwrap_or_default();
    // SAFETY: `credentials` and `length` point to initialized, correctly-sized storage and the
    // descriptor remains owned by the managed socket handle while `handle` is held.
    let result = unsafe {
        libc::getsockopt(
            handle.socket_type.raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            std::ptr::from_mut(&mut credentials).cast(),
            std::ptr::from_mut(&mut length),
        )
    };
    if result < 0 {
        return Err(socket_option_error(MESSAGE, &io::Error::last_os_error()));
    }
    if credentials.uid == u32::MAX {
        return Err(socket_option_error(
            MESSAGE,
            &io::Error::from_raw_os_error(libc::EINVAL),
        ));
    }

    let uid = i64::from(credentials.uid);
    let gid = i64::from(credentials.gid);
    Ok((uid << 32) | (gid & 0xffff_ffff_i64))
}

#[cfg(target_os = "linux")]
fn socket_option_supported(level: i32, option: i32) -> bool {
    let socket = Socket::new(Domain::IPV6, Type::STREAM, Some(Protocol::TCP))
        .or_else(|_| Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)));
    let Ok(socket) = socket else {
        return false;
    };

    match get_raw_int_socket_option(socket.as_raw_fd(), level, option) {
        Ok(_) => true,
        Err(error) => error.raw_os_error() != Some(libc::ENOPROTOOPT),
    }
}

#[cfg(not(target_os = "linux"))]
fn unsupported_platform<T>() -> Result<T> {
    Err(JavaError::UnsupportedOperationException(UNSUPPORTED_PLATFORM.to_string()).into())
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getIpDontFragment0(IZ)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_ip_dont_fragment_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let is_ipv6 = parameters.pop_bool()?;
    let fd = parameters.pop_int()?;
    #[cfg(target_os = "linux")]
    {
        let (level, option) = if is_ipv6 {
            (libc::IPPROTO_IPV6, libc::IPV6_MTU_DISCOVER)
        } else {
            (libc::IPPROTO_IP, libc::IP_MTU_DISCOVER)
        };
        let value = get_int_socket_option(
            &thread,
            fd,
            level,
            option,
            "get option IP_DONTFRAGMENT failed",
        )
        .await?;
        return Ok(Some(Value::from(value == libc::IP_PMTUDISC_DO)));
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd, is_ipv6);
        unsupported_platform()
    }
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getQuickAck0(I)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_quick_ack_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    #[cfg(target_os = "linux")]
    {
        let value = get_int_socket_option(
            &thread,
            fd,
            libc::IPPROTO_TCP,
            libc::TCP_QUICKACK,
            "get option TCP_QUICKACK failed",
        )
        .await?;
        return Ok(Some(Value::from(value != 0)));
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd);
        unsupported_platform()
    }
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getSoPeerCred0(I)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_so_peer_cred_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    #[cfg(target_os = "linux")]
    {
        return Ok(Some(Value::Long(get_peer_credentials(&thread, fd).await?)));
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd);
        unsupported_platform()
    }
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getTcpKeepAliveIntvl0(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_tcp_keep_alive_intvl_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    #[cfg(target_os = "linux")]
    {
        let value = get_int_socket_option(
            &thread,
            fd,
            libc::IPPROTO_TCP,
            libc::TCP_KEEPINTVL,
            "get option TCP_KEEPINTVL failed",
        )
        .await?;
        return Ok(Some(Value::Int(value)));
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd);
        unsupported_platform()
    }
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getTcpKeepAliveProbes0(I)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_tcp_keep_alive_probes_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let value = get_int_socket_option(
        &thread,
        fd,
        libc::IPPROTO_TCP,
        libc::TCP_KEEPCNT,
        "get option TCP_KEEPCNT failed",
    )
    .await?;
    Ok(Some(Value::Int(value)))
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getTcpKeepAliveTime0(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_tcp_keep_alive_time_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    #[cfg(target_os = "linux")]
    {
        let value = get_int_socket_option(
            &thread,
            fd,
            libc::IPPROTO_TCP,
            libc::TCP_KEEPIDLE,
            "get option TCP_KEEPIDLE failed",
        )
        .await?;
        return Ok(Some(Value::Int(value)));
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd);
        unsupported_platform()
    }
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.keepAliveOptionsSupported0()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn keep_alive_options_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    let supported = socket_option_supported(libc::IPPROTO_TCP, libc::TCP_KEEPIDLE)
        && socket_option_supported(libc::IPPROTO_TCP, libc::TCP_KEEPCNT)
        && socket_option_supported(libc::IPPROTO_TCP, libc::TCP_KEEPINTVL);
    #[cfg(not(target_os = "linux"))]
    let supported = false;
    Ok(Some(Value::from(supported)))
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.quickAckSupported0()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn quick_ack_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    let supported = socket_option_supported(libc::IPPROTO_TCP, libc::TCP_QUICKACK);
    #[cfg(not(target_os = "linux"))]
    let supported = false;
    Ok(Some(Value::from(supported)))
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setIpDontFragment0(IZZ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_ip_dont_fragment_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let is_ipv6 = parameters.pop_bool()?;
    let dont_fragment = parameters.pop_bool()?;
    let fd = parameters.pop_int()?;
    #[cfg(target_os = "linux")]
    {
        let (level, option) = if is_ipv6 {
            (libc::IPPROTO_IPV6, libc::IPV6_MTU_DISCOVER)
        } else {
            (libc::IPPROTO_IP, libc::IP_MTU_DISCOVER)
        };
        let value = if dont_fragment {
            libc::IP_PMTUDISC_DO
        } else {
            libc::IP_PMTUDISC_DONT
        };
        set_int_socket_option(
            &thread,
            fd,
            level,
            option,
            value,
            "set option IP_DONTFRAGMENT failed",
        )
        .await?;
        return Ok(None);
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd, dont_fragment, is_ipv6);
        unsupported_platform()
    }
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setQuickAck0(IZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_quick_ack_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let on = parameters.pop_bool()?;
    let fd = parameters.pop_int()?;
    #[cfg(target_os = "linux")]
    {
        set_int_socket_option(
            &thread,
            fd,
            libc::IPPROTO_TCP,
            libc::TCP_QUICKACK,
            i32::from(on),
            "set option TCP_QUICKACK failed",
        )
        .await?;
        return Ok(None);
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd, on);
        unsupported_platform()
    }
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setTcpKeepAliveIntvl0(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_tcp_keep_alive_intvl_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    #[cfg(target_os = "linux")]
    {
        set_int_socket_option(
            &thread,
            fd,
            libc::IPPROTO_TCP,
            libc::TCP_KEEPINTVL,
            value,
            "set option TCP_KEEPINTVL failed",
        )
        .await?;
        return Ok(None);
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd, value);
        unsupported_platform()
    }
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setTcpKeepAliveProbes0(II)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_tcp_keep_alive_probes_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    set_int_socket_option(
        &thread,
        fd,
        libc::IPPROTO_TCP,
        libc::TCP_KEEPCNT,
        value,
        "set option TCP_KEEPCNT failed",
    )
    .await?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setTcpKeepAliveTime0(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_tcp_keep_alive_time_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    #[cfg(target_os = "linux")]
    {
        set_int_socket_option(
            &thread,
            fd,
            libc::IPPROTO_TCP,
            libc::TCP_KEEPIDLE,
            value,
            "set option TCP_KEEPIDLE failed",
        )
        .await?;
        return Ok(None);
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, fd, value);
        unsupported_platform()
    }
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.incomingNapiIdSupported0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn incoming_napi_id_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(socket_option_supported(
        libc::SOL_SOCKET,
        libc::SO_INCOMING_NAPI_ID,
    ))))
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getIncomingNapiId0(I)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_incoming_napi_id_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let value = get_int_socket_option(
        &thread,
        fd,
        libc::SOL_SOCKET,
        libc::SO_INCOMING_NAPI_ID,
        "get option SO_INCOMING_NAPI_ID failed",
    )
    .await?;
    Ok(Some(Value::Int(value)))
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getTcpkeepAliveProbes0(I)I",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_tcpkeep_alive_probes0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let value = get_int_socket_option(
        &thread,
        fd,
        libc::IPPROTO_TCP,
        libc::TCP_KEEPCNT,
        "get option TCP_KEEPCNT failed",
    )
    .await?;
    Ok(Some(Value::Int(value)))
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setTcpkeepAliveProbes0(II)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn set_tcpkeep_alive_probes0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    set_int_socket_option(
        &thread,
        fd,
        libc::IPPROTO_TCP,
        libc::TCP_KEEPCNT,
        value,
        "set option TCP_KEEPCNT failed",
    )
    .await?;
    Ok(None)
}

#[cfg(target_os = "linux")]
#[intrinsic_method("jdk/net/LinuxSocketOptions.getIncomingNapiId0(I)I", Equal(JAVA_17))]
#[async_method]
pub async fn get_incoming_napi_id0_linux_v17<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let value = get_int_socket_option(
        &thread,
        fd,
        libc::SOL_SOCKET,
        libc::SO_INCOMING_NAPI_ID,
        "get option SO_INCOMING_NAPI_ID failed",
    )
    .await?;
    Ok(Some(Value::Int(value)))
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getTcpkeepAliveProbes0(I)I",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_tcpkeep_alive_probes0_linux_v11<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let value = get_int_socket_option(
        &thread,
        fd,
        libc::IPPROTO_TCP,
        libc::TCP_KEEPCNT,
        "get option TCP_KEEPCNT failed",
    )
    .await?;
    Ok(Some(Value::Int(value)))
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.incomingNapiIdSupported0()Z",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn incoming_napi_id_supported0_linux_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(socket_option_supported(
        libc::SOL_SOCKET,
        libc::SO_INCOMING_NAPI_ID,
    ))))
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setTcpkeepAliveProbes0(II)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn set_tcpkeep_alive_probes0_linux_v11<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    set_int_socket_option(
        &thread,
        fd,
        libc::IPPROTO_TCP,
        libc::TCP_KEEPCNT,
        value,
        "set option TCP_KEEPCNT failed",
    )
    .await?;
    Ok(None)
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_keep_alive_options_supported_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = keep_alive_options_supported_0(thread, Parameters::default()).await?;
        #[cfg(target_os = "linux")]
        let expected = socket_option_supported(libc::IPPROTO_TCP, libc::TCP_KEEPIDLE)
            && socket_option_supported(libc::IPPROTO_TCP, libc::TCP_KEEPCNT)
            && socket_option_supported(libc::IPPROTO_TCP, libc::TCP_KEEPINTVL);
        #[cfg(not(target_os = "linux"))]
        let expected = false;
        assert_eq!(result, Some(Value::from(expected)));
        Ok(())
    }

    #[tokio::test]
    async fn test_quick_ack_supported_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = quick_ack_supported_0(thread, Parameters::default()).await?;
        #[cfg(target_os = "linux")]
        let expected = socket_option_supported(libc::IPPROTO_TCP, libc::TCP_QUICKACK);
        #[cfg(not(target_os = "linux"))]
        let expected = false;
        assert_eq!(result, Some(Value::from(expected)));
        Ok(())
    }

    #[cfg(target_os = "linux")]
    async fn register_socket<V: VM>(vm: &V, fd: i32, socket: Socket) -> Result<()> {
        use ristretto_types::handles::{SocketHandle, SocketType};

        vm.socket_handles()
            .insert(fd, SocketHandle::new(SocketType::Raw(socket)))
            .await
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_tcp_options_round_trip() -> Result<()> {
        const FD: i32 = 41;

        let (vm, thread) = crate::test::thread().await?;
        let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
        register_socket(vm.as_ref(), FD, socket).await?;

        assert_eq!(
            set_quick_ack_0(
                thread.clone(),
                Parameters::new(vec![Value::Int(FD), Value::from(true)]),
            )
            .await?,
            None
        );
        assert_eq!(
            get_quick_ack_0(thread.clone(), Parameters::new(vec![Value::Int(FD)])).await?,
            Some(Value::from(true))
        );

        assert_eq!(
            set_tcp_keep_alive_time_0(
                thread.clone(),
                Parameters::new(vec![Value::Int(FD), Value::Int(47)]),
            )
            .await?,
            None
        );
        assert_eq!(
            get_tcp_keep_alive_time_0(thread.clone(), Parameters::new(vec![Value::Int(FD)]),)
                .await?,
            Some(Value::Int(47))
        );

        assert_eq!(
            set_tcp_keep_alive_intvl_0(
                thread.clone(),
                Parameters::new(vec![Value::Int(FD), Value::Int(31)]),
            )
            .await?,
            None
        );
        assert_eq!(
            get_tcp_keep_alive_intvl_0(thread.clone(), Parameters::new(vec![Value::Int(FD)]),)
                .await?,
            Some(Value::Int(31))
        );

        assert_eq!(
            set_tcp_keep_alive_probes_0(
                thread.clone(),
                Parameters::new(vec![Value::Int(FD), Value::Int(5)]),
            )
            .await?,
            None
        );
        assert_eq!(
            get_tcp_keep_alive_probes_0(thread.clone(), Parameters::new(vec![Value::Int(FD)]),)
                .await?,
            Some(Value::Int(5))
        );

        assert_eq!(
            set_tcpkeep_alive_probes0(
                thread.clone(),
                Parameters::new(vec![Value::Int(FD), Value::Int(6)]),
            )
            .await?,
            None
        );
        assert_eq!(
            get_tcpkeep_alive_probes0(thread.clone(), Parameters::new(vec![Value::Int(FD)]),)
                .await?,
            Some(Value::Int(6))
        );

        assert_eq!(
            set_tcpkeep_alive_probes0_linux_v11(
                thread.clone(),
                Parameters::new(vec![Value::Int(FD), Value::Int(7)]),
            )
            .await?,
            None
        );
        assert_eq!(
            get_tcpkeep_alive_probes0_linux_v11(thread, Parameters::new(vec![Value::Int(FD)]),)
                .await?,
            Some(Value::Int(7))
        );

        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_ip_dont_fragment_round_trip() -> Result<()> {
        const FD: i32 = 42;

        let (vm, thread) = crate::test::thread().await?;
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        register_socket(vm.as_ref(), FD, socket).await?;

        assert_eq!(
            set_ip_dont_fragment_0(
                thread.clone(),
                Parameters::new(vec![Value::Int(FD), Value::from(true), Value::from(false)]),
            )
            .await?,
            None
        );
        assert_eq!(
            get_ip_dont_fragment_0(
                thread.clone(),
                Parameters::new(vec![Value::Int(FD), Value::from(false)]),
            )
            .await?,
            Some(Value::from(true))
        );

        assert_eq!(
            set_ip_dont_fragment_0(
                thread.clone(),
                Parameters::new(vec![Value::Int(FD), Value::from(false), Value::from(false)]),
            )
            .await?,
            None
        );
        assert_eq!(
            get_ip_dont_fragment_0(
                thread,
                Parameters::new(vec![Value::Int(FD), Value::from(false)]),
            )
            .await?,
            Some(Value::from(false))
        );

        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_peer_credentials() -> Result<()> {
        use std::os::unix::fs::MetadataExt;
        use std::os::unix::net::UnixStream;

        const FD: i32 = 43;

        let (vm, thread) = crate::test::thread().await?;
        let (stream, _peer) = UnixStream::pair()?;
        register_socket(vm.as_ref(), FD, Socket::from(stream)).await?;

        let metadata = std::fs::metadata("/proc/self")?;
        let expected = (i64::from(metadata.uid()) << 32) | i64::from(metadata.gid());
        assert_eq!(
            get_so_peer_cred_0(thread, Parameters::new(vec![Value::Int(FD)])).await?,
            Some(Value::Long(expected))
        );

        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_incoming_napi_id() -> Result<()> {
        const FD: i32 = 44;

        let (vm, thread) = crate::test::thread().await?;
        let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
        register_socket(vm.as_ref(), FD, socket).await?;

        let expected_supported = Some(Value::from(socket_option_supported(
            libc::SOL_SOCKET,
            libc::SO_INCOMING_NAPI_ID,
        )));
        assert_eq!(
            incoming_napi_id_supported_0(thread.clone(), Parameters::default()).await?,
            expected_supported
        );
        assert_eq!(
            incoming_napi_id_supported0_linux_v17(thread.clone(), Parameters::default()).await?,
            expected_supported
        );
        assert_eq!(
            get_incoming_napi_id_0(thread.clone(), Parameters::new(vec![Value::Int(FD)]),).await?,
            Some(Value::Int(0))
        );
        assert_eq!(
            get_incoming_napi_id0_linux_v17(thread, Parameters::new(vec![Value::Int(FD)]),).await?,
            Some(Value::Int(0))
        );

        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_invalid_descriptor_throws_socket_exception() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_quick_ack_0(thread, Parameters::new(vec![Value::Int(i32::MAX)])).await;
        assert!(matches!(
            result,
            Err(Error::JavaError(JavaError::SocketException(_)))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_missing_parameters() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;

        assert!(
            get_ip_dont_fragment_0(thread.clone(), Parameters::default())
                .await
                .is_err()
        );
        assert!(
            get_quick_ack_0(thread.clone(), Parameters::default())
                .await
                .is_err()
        );
        assert!(
            get_so_peer_cred_0(thread.clone(), Parameters::default())
                .await
                .is_err()
        );
        assert!(
            get_tcp_keep_alive_intvl_0(thread.clone(), Parameters::default())
                .await
                .is_err()
        );
        assert!(
            get_tcp_keep_alive_time_0(thread.clone(), Parameters::default())
                .await
                .is_err()
        );
        assert!(
            set_ip_dont_fragment_0(thread.clone(), Parameters::default())
                .await
                .is_err()
        );
        assert!(
            set_quick_ack_0(thread.clone(), Parameters::default())
                .await
                .is_err()
        );
        assert!(
            set_tcp_keep_alive_intvl_0(thread.clone(), Parameters::default())
                .await
                .is_err()
        );
        assert!(
            set_tcp_keep_alive_time_0(thread, Parameters::default())
                .await
                .is_err()
        );

        Ok(())
    }
}
