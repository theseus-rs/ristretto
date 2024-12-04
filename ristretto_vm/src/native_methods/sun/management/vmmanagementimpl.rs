use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.VMManagementImpl`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/VMManagementImpl";
    registry.register(
        class_name,
        "getAvailableProcessors",
        "()I",
        get_available_processors,
    );
    registry.register(
        class_name,
        "getClassInitializationTime",
        "()J",
        get_class_initialization_time,
    );
    registry.register(
        class_name,
        "getClassLoadingTime",
        "()J",
        get_class_loading_time,
    );
    registry.register(
        class_name,
        "getClassVerificationTime",
        "()J",
        get_class_verification_time,
    );
    registry.register(
        class_name,
        "getDaemonThreadCount",
        "()I",
        get_daemon_thread_count,
    );
    registry.register(
        class_name,
        "getInitializedClassCount",
        "()J",
        get_initialized_class_count,
    );
    registry.register(
        class_name,
        "getLiveThreadCount",
        "()I",
        get_live_thread_count,
    );
    registry.register(
        class_name,
        "getLoadedClassSize",
        "()J",
        get_loaded_class_size,
    );
    registry.register(class_name, "getMethodDataSize", "()J", get_method_data_size);
    registry.register(
        class_name,
        "getPeakThreadCount",
        "()I",
        get_peak_thread_count,
    );
    registry.register(class_name, "getProcessId", "()I", get_process_id);
    registry.register(class_name, "getSafepointCount", "()J", get_safepoint_count);
    registry.register(
        class_name,
        "getSafepointSyncTime",
        "()J",
        get_safepoint_sync_time,
    );
    registry.register(class_name, "getStartupTime", "()J", get_startup_time);
    registry.register(
        class_name,
        "getTotalApplicationNonStoppedTime",
        "()J",
        get_total_application_non_stopped_time,
    );
    registry.register(
        class_name,
        "getTotalClassCount",
        "()J",
        get_total_class_count,
    );
    registry.register(
        class_name,
        "getTotalCompileTime",
        "()J",
        get_total_compile_time,
    );
    registry.register(
        class_name,
        "getTotalSafepointTime",
        "()J",
        get_total_safepoint_time,
    );
    registry.register(
        class_name,
        "getTotalThreadCount",
        "()J",
        get_total_thread_count,
    );
    registry.register(
        class_name,
        "getUnloadedClassCount",
        "()J",
        get_unloaded_class_count,
    );
    registry.register(
        class_name,
        "getUnloadedClassSize",
        "()J",
        get_unloaded_class_size,
    );
    registry.register(class_name, "getUptime0", "()J", get_uptime_0);
    registry.register(class_name, "getVerboseClass", "()Z", get_verbose_class);
    registry.register(class_name, "getVerboseGC", "()Z", get_verbose_gc);
    registry.register(
        class_name,
        "getVersion0",
        "()Ljava/lang/String;",
        get_version_0,
    );
    registry.register(
        class_name,
        "getVmArguments0",
        "()[Ljava/lang/String;",
        get_vm_arguments_0,
    );
    registry.register(
        class_name,
        "initOptionalSupportFields",
        "()V",
        init_optional_support_fields,
    );
    registry.register(
        class_name,
        "isThreadAllocatedMemoryEnabled",
        "()Z",
        is_thread_allocated_memory_enabled,
    );
    registry.register(
        class_name,
        "isThreadContentionMonitoringEnabled",
        "()Z",
        is_thread_contention_monitoring_enabled,
    );
    registry.register(
        class_name,
        "isThreadCpuTimeEnabled",
        "()Z",
        is_thread_cpu_time_enabled,
    );
}

#[async_recursion(?Send)]
async fn get_available_processors(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_class_initialization_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_class_loading_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_class_verification_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_daemon_thread_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_initialized_class_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_live_thread_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_loaded_class_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_method_data_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_peak_thread_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_process_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_safepoint_count(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_safepoint_sync_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_startup_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_total_application_non_stopped_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_total_class_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_total_compile_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_total_safepoint_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_total_thread_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_unloaded_class_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_unloaded_class_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_uptime_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_verbose_class(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_verbose_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_version_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_vm_arguments_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_optional_support_fields(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_thread_allocated_memory_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_thread_contention_monitoring_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_thread_cpu_time_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
