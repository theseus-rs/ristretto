#[cfg(target_os = "linux")]
use crate::net_helpers::inet_address_ipv4;
use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
#[cfg(target_os = "linux")]
use ristretto_classloader::Reference;
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
#[cfg(target_os = "linux")]
use ristretto_types::Error;
#[cfg(target_os = "linux")]
use ristretto_types::Error::InternalError;
use ristretto_types::{JavaError, Parameters, Result, Thread};
#[cfg(target_os = "linux")]
use ristretto_types::{VM, handles::SocketHandle, handles::SocketType};
#[cfg(target_os = "linux")]
use socket2::Socket;
#[cfg(target_os = "linux")]
use std::mem::size_of;
#[cfg(target_os = "linux")]
use std::net::{Ipv4Addr, Ipv6Addr};
#[cfg(target_os = "linux")]
use std::os::fd::{FromRawFd, RawFd};
use std::sync::Arc;
#[cfg(target_os = "linux")]
use std::sync::OnceLock;

#[cfg(not(target_os = "linux"))]
fn unsupported<T>() -> Result<T> {
    Err(
        JavaError::UnsupportedOperationException("SCTP not supported on this platform".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
pub(super) const IPPROTO_SCTP: i32 = 132;
#[cfg(target_os = "linux")]
const SOL_SCTP: i32 = IPPROTO_SCTP;
#[cfg(target_os = "linux")]
const SCTP_INITMSG: i32 = 2;
#[cfg(target_os = "linux")]
const SCTP_NODELAY: i32 = 3;
#[cfg(target_os = "linux")]
const SCTP_SET_PEER_PRIMARY_ADDR: i32 = 5;
#[cfg(target_os = "linux")]
const SCTP_PRIMARY_ADDR: i32 = 6;
#[cfg(target_os = "linux")]
const SCTP_DISABLE_FRAGMENTS: i32 = 8;
#[cfg(target_os = "linux")]
const SCTP_EXPLICIT_EOR: i32 = -1;
#[cfg(target_os = "linux")]
const SCTP_EVENTS: i32 = 11;
#[cfg(target_os = "linux")]
const SCTP_FRAGMENT_INTERLEAVE: i32 = 18;
#[cfg(target_os = "linux")]
const SCTP_SOCKOPT_BINDX_ADD: i32 = 100;
#[cfg(target_os = "linux")]
const SCTP_SOCKOPT_BINDX_REM: i32 = 101;
#[cfg(target_os = "linux")]
const SCTP_SOCKOPT_PEELOFF: i32 = 102;
#[cfg(target_os = "linux")]
const SCTP_GET_PEER_ADDRS: i32 = 108;
#[cfg(target_os = "linux")]
const SCTP_GET_LOCAL_ADDRS: i32 = 109;
#[cfg(target_os = "linux")]
pub(super) const SCTP_SNDRCV: i32 = 1;
#[cfg(target_os = "linux")]
pub(super) const SCTP_UNORDERED: u16 = 1;
#[cfg(target_os = "linux")]
pub(super) const SCTP_EOF: u16 = 0x0200;

#[cfg(target_os = "linux")]
static PRE_CLOSE_FD: OnceLock<RawFd> = OnceLock::new();

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Copy, Default)]
#[expect(clippy::struct_field_names)]
struct SctpInitMsg {
    sinit_num_ostreams: u16,
    sinit_max_instreams: u16,
    sinit_max_attempts: u16,
    sinit_max_init_timeo: u16,
}

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct SctpPeeloff {
    assoc_id: i32,
    sd: i32,
}

/// Linux's legacy SCTP ancillary-data layout used by the JDK SCTP native library.
#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct SctpSndRcvInfo {
    pub(super) stream: u16,
    pub(super) ssn: u16,
    pub(super) flags: u16,
    pub(super) ppid: u32,
    pub(super) context: u32,
    pub(super) time_to_live: u32,
    pub(super) tsn: u32,
    pub(super) cumulative_tsn: u32,
    pub(super) assoc_id: i32,
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy)]
pub(super) struct SocketInfo {
    pub(super) raw_fd: RawFd,
    pub(super) is_ipv6: bool,
    pub(super) non_blocking: bool,
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum IntOption {
    Integer(i32, i32),
    Linger,
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InetAddressValue {
    V4(Ipv4Addr),
    V6(Ipv6Addr, u32),
}

#[cfg(target_os = "linux")]
#[expect(clippy::cast_possible_truncation)]
pub(super) fn sock_len<T>() -> libc::socklen_t {
    size_of::<T>() as libc::socklen_t
}

#[cfg(target_os = "linux")]
pub(super) async fn socket_info<V: VM>(vm: &Arc<V>, fd: i32) -> Result<SocketInfo> {
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    Ok(SocketInfo {
        raw_fd: guard.socket_type.raw_fd(),
        is_ipv6: guard.is_ipv6,
        non_blocking: guard.non_blocking,
    })
}

#[cfg(target_os = "linux")]
pub(super) fn duplicate_socket(raw_fd: RawFd, operation: &str) -> Result<Socket> {
    #[expect(unsafe_code)]
    // SAFETY: dup does not take ownership of raw_fd and returns a separately owned descriptor.
    let duplicate = unsafe { libc::dup(raw_fd) };
    if duplicate < 0 {
        return Err(socket_error(operation, std::io::Error::last_os_error()));
    }
    #[expect(unsafe_code)]
    // SAFETY: duplicate is a fresh descriptor owned by this function.
    Ok(unsafe { Socket::from_raw_fd(duplicate) })
}

#[cfg(target_os = "linux")]
#[expect(
    clippy::needless_pass_by_value,
    reason = "the owned OS error is the complete result of the failed syscall"
)]
pub(super) fn socket_error(operation: &str, error: std::io::Error) -> Error {
    let message = format!("{operation}: {error}");
    match error.raw_os_error() {
        Some(libc::EPROTO) => JavaError::ProtocolException(message).into(),
        Some(libc::ECONNREFUSED | libc::ETIMEDOUT) => JavaError::ConnectException(message).into(),
        Some(libc::EHOSTUNREACH) => JavaError::NoRouteToHostException(message).into(),
        Some(libc::EADDRINUSE | libc::EADDRNOTAVAIL) => JavaError::BindException(message).into(),
        _ => JavaError::SocketException(message).into(),
    }
}

