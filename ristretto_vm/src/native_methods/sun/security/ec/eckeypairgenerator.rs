use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.security.ec.ECKeyPairGenerator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/security/ec/ECKeyPairGenerator";
    registry.register(
        class_name,
        "generateECKeyPair",
        "(I[B[B)[Ljava/lang/Object;",
        generate_ec_key_pair,
    );
    registry.register(class_name, "isCurveSupported", "([B)Z", is_curve_supported);
}

#[async_recursion(?Send)]
async fn generate_ec_key_pair(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECKeyPairGenerator.generateECKeyPair(I[B[B)[Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn is_curve_supported(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.security.ec.ECKeyPairGenerator.isCurveSupported([B)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/security/ec/ECKeyPairGenerator";
        assert!(registry
            .method(
                class_name,
                "generateECKeyPair",
                "(I[B[B)[Ljava/lang/Object;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "isCurveSupported", "([B)Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.security.ec.ECKeyPairGenerator.generateECKeyPair(I[B[B)[Ljava/lang/Object;"
    )]
    async fn test_generate_ec_key_pair() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = generate_ec_key_pair(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.security.ec.ECKeyPairGenerator.isCurveSupported([B)Z")]
    async fn test_is_curve_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_curve_supported(thread, Arguments::default()).await;
    }
}
