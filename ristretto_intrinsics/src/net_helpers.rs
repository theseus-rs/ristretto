use ristretto_classloader::{Reference, Value};
use ristretto_types::Error::InternalError;
use ristretto_types::{JavaError, Result, Thread, VM};
#[cfg(not(target_family = "wasm"))]
use socket2::SockAddr;
#[cfg(not(target_family = "wasm"))]
use std::ffi::CStr;
#[cfg(not(target_family = "wasm"))]
use std::mem::MaybeUninit;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
#[cfg(not(target_family = "wasm"))]
use std::net::{SocketAddrV4, SocketAddrV6};
use std::sync::Arc;

#[cfg(not(target_family = "wasm"))]
use ristretto_types::handles::SocketType;

/// Host representation of a Java `InetAddress`, including the IPv6 scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub(crate) enum InetAddressValue {
    V4(Ipv4Addr),
    V6(Ipv6Addr, u32),
}

impl InetAddressValue {
    #[must_use]
    pub(crate) fn ip(self) -> IpAddr {
        match self {
            Self::V4(address) => IpAddr::V4(address),
            Self::V6(address, _) => IpAddr::V6(address),
        }
    }

    #[must_use]
    pub(crate) fn octets(self) -> Vec<u8> {
        match self {
            Self::V4(address) => address.octets().to_vec(),
            Self::V6(address, _) => address.octets().to_vec(),
        }
    }

    #[must_use]
    pub(crate) fn scope_id(self) -> u32 {
        match self {
            Self::V4(_) => 0,
            Self::V6(_, scope_id) => scope_id,
        }
    }
}

/// Reinterprets a signed Java `byte` as the original unsigned network octet.
fn java_byte_to_u8(byte: i8) -> u8 {
    u8::from_ne_bytes(byte.to_ne_bytes())
}

/// Converts a Java `byte[4]` value into an IPv4 address.
///
/// Returns `None` when the slice is not exactly four bytes long.
pub(crate) fn ipv4_from_java_bytes(bytes: &[i8]) -> Option<Ipv4Addr> {
    let bytes: [i8; 4] = bytes.try_into().ok()?;
    Some(Ipv4Addr::from(bytes.map(java_byte_to_u8)))
}

/// Converts a Java `byte[16]` value into an IPv6 address.
///
/// Returns `None` when the slice is not exactly sixteen bytes long.
pub(crate) fn ipv6_from_java_bytes(bytes: &[i8]) -> Option<Ipv6Addr> {
    let bytes: [i8; 16] = bytes.try_into().ok()?;
    Some(Ipv6Addr::from(bytes.map(java_byte_to_u8)))
}

/// Converts Java's packed IPv4 `int` representation into an IPv4 address.
///
/// The raw bit pattern is preserved, so negative Java values still map to the
/// corresponding unsigned IPv4 octets.
pub(crate) fn ipv4_from_java_int(addr: i32) -> Ipv4Addr {
    let bits = u32::from_ne_bytes(addr.to_ne_bytes());
    Ipv4Addr::from(bits)
}

/// Reads the packed IPv4 `address` field from a Java `InetAddress` holder.
pub(crate) fn inet_address_int(inet_addr: &Value) -> Result<i32> {
    let holder_value = {
        let object = inet_addr.as_object_ref()?;
        object.value("holder")?
    };
    let holder = holder_value.as_object_ref()?;
    Ok(holder.value("address")?.as_i32()?)
}

/// Reads a Java `InetAddress` holder and converts its packed IPv4 value.
pub(crate) fn inet_address_ipv4(inet_addr: &Value) -> Result<Ipv4Addr> {
    Ok(ipv4_from_java_int(inet_address_int(inet_addr)?))
}

/// Converts a Java `InetAddress` to the address family used by a native socket.
/// IPv4 addresses are mapped when the socket is dual-stack; an IPv6 address can
/// only be used with an IPv4 socket when it is itself IPv4-mapped.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn inet_socket_address(
    inet_address: &Value,
    use_ipv6: bool,
    port: u16,
) -> Result<SockAddr> {
    let is_ipv6_address = inet_address
        .as_object_ref()?
        .class()
        .name()
        .ends_with("Inet6Address");
    if is_ipv6_address {
        let holder_value = inet_address.as_object_ref()?.value("holder6")?;
        let holder = holder_value.as_object_ref()?;
        let bytes_value = holder.value("ipaddress")?;
        let bytes = bytes_value.as_byte_vec_ref()?;
        let address = ipv6_from_java_bytes(&bytes).ok_or_else(|| {
            InternalError("Inet6Address.ipaddress must have 16 bytes".to_string())
        })?;
        let scope_id = u32::try_from(holder.value("scope_id")?.as_i32()?).unwrap_or(0);
        if use_ipv6 {
            return Ok(SockAddr::from(SocketAddrV6::new(
                address, port, 0, scope_id,
            )));
        }
        let address = address.to_ipv4_mapped().ok_or_else(|| {
            JavaError::SocketException(
                "IPv6 address cannot be used with an IPv4 socket".to_string(),
            )
        })?;
        return Ok(SockAddr::from(SocketAddrV4::new(address, port)));
    }

    let address = inet_address_ipv4(inet_address)?;
    if use_ipv6 {
        let address = if address.is_unspecified() {
            Ipv6Addr::UNSPECIFIED
        } else {
            address.to_ipv6_mapped()
        };
        Ok(SockAddr::from(SocketAddrV6::new(address, port, 0, 0)))
    } else {
        Ok(SockAddr::from(SocketAddrV4::new(address, port)))
    }
}

