use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_11, JAVA_17, JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/nio/ch/FileChannelImpl";

/// Register all intrinsic methods for `sun.nio.ch.FileChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(CLASS_NAME, "map0", "(IJJ)J", map_0);
    }

    if registry.java_major_version() == JAVA_17 {
        registry.register(CLASS_NAME, "map0", "(IJJZ)J", map_0);
        registry.register(
            CLASS_NAME,
            "maxDirectTransferSize0",
            "()I",
            max_direct_transfer_size_0,
        );
    }

    if registry.java_major_version() <= JAVA_17 {
        registry.register(CLASS_NAME, "initIDs", "()J", init_ids);
    }

    if registry.java_major_version() >= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "allocationGranularity0",
            "()J",
            allocation_granularity_0,
        );
        registry.register(CLASS_NAME, "map0", "(Ljava/io/FileDescriptor;IJJZ)J", map_0);
        registry.register(
            CLASS_NAME,
            "transferFrom0",
            "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;JJ)J",
            transfer_from_0,
        );
    }

    registry.register(
        CLASS_NAME,
        "transferTo0",
        "(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J",
        transfer_to_0,
    );
    registry.register(CLASS_NAME, "unmap0", "(JJ)I", unmap_0);
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn allocation_granularity_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.allocationGranularity0()J");
}

#[async_recursion(?Send)]
async fn map_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.map0(IJJ)J");
}

#[async_recursion(?Send)]
async fn max_direct_transfer_size_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.maxDirectTransferSize0()I");
}

#[async_recursion(?Send)]
async fn transfer_from_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.nio.ch.FileChannelImpl.transferFrom0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;JJ)J"
    );
}

#[async_recursion(?Send)]
async fn transfer_to_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.nio.ch.FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J"
    );
}

#[async_recursion(?Send)]
async fn unmap_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.unmap0(JJ)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileChannelImpl.allocationGranularity0()J"
    )]
    async fn test_allocation_granularity_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocation_granularity_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.FileChannelImpl.map0(IJJ)J")]
    async fn test_map_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = map_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileChannelImpl.maxDirectTransferSize0()I"
    )]
    async fn test_max_direct_transfer_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = max_direct_transfer_size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileChannelImpl.transferFrom0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;JJ)J"
    )]
    async fn test_transfer_from_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transfer_from_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.nio.ch.FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J"
    )]
    async fn test_transfer_to_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transfer_to_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.nio.ch.FileChannelImpl.unmap0(JJ)I")]
    async fn test_unmap_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unmap_0(thread, Parameters::default()).await;
    }
}
