use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::{Parameters, Result, Thread, VM};
use std::net::{Ipv4Addr, Ipv6Addr, ToSocketAddrs};
use std::sync::Arc;

#[intrinsic_method("java/net/Inet6AddressImpl.getHostByAddr([B)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_host_by_addr<T: ristretto_types::Thread + 'static>(
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
    let is_loopback = {
        let guard = array_ref.read();
        let bytes = guard.as_byte_vec_ref()?;
        if bytes.len() == 4 {
            #[expect(clippy::cast_sign_loss)]
            let addr = Ipv4Addr::new(
                bytes[0] as u8,
                bytes[1] as u8,
                bytes[2] as u8,
                bytes[3] as u8,
            );
            addr.is_loopback()
        } else if bytes.len() == 16 {
            #[expect(clippy::cast_sign_loss)]
            let octets: [u8; 16] = bytes
                .iter()
                .map(|&b| b as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .map_err(|_| InternalError("Invalid IPv6 address length".to_string()))?;
            let addr = Ipv6Addr::from(octets);
            addr.is_loopback()
        } else {
            false
        }
    };

    if is_loopback {
        let hostname = thread.intern_string("localhost").await?;
        return Ok(Some(hostname));
    }

    Err(InternalError(
        "Reverse DNS lookup not supported".to_string(),
    ))
}

#[intrinsic_method("java/net/Inet6AddressImpl.getLocalHostName()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_local_host_name<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let hostname = whoami::hostname().map_err(|error| InternalError(error.to_string()))?;
    let string_value = thread.intern_string(&hostname).await?;
    Ok(Some(string_value))
}

#[intrinsic_method("java/net/Inet6AddressImpl.isReachable0([BII[BII)Z", Any)]
#[async_method]
pub async fn is_reachable_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_int()?;
    let _if_scope = parameters.pop_int()?;
    let _if_ref = parameters.pop_reference()?;
    let _ttl = parameters.pop_int()?;
    let _scope = parameters.pop_int()?;
    let addr_ref = parameters.pop_reference()?;
    let Some(addr_ref) = addr_ref else {
        return Ok(Some(Value::Int(0)));
    };
    let guard = addr_ref.read();
    let bytes = guard.as_byte_vec_ref()?;

    if bytes.len() == 4 {
        #[expect(clippy::cast_sign_loss)]
        let addr = Ipv4Addr::new(
            bytes[0] as u8,
            bytes[1] as u8,
            bytes[2] as u8,
            bytes[3] as u8,
        );
        if addr.is_loopback() {
            return Ok(Some(Value::Int(1)));
        }
    } else if bytes.len() == 16 {
        #[expect(clippy::cast_sign_loss)]
        let octets: [u8; 16] = bytes
            .iter()
            .map(|&b| b as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| InternalError("Invalid IPv6 address length".to_string()))?;
        let addr = Ipv6Addr::from(octets);
        if addr.is_loopback() {
            return Ok(Some(Value::Int(1)));
        }
    }
    Ok(Some(Value::Int(0)))
}

/// Helper to perform the lookup and create the `InetAddress` array result.
async fn do_lookup_all_host_addr<T: ristretto_types::Thread + 'static>(
    thread: &Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host_value = parameters.pop()?;
    let hostname = host_value.as_string()?;

    let addr_bytes: Vec<u8> = if hostname == "localhost" || hostname == "127.0.0.1" {
        vec![127, 0, 0, 1]
    } else if let Ok(addr) = hostname.parse::<Ipv4Addr>() {
        addr.octets().to_vec()
    } else if let Ok(addr) = hostname.parse::<Ipv6Addr>() {
        addr.octets().to_vec()
    } else {
        let socket_addrs: Vec<_> = (hostname.as_str(), 0u16)
            .to_socket_addrs()
            .map_err(|e| InternalError(format!("Unknown host: {hostname}: {e}")))?
            .collect();
        socket_addrs
            .first()
            .map(|sa| match sa {
                std::net::SocketAddr::V4(v4) => v4.ip().octets().to_vec(),
                std::net::SocketAddr::V6(v6) => v6.ip().octets().to_vec(),
            })
            .ok_or_else(|| InternalError(format!("No address found for: {hostname}")))?
    };

    #[expect(clippy::cast_possible_wrap)]
    let byte_array: Box<[i8]> = addr_bytes.iter().map(|&b| b as i8).collect();
    let byte_array_value = Value::new_object(
        thread.vm()?.garbage_collector(),
        Reference::ByteArray(byte_array),
    );

    let host_string = thread.intern_string(&hostname).await?;

    let result = thread
        .invoke(
            "java.net.InetAddress",
            "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
            &[host_string, byte_array_value],
        )
        .await?;

    let Some(addr_value) = result else {
        return Err(InternalError("getByAddress returned null".to_string()));
    };

    let inet_addr_class = thread.class("[Ljava/net/InetAddress;").await?;
    let elements = vec![addr_value];
    let reference = Reference::try_from((inet_addr_class, elements))?;
    let array_value = Value::new_object(thread.vm()?.garbage_collector(), reference);
    Ok(Some(array_value))
}

#[intrinsic_method(
    "java/net/Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;)[Ljava/net/InetAddress;",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn lookup_all_host_addr_0<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    do_lookup_all_host_addr(&thread, parameters).await
}

#[intrinsic_method(
    "java/net/Inet6AddressImpl.lookupAllHostAddr(Ljava/lang/String;I)[Ljava/net/InetAddress;",
    GreaterThan(JAVA_17)
)]
#[async_method]
pub async fn lookup_all_host_addr_1<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _flags = parameters.pop_int()?;
    do_lookup_all_host_addr(&thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_all_host_addr_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lookup_all_host_addr_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lookup_all_host_addr_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
