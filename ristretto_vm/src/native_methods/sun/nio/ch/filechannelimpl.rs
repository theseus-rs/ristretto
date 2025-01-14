use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_19: Version = Version::Java19 { minor: 0 };

/// Register all native methods for `sun.nio.ch.FileChannelImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/FileChannelImpl";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(class_name, "map0", "(IJJ)J", map_0);
    } else {
        registry.register(
            class_name,
            "maxDirectTransferSize0",
            "()I",
            max_direct_transfer_size_0,
        );
    }

    if java_version == JAVA_17 {
        registry.register(class_name, "map0", "(IJJZ)J", map_0);
    }

    if java_version >= JAVA_19 {
        registry.register(
            class_name,
            "allocationGranularity0",
            "()J",
            allocation_granularity_0,
        );
        registry.register(class_name, "map0", "(Ljava/io/FileDescriptor;IJJZ)J", map_0);
        registry.register(
            class_name,
            "transferFrom0",
            "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;JJ)J",
            transfer_from_0,
        );
    }

    registry.register(
        class_name,
        "transferTo0",
        "(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J",
        transfer_to_0,
    );
    registry.register(class_name, "unmap0", "(JJ)I", unmap_0);
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn allocation_granularity_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.allocationGranularity0()J");
}

#[async_recursion(?Send)]
async fn map_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.map0(IJJ)J");
}

#[async_recursion(?Send)]
async fn max_direct_transfer_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.maxDirectTransferSize0()I");
}

#[async_recursion(?Send)]
async fn transfer_from_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.transferFrom0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;JJ)J");
}

#[async_recursion(?Send)]
async fn transfer_to_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J");
}

#[async_recursion(?Send)]
async fn unmap_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.FileChannelImpl.unmap0(JJ)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java19 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/nio/ch/FileChannelImpl";
        assert!(registry.method(class_name, "map0", "(IJJ)J").is_some());
        assert!(registry
            .method(class_name, "map0", "(Ljava/io/FileDescriptor;IJJZ)J")
            .is_some());
        assert!(registry
            .method(class_name, "allocationGranularity0", "()J")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "transferFrom0",
                "(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;JJ)J"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "transferTo0",
                "(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J"
            )
            .is_some());
        assert!(registry.method(class_name, "unmap0", "(JJ)I").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.FileChannelImpl.allocationGranularity0()J")]
    async fn test_allocation_granularity_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = allocation_granularity_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.FileChannelImpl.map0(IJJ)J")]
    async fn test_map_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = map_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.FileChannelImpl.maxDirectTransferSize0()I")]
    async fn test_max_direct_transfer_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = max_direct_transfer_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.FileChannelImpl.transferFrom0(Ljava/io/FileDescriptor;Ljava/io/FileDescriptor;JJ)J"
    )]
    async fn test_transfer_from_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transfer_from_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.nio.ch.FileChannelImpl.transferTo0(Ljava/io/FileDescriptor;JJLjava/io/FileDescriptor;)J"
    )]
    async fn test_transfer_to_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = transfer_to_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.nio.ch.FileChannelImpl.unmap0(JJ)I")]
    async fn test_unmap_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unmap_0(thread, Arguments::default()).await;
    }
}
