use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/ec/ECKeyPairGenerator.generateECKeyPair(I[B[B)[Ljava/lang/Object;",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn generate_ec_key_pair(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECKeyPairGenerator.generateECKeyPair(I[B[B)[Ljava/lang/Object;")
}

#[intrinsic_method(
    "sun/security/ec/ECKeyPairGenerator.isCurveSupported([B)Z",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_curve_supported(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECKeyPairGenerator.isCurveSupported([B)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.ec.ECKeyPairGenerator.generateECKeyPair(I[B[B)[Ljava/lang/Object;"
    )]
    async fn test_generate_ec_key_pair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = generate_ec_key_pair(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.ec.ECKeyPairGenerator.isCurveSupported([B)Z"
    )]
    async fn test_is_curve_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_curve_supported(thread, Parameters::default()).await;
    }
}
