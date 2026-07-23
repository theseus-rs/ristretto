use crate::net_helpers::{
    InetAddressValue, ipv4_from_java_bytes, is_reachable, java_inet_address, lookup_addresses,
    reverse_lookup,
};
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::Error::InternalError;
use ristretto_types::{Parameters, Result, Thread, VM};
use std::net::IpAddr;
use std::sync::Arc;

#[intrinsic_method("java/net/Inet4AddressImpl.getHostByAddr([B)Ljava/lang/String;", Any)]
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
        ipv4_from_java_bytes(bytes).ok_or_else(|| {
            ristretto_types::JavaError::UnknownHostException("invalid IPv4 address".to_string())
        })?
    };
    let hostname = reverse_lookup(IpAddr::V4(address)).await?;
    Ok(Some(thread.intern_string(&hostname).await?))
}

#[intrinsic_method("java/net/Inet4AddressImpl.getLocalHostName()Ljava/lang/String;", Any)]
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

#[intrinsic_method("java/net/Inet4AddressImpl.isReachable0([BI[BI)Z", Any)]
#[async_method]
pub async fn is_reachable_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // Descriptor order is address, timeout, interface address, TTL. Operands are
    // popped in reverse order from the native invocation parameter stack.
    let ttl = parameters.pop_int()?;
    let if_ref = parameters.pop_reference()?;
    let timeout = parameters.pop_int()?;
    let addr_ref = parameters.pop_reference()?;
    let Some(addr_ref) = addr_ref else {
        return Ok(Some(Value::Int(0)));
    };
    let address = {
        let guard = addr_ref.read();
        let bytes = guard.as_byte_vec_ref()?;
        let Some(address) = ipv4_from_java_bytes(bytes) else {
            return Ok(Some(Value::Int(0)));
        };
        InetAddressValue::V4(address)
    };
    let source = if let Some(source) = if_ref {
        let source = source.read();
        let bytes = source.as_byte_vec_ref()?;
        ipv4_from_java_bytes(bytes).map(InetAddressValue::V4)
    } else {
        None
    };
    Ok(Some(Value::Int(i32::from(
        is_reachable(address, source, ttl, timeout).await?,
    ))))
}

#[intrinsic_method(
    "java/net/Inet4AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;",
    Any
)]
#[async_method]
pub async fn lookup_all_host_addr<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host_value = parameters.pop()?;
    let hostname = host_value.as_string()?;

    let addresses = lookup_addresses(&hostname).await?;
    let mut elements = Vec::new();
    for address in addresses {
        if let IpAddr::V4(address) = address {
            elements.push(
                java_inet_address(&thread, InetAddressValue::V4(address), Some(&hostname)).await?,
            );
        }
    }
    if elements.is_empty() {
        return Err(ristretto_types::JavaError::UnknownHostException(hostname).into());
    }
    let inet_addr_class = thread.class("[Ljava/net/InetAddress;").await?;
    let reference = Reference::try_from((inet_addr_class, elements))?;
    let array_value = Value::new_object(thread.vm()?.garbage_collector(), reference);
    Ok(Some(array_value))
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
    async fn test_lookup_all_host_addr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_all_host_addr(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn address_operations_cover_success_and_validation_paths() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let loopback = byte_array(vm.as_ref(), &[127, 0, 0, 1]);
        assert!(
            get_host_by_addr(thread.clone(), Parameters::new(vec![loopback.clone()]))
                .await?
                .is_some()
        );
        assert!(
            get_host_by_addr(
                thread.clone(),
                Parameters::new(vec![byte_array(vm.as_ref(), &[127, 0, 0])])
            )
            .await
            .is_err()
        );
        assert!(
            get_host_by_addr(thread.clone(), Parameters::new(vec![Value::Object(None)]))
                .await
                .is_err()
        );

        let null_address = Parameters::new(vec![
            Value::Object(None),
            Value::Int(10),
            Value::Object(None),
            Value::Int(1),
        ]);
        assert_eq!(
            Some(Value::Int(0)),
            is_reachable_0(thread.clone(), null_address).await?
        );
        let invalid_address = Parameters::new(vec![
            byte_array(vm.as_ref(), &[127, 0, 0]),
            Value::Int(10),
            Value::Object(None),
            Value::Int(1),
        ]);
        assert_eq!(
            Some(Value::Int(0)),
            is_reachable_0(thread.clone(), invalid_address).await?
        );
        let reachable = Parameters::new(vec![
            loopback.clone(),
            Value::Int(10),
            loopback,
            Value::Int(1),
        ]);
        assert_eq!(
            Some(Value::Int(1)),
            is_reachable_0(thread.clone(), reachable).await?
        );

        let hostname = thread.intern_string("127.0.0.1").await?;
        let addresses = lookup_all_host_addr(thread, Parameters::new(vec![hostname])).await?;
        assert!(addresses.is_some());
        Ok(())
    }
}
