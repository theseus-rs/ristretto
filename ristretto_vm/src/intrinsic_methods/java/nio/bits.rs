use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/nio/Bits.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn copy_swap_memory_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.nio.Bits.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.nio.Bits.copySwapMemory0(Ljava/lang/Object;JLjava/lang/Object;JJJ)V"
    )]
    async fn test_copy_swap_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = copy_swap_memory_0(thread, Parameters::default()).await;
    }
}
