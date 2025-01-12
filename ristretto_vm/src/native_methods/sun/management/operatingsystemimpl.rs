use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.OperatingSystemImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/OperatingSystemImpl";
    registry.register(
        class_name,
        "getCommittedVirtualMemorySize0",
        "()J",
        get_committed_virtual_memory_size_0,
    );
    registry.register(
        class_name,
        "getFreePhysicalMemorySize0",
        "()J",
        get_free_physical_memory_size_0,
    );
    registry.register(
        class_name,
        "getFreeSwapSpaceSize0",
        "()J",
        get_free_swap_space_size_0,
    );
    registry.register(
        class_name,
        "getHostConfiguredCpuCount0",
        "()I",
        get_host_configured_cpu_count_0,
    );
    registry.register(
        class_name,
        "getHostOnlineCpuCount0",
        "()I",
        get_host_online_cpu_count_0,
    );
    registry.register(
        class_name,
        "getHostTotalCpuTicks0",
        "()J",
        get_host_total_cpu_ticks_0,
    );
    registry.register(
        class_name,
        "getMaxFileDescriptorCount0",
        "()J",
        get_max_file_descriptor_count_0,
    );
    registry.register(
        class_name,
        "getOpenFileDescriptorCount0",
        "()J",
        get_open_file_descriptor_count_0,
    );
    registry.register(
        class_name,
        "getProcessCpuLoad0",
        "()D",
        get_process_cpu_load_0,
    );
    registry.register(
        class_name,
        "getProcessCpuTime0",
        "()J",
        get_process_cpu_time_0,
    );
    registry.register(
        class_name,
        "getSingleCpuLoad0",
        "(I)D",
        get_single_cpu_load_0,
    );
    registry.register(
        class_name,
        "getSystemCpuLoad0",
        "()D",
        get_system_cpu_load_0,
    );
    registry.register(
        class_name,
        "getTotalPhysicalMemorySize0",
        "()J",
        get_total_physical_memory_size_0,
    );
    registry.register(
        class_name,
        "getTotalSwapSpaceSize0",
        "()J",
        get_total_swap_space_size_0,
    );
    registry.register(class_name, "initialize0", "()V", initialize_0);
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/management/OperatingSystemImpl";
        assert!(registry
            .method(class_name, "getCommittedVirtualMemorySize0", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getFreePhysicalMemorySize0", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getFreeSwapSpaceSize0", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getHostConfiguredCpuCount0", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "getHostOnlineCpuCount0", "()I")
            .is_some());
        assert!(registry
            .method(class_name, "getHostTotalCpuTicks0", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getMaxFileDescriptorCount0", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getOpenFileDescriptorCount0", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getProcessCpuLoad0", "()D")
            .is_some());
        assert!(registry
            .method(class_name, "getProcessCpuTime0", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getSingleCpuLoad0", "(I)D")
            .is_some());
        assert!(registry
            .method(class_name, "getSystemCpuLoad0", "()D")
            .is_some());
        assert!(registry
            .method(class_name, "getTotalPhysicalMemorySize0", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getTotalSwapSpaceSize0", "()J")
            .is_some());
        assert!(registry.method(class_name, "initialize0", "()V").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.OperatingSystemImpl.getCommittedVirtualMemorySize0()J"
    )]
    async fn test_get_committed_virtual_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_committed_virtual_memory_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getFreePhysicalMemorySize0()J")]
    async fn test_get_free_physical_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_free_physical_memory_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getFreeSwapSpaceSize0()J")]
    async fn test_get_free_swap_space_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_free_swap_space_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getHostConfiguredCpuCount0()I")]
    async fn test_get_host_configured_cpu_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_configured_cpu_count_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getHostOnlineCpuCount0()I")]
    async fn test_get_host_online_cpu_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_online_cpu_count_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getHostTotalCpuTicks0()J")]
    async fn test_get_host_total_cpu_ticks_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_host_total_cpu_ticks_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getMaxFileDescriptorCount0()J")]
    async fn test_get_max_file_descriptor_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_max_file_descriptor_count_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getOpenFileDescriptorCount0()J")]
    async fn test_get_open_file_descriptor_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_open_file_descriptor_count_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getProcessCpuLoad0()D")]
    async fn test_get_process_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_process_cpu_load_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getProcessCpuTime0()J")]
    async fn test_get_process_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_process_cpu_time_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getSingleCpuLoad0(I)D")]
    async fn test_get_single_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_single_cpu_load_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getSystemCpuLoad0()D")]
    async fn test_get_system_cpu_load_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_cpu_load_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getTotalPhysicalMemorySize0()J")]
    async fn test_get_total_physical_memory_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_physical_memory_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.getTotalSwapSpaceSize0()J")]
    async fn test_get_total_swap_space_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_swap_space_size_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.OperatingSystemImpl.initialize0()V")]
    async fn test_initialize_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = initialize_0(thread, Arguments::default()).await;
    }
}
