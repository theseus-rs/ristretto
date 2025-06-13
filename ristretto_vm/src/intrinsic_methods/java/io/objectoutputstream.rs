use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/io/ObjectOutputStream.doublesToBytes([DI[BII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn doubles_to_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.ObjectOutputStream.doublesToBytes([DI[BII)V")
}

#[intrinsic_method(
    "java/io/ObjectOutputStream.floatsToBytes([FI[BII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn floats_to_bytes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.ObjectOutputStream.floatsToBytes([FI[BII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectOutputStream.doublesToBytes([DI[BII)V"
    )]
    async fn test_doubles_to_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = doubles_to_bytes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectOutputStream.floatsToBytes([FI[BII)V"
    )]
    async fn test_floats_to_bytes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = floats_to_bytes(thread, Parameters::default()).await;
    }
}