#[cfg(target_os = "linux")]
fn inet_address_value(value: &Value) -> Result<InetAddressValue> {
    let guard = value.as_reference()?;
    if guard.class_name()? != "java/net/Inet6Address" {
        drop(guard);
        return Ok(InetAddressValue::V4(inet_address_ipv4(value)?));
    }
    let object = guard.as_object_ref()?;
    let holder_value = object.value("holder6")?;
    drop(guard);
    let holder = holder_value.as_object_ref()?;
    let address_value = holder.value("ipaddress")?;
    let scope_id = holder.value("scope_id")?.as_i32()?;
    let address_guard = address_value.as_reference()?;
    let Reference::ByteArray(bytes) = &*address_guard else {
        return Err(InternalError(
            "Inet6Address.ipaddress is not byte[]".to_string(),
        ));
    };
    let octets: [u8; 16] = bytes
        .iter()
        .map(|byte| u8::from_ne_bytes(byte.to_ne_bytes()))
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| InternalError("Inet6Address.ipaddress must have 16 bytes".to_string()))?;
    Ok(InetAddressValue::V6(
        Ipv6Addr::from(octets),
        u32::try_from(scope_id).unwrap_or(0),
    ))
}

#[cfg(target_os = "linux")]
fn make_sockaddr(
    address: InetAddressValue,
    port: u16,
    prefer_ipv6: bool,
) -> (libc::sockaddr_storage, libc::socklen_t) {
    #[expect(unsafe_code)]
    // SAFETY: sockaddr_storage is plain old data and all-zero is a valid initial state.
    let mut storage: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
    match address {
        InetAddressValue::V4(ipv4) if !prefer_ipv6 => {
            #[expect(unsafe_code)]
            // SAFETY: storage is large and aligned enough for sockaddr_in.
            let sockaddr =
                unsafe { &mut *std::ptr::from_mut(&mut storage).cast::<libc::sockaddr_in>() };
            sockaddr.sin_family = libc::sa_family_t::try_from(libc::AF_INET).unwrap_or(0);
            sockaddr.sin_port = port.to_be();
            sockaddr.sin_addr.s_addr = u32::from_ne_bytes(ipv4.octets());
            (storage, sock_len::<libc::sockaddr_in>())
        }
        InetAddressValue::V4(ipv4) => {
            let address = if ipv4.is_unspecified() {
                Ipv6Addr::UNSPECIFIED
            } else {
                ipv4.to_ipv6_mapped()
            };
            write_sockaddr_in6(storage, address, port, 0)
        }
        InetAddressValue::V6(ipv6, scope_id) => write_sockaddr_in6(storage, ipv6, port, scope_id),
    }
}

#[cfg(target_os = "linux")]
pub(super) fn socket_address_for_java(
    address: &Value,
    port: i32,
    prefer_ipv6: bool,
) -> Result<(libc::sockaddr_storage, libc::socklen_t)> {
    let port = u16::try_from(port).map_err(|error| InternalError(error.to_string()))?;
    Ok(make_sockaddr(
        inet_address_value(address)?,
        port,
        prefer_ipv6,
    ))
}

#[cfg(target_os = "linux")]
fn write_sockaddr_in6(
    mut storage: libc::sockaddr_storage,
    address: Ipv6Addr,
    port: u16,
    scope_id: u32,
) -> (libc::sockaddr_storage, libc::socklen_t) {
    #[expect(unsafe_code)]
    // SAFETY: storage is large and aligned enough for sockaddr_in6.
    let sockaddr = unsafe { &mut *std::ptr::from_mut(&mut storage).cast::<libc::sockaddr_in6>() };
    sockaddr.sin6_family = libc::sa_family_t::try_from(libc::AF_INET6).unwrap_or(0);
    sockaddr.sin6_port = port.to_be();
    sockaddr.sin6_addr.s6_addr = address.octets();
    sockaddr.sin6_scope_id = scope_id;
    (storage, sock_len::<libc::sockaddr_in6>())
}

#[cfg(target_os = "linux")]
fn sockaddr_bytes(storage: &libc::sockaddr_storage, length: libc::socklen_t) -> Result<&[u8]> {
    let length = usize::try_from(length)?;
    #[expect(unsafe_code)]
    // SAFETY: storage remains borrowed for the returned slice and length is a sockaddr size.
    Ok(unsafe { std::slice::from_raw_parts(std::ptr::from_ref(storage).cast::<u8>(), length) })
}

#[cfg(target_os = "linux")]
fn read_array<const N: usize>(buffer: &[u8], offset: usize) -> Option<[u8; N]> {
    let end = offset.checked_add(N)?;
    buffer.get(offset..end)?.try_into().ok()
}

