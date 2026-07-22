use crate::net_helpers::{InetAddressValue, inet_address_value, java_inet_address};
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::{JavaError, Parameters, Result, Thread, VM};
#[cfg(windows)]
use std::mem::size_of;
#[cfg(all(not(target_family = "wasm"), not(windows)))]
use std::net::IpAddr;
use std::net::Ipv4Addr;
#[cfg(any(windows, test))]
use std::net::Ipv6Addr;
use std::sync::Arc;
#[cfg(windows)]
use windows_sys::Win32::Networking::WinSock::{
    AF_INET, AF_INET6, SOCKADDR, SOCKADDR_IN, SOCKADDR_IN6,
};

#[derive(Clone, Debug)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "these values directly model independent NetworkInterface flags"
)]
struct InterfaceData {
    name: String,
    display_name: String,
    index: i32,
    addresses: Vec<(InetAddressValue, u8)>,
    mac: Option<Vec<u8>>,
    mtu: i32,
    up: bool,
    loopback: bool,
    point_to_point: bool,
    multicast: bool,
}

#[cfg(any(windows, test))]
fn windows_ipv6_scope(address: Ipv6Addr, scope: u32, interface_index: u32) -> u32 {
    if scope == 0 && (address.is_unicast_link_local() || address.is_multicast()) {
        interface_index
    } else {
        scope
    }
}

#[cfg(windows)]
#[expect(unsafe_code)]
unsafe fn windows_inet_address(
    socket_address: *const SOCKADDR,
    interface_index: u32,
) -> Option<(InetAddressValue, u8)> {
    if socket_address.is_null() {
        return None;
    }
    // SAFETY: the caller provides a valid sockaddr returned by GetAdaptersAddresses.
    let family = unsafe { (*socket_address).sa_family };
    if family == AF_INET {
        // SAFETY: the address family identifies the value as SOCKADDR_IN. Windows only
        // exposes SOCKADDR alignment in the pointer type, so read the larger value unaligned.
        let address = unsafe { socket_address.cast::<SOCKADDR_IN>().read_unaligned() };
        // SAFETY: reading the byte representation of the address union is valid.
        let bytes = unsafe { address.sin_addr.S_un.S_un_b };
        return Some((
            InetAddressValue::V4(Ipv4Addr::new(
                bytes.s_b1, bytes.s_b2, bytes.s_b3, bytes.s_b4,
            )),
            32,
        ));
    }
    if family == AF_INET6 {
        // SAFETY: the address family identifies the value as SOCKADDR_IN6. See the
        // SOCKADDR_IN case above for why this must be an unaligned read.
        let address = unsafe { socket_address.cast::<SOCKADDR_IN6>().read_unaligned() };
        // SAFETY: reading the byte representation of the address union is valid.
        let bytes = unsafe { address.sin6_addr.u.Byte };
        let ip = Ipv6Addr::from(bytes);
        let scope = unsafe { address.Anonymous.sin6_scope_id };
        let scope = windows_ipv6_scope(ip, scope, interface_index);
        return Some((InetAddressValue::V6(ip, scope), 128));
    }
    None
}

#[cfg(windows)]
#[expect(unsafe_code)]
unsafe fn windows_wide_string(pointer: *const u16) -> String {
    if pointer.is_null() {
        return String::new();
    }
    let mut length = 0;
    // SAFETY: Windows returns a nul-terminated string owned by the adapter buffer.
    while unsafe { *pointer.add(length) } != 0 {
        length += 1;
    }
    // SAFETY: `length` was determined by scanning the same allocation.
    String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(pointer, length) })
}

