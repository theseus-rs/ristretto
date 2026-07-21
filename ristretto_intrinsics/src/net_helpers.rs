use ristretto_classloader::Value;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::Error::InternalError;
use ristretto_types::Result;
#[cfg(not(target_family = "wasm"))]
use socket2::SockAddr;
use std::net::{Ipv4Addr, Ipv6Addr};
#[cfg(not(target_family = "wasm"))]
use std::net::{SocketAddrV4, SocketAddrV6};

#[cfg(not(target_family = "wasm"))]
use ristretto_types::handles::SocketType;

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
            ristretto_types::JavaError::SocketException(
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