/// Decode either an IPv4 or IPv6 Java address object.
pub(crate) fn inet_address_value(inet_addr: &Value) -> Result<InetAddressValue> {
    let reference = inet_addr.as_reference()?;
    let is_ipv6 = reference.class_name()?.ends_with("Inet6Address");
    if !is_ipv6 {
        drop(reference);
        return Ok(InetAddressValue::V4(inet_address_ipv4(inet_addr)?));
    }
    let object = reference.as_object_ref()?;
    let holder_value = object.value("holder6")?;
    drop(reference);
    let holder = holder_value.as_object_ref()?;
    let bytes_value = holder.value("ipaddress")?;
    let scope_id = u32::try_from(holder.value("scope_id")?.as_i32()?).unwrap_or(0);
    let bytes = bytes_value.as_byte_vec_ref()?;
    let address = ipv6_from_java_bytes(bytes.as_ref())
        .ok_or_else(|| InternalError("Inet6Address.ipaddress must contain 16 bytes".to_string()))?;
    Ok(InetAddressValue::V6(address, scope_id))
}

/// Update the mutable address fields of an existing Java `InetAddress`.
pub(crate) fn set_inet_address_value(inet_addr: &Value, address: InetAddressValue) -> Result<()> {
    match address {
        InetAddressValue::V4(address) => {
            let holder_value = {
                let object = inet_addr.as_object_ref()?;
                object.value("holder")?
            };
            let mut holder = holder_value.as_object_mut()?;
            let bits = i32::from_ne_bytes(u32::from(address).to_ne_bytes());
            holder.set_value("address", Value::Int(bits))?;
            holder.set_value("family", Value::Int(1))?;
        }
        InetAddressValue::V6(address, scope_id) => {
            let holder_value = {
                let object = inet_addr.as_object_ref()?;
                object.value("holder6")?
            };
            let bytes_value = {
                let holder = holder_value.as_object_ref()?;
                holder.value("ipaddress")?
            };
            let mut bytes = bytes_value.as_byte_vec_mut()?;
            #[expect(clippy::cast_possible_wrap)]
            for (target, source) in bytes.iter_mut().zip(address.octets()) {
                *target = source as i8;
            }
            drop(bytes);
            let mut holder = holder_value.as_object_mut()?;
            holder.set_value(
                "scope_id",
                Value::Int(i32::try_from(scope_id).unwrap_or(i32::MAX)),
            )?;
            holder.set_value("scope_id_set", Value::Int(i32::from(scope_id != 0)))?;
        }
    }
    Ok(())
}

/// Convert a Java address to a socket address, applying IPv4 mapping for a
/// dual-stack IPv6 socket when requested.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn socket_address(
    address: InetAddressValue,
    port: i32,
    use_ipv6_socket: bool,
) -> Result<SockAddr> {
    Ok(SockAddr::from(std_socket_address(
        address,
        port,
        use_ipv6_socket,
    )?))
}

/// Convert a Java address to a standard-library socket address without first
/// round-tripping through the platform socket ABI.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn std_socket_address(
    address: InetAddressValue,
    port: i32,
    use_ipv6_socket: bool,
) -> Result<SocketAddr> {
    let port = u16::try_from(port)
        .map_err(|_| JavaError::IllegalArgumentException(format!("port out of range: {port}")))?;
    let address = match (address, use_ipv6_socket) {
        (InetAddressValue::V4(address), false) => SocketAddr::V4(SocketAddrV4::new(address, port)),
        (InetAddressValue::V4(address), true) => {
            let address = if address.is_unspecified() {
                Ipv6Addr::UNSPECIFIED
            } else {
                address.to_ipv6_mapped()
            };
            SocketAddr::V6(SocketAddrV6::new(address, port, 0, 0))
        }
        (InetAddressValue::V6(address, scope_id), true) => {
            SocketAddr::V6(SocketAddrV6::new(address, port, 0, scope_id))
        }
        (InetAddressValue::V6(address, _), false) => {
            let address = address.to_ipv4_mapped().ok_or_else(|| {
                JavaError::SocketException(
                    "Protocol family unavailable for IPv6 address".to_string(),
                )
            })?;
            SocketAddr::V4(SocketAddrV4::new(address, port))
        }
    };
    Ok(address)
}