#[cfg(all(not(target_family = "wasm"), not(windows)))]
fn interfaces() -> Vec<InterfaceData> {
    use sysinfo::{InterfaceOperationalState, Networks};

    let networks = Networks::new_with_refreshed_list();
    let mut entries: Vec<_> = networks.iter().collect();
    entries.sort_by_key(|(name, _)| (*name).clone());
    entries
        .into_iter()
        .enumerate()
        .map(|(position, (name, network))| {
            let addresses: Vec<_> = network
                .ip_networks()
                .iter()
                .map(|network| {
                    let address = match network.addr {
                        IpAddr::V4(address) => InetAddressValue::V4(address),
                        // OpenJDK associates enumerated IPv6 addresses with
                        // their NetworkInterface even when the kernel sockaddr
                        // carries a zero scope (including global addresses).
                        IpAddr::V6(address) => InetAddressValue::V6(address, interface_index(name)),
                    };
                    (address, network.prefix)
                })
                .collect();
            let inferred_loopback = !addresses.is_empty()
                && addresses
                    .iter()
                    .all(|(address, _)| address.ip().is_loopback());
            let flags = interface_flags(name);
            let loopback = flags.map_or(inferred_loopback, |flags| flags & flag_loopback() != 0);
            let up = flags.map_or_else(
                || {
                    matches!(
                        network.operational_state(),
                        InterfaceOperationalState::Up
                            | InterfaceOperationalState::Dormant
                            | InterfaceOperationalState::Unknown
                    )
                },
                |flags| flags & flag_up() != 0,
            );
            let mac =
                (!network.mac_address().is_unspecified()).then(|| network.mac_address().0.to_vec());
            let index = interface_index(name);
            InterfaceData {
                name: name.clone(),
                display_name: name.clone(),
                index: i32::try_from(if index == 0 {
                    u32::try_from(position + 1).unwrap_or(u32::MAX)
                } else {
                    index
                })
                .unwrap_or(i32::MAX),
                addresses,
                mac,
                mtu: i32::try_from(network.mtu()).unwrap_or(i32::MAX),
                up,
                loopback,
                point_to_point: flags.is_some_and(|flags| flags & flag_point_to_point() != 0),
                multicast: flags.map_or(!loopback, |flags| flags & flag_multicast() != 0),
            }
        })
        .collect()
}

