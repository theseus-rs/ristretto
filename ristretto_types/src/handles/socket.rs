//! Socket handle types for managing TCP and UDP sockets.
//!
//! # Architecture: Async Socket I/O
//!
//! Socket handles use `tokio::net` types for true async I/O when possible:
//!
//! - **`Raw(socket2::Socket)`**: Pre-configured sockets before bind/connect/listen.
//!   Used during socket creation and option setting. Transitions to a specific
//!   async type once the socket's role is determined.
//!
//! - **`TcpStream(tokio::net::TcpStream)`**: Connected TCP sockets. Provides async
//!   `readable()`/`writable()` and `try_read()`/`try_write()` for non-blocking I/O
//!   that cooperates with the tokio runtime instead of blocking OS threads.
//!
//! - **`TcpListener(Arc<tokio::net::TcpListener>)`**: TCP server sockets. Arc-wrapped
//!   to allow cloning the handle for concurrent accept without removing from the map.
//!
//! - **`UdpSocket(tokio::net::UdpSocket)`**: UDP sockets. Provides async
//!   `send_to()`/`recv_from()` for datagram I/O.
//!
//! This design eliminates the need for `spawn_blocking` + `try_clone()` on I/O
//! operations, which previously tied up threads from the blocking thread pool.

#[cfg(not(target_family = "wasm"))]
use socket2::Socket;
#[cfg(not(target_family = "wasm"))]
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use std::time::Duration;

/// Represents the underlying socket type managed by the VM.
///
/// Sockets start as `Raw` during creation and transition to specific async types:
/// - After `listen()` -> `TcpListener`
/// - After `connect()` -> `TcpStream`
/// - After `bind()` (UDP) -> `UdpSocket`
/// - After `accept()` on a listener -> new `TcpStream`
#[cfg(not(target_family = "wasm"))]
#[derive(Debug)]
pub enum SocketType {
    /// Raw socket2 socket; used during creation and configuration before
    /// transitioning to a specific async socket type.
    Raw(Socket),
    /// Connected TCP stream for async read/write.
    TcpStream(Arc<tokio::net::TcpStream>),
    /// TCP server listener for async accept.
    TcpListener(Arc<tokio::net::TcpListener>),
    /// UDP socket for async send/receive.
    UdpSocket(tokio::net::UdpSocket),
}

#[cfg(not(target_family = "wasm"))]
impl SocketType {
    /// Returns a reference to the raw socket, if this is a `Raw` variant.
    pub fn as_raw(&self) -> Option<&Socket> {
        match self {
            SocketType::Raw(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the TCP stream, if this is a `TcpStream` variant.
    pub fn as_tcp_stream(&self) -> Option<&Arc<tokio::net::TcpStream>> {
        match self {
            SocketType::TcpStream(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the TCP listener, if this is a `TcpListener` variant.
    pub fn as_tcp_listener(&self) -> Option<&Arc<tokio::net::TcpListener>> {
        match self {
            SocketType::TcpListener(l) => Some(l),
            _ => None,
        }
    }

    /// Returns a reference to the UDP socket, if this is a `UdpSocket` variant.
    pub fn as_udp_socket(&self) -> Option<&tokio::net::UdpSocket> {
        match self {
            SocketType::UdpSocket(u) => Some(u),
            _ => None,
        }
    }

    /// Get the raw file descriptor for this socket type.
    ///
    /// This can be used with `socket2::SockRef` for option operations.
    #[cfg(unix)]
    #[must_use]
    pub fn raw_fd(&self) -> std::os::fd::RawFd {
        use std::os::fd::AsRawFd;
        match self {
            SocketType::Raw(s) => s.as_raw_fd(),
            SocketType::TcpStream(s) => s.as_raw_fd(),
            SocketType::TcpListener(s) => s.as_raw_fd(),
            SocketType::UdpSocket(s) => s.as_raw_fd(),
        }
    }

    /// Get the raw socket handle for this socket type (Windows).
    #[cfg(windows)]
    #[must_use]
    pub fn raw_socket(&self) -> std::os::windows::io::RawSocket {
        use std::os::windows::io::AsRawSocket;
        match self {
            SocketType::Raw(s) => s.as_raw_socket(),
            SocketType::TcpStream(s) => s.as_raw_socket(),
            SocketType::TcpListener(s) => s.as_raw_socket(),
            SocketType::UdpSocket(s) => s.as_raw_socket(),
        }
    }
}

/// A managed socket handle that bundles the socket type with per-socket metadata.
///
/// This consolidates what was previously spread across separate `SocketTimeouts`,
/// `SocketDomains`, and `SocketModes` maps into a single struct stored in the
/// `HandleManager`.
#[cfg(not(target_family = "wasm"))]
#[derive(Debug)]
pub struct SocketHandle {
    /// The underlying socket type (`Raw`, `TcpStream`, `TcpListener`, `UdpSocket`).
    pub socket_type: SocketType,
    /// The `SO_TIMEOUT` value for this socket. `None` means no timeout.
    pub timeout: Option<Duration>,
    /// Whether this socket uses IPv6.
    pub is_ipv6: bool,
    /// Whether this socket is in non-blocking mode.
    pub non_blocking: bool,
}

#[cfg(not(target_family = "wasm"))]
impl SocketHandle {
    /// Create a new `SocketHandle` with the given socket type and default metadata.
    #[must_use]
    pub fn new(socket_type: SocketType) -> Self {
        Self {
            socket_type,
            timeout: None,
            is_ipv6: false,
            non_blocking: false,
        }
    }

    /// Get the timeout in milliseconds. Returns 0 if no timeout is set.
    #[must_use]
    #[expect(clippy::cast_possible_truncation)]
    pub fn timeout_millis(&self) -> i32 {
        self.timeout.map_or(0, |d| d.as_millis() as i32)
    }
}