/// Convert an OS socket address to the corresponding Java address value.
pub(crate) fn inet_address_from_socket(address: SocketAddr) -> InetAddressValue {
    match address {
        SocketAddr::V4(address) => InetAddressValue::V4(*address.ip()),
        SocketAddr::V6(address) => address.ip().to_ipv4_mapped().map_or(
            InetAddressValue::V6(*address.ip(), address.scope_id()),
            InetAddressValue::V4,
        ),
    }
}

/// Build a Java `InetAddress` preserving IPv6 scope IDs.
pub(crate) async fn java_inet_address<T: Thread + 'static>(
    thread: &Arc<T>,
    address: InetAddressValue,
    host_name: Option<&str>,
) -> Result<Value> {
    let vm = thread.vm()?;
    #[expect(clippy::cast_possible_wrap)]
    let bytes: Box<[i8]> = address
        .octets()
        .into_iter()
        .map(|byte| byte as i8)
        .collect();
    let bytes = Value::new_object(vm.garbage_collector(), Reference::ByteArray(bytes));
    let host_name = if let Some(host_name) = host_name {
        thread.intern_string(host_name).await?
    } else {
        Value::Object(None)
    };
    let result = match address {
        InetAddressValue::V6(_, scope_id) if scope_id != 0 => {
            thread
                .invoke(
                    "java.net.Inet6Address",
                    "getByAddress(Ljava/lang/String;[BI)Ljava/net/Inet6Address;",
                    &[
                        host_name,
                        bytes,
                        Value::Int(i32::try_from(scope_id).unwrap_or(i32::MAX)),
                    ],
                )
                .await?
        }
        _ => {
            thread
                .invoke(
                    "java.net.InetAddress",
                    "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
                    &[host_name, bytes],
                )
                .await?
        }
    };
    result.ok_or_else(|| InternalError("InetAddress.getByAddress returned null".to_string()))
}

/// Build a Java `InetSocketAddress` from an OS socket address.
pub(crate) async fn java_inet_socket_address<T: Thread + 'static>(
    thread: &Arc<T>,
    address: SocketAddr,
) -> Result<Value> {
    let inet_address = java_inet_address(thread, inet_address_from_socket(address), None).await?;
    thread
        .object(
            "java.net.InetSocketAddress",
            "Ljava/net/InetAddress;I",
            &[inet_address, Value::Int(i32::from(address.port()))],
        )
        .await
}

/// Resolve all addresses for a host without blocking an async executor thread.
pub(crate) async fn lookup_addresses(host: &str) -> Result<Vec<IpAddr>> {
    if let Ok(address) = host.parse() {
        return Ok(vec![address]);
    }
    #[cfg(target_family = "wasm")]
    {
        if host.eq_ignore_ascii_case("localhost") {
            return Ok(vec![
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                IpAddr::V6(Ipv6Addr::LOCALHOST),
            ]);
        }
        Err(JavaError::UnknownHostException(host.to_string()).into())
    }
    #[cfg(not(target_family = "wasm"))]
    {
        let mut addresses = Vec::new();
        let resolved = tokio::net::lookup_host((host, 0))
            .await
            .map_err(|error| JavaError::UnknownHostException(format!("{host}: {error}")))?;
        for address in resolved {
            if !addresses.contains(&address.ip()) {
                addresses.push(address.ip());
            }
        }
        if addresses.is_empty() {
            return Err(JavaError::UnknownHostException(host.to_string()).into());
        }
        Ok(addresses)
    }
}

/// Perform a reverse lookup and require a DNS name, matching `getnameinfo`
/// with `NI_NAMEREQD` used by `OpenJDK`.
pub(crate) async fn reverse_lookup(address: IpAddr) -> Result<String> {
    #[cfg(target_family = "wasm")]
    {
        if address.is_loopback() {
            return Ok("localhost".to_string());
        }
        Err(JavaError::UnknownHostException(address.to_string()).into())
    }
    #[cfg(not(target_family = "wasm"))]
    {
        tokio::task::spawn_blocking(move || reverse_lookup_blocking(address))
            .await
            .map_err(|error| InternalError(format!("reverse lookup task failed: {error}")))?
    }
}

