use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[cfg(any(target_os = "linux", not(unix)))]
use ristretto_types::JavaError;

#[cfg(unix)]
use ristretto_classloader::Reference;
#[cfg(unix)]
use ristretto_types::Error::InternalError;
#[cfg(unix)]
use ristretto_types::VM;
#[cfg(unix)]
use ristretto_types::handles::{SocketHandle, SocketType};
#[cfg(unix)]
use socket2::Socket;
#[cfg(unix)]
use std::net::Ipv4Addr;
#[cfg(unix)]
use std::os::fd::{FromRawFd, RawFd};

#[cfg(not(unix))]
fn unsupported<T>() -> Result<T> {
    Err(
        JavaError::UnsupportedOperationException("SCTP not supported on this platform".to_string())
            .into(),
    )
}

#[cfg(unix)]
const IPPROTO_SCTP: i32 = 132;
#[cfg(unix)]
const SOL_SCTP: i32 = 132;
#[cfg(unix)]
const SCTP_INITMSG: i32 = 2;
#[cfg(unix)]
const SCTP_NODELAY: i32 = 3;
#[cfg(unix)]
const SCTP_DISABLE_FRAGMENTS: i32 = 8;
#[cfg(unix)]
const SCTP_FRAGMENT_INTERLEAVE: i32 = 18;
#[cfg(unix)]
const SCTP_PRIMARY_ADDR: i32 = 6;
#[cfg(unix)]
const SCTP_SET_PEER_PRIMARY_ADDR: i32 = 5;
#[cfg(unix)]
const SCTP_GET_PEER_ADDRS: i32 = 108;
#[cfg(unix)]
const SCTP_GET_LOCAL_ADDRS: i32 = 109;
#[cfg(unix)]
const SCTP_SOCKOPT_BINDX_ADD: i32 = 100;
#[cfg(unix)]
const SCTP_SOCKOPT_BINDX_REM: i32 = 101;
#[cfg(unix)]
const SCTP_SOCKOPT_PEELOFF: i32 = 102;

#[cfg(unix)]
#[repr(C)]
#[derive(Clone, Copy, Default)]
#[expect(clippy::struct_field_names)]
struct SctpInitMsg {
    sinit_num_ostreams: u16,
    sinit_max_instreams: u16,
    sinit_max_attempts: u16,
    sinit_max_init_timeo: u16,
}

#[cfg(unix)]
#[repr(C)]
struct SctpPeeloff {
    assoc_id: i32,
    sd: i32,
}

#[cfg(unix)]
#[repr(C)]
struct SctpPrim {
    assoc_id: i32,
    addr: libc::sockaddr_storage,
}

#[cfg(unix)]
fn errno_msg(prefix: &str) -> String {
    let err = std::io::Error::last_os_error();
    format!("{prefix}: {err}")
}

#[cfg(unix)]
#[expect(clippy::cast_possible_truncation)]
fn sock_len<T>() -> libc::socklen_t {
    std::mem::size_of::<T>() as libc::socklen_t
}

#[cfg(unix)]
async fn raw_fd_for<V: VM>(vm: &Arc<V>, fd: i32) -> Result<RawFd> {
    let guard = vm
        .socket_handles()
        .get(&fd)
        .await
        .ok_or_else(|| InternalError(format!("socket not found for fd {fd}")))?;
    Ok(guard.socket_type.raw_fd())
}

#[cfg(unix)]
fn inet_address_ipv4(value: &Value) -> Result<Ipv4Addr> {
    let holder_value = {
        let object = value.as_object_ref()?;
        object.value("holder")?
    };
    let holder = holder_value.as_object_ref()?;
    let address_int = holder.value("address")?.as_i32()?;
    #[expect(clippy::cast_sign_loss)]
    let bits = address_int as u32;
    Ok(Ipv4Addr::new(
        ((bits >> 24) & 0xFF) as u8,
        ((bits >> 16) & 0xFF) as u8,
        ((bits >> 8) & 0xFF) as u8,
        (bits & 0xFF) as u8,
    ))
}

