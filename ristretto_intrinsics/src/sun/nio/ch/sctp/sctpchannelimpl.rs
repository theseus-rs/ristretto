use super::sctpnet::{
    IPPROTO_SCTP, SCTP_SNDRCV, SCTP_UNORDERED, SctpSndRcvInfo, build_socket_address,
    control_buffer_size, duplicate_socket, parse_sockaddr, sock_len, socket_address_for_java,
    socket_error, socket_info, write_sctp_control,
};
use crate::java::io::socketfiledescriptor::get_fd;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::Error::InternalError;
use ristretto_types::{JavaError, Parameters, Result, Thread, VM};
use socket2::Socket;
use std::mem::size_of;
use std::os::fd::AsRawFd;
use std::sync::Arc;

const RESULT_MESSAGE: i32 = 1;
const RESULT_SEND_FAILED: i32 = 2;
const RESULT_ASSOCIATION_CHANGED: i32 = 3;
const RESULT_PEER_ADDRESS_CHANGED: i32 = 4;
const RESULT_SHUTDOWN: i32 = 5;

const SCTP_ASSOC_CHANGE: u16 = 0x8001;
const SCTP_PEER_ADDR_CHANGE: u16 = 0x8002;
const SCTP_SEND_FAILED: u16 = 0x8003;
const SCTP_SHUTDOWN_EVENT: u16 = 0x8005;

#[derive(Debug)]
enum ReceiveOutcome {
    Status(i32),
    Packet(ReceivedPacket),
}

#[derive(Debug)]
struct ReceivedPacket {
    data: Vec<u8>,
    source: Vec<u8>,
    info: Option<SctpSndRcvInfo>,
    flags: i32,
}

fn read_array<const N: usize>(buffer: &[u8], offset: usize) -> Option<[u8; N]> {
    let end = offset.checked_add(N)?;
    buffer.get(offset..end)?.try_into().ok()
}

fn read_u16(buffer: &[u8], offset: usize) -> Option<u16> {
    read_array::<2>(buffer, offset).map(u16::from_ne_bytes)
}

fn read_u32(buffer: &[u8], offset: usize) -> Option<u32> {
    read_array::<4>(buffer, offset).map(u32::from_ne_bytes)
}

fn read_i32(buffer: &[u8], offset: usize) -> Option<i32> {
    read_array::<4>(buffer, offset).map(i32::from_ne_bytes)
}

fn notification_type(buffer: &[u8]) -> Option<u16> {
    read_u16(buffer, 0)
}

fn poll_descriptor(raw_fd: i32, events: i16) -> std::io::Result<()> {
    let mut descriptor = libc::pollfd {
        fd: raw_fd,
        events,
        revents: 0,
    };
    loop {
        #[expect(unsafe_code)]
        // SAFETY: descriptor is writable and the array length is exactly one.
        let result = unsafe { libc::poll(&raw mut descriptor, 1, -1) };
        if result > 0 {
            return Ok(());
        }
        if result == 0 {
            continue;
        }
        let error = std::io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::EINTR) {
            return Err(error);
        }
        return Err(error);
    }
}