#[cfg(windows)]
#[expect(unsafe_code)]
fn interfaces() -> Vec<InterfaceData> {
    use std::ffi::CStr;
    use windows_sys::Win32::Foundation::ERROR_BUFFER_OVERFLOW;
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        GAA_FLAG_INCLUDE_ALL_INTERFACES, GAA_FLAG_INCLUDE_PREFIX, GetAdaptersAddresses,
        IF_TYPE_PPP, IF_TYPE_SOFTWARE_LOOPBACK, IF_TYPE_TUNNEL, IP_ADAPTER_ADDRESSES_LH,
        IP_ADAPTER_NO_MULTICAST,
    };
    use windows_sys::Win32::NetworkManagement::Ndis::IfOperStatusUp;
    use windows_sys::Win32::Networking::WinSock::AF_UNSPEC;

    let flags = GAA_FLAG_INCLUDE_PREFIX | GAA_FLAG_INCLUDE_ALL_INTERFACES;
    let mut size = 15_000u32;
    // The API writes a linked list containing pointer-aligned structures, so
    // use pointer-sized storage rather than casting an alignment-1 byte buffer.
    let mut buffer = Vec::<usize>::new();
    loop {
        let byte_length = usize::try_from(size).unwrap_or(15_000);
        buffer.resize(byte_length.div_ceil(size_of::<usize>()), 0);
        // SAFETY: `buffer` is writable for `size` bytes and the API reports a required larger
        // allocation through `size` before writing past it.
        let result = unsafe {
            GetAdaptersAddresses(
                u32::from(AF_UNSPEC),
                flags,
                std::ptr::null(),
                buffer.as_mut_ptr().cast::<IP_ADAPTER_ADDRESSES_LH>(),
                &raw mut size,
            )
        };
        if result == ERROR_BUFFER_OVERFLOW {
            continue;
        }
        if result != 0 {
            return Vec::new();
        }
        break;
    }

    let mut result = Vec::new();
    let mut adapter = buffer.as_ptr().cast::<IP_ADAPTER_ADDRESSES_LH>();
    while !adapter.is_null() {
        // SAFETY: every node belongs to the successfully populated adapter buffer.
        let data = unsafe { &*adapter };
        let name = if data.AdapterName.is_null() {
            format!("net{}", result.len())
        } else {
            // SAFETY: `AdapterName` is a nul-terminated string owned by the adapter buffer.
            unsafe { CStr::from_ptr(data.AdapterName.cast()) }
                .to_string_lossy()
                .into_owned()
        };
        // SAFETY: `FriendlyName` is a nul-terminated string owned by the adapter buffer.
        let display_name = unsafe { windows_wide_string(data.FriendlyName) };
        let ipv4_index = unsafe { data.Anonymous1.Anonymous.IfIndex };
        let index = if data.Ipv6IfIndex != 0 {
            data.Ipv6IfIndex
        } else {
            ipv4_index
        };
        let mut addresses = Vec::new();
        let mut unicast = data.FirstUnicastAddress;
        while !unicast.is_null() {
            // SAFETY: unicast nodes are linked from the current adapter node.
            let unicast_data = unsafe { &*unicast };
            let socket_address = unicast_data.Address.lpSockaddr;
            // SAFETY: the sockaddr is owned by the current unicast-address node.
            if let Some((address, max_prefix)) =
                unsafe { windows_inet_address(socket_address, index) }
            {
                addresses.push((address, unicast_data.OnLinkPrefixLength.min(max_prefix)));
            }
            unicast = unicast_data.Next;
        }
        let mac_length = usize::try_from(data.PhysicalAddressLength)
            .unwrap_or(0)
            .min(data.PhysicalAddress.len());
        let mac = data
            .PhysicalAddress
            .get(..mac_length)
            .filter(|address| !address.is_empty())
            .map(<[u8]>::to_vec);
        let adapter_flags = unsafe { data.Anonymous2.Flags };
        result.push(InterfaceData {
            name,
            display_name,
            index: i32::try_from(index).unwrap_or(i32::MAX),
            addresses,
            mac,
            mtu: i32::try_from(data.Mtu).unwrap_or(i32::MAX),
            up: data.OperStatus == IfOperStatusUp,
            loopback: data.IfType == IF_TYPE_SOFTWARE_LOOPBACK,
            point_to_point: matches!(data.IfType, IF_TYPE_PPP | IF_TYPE_TUNNEL),
            multicast: adapter_flags & IP_ADAPTER_NO_MULTICAST == 0,
        });
        adapter = data.Next;
    }
    result.sort_by_key(|interface| interface.index);
    result
}

#[cfg(target_family = "wasm")]
fn interfaces() -> Vec<InterfaceData> {
    vec![InterfaceData {
        name: "lo".to_string(),
        display_name: "lo".to_string(),
        index: 1,
        addresses: vec![(InetAddressValue::V4(Ipv4Addr::LOCALHOST), 8)],
        mac: None,
        mtu: 65_536,
        up: true,
        loopback: true,
        point_to_point: false,
        multicast: false,
    }]
}

#[cfg(unix)]
fn interface_index(name: &str) -> u32 {
    let Ok(name) = std::ffi::CString::new(name) else {
        return 0;
    };
    #[expect(unsafe_code)]
    // SAFETY: `name` is a valid, nul-terminated interface name.
    unsafe {
        libc::if_nametoindex(name.as_ptr())
    }
}

#[cfg(all(not(unix), not(windows)))]
fn interface_index(_name: &str) -> u32 {
    0
}

#[cfg(unix)]
#[expect(unsafe_code)]
fn interface_flags(name: &str) -> Option<u32> {
    let mut list = std::ptr::null_mut();
    // SAFETY: `list` is a valid output pointer and is released with
    // `freeifaddrs` before returning.
    if unsafe { libc::getifaddrs(&raw mut list) } != 0 {
        return None;
    }
    let mut current = list;
    let mut result = None;
    while !current.is_null() {
        // SAFETY: nodes belong to the list returned by `getifaddrs`.
        let address = unsafe { &*current };
        if !address.ifa_name.is_null() {
            // SAFETY: POSIX guarantees a nul-terminated interface name.
            let current_name = unsafe { std::ffi::CStr::from_ptr(address.ifa_name) };
            if current_name.to_bytes() == name.as_bytes() {
                result = Some(address.ifa_flags);
                break;
            }
        }
        current = address.ifa_next;
    }
    // SAFETY: `list` came from a successful `getifaddrs` call.
    unsafe { libc::freeifaddrs(list) };
    result
}