#[cfg(target_os = "linux")]
pub(super) fn parse_sockaddr(buffer: &[u8]) -> Option<(Vec<u8>, u16, u32, usize)> {
    let family = i32::from(u16::from_ne_bytes(read_array::<2>(buffer, 0)?));
    if family == libc::AF_INET {
        let port = u16::from_be_bytes(read_array::<2>(buffer, 2)?);
        let octets = read_array::<4>(buffer, 4)?.to_vec();
        Some((octets, port, 0, size_of::<libc::sockaddr_in>()))
    } else if family == libc::AF_INET6 {
        let port = u16::from_be_bytes(read_array::<2>(buffer, 2)?);
        let octets = read_array::<16>(buffer, 8)?;
        let scope_id = u32::from_ne_bytes(read_array::<4>(buffer, 24)?);
        let ipv6 = Ipv6Addr::from(octets);
        if let Some(ipv4) = ipv6.to_ipv4_mapped() {
            Some((
                ipv4.octets().to_vec(),
                port,
                0,
                size_of::<libc::sockaddr_in6>(),
            ))
        } else {
            Some((
                octets.to_vec(),
                port,
                scope_id,
                size_of::<libc::sockaddr_in6>(),
            ))
        }
    } else {
        None
    }
}

#[cfg(target_os = "linux")]
fn parse_sockaddrs(buffer: &[u8], count: usize) -> Result<Vec<(Vec<u8>, u16, u32)>> {
    let mut result = Vec::with_capacity(count);
    let mut offset = 0usize;
    for _ in 0..count {
        let (address, port, scope_id, length) = parse_sockaddr(buffer.get(offset..).unwrap_or(&[]))
            .ok_or_else(|| InternalError("invalid SCTP sockaddr list".to_string()))?;
        result.push((address, port, scope_id));
        offset = offset
            .checked_add(length)
            .ok_or_else(|| InternalError("SCTP sockaddr offset overflow".to_string()))?;
    }
    Ok(result)
}

#[cfg(target_os = "linux")]
pub(super) async fn build_socket_address<T: Thread + 'static>(
    thread: &Arc<T>,
    bytes: Vec<u8>,
    port: u16,
    scope_id: u32,
) -> Result<Value> {
    let vm = thread.vm()?;
    #[expect(clippy::cast_possible_wrap)]
    let byte_array: Box<[i8]> = bytes.iter().map(|&byte| byte as i8).collect();
    let byte_array_value =
        Value::new_object(vm.garbage_collector(), Reference::ByteArray(byte_array));
    let inet_address = if bytes.len() == 16 && scope_id != 0 {
        thread
            .invoke(
                "java.net.Inet6Address",
                "getByAddress(Ljava/lang/String;[BI)Ljava/net/Inet6Address;",
                &[
                    Value::Object(None),
                    byte_array_value,
                    Value::Int(i32::try_from(scope_id).unwrap_or(i32::MAX)),
                ],
            )
            .await?
    } else {
        thread
            .invoke(
                "java.net.InetAddress",
                "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
                &[Value::Object(None), byte_array_value],
            )
            .await?
    }
    .ok_or_else(|| InternalError("InetAddress.getByAddress returned null".to_string()))?;
    thread
        .object(
            "java.net.InetSocketAddress",
            "Ljava/net/InetAddress;I",
            &[inet_address, Value::Int(i32::from(port))],
        )
        .await
}

#[cfg(target_os = "linux")]
async fn build_socket_address_array<T: Thread + 'static>(
    thread: &Arc<T>,
    addresses: Vec<(Vec<u8>, u16, u32)>,
) -> Result<Value> {
    let vm = thread.vm()?;
    let mut elements = Vec::with_capacity(addresses.len());
    for (bytes, port, scope_id) in addresses {
        let value = build_socket_address(thread, bytes, port, scope_id).await?;
        let inner = value.as_reference()?.clone();
        elements.push(Some(inner));
    }
    let class = thread.class("[Ljava/net/SocketAddress;").await?;
    let array = Reference::new_array(vm.garbage_collector(), class, elements);
    Ok(Value::new_object(vm.garbage_collector(), array))
}

#[cfg(target_os = "linux")]
fn create_sctp_socket(one_to_one: bool) -> Result<(Socket, bool)> {
    let socket_type = if one_to_one {
        libc::SOCK_STREAM
    } else {
        libc::SOCK_SEQPACKET
    };
    #[expect(unsafe_code)]
    // SAFETY: socket has no borrowed pointers and its result is checked before ownership transfer.
    let mut raw = unsafe { libc::socket(libc::AF_INET6, socket_type, IPPROTO_SCTP) };
    let mut is_ipv6 = true;
    if raw < 0 {
        let ipv6_error = std::io::Error::last_os_error();
        raw = {
            #[expect(unsafe_code)]
            // SAFETY: same as the IPv6 socket attempt above.
            unsafe {
                libc::socket(libc::AF_INET, socket_type, IPPROTO_SCTP)
            }
        };
        is_ipv6 = false;
        if raw < 0 {
            let error = std::io::Error::last_os_error();
            if matches!(
                error.raw_os_error(),
                Some(libc::EPROTONOSUPPORT | libc::ESOCKTNOSUPPORT)
            ) {
                return Err(JavaError::UnsupportedOperationException(error.to_string()).into());
            }
            return Err(socket_error(
                &format!("SCTP socket (IPv6 attempt: {ipv6_error})"),
                error,
            ));
        }
    }
    #[expect(unsafe_code)]
    // SAFETY: raw is a valid socket descriptor now owned by socket.
    let socket = unsafe { Socket::from_raw_fd(raw) };

    if is_ipv6 {
        let off: libc::c_int = 0;
        #[expect(unsafe_code)]
        // SAFETY: all pointers and lengths describe initialized local storage.
        let result = unsafe {
            libc::setsockopt(
                raw,
                libc::IPPROTO_IPV6,
                libc::IPV6_V6ONLY,
                std::ptr::from_ref(&off).cast(),
                sock_len::<libc::c_int>(),
            )
        };
        if result < 0 {
            return Err(socket_error(
                "set IPV6_V6ONLY",
                std::io::Error::last_os_error(),
            ));
        }
    }

    // Subscribe to exactly the notification families consumed by SctpChannelImpl.
    let events = [1u8, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0];
    #[expect(unsafe_code)]
    // SAFETY: events is initialized and remains alive for the duration of setsockopt.
    let result = unsafe {
        libc::setsockopt(
            raw,
            SOL_SCTP,
            SCTP_EVENTS,
            events.as_ptr().cast(),
            libc::socklen_t::try_from(events.len()).unwrap_or(libc::socklen_t::MAX),
        )
    };
    if result < 0 {
        return Err(socket_error(
            "enable SCTP events",
            std::io::Error::last_os_error(),
        ));
    }
    Ok((socket, is_ipv6))
}