fn parse_control(message: &libc::msghdr) -> Option<SctpSndRcvInfo> {
    #[expect(unsafe_code)]
    // SAFETY: recvmsg initialized the control region and msg_controllen bounds traversal.
    unsafe {
        let mut header = libc::CMSG_FIRSTHDR(message);
        while !header.is_null() {
            let payload_length = libc::CMSG_LEN(u32::try_from(size_of::<SctpSndRcvInfo>()).ok()?)
                .try_into()
                .ok()?;
            if (*header).cmsg_level == IPPROTO_SCTP
                && (*header).cmsg_type == SCTP_SNDRCV
                && (*header).cmsg_len >= payload_length
            {
                return Some(std::ptr::read_unaligned(
                    libc::CMSG_DATA(header).cast::<SctpSndRcvInfo>(),
                ));
            }
            header = libc::CMSG_NXTHDR(message, header);
        }
    }
    None
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "ownership keeps the duplicated descriptor alive for the entire blocking operation"
)]
fn receive_packet(
    socket: Socket,
    length: usize,
    peek: bool,
    non_blocking: bool,
) -> std::io::Result<ReceiveOutcome> {
    let raw_fd = socket.as_raw_fd();
    let mut collected = Vec::new();
    let mut source_bytes = Vec::new();
    let mut control_info = None;
    let mut first = true;

    let final_flags = loop {
        let chunk_length = if first { length } else { 512 };
        let mut data = vec![0u8; chunk_length];
        #[expect(unsafe_code)]
        // SAFETY: sockaddr_storage is plain old data; recvmsg initializes the used portion.
        let mut source: libc::sockaddr_storage = unsafe { std::mem::zeroed() };
        let mut iovec = libc::iovec {
            iov_base: data.as_mut_ptr().cast(),
            iov_len: data.len(),
        };
        let mut control = vec![0u8; control_buffer_size()];
        #[expect(unsafe_code)]
        // SAFETY: msghdr is plain old data and every pointer field is initialized below.
        let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
        message.msg_name = std::ptr::from_mut(&mut source).cast();
        message.msg_namelen = sock_len::<libc::sockaddr_storage>();
        message.msg_iov = &raw mut iovec;
        message.msg_iovlen = 1;
        message.msg_control = control.as_mut_ptr().cast();
        #[cfg(target_env = "musl")]
        {
            message.msg_controllen = u32::try_from(control.len()).map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "SCTP control buffer length exceeds the platform limit",
                )
            })?;
        }
        #[cfg(not(target_env = "musl"))]
        {
            message.msg_controllen = control.len();
        }

        let flags = if peek { libc::MSG_PEEK } else { 0 };
        #[expect(unsafe_code)]
        // SAFETY: message points only to live, writable local storage.
        let received = unsafe { libc::recvmsg(raw_fd, &raw mut message, flags) };
        if received < 0 {
            let error = std::io::Error::last_os_error();
            match error.raw_os_error() {
                Some(libc::EAGAIN) => {
                    if non_blocking {
                        return Ok(ReceiveOutcome::Status(-2));
                    }
                    if let Err(error) = poll_descriptor(raw_fd, libc::POLLIN) {
                        if error.raw_os_error() == Some(libc::EINTR) {
                            return Ok(ReceiveOutcome::Status(-3));
                        }
                        return Err(error);
                    }
                    continue;
                }
                Some(libc::EINTR) => return Ok(ReceiveOutcome::Status(-3)),
                // Linux reports ENOTCONN for EOF on one-to-one SCTP sockets.
                Some(libc::ENOTCONN) => {
                    return Ok(ReceiveOutcome::Packet(ReceivedPacket {
                        data: Vec::new(),
                        source: Vec::new(),
                        info: None,
                        flags: 0,
                    }));
                }
                _ => return Err(error),
            }
        }
        let received = usize::try_from(received).unwrap_or(0);
        data.truncate(received);
        if first {
            let source_length = usize::try_from(message.msg_namelen).unwrap_or(0);
            source_bytes = {
                #[expect(unsafe_code)]
                // SAFETY: source is live and msg_namelen was supplied by recvmsg.
                unsafe {
                    std::slice::from_raw_parts(
                        std::ptr::from_ref(&source).cast::<u8>(),
                        source_length.min(size_of::<libc::sockaddr_storage>()),
                    )
                    .to_vec()
                }
            };
            control_info = parse_control(&message);
        }
        collected.extend_from_slice(&data);

        let is_notification = (message.msg_flags & libc::MSG_NOTIFICATION) != 0;
        let is_complete = (message.msg_flags & libc::MSG_EOR) != 0;
        if !is_notification || is_complete || peek {
            break message.msg_flags;
        }
        first = false;
    };

    Ok(ReceiveOutcome::Packet(ReceivedPacket {
        data: collected,
        source: source_bytes,
        info: control_info,
        flags: final_flags,
    }))
}

fn set_result_container(container: &Value, result_type: i32, value: Value) -> Result<()> {
    let mut object = container.as_object_mut()?;
    object.set_value("value", value)?;
    object.set_value("type", Value::Int(result_type))?;
    Ok(())
}