#[cfg(unix)]
fn make_sockaddr_in6(ipv4: Ipv4Addr, port: u16) -> libc::sockaddr_in6 {
    #[expect(unsafe_code)]
    // SAFETY: sockaddr_in6 is plain old data; zeroing it is valid.
    let mut sa: libc::sockaddr_in6 = unsafe { std::mem::zeroed() };
    sa.sin6_family = libc::sa_family_t::try_from(libc::AF_INET6).unwrap_or(0);
    sa.sin6_port = port.to_be();
    let mapped = ipv4.to_ipv6_mapped();
    sa.sin6_addr.s6_addr = mapped.octets();
    sa
}

#[cfg(unix)]
fn parse_sockaddrs(buf: &[u8], count: usize) -> Vec<(Vec<u8>, u16)> {
    let mut result = Vec::with_capacity(count);
    let mut offset = 0usize;
    for _ in 0..count {
        if offset + 2 > buf.len() {
            break;
        }
        let family = i32::from(u16::from_ne_bytes([buf[offset], buf[offset + 1]]));
        if family == libc::AF_INET6 {
            if offset + 28 > buf.len() {
                break;
            }
            let port = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]);
            let mut octets = [0u8; 16];
            octets.copy_from_slice(&buf[offset + 8..offset + 24]);
            let v6 = std::net::Ipv6Addr::from(octets);
            if let Some(v4) = v6.to_ipv4_mapped() {
                result.push((v4.octets().to_vec(), port));
            } else {
                result.push((octets.to_vec(), port));
            }
            offset += 28;
        } else if family == libc::AF_INET {
            if offset + 16 > buf.len() {
                break;
            }
            let port = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]);
            let octets = vec![
                buf[offset + 4],
                buf[offset + 5],
                buf[offset + 6],
                buf[offset + 7],
            ];
            result.push((octets, port));
            offset += 16;
        } else {
            break;
        }
    }
    result
}

#[cfg(unix)]
async fn build_socket_address_array<T: Thread + 'static>(
    thread: &Arc<T>,
    addrs: Vec<(Vec<u8>, u16)>,
) -> Result<Value> {
    let vm = thread.vm()?;
    let mut elements: Vec<Option<Reference>> = Vec::with_capacity(addrs.len());
    for (bytes, port) in addrs {
        #[expect(clippy::cast_possible_wrap)]
        let byte_array: Box<[i8]> = bytes.iter().map(|&b| b as i8).collect();
        let byte_array_value =
            Value::new_object(vm.garbage_collector(), Reference::ByteArray(byte_array));
        let inet_addr = thread
            .invoke(
                "java.net.InetAddress",
                "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
                &[Value::Object(None), byte_array_value],
            )
            .await?
            .ok_or_else(|| InternalError("getByAddress returned null".to_string()))?;
        let isa = thread
            .object(
                "java.net.InetSocketAddress",
                "Ljava/net/InetAddress;I",
                &[inet_addr, Value::Int(i32::from(port))],
            )
            .await?;
        let inner = {
            let guard = isa.as_reference()?;
            (*guard).clone()
        };
        elements.push(Some(inner));
    }
    let class = thread.class("[Ljava/net/SocketAddress;").await?;
    let array = Reference::new_array(vm.garbage_collector(), class, elements);
    Ok(Value::new_object(vm.garbage_collector(), array))
}

#[cfg(unix)]
fn create_sctp_socket(one_to_one: bool) -> Result<Socket> {
    let sock_type = if one_to_one {
        libc::SOCK_STREAM
    } else {
        libc::SOCK_SEQPACKET
    };
    #[expect(unsafe_code)]
    // SAFETY: libc::socket is a standard POSIX call; we check the return value.
    let raw = unsafe { libc::socket(libc::AF_INET6, sock_type, IPPROTO_SCTP) };
    if raw < 0 {
        return Err(InternalError(errno_msg("sctp socket")));
    }
    let off: libc::c_int = 0;
    #[expect(unsafe_code)]
    // SAFETY: setsockopt is called on a valid fd we just created; off lives for the call.
    let rc = unsafe {
        libc::setsockopt(
            raw,
            libc::IPPROTO_IPV6,
            libc::IPV6_V6ONLY,
            std::ptr::from_ref(&off).cast(),
            sock_len::<libc::c_int>(),
        )
    };
    if rc < 0 {
        #[expect(unsafe_code)]
        // SAFETY: we are closing the fd we just opened.
        unsafe {
            libc::close(raw);
        }
        return Err(InternalError(errno_msg("set IPV6_V6ONLY")));
    }
    #[expect(unsafe_code)]
    // SAFETY: raw is a valid socket fd we own; transferring ownership to socket2.
    Ok(unsafe { Socket::from_raw_fd(raw) })
}

