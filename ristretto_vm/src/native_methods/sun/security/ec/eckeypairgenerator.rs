use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/security/ec/ECKeyPairGenerator";

/// Register all native methods for `sun.security.ec.ECKeyPairGenerator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "generateECKeyPair",
        "(I[B[B)[Ljava/lang/Object;",
        generate_ec_key_pair,
    );
    registry.register(CLASS_NAME, "isCurveSupported", "([B)Z", is_curve_supported);
}

#[async_recursion(?Send)]
async fn generate_ec_key_pair(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECKeyPairGenerator.generateECKeyPair(I[B[B)[Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn is_curve_supported(
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