async fn byte_buffer<T: Thread + 'static>(thread: &Arc<T>, bytes: &[u8]) -> Result<Value> {
    let vm = thread.vm()?;
    let buffer = thread
        .try_invoke(
            "java.nio.ByteBuffer",
            "allocateDirect(I)Ljava/nio/ByteBuffer;",
            &[Value::Int(i32::try_from(bytes.len())?)],
        )
        .await?;
    let address = {
        let object = buffer.as_object_ref()?;
        object.value("address")?.as_i64()?
    };
    vm.native_memory().write_bytes(address, bytes);
    Ok(buffer)
}

#[expect(
    clippy::too_many_lines,
    reason = "keeping all kernel notification layouts in one match makes their offsets auditable"
)]
async fn handle_notification<T: Thread + 'static>(
    thread: &Arc<T>,
    result_container: &Value,
    packet: &ReceivedPacket,
) -> Result<bool> {
    let Some(kind) = notification_type(&packet.data) else {
        return Err(InternalError("truncated SCTP notification".to_string()));
    };
    let declared_length = usize::try_from(
        read_u32(&packet.data, 4)
            .ok_or_else(|| InternalError("truncated SCTP notification header".to_string()))?,
    )?;
    if declared_length > packet.data.len() {
        return Err(InternalError(format!(
            "truncated SCTP notification: expected {declared_length}, received {}",
            packet.data.len()
        )));
    }

    match kind {
        SCTP_ASSOC_CHANGE => {
            let state = i32::from(
                read_u16(&packet.data, 8)
                    .ok_or_else(|| InternalError("truncated association change".to_string()))?,
            ) + 1;
            let outbound = i32::from(
                read_u16(&packet.data, 12)
                    .ok_or_else(|| InternalError("truncated association change".to_string()))?,
            );
            let inbound = i32::from(
                read_u16(&packet.data, 14)
                    .ok_or_else(|| InternalError("truncated association change".to_string()))?,
            );
            let association_id = read_i32(&packet.data, 16)
                .ok_or_else(|| InternalError("truncated association change".to_string()))?;
            let value = thread
                .object(
                    "sun.nio.ch.sctp.AssociationChange",
                    "IIII",
                    &[
                        Value::Int(association_id),
                        Value::Int(state),
                        Value::Int(outbound),
                        Value::Int(inbound),
                    ],
                )
                .await?;
            set_result_container(result_container, RESULT_ASSOCIATION_CHANGED, value)?;
            Ok(true)
        }
        SCTP_PEER_ADDR_CHANGE => {
            let (bytes, port, scope_id, _) = parse_sockaddr(
                packet
                    .data
                    .get(8..)
                    .ok_or_else(|| InternalError("truncated peer address change".to_string()))?,
            )
            .ok_or_else(|| InternalError("invalid peer address change sockaddr".to_string()))?;
            let address = build_socket_address(thread, bytes, port, scope_id).await?;
            let state = read_i32(&packet.data, 136)
                .ok_or_else(|| InternalError("truncated peer address change".to_string()))?
                .checked_add(1)
                .ok_or_else(|| InternalError("peer address state overflow".to_string()))?;
            let association_id = read_i32(&packet.data, 144)
                .ok_or_else(|| InternalError("truncated peer address change".to_string()))?;
            let value = thread
                .object(
                    "sun.nio.ch.sctp.PeerAddrChange",
                    "ILjava/net/SocketAddress;I",
                    &[Value::Int(association_id), address, Value::Int(state)],
                )
                .await?;
            set_result_container(result_container, RESULT_PEER_ADDRESS_CHANGED, value)?;
            Ok(true)
        }
        SCTP_SEND_FAILED => {
            let association_id = read_i32(&packet.data, 44)
                .ok_or_else(|| InternalError("truncated send failed notification".to_string()))?;
            let error_code = read_u32(&packet.data, 8)
                .ok_or_else(|| InternalError("truncated send failed notification".to_string()))?;
            let stream_number = read_u16(&packet.data, 12)
                .ok_or_else(|| InternalError("truncated send failed notification".to_string()))?;
            let (bytes, port, scope_id, _) = parse_sockaddr(&packet.source)
                .ok_or_else(|| InternalError("invalid send failed source sockaddr".to_string()))?;
            let address = build_socket_address(thread, bytes, port, scope_id).await?;
            let failed_data = packet.data.get(48..declared_length).unwrap_or(&[]);
            let buffer = byte_buffer(thread, failed_data).await?;
            let value = thread
                .object(
                    "sun.nio.ch.sctp.SendFailed",
                    "ILjava/net/SocketAddress;Ljava/nio/ByteBuffer;II",
                    &[
                        Value::Int(association_id),
                        address,
                        buffer,
                        Value::Int(i32::from_ne_bytes(error_code.to_ne_bytes())),
                        Value::Int(i32::from(stream_number)),
                    ],
                )
                .await?;
            set_result_container(result_container, RESULT_SEND_FAILED, value)?;
            Ok(true)
        }
        SCTP_SHUTDOWN_EVENT => {
            let association_id = read_i32(&packet.data, 8)
                .ok_or_else(|| InternalError("truncated shutdown notification".to_string()))?;
            let value = thread
                .object(
                    "sun.nio.ch.sctp.Shutdown",
                    "I",
                    &[Value::Int(association_id)],
                )
                .await?;
            set_result_container(result_container, RESULT_SHUTDOWN, value)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpChannelImpl.checkConnect(Ljava/io/FileDescriptor;ZZ)I",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn check_connect<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ready = parameters.pop_bool()?;
    let block = parameters.pop_bool()?;
    let fd_value = parameters.pop()?;
    let fd = get_fd(&fd_value)?;
    let vm = thread.vm()?;
    let info = socket_info(&vm, fd).await?;
    let socket = duplicate_socket(info.raw_fd, "duplicate SCTP socket for checkConnect")?;
    let outcome = tokio::task::spawn_blocking(move || -> std::io::Result<i32> {
        let raw_fd = socket.as_raw_fd();
        let mut revents = libc::POLLOUT;
        if !ready {
            let mut descriptor = libc::pollfd {
                fd: raw_fd,
                events: libc::POLLOUT,
                revents: 0,
            };
            #[expect(unsafe_code)]
            // SAFETY: descriptor is writable and the array length is exactly one.
            let result = unsafe { libc::poll(&raw mut descriptor, 1, if block { -1 } else { 0 }) };
            if result < 0 {
                let error = std::io::Error::last_os_error();
                if error.raw_os_error() == Some(libc::EINTR) {
                    return Ok(-3);
                }
                return Err(error);
            }
            if result == 0 {
                return Ok(-2);
            }
            revents = descriptor.revents;
        }
        let mut socket_error_value: libc::c_int = 0;
        let mut length = sock_len::<libc::c_int>();
        #[expect(unsafe_code)]
        // SAFETY: value and length are writable and correctly sized.
        let result = unsafe {
            libc::getsockopt(
                raw_fd,
                libc::SOL_SOCKET,
                libc::SO_ERROR,
                std::ptr::from_mut(&mut socket_error_value).cast(),
                &raw mut length,
            )
        };
        if result < 0 {
            return Err(std::io::Error::last_os_error());
        }
        if socket_error_value != 0 {
            return Err(std::io::Error::from_raw_os_error(socket_error_value));
        }
        if (revents & libc::POLLHUP) != 0 {
            return Err(std::io::Error::from_raw_os_error(libc::ENOTCONN));
        }
        Ok(1)
    })
    .await
    .map_err(|error| InternalError(format!("SCTP checkConnect task failed: {error}")))?;
    match outcome {
        Ok(value) => Ok(Some(Value::Int(value))),
        Err(error) => Err(socket_error("SCTP checkConnect", error)),
    }
}

#[intrinsic_method("sun/nio/ch/sctp/SctpChannelImpl.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpChannelImpl.receive0(ILsun/nio/ch/sctp/ResultContainer;JIZ)I",
    Any
)]
#[async_method]
pub async fn receive0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let peek = parameters.pop_bool()?;
    let length =
        usize::try_from(parameters.pop_int()?).map_err(|error| InternalError(error.to_string()))?;
    let address = parameters.pop_long()?;
    let result_container = parameters.pop()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    if length > 0 && vm.native_memory().try_read_bytes(address, length).is_none() {
        return Err(InternalError(format!(
            "receive0 native memory range is invalid: address={address}, length={length}"
        )));
    }
    let info = socket_info(&vm, fd).await?;

    loop {
        let socket = duplicate_socket(info.raw_fd, "duplicate SCTP socket for receive")?;
        let outcome = tokio::task::spawn_blocking(move || {
            receive_packet(socket, length, peek, info.non_blocking)
        })
        .await
        .map_err(|error| InternalError(format!("SCTP receive task failed: {error}")))?
        .map_err(|error| socket_error("SCTP receive", error))?;

        let packet = match outcome {
            ReceiveOutcome::Packet(packet) => packet,
            ReceiveOutcome::Status(status) => return Ok(Some(Value::Int(status))),
        };
        if (packet.flags & libc::MSG_NOTIFICATION) != 0 {
            if handle_notification(&thread, &result_container, &packet).await? {
                return Ok(Some(Value::Int(0)));
            }
            if peek {
                return Err(InternalError(
                    "unhandled SCTP notification encountered while peeking".to_string(),
                ));
            }
            continue;
        }

        if !packet.data.is_empty() {
            vm.native_memory().write_bytes(address, &packet.data);
        }
        let socket_address =
            if let Some((bytes, port, scope_id, _)) = parse_sockaddr(&packet.source) {
                build_socket_address(&thread, bytes, port, scope_id).await?
            } else {
                Value::Object(None)
            };
        let bytes = if packet.data.is_empty() {
            -1
        } else {
            i32::try_from(packet.data.len()).unwrap_or(i32::MAX)
        };
        let info = packet.info.unwrap_or_default();
        let message_info = thread
            .object(
                "sun.nio.ch.sctp.MessageInfoImpl",
                "ILjava/net/SocketAddress;IIZZI",
                &[
                    Value::Int(info.assoc_id),
                    socket_address,
                    Value::Int(bytes),
                    Value::Int(i32::from(info.stream)),
                    Value::from((packet.flags & libc::MSG_EOR) != 0),
                    Value::from((info.flags & SCTP_UNORDERED) != 0),
                    Value::Int(i32::from_ne_bytes(u32::from_be(info.ppid).to_ne_bytes())),
                ],
            )
            .await?;
        set_result_container(&result_container, RESULT_MESSAGE, message_info)?;
        let received = i32::try_from(packet.data.len()).unwrap_or(i32::MAX);
        return Ok(Some(Value::Int(received)));
    }
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "the worker owns its descriptor and payload for the entire blocking operation"
)]
fn send_packet(
    socket: Socket,
    data: Vec<u8>,
    target: Option<(libc::sockaddr_storage, libc::socklen_t)>,
    control_info: SctpSndRcvInfo,
    non_blocking: bool,
) -> std::io::Result<i32> {
    let raw_fd = socket.as_raw_fd();
    loop {
        let mut iovec = libc::iovec {
            iov_base: data.as_ptr().cast_mut().cast(),
            iov_len: data.len(),
        };
        #[expect(unsafe_code)]
        // SAFETY: msghdr is plain old data and every pointer field is initialized below.
        let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
        message.msg_iov = &raw mut iovec;
        message.msg_iovlen = 1;
        if let Some((address, length)) = target.as_ref() {
            message.msg_name = std::ptr::from_ref(address).cast_mut().cast();
            message.msg_namelen = *length;
        }
        let mut control = vec![0u8; control_buffer_size()];
        write_sctp_control(&mut message, &mut control, control_info)
            .map_err(std::io::Error::other)?;
        #[expect(unsafe_code)]
        // SAFETY: message points only to live local storage for the duration of sendmsg.
        let sent = unsafe { libc::sendmsg(raw_fd, &raw const message, 0) };
        if sent >= 0 {
            return Ok(i32::try_from(sent).unwrap_or(i32::MAX));
        }
        let error = std::io::Error::last_os_error();
        match error.raw_os_error() {
            Some(libc::EAGAIN) => {
                if non_blocking {
                    return Ok(-2);
                }
                if let Err(error) = poll_descriptor(raw_fd, libc::POLLOUT) {
                    if error.raw_os_error() == Some(libc::EINTR) {
                        return Ok(-3);
                    }
                    return Err(error);
                }
            }
            Some(libc::EINTR) => return Ok(-3),
            Some(libc::EPIPE) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "Socket is shutdown for writing",
                ));
            }
            _ => return Err(error),
        }
    }
}