#[cfg(not(target_family = "wasm"))]
#[expect(unsafe_code)]
fn reverse_lookup_blocking(address: IpAddr) -> Result<String> {
    let socket_address = SockAddr::from(SocketAddr::new(address, 0));
    let mut host = [0 as std::ffi::c_char; 1025];
    #[cfg(unix)]
    let result = unsafe {
        libc::getnameinfo(
            socket_address.as_ptr().cast(),
            socket_address.len(),
            host.as_mut_ptr(),
            libc::socklen_t::try_from(host.len()).unwrap_or(libc::socklen_t::MAX),
            std::ptr::null_mut(),
            0,
            libc::NI_NAMEREQD,
        )
    };
    #[cfg(windows)]
    let result = unsafe {
        use windows_sys::Win32::Networking::WinSock::{NI_NAMEREQD, getnameinfo};
        getnameinfo(
            socket_address.as_ptr().cast(),
            socket_address.len(),
            host.as_mut_ptr().cast(),
            u32::try_from(host.len()).unwrap_or(u32::MAX),
            std::ptr::null_mut(),
            0,
            NI_NAMEREQD.cast_signed(),
        )
    };
    if result != 0 {
        return Err(JavaError::UnknownHostException(address.to_string()).into());
    }
    let host = unsafe { CStr::from_ptr(host.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    Ok(host)
}

/// Test reachability using an ICMP echo when raw sockets are available, then
/// the same TCP echo-service fallback used by `OpenJDK` when they are not.
pub(crate) async fn is_reachable(
    address: InetAddressValue,
    source: Option<InetAddressValue>,
    ttl: i32,
    timeout_millis: i32,
) -> Result<bool> {
    if timeout_millis < 0 || ttl < 0 {
        return Err(JavaError::IllegalArgumentException(
            "timeout and ttl must not be negative".to_string(),
        )
        .into());
    }
    if address.ip().is_loopback() {
        return Ok(true);
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = (source, ttl, timeout_millis);
        Ok(false)
    }
    #[cfg(not(target_family = "wasm"))]
    {
        let timeout =
            std::time::Duration::from_millis(u64::try_from(timeout_millis.max(1)).unwrap_or(1));
        tokio::task::spawn_blocking(move || {
            if icmp_reachable(address, source, ttl, timeout).unwrap_or(false) {
                return Ok(true);
            }
            tcp_echo_reachable(address, source, ttl, timeout)
        })
        .await
        .map_err(|error| InternalError(format!("reachability task failed: {error}")))?
    }
}

#[cfg(not(target_family = "wasm"))]
fn icmp_reachable(
    address: InetAddressValue,
    source: Option<InetAddressValue>,
    ttl: i32,
    timeout: std::time::Duration,
) -> std::io::Result<bool> {
    let (domain, protocol, request_type) = match address {
        InetAddressValue::V4(_) => (socket2::Domain::IPV4, socket2::Protocol::ICMPV4, 8),
        InetAddressValue::V6(_, _) => (socket2::Domain::IPV6, socket2::Protocol::ICMPV6, 128),
    };
    let socket = socket2::Socket::new(domain, socket2::Type::RAW, Some(protocol))?;
    if let Some(source) = source
        && source.ip().is_ipv4() == address.ip().is_ipv4()
    {
        socket.bind(
            &socket_address(source, 0, source.ip().is_ipv6())
                .map_err(|error| std::io::Error::other(error.to_string()))?,
        )?;
    }
    if ttl > 0 {
        #[expect(clippy::cast_sign_loss)]
        if address.ip().is_ipv4() {
            socket.set_ttl_v4(ttl as u32)?;
        } else {
            socket.set_unicast_hops_v6(ttl as u32)?;
        }
    }
    socket.set_read_timeout(Some(timeout))?;
    socket.set_write_timeout(Some(timeout))?;
    let identifier = u16::try_from(std::process::id() & 0xffff).unwrap_or(1);
    let mut packet = [0u8; 16];
    packet[0] = request_type;
    packet[4..6].copy_from_slice(&identifier.to_be_bytes());
    packet[6..8].copy_from_slice(&1u16.to_be_bytes());
    packet[8..].copy_from_slice(b"RISTRET0");
    if address.ip().is_ipv4() {
        let checksum = internet_checksum(&packet);
        packet[2..4].copy_from_slice(&checksum.to_be_bytes());
    }
    let target = socket_address(address, 0, address.ip().is_ipv6())
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    socket.send_to(&packet, &target)?;
    let deadline = std::time::Instant::now() + timeout;
    let mut buffer = [MaybeUninit::new(0u8); 2048];
    loop {
        match socket.recv_from(&mut buffer) {
            Ok((length, source)) => {
                if length > 0
                    && source
                        .as_socket()
                        .is_some_and(|source| source.ip() == address.ip())
                {
                    return Ok(true);
                }
            }
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
                ) =>
            {
                return Ok(false);
            }
            Err(error) => return Err(error),
        }
        if std::time::Instant::now() >= deadline {
            return Ok(false);
        }
    }
}

#[cfg(not(target_family = "wasm"))]
fn tcp_echo_reachable(
    address: InetAddressValue,
    source: Option<InetAddressValue>,
    ttl: i32,
    timeout: std::time::Duration,
) -> Result<bool> {
    let domain = if address.ip().is_ipv6() {
        socket2::Domain::IPV6
    } else {
        socket2::Domain::IPV4
    };
    let socket = socket2::Socket::new(domain, socket2::Type::STREAM, Some(socket2::Protocol::TCP))
        .map_err(|error| socket_io_error("reachability socket", error))?;
    if let Some(source) = source
        && source.ip().is_ipv4() == address.ip().is_ipv4()
    {
        socket
            .bind(&socket_address(source, 0, source.ip().is_ipv6())?)
            .map_err(|error| socket_io_error("reachability bind", error))?;
    }
    if ttl > 0 {
        #[expect(clippy::cast_sign_loss)]
        if address.ip().is_ipv4() {
            socket
                .set_ttl_v4(ttl as u32)
                .map_err(|error| socket_io_error("reachability ttl", error))?;
        } else {
            socket
                .set_unicast_hops_v6(ttl as u32)
                .map_err(|error| socket_io_error("reachability ttl", error))?;
        }
    }
    let target = socket_address(address, 7, address.ip().is_ipv6())?;
    match socket.connect_timeout(&target, timeout) {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::ConnectionRefused => Ok(true),
        Err(error)
            if matches!(
                error.kind(),
                std::io::ErrorKind::TimedOut
                    | std::io::ErrorKind::WouldBlock
                    | std::io::ErrorKind::NetworkUnreachable
                    | std::io::ErrorKind::HostUnreachable
            ) =>
        {
            Ok(false)
        }
        Err(_) => Ok(false),
    }
}

#[cfg(not(target_family = "wasm"))]
fn internet_checksum(bytes: &[u8]) -> u16 {
    let mut sum = 0u32;
    for chunk in bytes.chunks(2) {
        let word = match chunk {
            [first, second] => u16::from_be_bytes([*first, *second]),
            [first] => u16::from(*first) << 8,
            _ => 0,
        };
        sum = sum.wrapping_add(u32::from(word));
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !u16::try_from(sum).unwrap_or(u16::MAX)
}

/// Reads the `value` field from a boxed Java integer-like object.
pub(crate) fn boxed_int_value(value: &Value) -> Result<i32> {
    let obj = value.as_object_ref()?;
    Ok(obj.value("value")?.as_i32()?)
}

/// Builds a borrowed `socket2::Socket` view over a managed socket handle.
///
/// The returned socket is wrapped in `ManuallyDrop` so option operations do not
/// close the descriptor owned by the VM socket handle.
#[cfg(not(target_family = "wasm"))]
#[expect(unsafe_code)]
pub(crate) fn socket_from_type(
    socket_type: &SocketType,
) -> std::mem::ManuallyDrop<socket2::Socket> {
    #[cfg(unix)]
    {
        use std::os::fd::FromRawFd;
        std::mem::ManuallyDrop::new(unsafe { socket2::Socket::from_raw_fd(socket_type.raw_fd()) })
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::FromRawSocket;
        std::mem::ManuallyDrop::new(unsafe {
            socket2::Socket::from_raw_socket(socket_type.raw_socket())
        })
    }
}

/// Return the number of bytes immediately readable from a socket.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn socket_available(socket_type: &SocketType) -> Result<i32> {
    #[cfg(unix)]
    {
        let mut available: libc::c_int = 0;
        #[expect(unsafe_code)]
        // SAFETY: the descriptor is owned by the managed socket and the ioctl
        // receives a valid pointer to an integer for the duration of the call.
        let result =
            unsafe { libc::ioctl(socket_type.raw_fd(), libc::FIONREAD, &raw mut available) };
        if result < 0 {
            return Err(socket_io_error(
                "ioctl FIONREAD",
                std::io::Error::last_os_error(),
            ));
        }
        Ok(available.max(0))
    }
    #[cfg(windows)]
    {
        use windows_sys::Win32::Networking::WinSock::{FIONREAD, SOCKET_ERROR, ioctlsocket};
        let mut available = 0u32;
        #[expect(unsafe_code)]
        // SAFETY: the socket is valid while borrowed and `available` is a
        // writable output parameter of the expected type.
        let result = unsafe {
            ioctlsocket(
                usize::try_from(socket_type.raw_socket()).unwrap_or(usize::MAX),
                FIONREAD,
                &raw mut available,
            )
        };
        if result == SOCKET_ERROR {
            return Err(socket_io_error(
                "ioctlsocket FIONREAD",
                std::io::Error::last_os_error(),
            ));
        }
        Ok(i32::try_from(available).unwrap_or(i32::MAX))
    }
}

/// Translate an OS socket failure into its Java-visible exception class.
#[cfg(not(target_family = "wasm"))]
#[expect(
    clippy::needless_pass_by_value,
    reason = "callers consume transient OS errors while constructing owned Java exceptions"
)]
pub(crate) fn socket_io_error(operation: &str, error: std::io::Error) -> ristretto_types::Error {
    let message = format!("{operation}: {error}");
    match error.kind() {
        std::io::ErrorKind::AddrInUse
        | std::io::ErrorKind::AddrNotAvailable
        | std::io::ErrorKind::PermissionDenied
            if operation.starts_with("bind") =>
        {
            JavaError::BindException(message).into()
        }
        std::io::ErrorKind::ConnectionRefused => JavaError::ConnectException(message).into(),
        std::io::ErrorKind::TimedOut => JavaError::SocketTimeoutException(message).into(),
        std::io::ErrorKind::ConnectionReset => JavaError::ConnectionResetException(message).into(),
        std::io::ErrorKind::NotConnected | std::io::ErrorKind::BrokenPipe => {
            JavaError::SocketException(message).into()
        }
        _ => {
            #[cfg(unix)]
            if matches!(error.raw_os_error(), Some(code) if code == libc::EHOSTUNREACH || code == libc::ENETUNREACH)
            {
                return JavaError::NoRouteToHostException(message).into();
            }
            #[cfg(windows)]
            if matches!(error.raw_os_error(), Some(10051 | 10065)) {
                return JavaError::NoRouteToHostException(message).into();
            }
            JavaError::SocketException(message).into()
        }
    }
}