#[cfg(unix)]
fn map_int_option(opt: i32) -> Option<(i32, i32)> {
    match opt {
        1 => Some((SOL_SCTP, SCTP_DISABLE_FRAGMENTS)),
        3 => Some((SOL_SCTP, SCTP_FRAGMENT_INTERLEAVE)),
        4 => Some((SOL_SCTP, SCTP_NODELAY)),
        5 => Some((libc::SOL_SOCKET, libc::SO_SNDBUF)),
        6 => Some((libc::SOL_SOCKET, libc::SO_RCVBUF)),
        _ => None,
    }
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let _prefer_ipv6 = parameters.pop_bool()?;
        let add = parameters.pop_bool()?;
        let addrs_length = parameters.pop_int()?;
        let port = parameters.pop_int()?;
        let addrs_value = parameters.pop()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;

        let mut buf: Vec<u8> = Vec::with_capacity(
            usize::try_from(addrs_length).map_err(|e| InternalError(e.to_string()))? * 28,
        );
        {
            let guard = addrs_value.as_reference()?;
            let Reference::Array(object_array) = &*guard else {
                return Err(InternalError("expected InetAddress array".to_string()));
            };
            let count = usize::try_from(addrs_length).map_err(|e| InternalError(e.to_string()))?;
            for i in 0..count {
                let element = object_array
                    .elements
                    .get(i)
                    .ok_or_else(|| InternalError("addr index out of bounds".to_string()))?;
                let ipv4 = inet_address_ipv4(element)?;
                #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                let sa = make_sockaddr_in6(ipv4, port as u16);
                #[expect(unsafe_code)]
                // SAFETY: sa is a sockaddr_in6 we just constructed; reading its bytes is valid.
                let bytes: &[u8] = unsafe {
                    std::slice::from_raw_parts(
                        std::ptr::from_ref(&sa).cast::<u8>(),
                        std::mem::size_of::<libc::sockaddr_in6>(),
                    )
                };
                buf.extend_from_slice(bytes);
            }
        }

        let opt = if add {
            SCTP_SOCKOPT_BINDX_ADD
        } else {
            SCTP_SOCKOPT_BINDX_REM
        };
        #[expect(unsafe_code)]
        // SAFETY: setsockopt with our owned buffer; len matches buf.
        let rc = unsafe {
            libc::setsockopt(
                raw,
                SOL_SCTP,
                opt,
                buf.as_ptr().cast(),
                libc::socklen_t::try_from(buf.len()).unwrap_or(libc::socklen_t::MAX),
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp bindx")));
        }
        Ok(None)
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let assoc_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        let mut arg = SctpPeeloff { assoc_id, sd: 0 };
        let mut len = sock_len::<SctpPeeloff>();
        #[expect(unsafe_code)]
        // SAFETY: getsockopt writes back into &mut arg of correct size.
        let rc = unsafe {
            libc::getsockopt(
                raw,
                SOL_SCTP,
                SCTP_SOCKOPT_PEELOFF,
                std::ptr::from_mut(&mut arg).cast(),
                &raw mut len,
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp peeloff")));
        }
        #[expect(unsafe_code)]
        // SAFETY: arg.sd is a valid fd returned by the kernel; transfer ownership to socket2.
        let socket = unsafe { Socket::from_raw_fd(arg.sd) };
        let new_fd = vm.next_nio_fd();
        let mut handle = SocketHandle::new(SocketType::Raw(socket));
        handle.is_ipv6 = true;
        vm.socket_handles().insert(new_fd, handle).await?;
        Ok(Some(Value::Int(new_fd)))
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let _ = vm.socket_handles().remove(&fd).await;
        Ok(None)
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let port = parameters.pop_int()?;
        let inet_addr = parameters.pop()?;
        let fd = parameters.pop_int()?;
        let ipv4 = inet_address_ipv4(&inet_addr)?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let sa = make_sockaddr_in6(ipv4, port as u16);
        #[expect(unsafe_code)]
        // SAFETY: connect with sockaddr_in6 of correct size on a valid fd.
        let rc = unsafe {
            libc::connect(
                raw,
                std::ptr::from_ref(&sa).cast(),
                sock_len::<libc::sockaddr_in6>(),
            )
        };
        if rc < 0 {
            let err = std::io::Error::last_os_error();
            return Err(ristretto_types::JavaError::ConnectException(err.to_string()).into());
        }
        Ok(Some(Value::Int(0)))
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let out_array = parameters.pop()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        let mut msg = SctpInitMsg::default();
        let mut len = sock_len::<SctpInitMsg>();
        #[expect(unsafe_code)]
        // SAFETY: getsockopt writes into msg with declared size.
        let rc = unsafe {
            libc::getsockopt(
                raw,
                SOL_SCTP,
                SCTP_INITMSG,
                std::ptr::from_mut(&mut msg).cast(),
                &raw mut len,
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp getInitMsg")));
        }
        let mut guard = out_array.as_reference_mut()?;
        if let Reference::IntArray(arr) = &mut *guard
            && arr.len() >= 2
        {
            arr[0] = i32::from(msg.sinit_num_ostreams);
            arr[1] = i32::from(msg.sinit_max_instreams);
        }
        Ok(None)
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let opt_name = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let (level, opt) = map_int_option(opt_name).ok_or_else(|| {
            ristretto_types::Error::JavaError(
                ristretto_types::JavaError::UnsupportedOperationException(format!(
                    "SCTP option {opt_name} not supported"
                )),
            )
        })?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        let mut value: libc::c_int = 0;
        let mut len = sock_len::<libc::c_int>();
        #[expect(unsafe_code)]
        // SAFETY: getsockopt writes into value of declared size.
        let rc = unsafe {
            libc::getsockopt(
                raw,
                level,
                opt,
                std::ptr::from_mut(&mut value).cast(),
                &raw mut len,
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp getsockopt")));
        }
        Ok(Some(Value::Int(value)))
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        let mut buf = vec![0u8; 4096];
        buf[0..4].copy_from_slice(&0i32.to_ne_bytes());
        let mut len = libc::socklen_t::try_from(buf.len()).unwrap_or(libc::socklen_t::MAX);
        #[expect(unsafe_code)]
        // SAFETY: getsockopt writes into buf with declared length.
        let rc = unsafe {
            libc::getsockopt(
                raw,
                SOL_SCTP,
                SCTP_GET_LOCAL_ADDRS,
                buf.as_mut_ptr().cast(),
                &raw mut len,
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp getaddrs")));
        }
        let count = u32::from_ne_bytes([buf[4], buf[5], buf[6], buf[7]]) as usize;
        let parsed = parse_sockaddrs(&buf[8..len as usize], count);
        let value = build_socket_address_array(&thread, parsed).await?;
        Ok(Some(value))
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let assoc_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        #[expect(unsafe_code)]
        // SAFETY: SctpPrim is plain old data; zeroing is valid.
        let mut prim: SctpPrim = unsafe { std::mem::zeroed() };
        prim.assoc_id = assoc_id;
        let mut len = sock_len::<SctpPrim>();
        #[expect(unsafe_code)]
        // SAFETY: getsockopt writes into prim of declared size.
        let rc = unsafe {
            libc::getsockopt(
                raw,
                SOL_SCTP,
                SCTP_PRIMARY_ADDR,
                std::ptr::from_mut(&mut prim).cast(),
                &raw mut len,
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp getPrimAddr")));
        }
        #[expect(unsafe_code)]
        // SAFETY: prim.addr is a sockaddr_storage; reading its bytes is valid.
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                std::ptr::from_ref(&prim.addr).cast::<u8>(),
                std::mem::size_of::<libc::sockaddr_storage>(),
            )
        };
        let addrs = parse_sockaddrs(bytes, 1);
        let array_value = build_socket_address_array(&thread, addrs).await?;
        let guard = array_value.as_reference()?;
        if let Reference::Array(arr) = &*guard
            && let Some(first) = arr.elements.first()
        {
            return Ok(Some(first.clone()));
        }
        Ok(Some(Value::Object(None)))
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let assoc_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        let mut buf = vec![0u8; 4096];
        buf[0..4].copy_from_slice(&assoc_id.to_ne_bytes());
        let mut len = libc::socklen_t::try_from(buf.len()).unwrap_or(libc::socklen_t::MAX);
        #[expect(unsafe_code)]
        // SAFETY: getsockopt writes into buf with declared length.
        let rc = unsafe {
            libc::getsockopt(
                raw,
                SOL_SCTP,
                SCTP_GET_PEER_ADDRS,
                buf.as_mut_ptr().cast(),
                &raw mut len,
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp getaddrs")));
        }
        let count = u32::from_ne_bytes([buf[4], buf[5], buf[6], buf[7]]) as usize;
        let parsed = parse_sockaddrs(&buf[8..len as usize], count);
        let value = build_socket_address_array(&thread, parsed).await?;
        Ok(Some(value))
    }
    #[cfg(not(unix))]
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
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.listen0(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn listen_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let backlog = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        #[expect(unsafe_code)]
        // SAFETY: listen on a valid fd retrieved from the socket handle map.
        let rc = unsafe { libc::listen(raw, backlog) };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp listen")));
        }
        Ok(None)
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        #[expect(unsafe_code)]
        // SAFETY: shutdown on a valid fd; failures are silently ignored as per JDK behaviour.
        unsafe {
            libc::shutdown(raw, libc::SHUT_WR);
        }
        Ok(None)
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let max_in = parameters.pop_int()?;
        let max_out = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let msg = SctpInitMsg {
            sinit_num_ostreams: max_out as u16,
            sinit_max_instreams: max_in as u16,
            sinit_max_attempts: 0,
            sinit_max_init_timeo: 0,
        };
        #[expect(unsafe_code)]
        // SAFETY: setsockopt reads from msg of declared size.
        let rc = unsafe {
            libc::setsockopt(
                raw,
                SOL_SCTP,
                SCTP_INITMSG,
                std::ptr::from_ref(&msg).cast(),
                sock_len::<SctpInitMsg>(),
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp setInitMsg")));
        }
        Ok(None)
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let value = parameters.pop_int()?;
        let opt_name = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let (level, opt) = map_int_option(opt_name).ok_or_else(|| {
            ristretto_types::Error::JavaError(
                ristretto_types::JavaError::UnsupportedOperationException(format!(
                    "SCTP option {opt_name} not supported"
                )),
            )
        })?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        let v: libc::c_int = value;
        #[expect(unsafe_code)]
        // SAFETY: setsockopt reads from v of declared size.
        let rc = unsafe {
            libc::setsockopt(
                raw,
                level,
                opt,
                std::ptr::from_ref(&v).cast(),
                sock_len::<libc::c_int>(),
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp setsockopt")));
        }
        Ok(None)
    }
    #[cfg(not(unix))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let _prefer_ipv6 = parameters.pop_bool()?;
        let port = parameters.pop_int()?;
        let inet_addr = parameters.pop()?;
        let assoc_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let ipv4 = inet_address_ipv4(&inet_addr)?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        #[expect(unsafe_code)]
        // SAFETY: SctpPrim is plain old data; zeroing is valid.
        let mut prim: SctpPrim = unsafe { std::mem::zeroed() };
        prim.assoc_id = assoc_id;
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let sa = make_sockaddr_in6(ipv4, port as u16);
        #[expect(unsafe_code)]
        // SAFETY: copy sa bytes into the storage field; both have valid layouts.
        unsafe {
            std::ptr::copy_nonoverlapping(
                std::ptr::from_ref(&sa).cast::<u8>(),
                std::ptr::from_mut(&mut prim.addr).cast::<u8>(),
                std::mem::size_of::<libc::sockaddr_in6>(),
            );
        }
        #[expect(unsafe_code)]
        // SAFETY: setsockopt reads from prim of declared size.
        let rc = unsafe {
            libc::setsockopt(
                raw,
                SOL_SCTP,
                SCTP_SET_PEER_PRIMARY_ADDR,
                std::ptr::from_ref(&prim).cast(),
                sock_len::<SctpPrim>(),
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp setPeerPrimAddr")));
        }
        Ok(None)
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let port = parameters.pop_int()?;
        let inet_addr = parameters.pop()?;
        let assoc_id = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let ipv4 = inet_address_ipv4(&inet_addr)?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        #[expect(unsafe_code)]
        // SAFETY: SctpPrim is plain old data; zeroing is valid.
        let mut prim: SctpPrim = unsafe { std::mem::zeroed() };
        prim.assoc_id = assoc_id;
        #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let sa = make_sockaddr_in6(ipv4, port as u16);
        #[expect(unsafe_code)]
        // SAFETY: copy sa bytes into the storage field; both have valid layouts.
        unsafe {
            std::ptr::copy_nonoverlapping(
                std::ptr::from_ref(&sa).cast::<u8>(),
                std::ptr::from_mut(&mut prim.addr).cast::<u8>(),
                std::mem::size_of::<libc::sockaddr_in6>(),
            );
        }
        #[expect(unsafe_code)]
        // SAFETY: setsockopt reads from prim of declared size.
        let rc = unsafe {
            libc::setsockopt(
                raw,
                SOL_SCTP,
                SCTP_PRIMARY_ADDR,
                std::ptr::from_ref(&prim).cast(),
                sock_len::<SctpPrim>(),
            )
        };
        if rc < 0 {
            return Err(InternalError(errno_msg("sctp setPrimAddr")));
        }
        Ok(None)
    }
    #[cfg(not(unix))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpNet.shutdown0(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn shutdown_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let _assoc = parameters.pop_int()?;
        let fd = parameters.pop_int()?;
        let vm = thread.vm()?;
        let raw = raw_fd_for(&vm, fd).await?;
        #[expect(unsafe_code)]
        // SAFETY: shutdown on a valid fd retrieved from the socket handle map.
        unsafe {
            libc::shutdown(raw, libc::SHUT_RDWR);
        }
        Ok(None)
    }
    #[cfg(not(unix))]
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
    #[cfg(unix)]
    {
        let mut parameters = parameters;
        let one_to_one = parameters.pop_bool()?;
        let socket = create_sctp_socket(one_to_one)?;
        let vm = thread.vm()?;
        let fd = vm.next_nio_fd();
        let mut handle = SocketHandle::new(SocketType::Raw(socket));
        handle.is_ipv6 = true;
        vm.socket_handles().insert(fd, handle).await?;
        Ok(Some(Value::Int(fd)))
    }
    #[cfg(not(unix))]
    {
        let _ = (thread, parameters);
        unsupported()
    }
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn bindx_le_v8<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _addrs = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.branch0(II)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn branch0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _assoc_id = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpNet.branch0(II)I".to_string()).into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.close0(I)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn close0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpNet.close0(I)V".to_string()).into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.connect0(ILjava/net/InetAddress;I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn connect0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _remote_port = parameters.pop_int()?;
    let _remote = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpNet.connect0(ILjava/net/InetAddress;I)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getInitMsgOption0(I[I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_init_msg_option0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _ret_vals = parameters.pop_reference()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpNet.getInitMsgOption0(I[I)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getIntOption0(II)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_int_option0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _opt = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpNet.getIntOption0(II)I".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_local_addresses0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_prim_addr_option0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _assoc_id = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_remote_addresses0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _assoc_id = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.init()V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn init_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpNet.init()V".to_string()).into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.listen0(II)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn listen0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _backlog = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpNet.listen0(II)V".to_string()).into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.preClose0(I)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn pre_close0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpNet.preClose0(I)V".to_string()).into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setInitMsgOption0(III)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_init_msg_option0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_int()?;
    let _arg1 = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpNet.setInitMsgOption0(III)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setIntOption0(III)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_int_option0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg = parameters.pop_int()?;
    let _opt = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpNet.setIntOption0(III)V".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_peer_prim_addr_option0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _prefer_i_pv6 = parameters.pop_bool()?;
    let _port = parameters.pop_int()?;
    let _ia = parameters.pop_reference()?;
    let _assoc_id = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/nio/ch/sctp/SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_prim_addr_option0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_int()?;
    let _ia = parameters.pop_reference()?;
    let _assoc_id = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/ch/sctp/SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.shutdown0(II)V", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn shutdown0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _assoc_id = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpNet.shutdown0(II)V".to_string())
            .into(),
    )
}

#[cfg(target_os = "linux")]
#[intrinsic_method("sun/nio/ch/sctp/SctpNet.socket0(Z)I", GreaterThanOrEqual(JAVA_11))]
#[async_method]
pub async fn socket0_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _one_to_one = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError("sun/nio/ch/sctp/SctpNet.socket0(Z)I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bindx() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = bindx(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_branch_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = branch_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_close_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = close_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connect_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = connect_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_init_msg_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_init_msg_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_int_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_int_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_local_addresses_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_local_addresses_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_prim_addr_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_prim_addr_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_remote_addresses_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = get_remote_addresses_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_listen_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = listen_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pre_close_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = pre_close_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_init_msg_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_init_msg_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_int_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_int_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_peer_prim_addr_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_peer_prim_addr_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_prim_addr_option_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_prim_addr_option_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_shutdown_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = shutdown_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(not(unix))]
    #[tokio::test]
    async fn test_socket_0() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_bool(true);
        let result = socket_0(thread, params).await;
        assert!(result.is_err());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn test_socket_0() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let mut params = Parameters::default();
        params.push_bool(true);
        let result = socket_0(thread, params).await;
        if let Err(e) = &result {
            let msg = e.to_string();
            if msg.contains("SCTP")
                || msg.contains("Protocol")
                || msg.contains("not supported")
                || msg.contains("Operation not permitted")
            {
                return Ok(());
            }
        }
        let _ = result?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_bindx_le_v8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = bindx_le_v8(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.bindx(I[Ljava/net/InetAddress;IIZZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_branch0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            branch0_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.branch0(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_close0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close0_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.close0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_connect0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = connect0_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.connect0(ILjava/net/InetAddress;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_init_msg_option0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_init_msg_option0_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.getInitMsgOption0(I[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_int_option0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_int_option0_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.getIntOption0(II)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_local_addresses0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_local_addresses0_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.getLocalAddresses0(I)[Ljava/net/SocketAddress;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_prim_addr_option0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_prim_addr_option0_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.getPrimAddrOption0(II)Ljava/net/SocketAddress;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_remote_addresses0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_remote_addresses0_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.getRemoteAddresses0(II)[Ljava/net/SocketAddress;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.init()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_listen0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            listen0_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.listen0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_pre_close0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = pre_close0_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.preClose0(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_init_msg_option0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_init_msg_option0_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.setInitMsgOption0(III)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_int_option0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_int_option0_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.setIntOption0(III)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_peer_prim_addr_option0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_peer_prim_addr_option0_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.setPeerPrimAddrOption0(IILjava/net/InetAddress;IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_prim_addr_option0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_prim_addr_option0_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.setPrimAddrOption0(IILjava/net/InetAddress;I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_shutdown0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            shutdown0_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.shutdown0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_socket0_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = socket0_linux_ge_v11(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/nio/ch/sctp/SctpNet.socket0(Z)I",
            result.unwrap_err().to_string()
        );
    }
}
