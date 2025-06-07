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
    "java/io/ObjectInputStream.bytesToDoubles([BI[DII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn bytes_to_doubles(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.ObjectInputStream.bytesToDoubles([BI[DII)V")
}

#[intrinsic_method(
    "java/io/ObjectInputStream.bytesToFloats([BI[FII)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn bytes_to_floats(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.io.ObjectInputStream.bytesToFloats([BI[FII)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectInputStream.bytesToDoubles([BI[DII)V"
    )]
    async fn test_bytes_to_doubles() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bytes_to_doubles(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.io.ObjectInputStream.bytesToFloats([BI[FII)V"
    )]
    async fn test_bytes_to_floats() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = bytes_to_floats(thread, Parameters::default()).await;
    }
}
