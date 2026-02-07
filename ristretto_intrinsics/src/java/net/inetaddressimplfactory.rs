use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/InetAddressImplFactory.isIPv6Supported()Z",
    LessThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn is_ipv_6_supported<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.InetAddressImplFactory.isIPv6Supported()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.InetAddressImplFactory.isIPv6Supported()Z"
    )]
    async fn test_is_ipv_6_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_ipv_6_supported(thread, Parameters::default()).await;
    }
}