#[cfg(not(unix))]
fn interface_flags(_name: &str) -> Option<u32> {
    None
}

#[cfg(unix)]
const fn flag_up() -> u32 {
    libc::IFF_UP as u32
}
#[cfg(not(unix))]
const fn flag_up() -> u32 {
    0
}
#[cfg(unix)]
const fn flag_loopback() -> u32 {
    libc::IFF_LOOPBACK as u32
}
#[cfg(not(unix))]
const fn flag_loopback() -> u32 {
    0
}
#[cfg(unix)]
const fn flag_point_to_point() -> u32 {
    libc::IFF_POINTOPOINT as u32
}
#[cfg(not(unix))]
const fn flag_point_to_point() -> u32 {
    0
}
#[cfg(unix)]
const fn flag_multicast() -> u32 {
    libc::IFF_MULTICAST as u32
}
#[cfg(not(unix))]
const fn flag_multicast() -> u32 {
    0
}

fn find_interface(name: Option<&str>, index: i32) -> Option<InterfaceData> {
    interfaces().into_iter().find(|interface| {
        name.is_some_and(|name| interface.name == name) || (index > 0 && interface.index == index)
    })
}

fn address_matches(candidate: InetAddressValue, requested: InetAddressValue) -> bool {
    match (candidate, requested) {
        (InetAddressValue::V4(left), InetAddressValue::V4(right)) => left == right,
        (InetAddressValue::V6(left, left_scope), InetAddressValue::V6(right, right_scope)) => {
            left == right && (right_scope == 0 || left_scope == 0 || left_scope == right_scope)
        }
        _ => false,
    }
}

pub(crate) fn interface_index_by_address(address: InetAddressValue) -> Option<i32> {
    interfaces().into_iter().find_map(|interface| {
        interface
            .addresses
            .iter()
            .any(|(candidate, _)| address_matches(*candidate, address))
            .then_some(interface.index)
    })
}

pub(crate) fn interface_address_by_index(index: i32) -> Option<InetAddressValue> {
    let interface = find_interface(None, index)?;
    interface
        .addresses
        .iter()
        .find_map(|(address, _)| matches!(address, InetAddressValue::V4(_)).then_some(*address))
        .or_else(|| interface.addresses.first().map(|(address, _)| *address))
}

