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