#[intrinsic_method(
    "sun/nio/ch/sctp/SctpChannelImpl.send0(IJILjava/net/InetAddress;IIIZI)I",
    Any
)]
#[async_method]
pub async fn send0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let ppid = parameters.pop_int()?;
    let unordered = parameters.pop_bool()?;
    let stream_number = parameters.pop_int()?;
    let association_id = parameters.pop_int()?;
    let port = parameters.pop_int()?;
    let target_address = parameters.pop()?;
    let length =
        usize::try_from(parameters.pop_int()?).map_err(|error| InternalError(error.to_string()))?;
    let address = parameters.pop_long()?;
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let data = if length == 0 {
        Vec::new()
    } else {
        vm.native_memory()
            .try_read_bytes(address, length)
            .ok_or_else(|| {
                InternalError(format!(
                    "send0 native memory range is invalid: address={address}, length={length}"
                ))
            })?
    };
    let info = socket_info(&vm, fd).await?;
    let target = if matches!(target_address, Value::Object(None)) {
        None
    } else {
        Some(socket_address_for_java(
            &target_address,
            port,
            info.is_ipv6,
        )?)
    };
    let control_info = SctpSndRcvInfo {
        stream: u16::try_from(stream_number).unwrap_or(0),
        flags: if unordered { SCTP_UNORDERED } else { 0 },
        ppid: if ppid > 0 {
            u32::from_ne_bytes(ppid.to_ne_bytes()).to_be()
        } else {
            0
        },
        assoc_id: if association_id > 0 {
            association_id
        } else {
            0
        },
        ..SctpSndRcvInfo::default()
    };
    let socket = duplicate_socket(info.raw_fd, "duplicate SCTP socket for send")?;
    let outcome = tokio::task::spawn_blocking(move || {
        send_packet(socket, data, target, control_info, info.non_blocking)
    })
    .await
    .map_err(|error| InternalError(format!("SCTP send task failed: {error}")))?;
    match outcome {
        Ok(sent) => Ok(Some(Value::Int(sent))),
        Err(error) if error.kind() == std::io::ErrorKind::BrokenPipe => {
            Err(JavaError::SocketException(error.to_string()).into())
        }
        Err(error) => Err(socket_error("SCTP send", error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_header_parsing() {
        let mut notification = [0u8; 8];
        notification[..2].copy_from_slice(&SCTP_ASSOC_CHANGE.to_ne_bytes());
        notification[4..].copy_from_slice(&8u32.to_ne_bytes());
        assert_eq!(Some(SCTP_ASSOC_CHANGE), notification_type(&notification));
        assert_eq!(Some(8), read_u32(&notification, 4));
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        assert_eq!(None, init_ids(thread, Parameters::default()).await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_send_failed_buffer_is_direct() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let buffer = byte_buffer(&thread, b"sctp")
            .await
            .expect("create direct byte buffer");
        let is_direct = thread
            .try_invoke(
                "java.nio.DirectByteBuffer",
                "isDirect()Z",
                std::slice::from_ref(&buffer),
            )
            .await
            .expect("invoke isDirect");
        assert!(is_direct.as_bool()?);
        let first = thread
            .try_invoke(
                "java.nio.DirectByteBuffer",
                "get(I)B",
                &[buffer, Value::Int(0)],
            )
            .await
            .expect("read direct byte buffer");
        assert_eq!(i32::from(b's'), first.as_i32()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_receive_rejects_missing_parameters() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        assert!(receive0(thread, Parameters::default()).await.is_err());
    }

    #[tokio::test]
    async fn test_send_rejects_missing_parameters() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        assert!(send0(thread, Parameters::default()).await.is_err());
    }
}