async fn create_interface<T: Thread + 'static>(
    thread: &Arc<T>,
    interface: &InterfaceData,
) -> Result<Value> {
    let vm = thread.vm()?;
    let mut addresses = Vec::with_capacity(interface.addresses.len());
    let mut bindings = Vec::with_capacity(interface.addresses.len());
    for (address, prefix) in &interface.addresses {
        let java_address = java_inet_address(thread, *address, None).await?;
        addresses.push(java_address.clone());
        let broadcast = match address {
            InetAddressValue::V4(address)
                if !interface.point_to_point && (!interface.loopback || cfg!(windows)) =>
            {
                let prefix = u32::from((*prefix).min(32));
                let mask = if prefix == 0 {
                    0
                } else {
                    u32::MAX << (32 - prefix)
                };
                let broadcast = Ipv4Addr::from(u32::from(*address) | !mask);
                java_inet_address(thread, InetAddressValue::V4(broadcast), None).await?
            }
            _ => Value::Object(None),
        };
        let binding = thread.object("java.net.InterfaceAddress", "", &[]).await?;
        {
            let mut binding = binding.as_object_mut()?;
            binding.set_value("address", java_address)?;
            binding.set_value("broadcast", broadcast)?;
            binding.set_value("maskLength", Value::Int(i32::from(*prefix)))?;
        }
        bindings.push(binding);
    }
    let address_class = thread.class("[Ljava/net/InetAddress;").await?;
    let addresses = Value::new_object(
        vm.garbage_collector(),
        Reference::try_from((address_class, addresses))?,
    );
    let binding_class = thread.class("[Ljava/net/InterfaceAddress;").await?;
    let bindings = Value::new_object(
        vm.garbage_collector(),
        Reference::try_from((binding_class, bindings))?,
    );
    let child_class = thread.class("[Ljava/net/NetworkInterface;").await?;
    let children = Value::new_object(
        vm.garbage_collector(),
        Reference::try_from((child_class, Vec::new()))?,
    );
    let name = thread.intern_string(&interface.name).await?;
    let display_name = thread.intern_string(&interface.display_name).await?;
    let value = thread.object("java.net.NetworkInterface", "", &[]).await?;
    {
        let mut object = value.as_object_mut()?;
        object.set_value("name", name)?;
        object.set_value("displayName", display_name)?;
        object.set_value("index", Value::Int(interface.index))?;
        object.set_value("addrs", addresses)?;
        object.set_value("bindings", bindings)?;
        object.set_value("childs", children)?;
        object.set_value("parent", Value::Object(None))?;
        object.set_value("virtual", Value::Int(0))?;
    }
    Ok(value)
}

pub(crate) async fn interface_by_index<T: Thread + 'static>(
    thread: &Arc<T>,
    index: i32,
) -> Result<Value> {
    match find_interface(None, index) {
        Some(interface) => create_interface(thread, &interface).await,
        None => Ok(Value::Object(None)),
    }
}

#[intrinsic_method(
    "java/net/NetworkInterface.boundInetAddress0(Ljava/net/InetAddress;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn bound_inet_address_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = parameters.pop()?;
    let address = inet_address_value(&address)?;
    let bound = interfaces().iter().any(|interface| {
        interface
            .addresses
            .iter()
            .any(|(candidate, _)| address_matches(*candidate, address))
    });
    Ok(Some(Value::Int(i32::from(bound))))
}

#[intrinsic_method("java/net/NetworkInterface.getAll()[Ljava/net/NetworkInterface;", Any)]
#[async_method]
pub async fn get_all<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // Native enumeration constructs InetAddress instances while NetworkInterface itself may still
    // be initializing (DefaultInterface calls getAll from its class initializer). Initialize the
    // address hierarchy first so Java factory calls cannot observe a partially initialized class.
    thread.class("java/net/InetAddress").await?;
    let mut values = Vec::new();
    for interface in interfaces() {
        values.push(create_interface(&thread, &interface).await?);
    }
    let class = thread.class("[Ljava/net/NetworkInterface;").await?;
    let reference = Reference::try_from((class, values))?;
    Ok(Some(Value::new_object(
        thread.vm()?.garbage_collector(),
        reference,
    )))
}

#[intrinsic_method(
    "java/net/NetworkInterface.getByIndex0(I)Ljava/net/NetworkInterface;",
    Any
)]
#[async_method]
pub async fn get_by_index_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    Ok(Some(interface_by_index(&thread, index).await?))
}