/// Signal and remove a managed socket handle.
#[cfg(not(target_family = "wasm"))]
pub(crate) async fn close_socket<V: VM + ?Sized>(vm: &V, fd: i32) {
    if let Some(handle) = vm.socket_handles().get(&fd).await {
        handle.close();
    }
    vm.socket_handles().remove(&fd).await;
}

/// Replace a managed socket while retaining metadata and cancellation state.
#[cfg(not(target_family = "wasm"))]
pub(crate) async fn replace_socket<V: VM + ?Sized>(
    vm: &V,
    fd: i32,
    socket_type: SocketType,
) -> Result<()> {
    let old = vm
        .socket_handles()
        .remove(&fd)
        .await
        .ok_or_else(|| JavaError::SocketException("Socket closed".to_string()))?;
    vm.socket_handles()
        .insert(fd, old.with_socket_type(socket_type))
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(target_family = "wasm"))]
    use ristretto_types::Error;

    #[test]
    fn ipv4_from_java_bytes_reinterprets_signed_octets() {
        let bytes = [-1, -128, 0, 127];
        assert_eq!(
            Some(Ipv4Addr::new(255, 128, 0, 127)),
            ipv4_from_java_bytes(&bytes)
        );
    }

    #[test]
    fn ipv4_from_java_bytes_rejects_wrong_length() {
        assert_eq!(None, ipv4_from_java_bytes(&[127, 0, 0]));
    }

    #[test]
    fn ipv6_from_java_bytes_reinterprets_signed_octets() {
        let bytes = [-1, -2, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, -128];
        assert_eq!(
            Some(Ipv6Addr::from([
                255, 254, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 128,
            ])),
            ipv6_from_java_bytes(&bytes)
        );
    }

    #[test]
    fn ipv6_from_java_bytes_rejects_wrong_length() {
        assert_eq!(None, ipv6_from_java_bytes(&[0; 15]));
    }

    #[test]
    fn ipv4_from_java_int_preserves_raw_bits() {
        assert_eq!(Ipv4Addr::LOCALHOST, ipv4_from_java_int(0x7f00_0001));
        assert_eq!(Ipv4Addr::BROADCAST, ipv4_from_java_int(-1));
    }

    #[test]
    #[cfg(not(target_family = "wasm"))]
    fn std_socket_address_preserves_dual_stack_loopback() -> Result<()> {
        assert_eq!(
            SocketAddr::from((Ipv4Addr::LOCALHOST.to_ipv6_mapped(), 4321)),
            std_socket_address(InetAddressValue::V4(Ipv4Addr::LOCALHOST), 4321, true)?
        );
        Ok(())
    }

    #[test]
    fn inet_address_values_and_socket_conversions() -> Result<()> {
        let v4 = InetAddressValue::V4(Ipv4Addr::new(192, 0, 2, 1));
        assert_eq!(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)), v4.ip());
        assert_eq!(vec![192, 0, 2, 1], v4.octets());
        assert_eq!(0, v4.scope_id());

        let v6_address = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
        let v6 = InetAddressValue::V6(v6_address, 7);
        assert_eq!(IpAddr::V6(v6_address), v6.ip());
        assert_eq!(v6_address.octets(), v6.octets().as_slice());
        assert_eq!(7, v6.scope_id());

        #[cfg(not(target_family = "wasm"))]
        {
            assert_eq!(
                SocketAddr::from((Ipv4Addr::new(192, 0, 2, 1), 1234)),
                std_socket_address(v4, 1234, false)?
            );
            assert_eq!(
                SocketAddr::from((Ipv4Addr::new(192, 0, 2, 1).to_ipv6_mapped(), 1234)),
                std_socket_address(v4, 1234, true)?
            );
            assert_eq!(
                SocketAddr::from((Ipv6Addr::UNSPECIFIED, 1234)),
                std_socket_address(InetAddressValue::V4(Ipv4Addr::UNSPECIFIED), 1234, true)?
            );
            assert_eq!(
                SocketAddr::V6(SocketAddrV6::new(v6_address, 1234, 0, 7)),
                std_socket_address(v6, 1234, true)?
            );
            assert!(std_socket_address(v6, 1234, false).is_err());
            assert!(std_socket_address(v4, -1, false).is_err());
            assert!(std_socket_address(v4, 65_536, false).is_err());

            let mapped = Ipv4Addr::LOCALHOST.to_ipv6_mapped();
            assert_eq!(
                SocketAddr::from((Ipv4Addr::LOCALHOST, 80)),
                std_socket_address(InetAddressValue::V6(mapped, 0), 80, false)?
            );
            assert_eq!(
                InetAddressValue::V4(Ipv4Addr::LOCALHOST),
                inet_address_from_socket(SocketAddr::from((mapped, 80)))
            );
            assert_eq!(
                v6,
                inet_address_from_socket(SocketAddr::V6(SocketAddrV6::new(v6_address, 80, 0, 7)))
            );
            assert_eq!(
                SocketAddr::from((Ipv4Addr::new(192, 0, 2, 1), 1234)),
                socket_address(v4, 1234, false)?
                    .as_socket()
                    .expect("internet address")
            );
        }
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[tokio::test]
    async fn java_inet_address_round_trips() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let v4 = java_inet_address(
            &thread,
            InetAddressValue::V4(Ipv4Addr::LOCALHOST),
            Some("loopback"),
        )
        .await?;
        assert_eq!(
            InetAddressValue::V4(Ipv4Addr::LOCALHOST),
            inet_address_value(&v4)?
        );
        assert_eq!(0x7f00_0001, inet_address_int(&v4)?);
        assert_eq!(Ipv4Addr::LOCALHOST, inet_address_ipv4(&v4)?);
        assert_eq!(
            SocketAddr::from((Ipv4Addr::LOCALHOST, 4321)),
            inet_socket_address(&v4, false, 4321)?
                .as_socket()
                .expect("IPv4 socket address")
        );
        assert_eq!(
            SocketAddr::from((Ipv4Addr::LOCALHOST.to_ipv6_mapped(), 4321)),
            inet_socket_address(&v4, true, 4321)?
                .as_socket()
                .expect("dual-stack socket address")
        );
        set_inet_address_value(&v4, InetAddressValue::V4(Ipv4Addr::new(192, 0, 2, 9)))?;
        assert_eq!(
            InetAddressValue::V4(Ipv4Addr::new(192, 0, 2, 9)),
            inet_address_value(&v4)?
        );

        let v6 =
            java_inet_address(&thread, InetAddressValue::V6(Ipv6Addr::LOCALHOST, 0), None).await?;
        assert_eq!(
            InetAddressValue::V6(Ipv6Addr::LOCALHOST, 0),
            inet_address_value(&v6)?
        );
        assert_eq!(
            SocketAddr::from((Ipv6Addr::LOCALHOST, 4321)),
            inet_socket_address(&v6, true, 4321)?
                .as_socket()
                .expect("IPv6 socket address")
        );
        assert!(inet_socket_address(&v6, false, 4321).is_err());
        set_inet_address_value(
            &v6,
            InetAddressValue::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1), 3),
        )?;
        assert_eq!(
            InetAddressValue::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1), 3),
            inet_address_value(&v6)?
        );

        let scoped = java_inet_address(
            &thread,
            InetAddressValue::V6(Ipv6Addr::LOCALHOST, 1),
            Some("scoped-loopback"),
        )
        .await?;
        assert_eq!(
            InetAddressValue::V6(Ipv6Addr::LOCALHOST, 1),
            inet_address_value(&scoped)?
        );
        let socket_address =
            java_inet_socket_address(&thread, SocketAddr::from((Ipv4Addr::LOCALHOST, 8080)))
                .await?;
        assert_eq!(
            "java/net/InetSocketAddress",
            socket_address.as_object_ref()?.class().name()
        );

        let integer = thread
            .object("java.lang.Integer", "I", &[Value::Int(42)])
            .await?;
        assert_eq!(42, boxed_int_value(&integer)?);
        Ok(())
    }

    #[tokio::test]
    async fn lookup_reverse_and_reachability_paths() -> Result<()> {
        assert_eq!(
            vec![IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1))],
            lookup_addresses("192.0.2.1").await?
        );
        assert!(!lookup_addresses("localhost").await?.is_empty());
        assert!(
            lookup_addresses("definitely-not-a-host.invalid")
                .await
                .is_err()
        );
        assert!(
            !reverse_lookup(IpAddr::V4(Ipv4Addr::LOCALHOST))
                .await?
                .is_empty()
        );
        let _unknown_reverse = reverse_lookup(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1))).await;

        assert!(is_reachable(InetAddressValue::V4(Ipv4Addr::LOCALHOST), None, 0, 1).await?);
        assert!(is_reachable(InetAddressValue::V6(Ipv6Addr::LOCALHOST, 0), None, 0, 1).await?);
        assert!(
            is_reachable(InetAddressValue::V4(Ipv4Addr::LOCALHOST), None, -1, 1)
                .await
                .is_err()
        );
        assert!(
            is_reachable(InetAddressValue::V4(Ipv4Addr::LOCALHOST), None, 1, -1)
                .await
                .is_err()
        );
        let _unreachable = is_reachable(
            InetAddressValue::V4(Ipv4Addr::new(192, 0, 2, 1)),
            Some(InetAddressValue::V4(Ipv4Addr::UNSPECIFIED)),
            1,
            1,
        )
        .await?;
        #[cfg(not(target_family = "wasm"))]
        let _ipv6_echo = tcp_echo_reachable(
            InetAddressValue::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1), 0),
            Some(InetAddressValue::V6(Ipv6Addr::UNSPECIFIED, 0)),
            1,
            std::time::Duration::from_millis(1),
        );
        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn checksum_and_socket_error_mappings() {
        assert_eq!(0xffff, internet_checksum(&[]));
        assert_eq!(0xfbfd, internet_checksum(&[1, 2, 3]));
        assert_eq!(0, internet_checksum(&[0xff, 0xff]));
        assert_eq!(0, internet_checksum(&[0xff, 0xff, 0xff, 0xff]));

        for kind in [
            std::io::ErrorKind::AddrInUse,
            std::io::ErrorKind::AddrNotAvailable,
            std::io::ErrorKind::PermissionDenied,
        ] {
            assert!(matches!(
                socket_io_error("bind", std::io::Error::from(kind)),
                Error::JavaError(JavaError::BindException(_))
            ));
        }
        assert!(matches!(
            socket_io_error(
                "connect",
                std::io::Error::from(std::io::ErrorKind::ConnectionRefused)
            ),
            Error::JavaError(JavaError::ConnectException(_))
        ));
        assert!(matches!(
            socket_io_error("read", std::io::Error::from(std::io::ErrorKind::TimedOut)),
            Error::JavaError(JavaError::SocketTimeoutException(_))
        ));
        assert!(matches!(
            socket_io_error(
                "read",
                std::io::Error::from(std::io::ErrorKind::ConnectionReset)
            ),
            Error::JavaError(JavaError::ConnectionResetException(_))
        ));
        for kind in [
            std::io::ErrorKind::NotConnected,
            std::io::ErrorKind::BrokenPipe,
            std::io::ErrorKind::Other,
        ] {
            assert!(matches!(
                socket_io_error("write", std::io::Error::from(kind)),
                Error::JavaError(JavaError::SocketException(_))
            ));
        }
        #[cfg(unix)]
        assert!(matches!(
            socket_io_error(
                "connect",
                std::io::Error::from_raw_os_error(libc::EHOSTUNREACH)
            ),
            Error::JavaError(JavaError::NoRouteToHostException(_))
        ));
    }

    #[cfg(not(target_family = "wasm"))]
    #[tokio::test]
    async fn managed_socket_helpers_preserve_state_and_close() -> Result<()> {
        let (vm, _thread) = crate::test::java17_thread().await?;
        let fd = crate::java::net::socket_ops::create(vm.as_ref(), false, false).await?;
        {
            let mut handle = vm
                .socket_handles()
                .get_mut(&fd)
                .await
                .expect("managed socket");
            handle.timeout = Some(std::time::Duration::from_millis(23));
            handle.is_ipv6 = false;
            handle.non_blocking = true;
        }
        let lifecycle = vm
            .socket_handles()
            .get(&fd)
            .await
            .expect("managed socket")
            .lifecycle
            .clone();
        let replacement = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )?;
        replace_socket(vm.as_ref(), fd, SocketType::Raw(replacement)).await?;
        let handle = vm.socket_handles().get(&fd).await.expect("replaced socket");
        assert_eq!(23, handle.timeout_millis());
        assert!(handle.non_blocking);
        assert!(Arc::ptr_eq(&lifecycle, &handle.lifecycle));
        assert_eq!(0, socket_available(&handle.socket_type)?);
        drop(handle);

        close_socket(vm.as_ref(), fd).await;
        assert!(lifecycle.is_closed());
        close_socket(vm.as_ref(), fd).await;
        let replacement = socket2::Socket::new(
            socket2::Domain::IPV4,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )?;
        assert!(
            replace_socket(vm.as_ref(), fd, SocketType::Raw(replacement))
                .await
                .is_err()
        );
        Ok(())
    }
}
