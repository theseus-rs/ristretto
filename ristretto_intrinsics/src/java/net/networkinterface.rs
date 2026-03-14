use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, VM};
use std::sync::Arc;

/// Check if the given interface name represents a loopback interface.
#[cfg(target_os = "macos")]
fn is_loopback_name(name: &str) -> bool {
    name == "lo0"
}

/// Check if the given interface name represents a loopback interface.
#[cfg(not(target_os = "macos"))]
fn is_loopback_name(name: &str) -> bool {
    name == "lo"
}

/// Get the platform specific loopback interface name.
#[cfg(target_os = "macos")]
fn loopback_name() -> &'static str {
    "lo0"
}

/// Get the platform specific loopback interface name.
#[cfg(not(target_os = "macos"))]
fn loopback_name() -> &'static str {
    "lo"
}

/// Create a loopback `NetworkInterface` Java object.
async fn create_loopback_interface<T: ristretto_types::Thread + 'static>(
    thread: &Arc<T>,
) -> Result<Value> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let lo_name = loopback_name();
    let name_value = thread.intern_string(lo_name).await?;
    let display_name_value = thread.intern_string(lo_name).await?;

    // Create InetAddress for 127.0.0.1
    let loopback_bytes: Box<[i8]> = vec![127, 0, 0, 1].into_boxed_slice();
    let byte_array_value = Value::new_object(gc, Reference::ByteArray(loopback_bytes));
    let inet_addr = thread
        .invoke(
            "java.net.InetAddress",
            "getByAddress(Ljava/lang/String;[B)Ljava/net/InetAddress;",
            &[Value::Object(None), byte_array_value],
        )
        .await?
        .unwrap_or(Value::Object(None));

    // Create InetAddress[] for addrs
    let inet_addr_class = thread.class("[Ljava/net/InetAddress;").await?;
    let addrs_ref = Reference::try_from((inet_addr_class, vec![inet_addr]))?;
    let addrs_value = Value::new_object(gc, addrs_ref);

    // Create empty NetworkInterface[] for childs
    let ni_arr_class = thread.class("[Ljava/net/NetworkInterface;").await?;
    let childs_ref = Reference::try_from((ni_arr_class, vec![]))?;
    let childs_value = Value::new_object(gc, childs_ref);

    // Create empty InterfaceAddress[] for bindings
    let bindings_class = thread.class("[Ljava/net/InterfaceAddress;").await?;
    let bindings_ref = Reference::try_from((bindings_class, vec![]))?;
    let bindings_value = Value::new_object(gc, bindings_ref);

    // Create the NetworkInterface object with no-arg constructor
    let ni_value = thread
        .object("java.net.NetworkInterface", "()V", &[])
        .await?;
    {
        let mut ni_obj = ni_value.as_object_mut()?;
        ni_obj.set_value("name", name_value)?;
        ni_obj.set_value("displayName", display_name_value)?;
        ni_obj.set_value("index", Value::Int(1))?;
        ni_obj.set_value("addrs", addrs_value)?;
        ni_obj.set_value("bindings", bindings_value)?;
        ni_obj.set_value("childs", childs_value)?;
        ni_obj.set_value("parent", Value::Object(None))?;
        ni_obj.set_value("virtual", Value::Int(0))?;
    }

    Ok(ni_value)
}

#[intrinsic_method(
    "java/net/NetworkInterface.boundInetAddress0(Ljava/net/InetAddress;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn bound_inet_address_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _addr = parameters.pop_reference()?;
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method("java/net/NetworkInterface.getAll()[Ljava/net/NetworkInterface;", Any)]
#[async_method]
pub async fn get_all<T: ristretto_types::Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let gc = vm.garbage_collector();

    let ni_value = create_loopback_interface(&thread).await?;

    let ni_arr_class = thread.class("[Ljava/net/NetworkInterface;").await?;
    let result_ref = Reference::try_from((ni_arr_class, vec![ni_value]))?;
    Ok(Some(Value::new_object(gc, result_ref)))
}

#[intrinsic_method(
    "java/net/NetworkInterface.getByIndex0(I)Ljava/net/NetworkInterface;",
    Any
)]
#[async_method]
pub async fn get_by_index_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "java/net/NetworkInterface.getByInetAddress0(Ljava/net/InetAddress;)Ljava/net/NetworkInterface;",
    Any
)]
#[async_method]
pub async fn get_by_inet_address_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _addr = parameters.pop_reference()?;
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "java/net/NetworkInterface.getByName0(Ljava/lang/String;)Ljava/net/NetworkInterface;",
    Any
)]
#[async_method]
pub async fn get_by_name_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _name = parameters.pop()?;
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/net/NetworkInterface.getMTU0(Ljava/lang/String;I)I", Any)]
#[async_method]
pub async fn get_mtu_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    let _name = parameters.pop()?;
    Ok(Some(Value::Int(65536)))
}

#[intrinsic_method("java/net/NetworkInterface.getMacAddr0([BLjava/lang/String;I)[B", Any)]
#[async_method]
pub async fn get_mac_addr_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    let _name = parameters.pop()?;
    let _in_addr = parameters.pop_reference()?;
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method("java/net/NetworkInterface.init()V", Any)]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/net/NetworkInterface.isLoopback0(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub async fn is_loopback_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let name = parameters.pop()?;
    let is_loopback = if let Ok(name_str) = name.as_string() {
        is_loopback_name(&name_str) || index == 1
    } else {
        index == 1
    };
    Ok(Some(Value::Int(i32::from(is_loopback))))
}

#[intrinsic_method("java/net/NetworkInterface.isP2P0(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub async fn is_p2p_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    let _name = parameters.pop()?;
    Ok(Some(Value::Int(0)))
}

#[intrinsic_method("java/net/NetworkInterface.isUp0(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub async fn is_up_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    let _name = parameters.pop()?;
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "java/net/NetworkInterface.supportsMulticast0(Ljava/lang/String;I)Z",
    Any
)]
#[async_method]
pub async fn supports_multicast_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _index = parameters.pop_int()?;
    let _name = parameters.pop()?;
    Ok(Some(Value::Int(1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bound_inet_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bound_inet_address_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_all() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_all(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_by_index_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_by_index_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_by_inet_address_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_by_inet_address_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_by_name_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_by_name_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_mtu_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mtu_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_mac_addr_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_mac_addr_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_loopback_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_loopback_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_is_p2p_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_p2p_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_is_up_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_up_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_supports_multicast_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = supports_multicast_0(thread, Parameters::default()).await;
    }
}