#[intrinsic_method(
    "java/net/NetworkInterface.getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;",
    Any
)]
#[async_method]
pub async fn get_by_inet_address_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let address = inet_address_value(&parameters.pop()?)?;
    let interface = interfaces().into_iter().find(|interface| {
        interface
            .addresses
            .iter()
            .any(|(candidate, _)| address_matches(*candidate, address))
    });
    match interface {
        Some(interface) => Ok(Some(create_interface(&thread, &interface).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

#[intrinsic_method(
    "java/net/NetworkInterface.getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;",
    Any
)]
#[async_method]
pub async fn get_by_name_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let name = parameters.pop()?.as_string()?;
    match find_interface(Some(&name), 0) {
        Some(interface) => Ok(Some(create_interface(&thread, &interface).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

fn pop_interface(parameters: &mut Parameters) -> Result<Option<InterfaceData>> {
    let index = parameters.pop_int()?;
    let name = parameters.pop()?.as_string()?;
    Ok(find_interface(Some(&name), index))
}

#[intrinsic_method("java/net/NetworkInterface.getMTU0(Ljava/lang/String;I)I", Any)]
#[async_method]
pub async fn get_mtu_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let interface = pop_interface(&mut parameters)?
        .ok_or_else(|| JavaError::SocketException("Network interface not found".to_string()))?;
    Ok(Some(Value::Int(interface.mtu)))
}

#[intrinsic_method("java/net/NetworkInterface.getMacAddr0([BLjava/lang/String;I)[B", Any)]
#[async_method]
pub async fn get_mac_addr_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let interface = pop_interface(&mut parameters)?;
    let _address = parameters.pop_reference()?;
    let Some(mac) = interface.and_then(|interface| interface.mac) else {
        return Ok(Some(Value::Object(None)));
    };
    #[expect(clippy::cast_possible_wrap)]
    let bytes: Box<[i8]> = mac.into_iter().map(|byte| byte as i8).collect();
    Ok(Some(Value::new_object(
        thread.vm()?.garbage_collector(),
        Reference::ByteArray(bytes),
    )))
}

#[intrinsic_method("java/net/NetworkInterface.init()V", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

fn interface_flag_value(
    parameters: &mut Parameters,
    get_flag: impl FnOnce(&InterfaceData) -> bool,
) -> Result<Option<Value>> {
    let interface = pop_interface(parameters)?
        .ok_or_else(|| JavaError::SocketException("Network interface not found".to_string()))?;
    Ok(Some(Value::Int(i32::from(get_flag(&interface)))))
}

// These definitions must remain explicit: the intrinsic registry scans Rust source before
// macro expansion and therefore cannot discover intrinsic attributes generated by macro_rules!.
#[intrinsic_method("java/net/NetworkInterface.isLoopback0(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub async fn is_loopback_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    interface_flag_value(&mut parameters, |interface| interface.loopback)
}

#[intrinsic_method("java/net/NetworkInterface.isP2P0(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub async fn is_p2p_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    interface_flag_value(&mut parameters, |interface| interface.point_to_point)
}

#[intrinsic_method("java/net/NetworkInterface.isUp0(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub async fn is_up_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    interface_flag_value(&mut parameters, |interface| interface.up)
}

#[intrinsic_method(
    "java/net/NetworkInterface.supportsMulticast0(Ljava/lang/String;I)Z",
    Any
)]
#[async_method]
pub async fn supports_multicast_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    interface_flag_value(&mut parameters, |interface| interface.multicast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv6Addr;

    #[test]
    fn enumerates_at_least_one_interface() {
        assert!(!interfaces().is_empty());
    }

    #[test]
    fn ipv6_address_matching_respects_explicit_scopes() {
        let address = Ipv6Addr::LOCALHOST;
        assert!(address_matches(
            InetAddressValue::V6(address, 4),
            InetAddressValue::V6(address, 0)
        ));
        assert!(address_matches(
            InetAddressValue::V6(address, 4),
            InetAddressValue::V6(address, 4)
        ));
        assert!(!address_matches(
            InetAddressValue::V6(address, 4),
            InetAddressValue::V6(address, 5)
        ));
    }

    #[test]
    fn windows_only_scopes_addresses_that_require_a_zone() {
        assert_eq!(
            windows_ipv6_scope("2001:db8::1".parse().expect("global IPv6 address"), 0, 7),
            0
        );
        assert_eq!(
            windows_ipv6_scope("fe80::1".parse().expect("link-local IPv6 address"), 0, 7),
            7
        );
        assert_eq!(
            windows_ipv6_scope("ff02::1".parse().expect("multicast IPv6 address"), 0, 7),
            7
        );
        assert_eq!(
            windows_ipv6_scope("fe80::1".parse().expect("link-local IPv6 address"), 3, 7),
            3
        );
    }
}
