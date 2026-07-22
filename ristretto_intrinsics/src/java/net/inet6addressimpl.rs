use crate::net_helpers::{
    InetAddressValue, ipv4_from_java_bytes, ipv6_from_java_bytes, is_reachable, java_inet_address,
    lookup_addresses, reverse_lookup,
};
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::Error::InternalError;
use ristretto_types::{Parameters, Result, Thread, VM};
use std::net::IpAddr;
use std::sync::Arc;

#[intrinsic_method("java/net/Inet6AddressImpl.getHostByAddr([B)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_host_by_addr<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let array_ref = parameters.pop_reference()?;
    let Some(array_ref) = array_ref else {
        return Err(ristretto_types::JavaError::NullPointerException(Some(
            "byte array is null".to_string(),
        ))
        .into());
    };
    let address = {
        let guard = array_ref.read();
        let bytes = guard.as_byte_vec_ref()?;
        if let Some(addr) = ipv4_from_java_bytes(bytes) {
            IpAddr::V4(addr)
        } else if let Some(addr) = ipv6_from_java_bytes(bytes) {
            IpAddr::V6(addr)
        } else {
            return Err(ristretto_types::JavaError::UnknownHostException(
                "invalid IP address".to_string(),
            )
            .into());
        }
    };
    let hostname = reverse_lookup(address).await?;
    Ok(Some(thread.intern_string(&hostname).await?))
}

#[intrinsic_method("java/net/Inet6AddressImpl.getLocalHostName()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_local_host_name<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    let hostname = whoami::hostname().map_err(|error| InternalError(error.to_string()))?;
    #[cfg(target_family = "wasm")]
    let hostname = String::from("localhost");
    let string_value = thread.intern_string(&hostname).await?;
    Ok(Some(string_value))
}

#[intrinsic_method("java/net/Inet6AddressImpl.isReachable0([BII[BII)Z", Any)]
#[async_method]
pub async fn is_reachable_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Descriptor order is address, scope, timeout, interface address, TTL,
    // interface scope. Operands are popped in reverse order.
    let if_scope = parameters.pop_int()?;
    let ttl = parameters.pop_int()?;
    let if_ref = parameters.pop_reference()?;
    let timeout = parameters.pop_int()?;
    let scope = parameters.pop_int()?;
    let addr_ref = parameters.pop_reference()?;
    let Some(addr_ref) = addr_ref else {
        return Ok(Some(Value::Int(0)));
    };
    let address = {
        let guard = addr_ref.read();
        let bytes = guard.as_byte_vec_ref()?;
        if let Some(address) = ipv4_from_java_bytes(bytes) {
            InetAddressValue::V4(address)
        } else if let Some(address) = ipv6_from_java_bytes(bytes) {
            InetAddressValue::V6(address, u32::try_from(scope).unwrap_or(0))
        } else {
            return Ok(Some(Value::Int(0)));
        }
    };
    let source = if let Some(source) = if_ref {
        let source = source.read();
        let bytes = source.as_byte_vec_ref()?;
        if let Some(address) = ipv4_from_java_bytes(bytes) {
            Some(InetAddressValue::V4(address))
        } else {
            ipv6_from_java_bytes(bytes)
                .map(|address| InetAddressValue::V6(address, u32::try_from(if_scope).unwrap_or(0)))
        }
    } else {
        None
    };
    Ok(Some(Value::Int(i32::from(
        is_reachable(address, source, ttl, timeout).await?,
    ))))
}

/// Helper to perform the lookup and create the `InetAddress` array result.
async fn do_lookup_all_host_addr<T: Thread + 'static>(
    thread: &Arc<T>,
    mut parameters: Parameters,
    flags: Option<i32>,
) -> Result<Option<Value>> {
    let host_value = parameters.pop()?;
    let hostname = host_value.as_string()?;

    let mut addresses = lookup_addresses(&hostname).await?;
    if let Some(flags) = flags {
        const IPV4: i32 = 1;
        const IPV6: i32 = 2;
        const IPV4_FIRST: i32 = 4;
        const IPV6_FIRST: i32 = 8;
        let families = flags & (IPV4 | IPV6);
        addresses.retain(|address| match address {
            IpAddr::V4(_) => families == 0 || flags & IPV4 != 0,
            IpAddr::V6(_) => families == 0 || flags & IPV6 != 0,
        });
        if flags & IPV4_FIRST != 0 {
            addresses.sort_by_key(|address| i32::from(address.is_ipv6()));
        } else if flags & IPV6_FIRST != 0 {
            addresses.sort_by_key(|address| i32::from(address.is_ipv4()));
        }
    }
    if addresses.is_empty() {
        return Err(ristretto_types::JavaError::UnknownHostException(hostname).into());
    }
    let mut elements = Vec::with_capacity(addresses.len());
    for address in addresses {
        let address = match address {
            IpAddr::V4(address) => InetAddressValue::V4(address),
            IpAddr::V6(address) => InetAddressValue::V6(address, 0),
        };
        elements.push(java_inet_address(thread, address, Some(&hostname)).await?);
    }
    let inet_addr_class = thread.class("[Ljava/net/InetAddress;").await?;
    let reference = Reference::try_from((inet_addr_class, elements))?;
    let array_value = Value::new_object(thread.vm()?.garbage_collector(), reference);
    Ok(Some(array_value))
}