#[cfg(target_os = "linux")]
fn map_int_option(option: i32) -> Option<IntOption> {
    match option {
        1 => Some(IntOption::Integer(SOL_SCTP, SCTP_DISABLE_FRAGMENTS)),
        2 => Some(IntOption::Integer(SOL_SCTP, SCTP_EXPLICIT_EOR)),
        3 => Some(IntOption::Integer(SOL_SCTP, SCTP_FRAGMENT_INTERLEAVE)),
        4 => Some(IntOption::Integer(SOL_SCTP, SCTP_NODELAY)),
        5 => Some(IntOption::Integer(libc::SOL_SOCKET, libc::SO_SNDBUF)),
        6 => Some(IntOption::Integer(libc::SOL_SOCKET, libc::SO_RCVBUF)),
        7 => Some(IntOption::Linger),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
fn unsupported_option(option: i32) -> Error {
    JavaError::SocketException(format!("Unsupported SCTP socket option: {option}")).into()
}

#[cfg(target_os = "linux")]
#[expect(
    clippy::needless_pass_by_value,
    reason = "the owned OS error is the complete result of the failed syscall"
)]
fn socket_option_error(operation: &str, error: std::io::Error) -> Error {
    JavaError::SocketException(format!("{operation}: {error}")).into()
}

