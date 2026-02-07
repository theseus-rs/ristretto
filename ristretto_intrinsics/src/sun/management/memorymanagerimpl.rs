use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/MemoryManagerImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;",
    Any
)]
#[async_method]
pub async fn get_memory_pools_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.MemoryManagerImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.MemoryManagerImpl.getMemoryPools0()[Ljava/lang/management/MemoryPoolMXBean;"
    )]
    async fn test_get_memory_pools_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_memory_pools_0(thread, Parameters::default()).await;
    }
}
