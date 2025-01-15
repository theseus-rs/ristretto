use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/OperatingSystemImpl";

/// Register all native methods for `sun.management.OperatingSystemImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getCommittedVirtualMemorySize0",
        "()J",
        get_committed_virtual_memory_size_0,
    );
    registry.register(
        CLASS_NAME,
        "getFreePhysicalMemorySize0",
        "()J",
        get_free_physical_memory_size_0,
    );
    registry.register(
        CLASS_NAME,
        "getFreeSwapSpaceSize0",
        "()J",
        get_free_swap_space_size_0,
    );
    registry.register(
        CLASS_NAME,
        "getHostConfiguredCpuCount0",
        "()I",
        get_host_configured_cpu_count_0,
    );
    registry.register(
        CLASS_NAME,
        "getHostOnlineCpuCount0",
        "()I",
        get_host_online_cpu_count_0,
    );
    registry.register(
        CLASS_NAME,
        "getHostTotalCpuTicks0",
        "()J",
        get_host_total_cpu_ticks_0,
    );
    registry.register(
        CLASS_NAME,
        "getMaxFileDescriptorCount0",
        "()J",
        get_max_file_descriptor_count_0,
    );
    registry.register(
        CLASS_NAME,
        "getOpenFileDescriptorCount0",
        "()J",
        get_open_file_descriptor_count_0,
    );
    registry.register(
        CLASS_NAME,
        "getProcessCpuLoad0",
        "()D",
        get_process_cpu_load_0,
    );
    registry.register(
        CLASS_NAME,
        "getProcessCpuTime0",
        "()J",
        get_process_cpu_time_0,
    );
    registry.register(
        CLASS_NAME,
        "getSingleCpuLoad0",
        "(I)D",
        get_single_cpu_load_0,
    );
    registry.register(
        CLASS_NAME,
        "getSystemCpuLoad0",
        "()D",
        get_system_cpu_load_0,
    );
    registry.register(
        CLASS_NAME,
        "getTotalPhysicalMemorySize0",
        "()J",
        get_total_physical_memory_size_0,
    );
    registry.register(
        CLASS_NAME,
        "getTotalSwapSpaceSize0",
        "()J",
        get_total_swap_space_size_0,
    );
    registry.register(CLASS_NAME, "initialize0", "()V", initialize_0);
}

#[async_recursion(?Send)]
async fn get_committed_virtual_memory_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getCommittedVirtualMemorySize0()J")
}

#[async_recursion(?Send)]
async fn get_free_physical_memory_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getFreePhysicalMemorySize0()J")
}

#[async_recursion(?Send)]
async fn get_free_swap_space_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getFreeSwapSpaceSize0()J")
}

#[async_recursion(?Send)]
async fn get_host_configured_cpu_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getHostConfiguredCpuCount0()I")
}

#[async_recursion(?Send)]
async fn get_host_online_cpu_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getHostOnlineCpuCount0()I")
}

#[async_recursion(?Send)]
async fn get_host_total_cpu_ticks_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getHostTotalCpuTicks0()J")
}

#[async_recursion(?Send)]
async fn get_max_file_descriptor_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getMaxFileDescriptorCount0()J")
}

#[async_recursion(?Send)]
async fn get_open_file_descriptor_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getOpenFileDescriptorCount0()J")
}

#[async_recursion(?Send)]
async fn get_process_cpu_load_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getProcessCpuLoad0()D")
}

#[async_recursion(?Send)]
async fn get_process_cpu_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getProcessCpuTime0()J")
}

#[async_recursion(?Send)]
async fn get_single_cpu_load_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getSingleCpuLoad0(I)D")
}

#[async_recursion(?Send)]
async fn get_system_cpu_load_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getSystemCpuLoad0()D")
}

#[async_recursion(?Send)]
async fn get_total_physical_memory_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getTotalPhysicalMemorySize0()J")
}

#[async_recursion(?Send)]
async fn get_total_swap_space_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.getTotalSwapSpaceSize0()J")
}

#[async_recursion(?Send)]
async fn initialize_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.OperatingSystemImpl.initialize0()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getCommittedVirtualMemorySize0()J"
    )]
    async fn test_get_committed_virtual_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_committed_virtual_memory_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getFreePhysicalMemorySize0()J"
    )]
    async fn test_get_free_physical_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_free_physical_memory_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getFreeSwapSpaceSize0()J"
    )]
    async fn test_get_free_swap_space_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_free_swap_space_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getHostConfiguredCpuCount0()I"
    )]
    async fn test_get_host_configured_cpu_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_configured_cpu_count_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getHostOnlineCpuCount0()I"
    )]
    async fn test_get_host_online_cpu_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_online_cpu_count_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getHostTotalCpuTicks0()J"
    )]
    async fn test_get_host_total_cpu_ticks_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_total_cpu_ticks_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getMaxFileDescriptorCount0()J"
    )]
    async fn test_get_max_file_descriptor_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_max_file_descriptor_count_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getOpenFileDescriptorCount0()J"
    )]
    async fn test_get_open_file_descriptor_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_open_file_descriptor_count_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getProcessCpuLoad0()D"
    )]
    async fn test_get_process_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_process_cpu_load_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getProcessCpuTime0()J"
    )]
    async fn test_get_process_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_process_cpu_time_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getSingleCpuLoad0(I)D"
    )]
    async fn test_get_single_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_single_cpu_load_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getSystemCpuLoad0()D"
    )]
    async fn test_get_system_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_cpu_load_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getTotalPhysicalMemorySize0()J"
    )]
    async fn test_get_total_physical_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_physical_memory_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.getTotalSwapSpaceSize0()J"
    )]
    async fn test_get_total_swap_space_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_swap_space_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.OperatingSystemImpl.initialize0()V"
    )]
    async fn test_initialize_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = initialize_0(thread, Arguments::default()).await;
    }
}
