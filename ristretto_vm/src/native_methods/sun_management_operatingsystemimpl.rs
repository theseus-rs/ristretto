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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_committed_virtual_memory_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_free_physical_memory_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_free_swap_space_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_host_configured_cpu_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_host_online_cpu_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_host_total_cpu_ticks_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_max_file_descriptor_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_open_file_descriptor_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_process_cpu_load_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_process_cpu_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_single_cpu_load_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_system_cpu_load_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_total_physical_memory_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_total_swap_space_size_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn initialize_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
