use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, Thread, VM};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/InetAddressImplFactory.isIPv6Supported()Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn is_ipv_6_supported<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let prefer_ipv4_stack = thread
        .vm()?
        .system_properties()
        .get("java.net.preferIPv4Stack")
        .is_some_and(|value| value.eq_ignore_ascii_case("true"));
    if prefer_ipv4_stack {
        return Ok(Some(Value::Int(0)));
    }
    super::inetaddress::is_ipv_6_supported(thread, Parameters::default()).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_ipv_6_supported() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = is_ipv_6_supported(thread, Parameters::default()).await?;
        #[cfg(not(target_family = "wasm"))]
        let expected = socket2::Socket::new(
            socket2::Domain::IPV6,
            socket2::Type::DGRAM,
            Some(socket2::Protocol::UDP),
        )
        .is_ok();
        #[cfg(target_family = "wasm")]
        let expected = false;
        assert_eq!(result, Some(Value::from(expected)));
        Ok(())
    }
}