#[cfg(target_os = "linux")]
async fn get_addresses<T: Thread + 'static>(
    thread: &Arc<T>,
    fd: i32,
    association_id: i32,
    option: i32,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let info = socket_info(&vm, fd).await?;
    // Linux caps an individual socket option buffer at 64 KiB.
    let mut buffer = vec![0u8; u16::MAX as usize];
    buffer
        .get_mut(..4)
        .ok_or_else(|| InternalError("SCTP address buffer is too small".to_string()))?
        .copy_from_slice(&association_id.to_ne_bytes());
    let mut length = libc::socklen_t::try_from(buffer.len()).unwrap_or(libc::socklen_t::MAX);
    #[expect(unsafe_code)]
    // SAFETY: buffer and length are writable and describe their exact capacities.
    let result = unsafe {
        libc::getsockopt(
            info.raw_fd,
            SOL_SCTP,
            option,
            buffer.as_mut_ptr().cast(),
            &raw mut length,
        )
    };
    if result < 0 {
        return Err(socket_error(
            "get SCTP addresses",
            std::io::Error::last_os_error(),
        ));
    }
    // Linux reports the length of the flexible `addrs` member, excluding the eight-byte
    // sctp_getaddrs header, even though it writes that header at the start of the buffer.
    let address_bytes = usize::try_from(length)?;
    let buffer_end = 8usize
        .checked_add(address_bytes)
        .ok_or_else(|| InternalError("SCTP address buffer length overflow".to_string()))?;
    if buffer_end > buffer.len() {
        return Err(InternalError(format!(
            "invalid SCTP address buffer length: {address_bytes}"
        )));
    }
    let count = usize::try_from(u32::from_ne_bytes(
        read_array::<4>(&buffer, 4)
            .ok_or_else(|| InternalError("missing SCTP address count".to_string()))?,
    ))?;
    if count == 0 {
        return Ok(Some(Value::Object(None)));
    }
    let address_buffer = buffer
        .get(8..buffer_end)
        .ok_or_else(|| InternalError("invalid SCTP address buffer range".to_string()))?;
    let addresses = parse_sockaddrs(address_buffer, count)?;
    Ok(Some(build_socket_address_array(thread, addresses).await?))
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn bindx<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let prefer_ipv6 = parameters.pop_bool()?;
        let add = parameters.pop_bool()?;
        let address_count = usize::try_from(parameters.pop_int()?)
            .map_err(|error| InternalError(error.to_string()))?;
        let port = u16::try_from(parameters.pop_int()?)
            .map_err(|error| InternalError(error.to_string()))?;
        let address_array = parameters.pop()?;
        let fd = parameters.pop_int()?;
        if address_count == 0 {
            return Ok(None);
        }
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        let mut buffer = Vec::with_capacity(address_count.saturating_mul(28));
        let guard = address_array.as_reference()?;
        let Reference::Array(array) = &*guard else {
            return Err(InternalError("expected InetAddress[]".to_string()));
        };
        for index in 0..address_count {
            let address =
                array
                    .elements
                    .get(index)
                    .ok_or(JavaError::ArrayIndexOutOfBoundsException {
                        index: i32::try_from(index).unwrap_or(i32::MAX),
                        length: array.elements.len(),
                    })?;
            let address = inet_address_value(address)?;
            let (sockaddr, length) = make_sockaddr(address, port, prefer_ipv6 && info.is_ipv6);
            buffer.extend_from_slice(sockaddr_bytes(&sockaddr, length)?);
        }
        drop(guard);
        let option = if add {
            SCTP_SOCKOPT_BINDX_ADD
        } else {
            SCTP_SOCKOPT_BINDX_REM
        };
        #[expect(unsafe_code)]
        // SAFETY: buffer is initialized and its exact byte length is supplied.
        let result = unsafe {
            libc::setsockopt(
                info.raw_fd,
                SOL_SCTP,
                option,
                buffer.as_ptr().cast(),
                libc::socklen_t::try_from(buffer.len()).unwrap_or(libc::socklen_t::MAX),
            )
        };
        if result < 0 {
            return Err(socket_error("SCTP bindx", std::io::Error::last_os_error()));
        }
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.branch0(II)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn branch_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let association_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        let mut peeloff = SctpPeeloff {
            assoc_id: association_id,
            sd: -1,
        };
        let mut length = sock_len::<SctpPeeloff>();
        #[expect(unsafe_code)]
        // SAFETY: peeloff and length are writable and correctly sized.
        let result = unsafe {
            libc::getsockopt(
                info.raw_fd,
                SOL_SCTP,
                SCTP_SOCKOPT_PEELOFF,
                std::ptr::from_mut(&mut peeloff).cast(),
                &raw mut length,
            )
        };
        if result < 0 {
            return Err(socket_error(
                "SCTP peeloff",
                std::io::Error::last_os_error(),
            ));
        }
        if peeloff.sd < 0 {
            return Err(InternalError(
                "SCTP peeloff returned an invalid fd".to_string(),
            ));
        }
        #[expect(unsafe_code)]
        // SAFETY: the kernel returned peeloff.sd as a new descriptor owned by the caller.
        let socket = unsafe { Socket::from_raw_fd(peeloff.sd) };
        let new_fd = vm.next_nio_fd();
        let mut handle = SocketHandle::new(SocketType::Raw(socket));
        handle.is_ipv6 = info.is_ipv6;
        handle.non_blocking = info.non_blocking;
        vm.socket_handles().insert(new_fd, handle).await?;
        Ok(Some(Value::Int(new_fd)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.close0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn close_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let fd = parameters.pop_int()?;
        if fd != -1 {
            let vm = thread.vm()?;
            let _removed = vm.socket_handles().remove(&fd).await;
        }
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.connect0(ILjava/net/InetAddress;I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn connect_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let port = u16::try_from(parameters.pop_int()?)
            .map_err(|error| InternalError(error.to_string()))?;
        let address = inet_address_value(&parameters.pop()?)?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        let socket = duplicate_socket(info.raw_fd, "duplicate SCTP socket for connect")?;
        let (sockaddr, length) = make_sockaddr(address, port, info.is_ipv6);
        let outcome = tokio::task::spawn_blocking(move || {
            #[expect(unsafe_code)]
            // SAFETY: sockaddr has the matching length and socket owns a valid descriptor.
            let result = unsafe {
                libc::connect(
                    std::os::fd::AsRawFd::as_raw_fd(&socket),
                    std::ptr::from_ref(&sockaddr).cast(),
                    length,
                )
            };
            if result == 0 {
                Ok(1)
            } else {
                let error = std::io::Error::last_os_error();
                match error.raw_os_error() {
                    Some(libc::EINPROGRESS | libc::EALREADY) => Ok(-2),
                    Some(libc::EINTR) => Ok(-3),
                    _ => Err(error),
                }
            }
        })
        .await
        .map_err(|error| InternalError(format!("SCTP connect task failed: {error}")))?;
        match outcome {
            Ok(value) => Ok(Some(Value::Int(value))),
            Err(error) => Err(socket_error("SCTP connect", error)),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getInitMsgOption0(I[I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_init_msg_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let output = parameters.pop()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        let mut message = SctpInitMsg::default();
        let mut length = sock_len::<SctpInitMsg>();
        #[expect(unsafe_code)]
        // SAFETY: message and length are writable and correctly sized.
        let result = unsafe {
            libc::getsockopt(
                info.raw_fd,
                SOL_SCTP,
                SCTP_INITMSG,
                std::ptr::from_mut(&mut message).cast(),
                &raw mut length,
            )
        };
        if result < 0 {
            return Err(socket_option_error(
                "get SCTP init message option",
                std::io::Error::last_os_error(),
            ));
        }
        let mut guard = output.as_reference_mut()?;
        let Reference::IntArray(values) = &mut *guard else {
            return Err(InternalError("expected int[]".to_string()));
        };
        let output_length = values.len();
        let [inbound, outbound, ..] = values.as_mut() else {
            return Err(JavaError::ArrayIndexOutOfBoundsException {
                index: 1,
                length: output_length,
            }
            .into());
        };
        *inbound = i32::from(message.sinit_max_instreams);
        *outbound = i32::from(message.sinit_num_ostreams);
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.getIntOption0(II)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_int_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let option_number = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let option =
            map_int_option(option_number).ok_or_else(|| unsupported_option(option_number))?;
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        let value = match option {
            IntOption::Integer(level, name) => {
                let mut value: libc::c_int = 0;
                let mut length = sock_len::<libc::c_int>();
                #[expect(unsafe_code)]
                // SAFETY: value and length are writable and correctly sized.
                let result = unsafe {
                    libc::getsockopt(
                        info.raw_fd,
                        level,
                        name,
                        std::ptr::from_mut(&mut value).cast(),
                        &raw mut length,
                    )
                };
                if result < 0 {
                    return Err(socket_option_error(
                        "get SCTP socket option",
                        std::io::Error::last_os_error(),
                    ));
                }
                // Linux doubles socket buffer values internally. OpenJDK's NET_GetSockOpt
                // compensates before exposing the value through SocketOption.getOption.
                if matches!(option_number, 5 | 6) {
                    value /= 2;
                }
                value
            }
            IntOption::Linger => {
                let mut linger = libc::linger {
                    l_onoff: 0,
                    l_linger: 0,
                };
                let mut length = sock_len::<libc::linger>();
                #[expect(unsafe_code)]
                // SAFETY: linger and length are writable and correctly sized.
                let result = unsafe {
                    libc::getsockopt(
                        info.raw_fd,
                        libc::SOL_SOCKET,
                        libc::SO_LINGER,
                        std::ptr::from_mut(&mut linger).cast(),
                        &raw mut length,
                    )
                };
                if result < 0 {
                    return Err(socket_option_error(
                        "get SCTP linger option",
                        std::io::Error::last_os_error(),
                    ));
                }
                if linger.l_onoff == 0 {
                    -1
                } else {
                    linger.l_linger
                }
            }
        };
        Ok(Some(Value::Int(value)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_local_addresses_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let fd = parameters.pop_int()?;
        get_addresses(&thread, fd, 0, SCTP_GET_LOCAL_ADDRS).await
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_prim_addr_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let association_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        // struct sctp_prim is packed to four-byte alignment on Linux.
        let mut primary = [0u8; 4 + size_of::<libc::sockaddr_storage>()];
        primary[..4].copy_from_slice(&association_id.to_ne_bytes());
        let mut length = libc::socklen_t::try_from(primary.len()).unwrap_or(libc::socklen_t::MAX);
        #[expect(unsafe_code)]
        // SAFETY: primary and length are writable and describe their exact capacities.
        let result = unsafe {
            libc::getsockopt(
                info.raw_fd,
                SOL_SCTP,
                SCTP_PRIMARY_ADDR,
                primary.as_mut_ptr().cast(),
                &raw mut length,
            )
        };
        if result < 0 {
            return Err(socket_option_error(
                "get SCTP primary address",
                std::io::Error::last_os_error(),
            ));
        }
        let (bytes, port, scope_id, _) = parse_sockaddr(&primary[4..])
            .ok_or_else(|| InternalError("invalid SCTP primary address".to_string()))?;
        Ok(Some(
            build_socket_address(&thread, bytes, port, scope_id).await?,
        ))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_remote_addresses_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let association_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        get_addresses(&thread, fd, association_id, SCTP_GET_PEER_ADDRS).await
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.init()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        if PRE_CLOSE_FD.get().is_none() {
            let mut descriptors = [-1; 2];
            #[expect(unsafe_code)]
            // SAFETY: descriptors has exactly the two writable entries required by socketpair.
            let result = unsafe {
                libc::socketpair(
                    libc::PF_UNIX,
                    libc::SOCK_STREAM,
                    0,
                    descriptors.as_mut_ptr(),
                )
            };
            if result < 0 {
                return Err(JavaError::IoException(format!(
                    "SCTP pre-close socketpair failed: {}",
                    std::io::Error::last_os_error()
                ))
                .into());
            }
            #[expect(unsafe_code)]
            // SAFETY: socketpair initialized both descriptors; closing the peer makes reads on
            // the retained descriptor immediately reach EOF after it is duplicated.
            unsafe {
                libc::close(descriptors[1]);
            }
            if PRE_CLOSE_FD.set(descriptors[0]).is_err() {
                #[expect(unsafe_code)]
                // SAFETY: another initializer won the race, so this descriptor is unowned.
                unsafe {
                    libc::close(descriptors[0]);
                }
            }
        }
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    unsupported()
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.listen0(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn listen_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let backlog = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        #[expect(unsafe_code)]
        // SAFETY: info.raw_fd is owned by the VM and valid while its handle remains registered.
        let result = unsafe { libc::listen(info.raw_fd, backlog) };
        if result < 0 {
            return Err(socket_error("SCTP listen", std::io::Error::last_os_error()));
        }
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.preClose0(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn pre_close_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let fd = parameters.pop_int()?;
        if fd != -1 {
            let vm = thread.vm()?;
            let info = socket_info(&vm, fd).await?;
            if let Some(pre_close_fd) = PRE_CLOSE_FD.get() {
                #[expect(unsafe_code)]
                // SAFETY: both descriptors are valid and dup2 atomically replaces the target
                // without transferring ownership of the retained pre-close descriptor.
                let result = unsafe { libc::dup2(*pre_close_fd, info.raw_fd) };
                if result < 0 {
                    return Err(JavaError::IoException(format!(
                        "SCTP pre-close dup2 failed: {}",
                        std::io::Error::last_os_error()
                    ))
                    .into());
                }
            }
        }
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setInitMsgOption0(III)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_init_msg_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let outbound = u16::try_from(parameters.pop_int()?)
            .map_err(|error| InternalError(error.to_string()))?;
        let inbound = u16::try_from(parameters.pop_int()?)
            .map_err(|error| InternalError(error.to_string()))?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        let message = SctpInitMsg {
            sinit_num_ostreams: outbound,
            sinit_max_instreams: inbound,
            ..SctpInitMsg::default()
        };
        #[expect(unsafe_code)]
        // SAFETY: message is initialized and its exact size is supplied.
        let result = unsafe {
            libc::setsockopt(
                info.raw_fd,
                SOL_SCTP,
                SCTP_INITMSG,
                std::ptr::from_ref(&message).cast(),
                sock_len::<SctpInitMsg>(),
            )
        };
        if result < 0 {
            return Err(socket_option_error(
                "set SCTP init message option",
                std::io::Error::last_os_error(),
            ));
        }
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.setIntOption0(III)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn set_int_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let mut value = parameters.pop_int()?;
        let option_number = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let option =
            map_int_option(option_number).ok_or_else(|| unsupported_option(option_number))?;
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        // Match NET_SetSockOpt: Linux needs at least 1 KiB of receive buffering so
        // that socket bookkeeping cannot consume the entire advertised buffer.
        if option_number == 6 && value < 1024 {
            value = 1024;
        }
        let result = match option {
            IntOption::Integer(level, name) => {
                #[expect(unsafe_code)]
                // SAFETY: value is initialized and its exact size is supplied.
                unsafe {
                    libc::setsockopt(
                        info.raw_fd,
                        level,
                        name,
                        std::ptr::from_ref(&value).cast(),
                        sock_len::<libc::c_int>(),
                    )
                }
            }
            IntOption::Linger => {
                let linger = if value >= 0 {
                    libc::linger {
                        l_onoff: 1,
                        l_linger: value,
                    }
                } else {
                    libc::linger {
                        l_onoff: 0,
                        l_linger: 0,
                    }
                };
                #[expect(unsafe_code)]
                // SAFETY: linger is initialized and its exact size is supplied.
                unsafe {
                    libc::setsockopt(
                        info.raw_fd,
                        libc::SOL_SOCKET,
                        libc::SO_LINGER,
                        std::ptr::from_ref(&linger).cast(),
                        sock_len::<libc::linger>(),
                    )
                }
            }
        };
        if result < 0 {
            return Err(socket_option_error(
                "set SCTP socket option",
                std::io::Error::last_os_error(),
            ));
        }
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[cfg(target_os = "linux")]
async fn set_primary_address<T: Thread + 'static>(
    thread: &Arc<T>,
    fd: i32,
    association_id: i32,
    address: &Value,
    port: i32,
    prefer_ipv6: bool,
    option: i32,
) -> Result<()> {
    let port = u16::try_from(port).map_err(|error| InternalError(error.to_string()))?;
    let address = inet_address_value(address)?;
    let vm = thread.vm()?;
    let info = socket_info(&vm, fd).await?;
    let (sockaddr, length) = make_sockaddr(address, port, prefer_ipv6 && info.is_ipv6);
    let sockaddr = sockaddr_bytes(&sockaddr, length)?;
    let mut primary = vec![0u8; 4 + size_of::<libc::sockaddr_storage>()];
    primary
        .get_mut(..4)
        .ok_or_else(|| InternalError("SCTP primary address buffer is too small".to_string()))?
        .copy_from_slice(&association_id.to_ne_bytes());
    primary
        .get_mut(4..4 + sockaddr.len())
        .ok_or_else(|| InternalError("SCTP primary sockaddr is too large".to_string()))?
        .copy_from_slice(sockaddr);
    #[expect(unsafe_code)]
    // SAFETY: primary matches Linux's packed sctp_prim/sctp_setpeerprim ABI.
    let result = unsafe {
        libc::setsockopt(
            info.raw_fd,
            SOL_SCTP,
            option,
            primary.as_ptr().cast(),
            libc::socklen_t::try_from(primary.len()).unwrap_or(libc::socklen_t::MAX),
        )
    };
    if result < 0 {
        return Err(socket_option_error(
            "set SCTP primary address",
            std::io::Error::last_os_error(),
        ));
    }
    Ok(())
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_peer_prim_addr_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let prefer_ipv6 = parameters.pop_bool()?;
        let port = parameters.pop_int()?;
        let address = parameters.pop()?;
        let association_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        set_primary_address(
            &thread,
            fd,
            association_id,
            &address,
            port,
            prefer_ipv6,
            SCTP_SET_PEER_PRIMARY_ADDR,
        )
        .await?;
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_prim_addr_option_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let port = parameters.pop_int()?;
        let address = parameters.pop()?;
        let association_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        set_primary_address(
            &thread,
            fd,
            association_id,
            &address,
            port,
            true,
            SCTP_PRIMARY_ADDR,
        )
        .await?;
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[cfg(target_os = "linux")]
pub(super) fn write_sctp_control(
    message: &mut libc::msghdr,
    control_buffer: &mut [u8],
    info: SctpSndRcvInfo,
) -> Result<()> {
    message.msg_control = control_buffer.as_mut_ptr().cast();
    #[cfg(target_env = "musl")]
    {
        message.msg_controllen = u32::try_from(control_buffer.len()).map_err(|_| {
            InternalError("SCTP control buffer length exceeds the platform limit".to_string())
        })?;
    }
    #[cfg(not(target_env = "musl"))]
    {
        message.msg_controllen = control_buffer.len();
    }
    #[expect(unsafe_code)]
    // SAFETY: message points at control_buffer, which has CMSG_SPACE bytes for this payload.
    unsafe {
        let header = libc::CMSG_FIRSTHDR(message);
        if header.is_null() {
            return Err(InternalError(
                "failed to create SCTP control header".to_string(),
            ));
        }
        (*header).cmsg_level = IPPROTO_SCTP;
        (*header).cmsg_type = SCTP_SNDRCV;
        (*header).cmsg_len =
            libc::CMSG_LEN(u32::try_from(size_of::<SctpSndRcvInfo>()).unwrap_or(u32::MAX))
                .try_into()
                .map_err(|_| {
                    InternalError(
                        "SCTP control message length exceeds the platform limit".to_string(),
                    )
                })?;
        std::ptr::write_unaligned(libc::CMSG_DATA(header).cast::<SctpSndRcvInfo>(), info);
        message.msg_controllen = (*header).cmsg_len;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub(super) fn control_buffer_size() -> usize {
    #[expect(unsafe_code)]
    // SAFETY: CMSG_SPACE only performs integer alignment arithmetic.
    unsafe {
        usize::try_from(libc::CMSG_SPACE(
            u32::try_from(size_of::<SctpSndRcvInfo>()).unwrap_or(u32::MAX),
        ))
        .unwrap_or(size_of::<libc::cmsghdr>() + size_of::<SctpSndRcvInfo>())
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.shutdown0(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn shutdown_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let association_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let info = socket_info(&vm, fd).await?;
        if association_id < 0 {
            #[expect(unsafe_code)]
            // SAFETY: shutdown borrows the descriptor and does not take ownership.
            unsafe {
                libc::shutdown(info.raw_fd, libc::SHUT_WR);
            }
            return Ok(None);
        }
        let socket = duplicate_socket(info.raw_fd, "duplicate SCTP socket for shutdown")?;
        let outcome = tokio::task::spawn_blocking(move || -> std::io::Result<()> {
            let mut iovec = libc::iovec {
                iov_base: std::ptr::null_mut(),
                iov_len: 0,
            };
            #[expect(unsafe_code)]
            // SAFETY: msghdr is plain old data; fields are filled before the syscall.
            let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
            message.msg_iov = &raw mut iovec;
            message.msg_iovlen = 1;
            let mut control = vec![0u8; control_buffer_size()];
            let control_info = SctpSndRcvInfo {
                assoc_id: association_id,
                flags: SCTP_EOF,
                ..SctpSndRcvInfo::default()
            };
            write_sctp_control(&mut message, &mut control, control_info)
                .map_err(std::io::Error::other)?;
            #[expect(unsafe_code)]
            // SAFETY: all msghdr pointers refer to live local storage.
            let result = unsafe {
                libc::sendmsg(
                    std::os::fd::AsRawFd::as_raw_fd(&socket),
                    &raw const message,
                    0,
                )
            };
            if result < 0 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        })
        .await
        .map_err(|error| InternalError(format!("SCTP shutdown task failed: {error}")))?;
        outcome.map_err(|error| socket_error("SCTP association shutdown", error))?;
        Ok(None)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.socket0(Z)I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn socket_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(target_os = "linux")]
    {
        let mut parameters = parameters;
        let one_to_one = parameters.pop_bool()?;
        let (socket, is_ipv6) = create_sctp_socket(one_to_one)?;
        let vm = thread.vm()?;
        let fd = vm.next_nio_fd();
        let mut handle = SocketHandle::new(SocketType::Raw(socket));
        handle.is_ipv6 = is_ipv6;
        vm.socket_handles().insert(fd, handle).await?;
        Ok(Some(Value::Int(fd)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

// Linux kept the same JNI signatures after Java 8. These registrations intentionally delegate
// to the common implementations so every supported LTS class library observes identical behavior.

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn bindx_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    bindx(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.branch0(II)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn branch0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    branch_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.close0(I)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn close0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    close_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.connect0(ILjava/net/InetAddress;I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn connect0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    connect_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getInitMsgOption0(I[I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_init_msg_option0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_init_msg_option_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getIntOption0(II)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_int_option0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_int_option_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_local_addresses0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_local_addresses_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_prim_addr_option0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_prim_addr_option_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_remote_addresses0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_remote_addresses_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.init()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    init(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.listen0(II)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn listen0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    listen_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.preClose0(I)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn pre_close0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    pre_close_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setInitMsgOption0(III)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_init_msg_option0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_init_msg_option_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setIntOption0(III)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_int_option0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_int_option_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_peer_prim_addr_option0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_peer_prim_addr_option_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_prim_addr_option0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    set_prim_addr_option_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.shutdown0(II)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn shutdown0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    shutdown_0(thread, parameters).await
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.socket0(Z)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn socket0_linux_ge_v11<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    socket_0(thread, parameters).await
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn test_int_option_mapping() {
        assert_eq!(
            Some(IntOption::Integer(SOL_SCTP, SCTP_DISABLE_FRAGMENTS)),
            map_int_option(1)
        );
        assert_eq!(Some(IntOption::Linger), map_int_option(7));
        assert_eq!(None, map_int_option(99));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_ipv4_sockaddr() -> Result<()> {
        let (storage, length) =
            make_sockaddr(InetAddressValue::V4(Ipv4Addr::LOCALHOST), 4242, false);
        let (address, port, scope_id, parsed_length) =
            parse_sockaddr(sockaddr_bytes(&storage, length)?)
                .ok_or_else(|| InternalError("parse sockaddr".to_string()))?;
        assert_eq!(vec![127, 0, 0, 1], address);
        assert_eq!(4242, port);
        assert_eq!(0, scope_id);
        assert_eq!(size_of::<libc::sockaddr_in>(), parsed_length);
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_ipv6_sockaddr() -> Result<()> {
        let address = Ipv6Addr::LOCALHOST;
        let (storage, length) = make_sockaddr(InetAddressValue::V6(address, 7), 4242, true);
        let (bytes, port, scope_id, parsed_length) =
            parse_sockaddr(sockaddr_bytes(&storage, length)?)
                .ok_or_else(|| InternalError("parse sockaddr".to_string()))?;
        assert_eq!(address.octets(), bytes.as_slice());
        assert_eq!(4242, port);
        assert_eq!(7, scope_id);
        assert_eq!(size_of::<libc::sockaddr_in6>(), parsed_length);
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_delegates_for_java_11() -> Result<()> {
        let (_vm, thread) = crate::test::java11_thread().await?;
        assert_eq!(
            None,
            init_linux_ge_v11(thread, Parameters::default()).await?
        );
        let pre_close_fd = *PRE_CLOSE_FD
            .get()
            .ok_or_else(|| InternalError("SCTP pre-close fd was not initialized".to_string()))?;
        #[expect(unsafe_code)]
        // SAFETY: fcntl only queries the descriptor retained by PRE_CLOSE_FD.
        let result = unsafe { libc::fcntl(pre_close_fd, libc::F_GETFD) };
        assert!(result >= 0);
        Ok(())
    }
}