#[intrinsic_method(
    "java/net/Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn lookup_all_host_addr_0<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    do_lookup_all_host_addr(&thread, parameters, None).await
}

#[intrinsic_method(
    "java/net/Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;I)[Ljava/net/InetAddress;",
    GreaterThan(JAVA_17)
)]
#[async_method]
pub async fn lookup_all_host_addr_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags = parameters.pop_int()?;
    do_lookup_all_host_addr(&thread, parameters, Some(flags)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn byte_array<V: VM + ?Sized>(vm: &V, bytes: &[i8]) -> Value {
        Value::new_object(
            vm.garbage_collector(),
            Reference::ByteArray(bytes.to_vec().into_boxed_slice()),
        )
    }

    #[tokio::test]
    async fn test_get_host_by_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_host_by_addr(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_local_host_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_local_host_name(thread, Parameters::default()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_is_reachable_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_reachable_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lookup_all_host_addr_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = lookup_all_host_addr_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lookup_all_host_addr_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_all_host_addr_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn address_operations_cover_both_families_and_flags() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let loopback_v4 = byte_array(vm.as_ref(), &[127, 0, 0, 1]);
        let loopback_v6 = byte_array(
            vm.as_ref(),
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        );
        assert!(
            get_host_by_addr(thread.clone(), Parameters::new(vec![loopback_v4.clone()]))
                .await?
                .is_some()
        );
        assert!(
            get_host_by_addr(thread.clone(), Parameters::new(vec![loopback_v6.clone()]))
                .await?
                .is_some()
        );
        assert!(
            get_host_by_addr(
                thread.clone(),
                Parameters::new(vec![byte_array(vm.as_ref(), &[0; 15])])
            )
            .await
            .is_err()
        );

        let null_address = Parameters::new(vec![
            Value::Object(None),
            Value::Int(0),
            Value::Int(10),
            Value::Object(None),
            Value::Int(1),
            Value::Int(0),
        ]);
        assert_eq!(
            Some(Value::Int(0)),
            is_reachable_0(thread.clone(), null_address).await?
        );
        let invalid_address = Parameters::new(vec![
            byte_array(vm.as_ref(), &[0; 15]),
            Value::Int(0),
            Value::Int(10),
            Value::Object(None),
            Value::Int(1),
            Value::Int(0),
        ]);
        assert_eq!(
            Some(Value::Int(0)),
            is_reachable_0(thread.clone(), invalid_address).await?
        );
        let reachable_v4 = Parameters::new(vec![
            loopback_v4.clone(),
            Value::Int(0),
            Value::Int(10),
            loopback_v4,
            Value::Int(1),
            Value::Int(0),
        ]);
        assert_eq!(
            Some(Value::Int(1)),
            is_reachable_0(thread.clone(), reachable_v4).await?
        );
        let reachable_v6 = Parameters::new(vec![
            loopback_v6.clone(),
            Value::Int(0),
            Value::Int(10),
            loopback_v6,
            Value::Int(1),
            Value::Int(0),
        ]);
        assert_eq!(
            Some(Value::Int(1)),
            is_reachable_0(thread.clone(), reachable_v6).await?
        );

        let ipv4_host = thread.intern_string("127.0.0.1").await?;
        assert!(
            lookup_all_host_addr_1(
                thread.clone(),
                Parameters::new(vec![ipv4_host, Value::Int(1 | 4)])
            )
            .await?
            .is_some()
        );
        let ipv6_host = thread.intern_string("::1").await?;
        assert!(
            lookup_all_host_addr_1(
                thread.clone(),
                Parameters::new(vec![ipv6_host, Value::Int(2 | 8)])
            )
            .await?
            .is_some()
        );
        let filtered_host = thread.intern_string("::1").await?;
        assert!(
            lookup_all_host_addr_1(thread, Parameters::new(vec![filtered_host, Value::Int(1)]))
                .await
                .is_err()
        );
        Ok(())
    }
}
