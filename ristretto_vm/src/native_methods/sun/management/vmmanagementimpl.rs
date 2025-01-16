use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/VMManagementImpl";

/// Register all native methods for `sun.management.VMManagementImpl`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getAvailableProcessors",
        "()I",
        get_available_processors,
    );
    registry.register(
        CLASS_NAME,
        "getClassInitializationTime",
        "()J",
        get_class_initialization_time,
    );
    registry.register(
        CLASS_NAME,
        "getClassLoadingTime",
        "()J",
        get_class_loading_time,
    );
    registry.register(
        CLASS_NAME,
        "getClassVerificationTime",
        "()J",
        get_class_verification_time,
    );
    registry.register(
        CLASS_NAME,
        "getDaemonThreadCount",
        "()I",
        get_daemon_thread_count,
    );
    registry.register(
        CLASS_NAME,
        "getInitializedClassCount",
        "()J",
        get_initialized_class_count,
    );
    registry.register(
        CLASS_NAME,
        "getLiveThreadCount",
        "()I",
        get_live_thread_count,
    );
    registry.register(
        CLASS_NAME,
        "getLoadedClassSize",
        "()J",
        get_loaded_class_size,
    );
    registry.register(CLASS_NAME, "getMethodDataSize", "()J", get_method_data_size);
    registry.register(
        CLASS_NAME,
        "getPeakThreadCount",
        "()I",
        get_peak_thread_count,
    );
    registry.register(CLASS_NAME, "getProcessId", "()I", get_process_id);
    registry.register(CLASS_NAME, "getSafepointCount", "()J", get_safepoint_count);
    registry.register(
        CLASS_NAME,
        "getSafepointSyncTime",
        "()J",
        get_safepoint_sync_time,
    );
    registry.register(CLASS_NAME, "getStartupTime", "()J", get_startup_time);
    registry.register(
        CLASS_NAME,
        "getTotalApplicationNonStoppedTime",
        "()J",
        get_total_application_non_stopped_time,
    );
    registry.register(
        CLASS_NAME,
        "getTotalClassCount",
        "()J",
        get_total_class_count,
    );
    registry.register(
        CLASS_NAME,
        "getTotalCompileTime",
        "()J",
        get_total_compile_time,
    );
    registry.register(
        CLASS_NAME,
        "getTotalSafepointTime",
        "()J",
        get_total_safepoint_time,
    );
    registry.register(
        CLASS_NAME,
        "getTotalThreadCount",
        "()J",
        get_total_thread_count,
    );
    registry.register(
        CLASS_NAME,
        "getUnloadedClassCount",
        "()J",
        get_unloaded_class_count,
    );
    registry.register(
        CLASS_NAME,
        "getUnloadedClassSize",
        "()J",
        get_unloaded_class_size,
    );
    registry.register(CLASS_NAME, "getUptime0", "()J", get_uptime_0);
    registry.register(CLASS_NAME, "getVerboseClass", "()Z", get_verbose_class);
    registry.register(CLASS_NAME, "getVerboseGC", "()Z", get_verbose_gc);
    registry.register(
        CLASS_NAME,
        "getVersion0",
        "()Ljava/lang/String;",
        get_version_0,
    );
    registry.register(
        CLASS_NAME,
        "getVmArguments0",
        "()[Ljava/lang/String;",
        get_vm_arguments_0,
    );
    registry.register(
        CLASS_NAME,
        "initOptionalSupportFields",
        "()V",
        init_optional_support_fields,
    );
    registry.register(
        CLASS_NAME,
        "isThreadAllocatedMemoryEnabled",
        "()Z",
        is_thread_allocated_memory_enabled,
    );
    registry.register(
        CLASS_NAME,
        "isThreadContentionMonitoringEnabled",
        "()Z",
        is_thread_contention_monitoring_enabled,
    );
    registry.register(
        CLASS_NAME,
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
    todo!("sun.management.VMManagementImpl.getAvailableProcessors()I")
}

#[async_recursion(?Send)]
async fn get_class_initialization_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getClassInitializationTime()J")
}

#[async_recursion(?Send)]
async fn get_class_loading_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getClassLoadingTime()J")
}

#[async_recursion(?Send)]
async fn get_class_verification_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getClassVerificationTime()J")
}

#[async_recursion(?Send)]
async fn get_daemon_thread_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getDaemonThreadCount()I")
}

#[async_recursion(?Send)]
async fn get_initialized_class_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getInitializedClassCount()J")
}

#[async_recursion(?Send)]
async fn get_live_thread_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getLiveThreadCount()I")
}

#[async_recursion(?Send)]
async fn get_loaded_class_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getLoadedClassSize()J")
}

#[async_recursion(?Send)]
async fn get_method_data_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getMethodDataSize()J")
}

#[async_recursion(?Send)]
async fn get_peak_thread_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getPeakThreadCount()I")
}

#[async_recursion(?Send)]
async fn get_process_id(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getProcessId()I")
}

#[async_recursion(?Send)]
async fn get_safepoint_count(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getSafepointCount()J")
}

#[async_recursion(?Send)]
async fn get_safepoint_sync_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getSafepointSyncTime()J")
}

#[async_recursion(?Send)]
async fn get_startup_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getStartupTime()J")
}

#[async_recursion(?Send)]
async fn get_total_application_non_stopped_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalApplicationNonStoppedTime()J")
}

#[async_recursion(?Send)]
async fn get_total_class_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalClassCount()J")
}

#[async_recursion(?Send)]
async fn get_total_compile_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalCompileTime()J")
}

#[async_recursion(?Send)]
async fn get_total_safepoint_time(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalSafepointTime()J")
}

#[async_recursion(?Send)]
async fn get_total_thread_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalThreadCount()J")
}

#[async_recursion(?Send)]
async fn get_unloaded_class_count(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getUnloadedClassCount()J")
}

#[async_recursion(?Send)]
async fn get_unloaded_class_size(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getUnloadedClassSize()J")
}

#[async_recursion(?Send)]
async fn get_uptime_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getUptime0()J")
}

#[async_recursion(?Send)]
async fn get_verbose_class(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getVerboseClass()Z")
}

#[async_recursion(?Send)]
async fn get_verbose_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getVerboseGC()Z")
}

#[async_recursion(?Send)]
async fn get_version_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getVersion0()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_vm_arguments_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getVmArguments0()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn init_optional_support_fields(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.initOptionalSupportFields()V")
}

#[async_recursion(?Send)]
async fn is_thread_allocated_memory_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.isThreadAllocatedMemoryEnabled()Z")
}

#[async_recursion(?Send)]
async fn is_thread_contention_monitoring_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.isThreadContentionMonitoringEnabled()Z")
}

#[async_recursion(?Send)]
async fn is_thread_cpu_time_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.isThreadCpuTimeEnabled()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getAvailableProcessors()I"
    )]
    async fn test_get_available_processors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_available_processors(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getClassInitializationTime()J"
    )]
    async fn test_get_class_initialization_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_initialization_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getClassLoadingTime()J"
    )]
    async fn test_get_class_loading_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_loading_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getClassVerificationTime()J"
    )]
    async fn test_get_class_verification_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_verification_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getDaemonThreadCount()I"
    )]
    async fn test_get_daemon_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_daemon_thread_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getInitializedClassCount()J"
    )]
    async fn test_get_initialized_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_initialized_class_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getLiveThreadCount()I"
    )]
    async fn test_get_live_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_live_thread_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getLoadedClassSize()J"
    )]
    async fn test_get_loaded_class_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_loaded_class_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getMethodDataSize()J"
    )]
    async fn test_get_method_data_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_method_data_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getPeakThreadCount()I"
    )]
    async fn test_get_peak_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_peak_thread_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getProcessId()I"
    )]
    async fn test_get_process_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_process_id(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getSafepointCount()J"
    )]
    async fn test_get_safepoint_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_safepoint_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getSafepointSyncTime()J"
    )]
    async fn test_get_safepoint_sync_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_safepoint_sync_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getStartupTime()J"
    )]
    async fn test_get_startup_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_startup_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalApplicationNonStoppedTime()J"
    )]
    async fn test_get_total_application_non_stopped_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_application_non_stopped_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalClassCount()J"
    )]
    async fn test_get_total_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_class_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalCompileTime()J"
    )]
    async fn test_get_total_compile_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_compile_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalSafepointTime()J"
    )]
    async fn test_get_total_safepoint_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_safepoint_time(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalThreadCount()J"
    )]
    async fn test_get_total_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_thread_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getUnloadedClassCount()J"
    )]
    async fn test_get_unloaded_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unloaded_class_count(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getUnloadedClassSize()J"
    )]
    async fn test_get_unloaded_class_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unloaded_class_size(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.management.VMManagementImpl.getUptime0()J")]
    async fn test_get_uptime_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_uptime_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getVerboseClass()Z"
    )]
    async fn test_get_verbose_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_verbose_class(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getVerboseGC()Z"
    )]
    async fn test_get_verbose_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_verbose_gc(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getVersion0()Ljava/lang/String;"
    )]
    async fn test_get_version_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_version_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getVmArguments0()[Ljava/lang/String;"
    )]
    async fn test_get_vm_arguments_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_vm_arguments_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.initOptionalSupportFields()V"
    )]
    async fn test_init_optional_support_fields() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_optional_support_fields(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.isThreadAllocatedMemoryEnabled()Z"
    )]
    async fn test_is_thread_allocated_memory_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_thread_allocated_memory_enabled(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.isThreadContentionMonitoringEnabled()Z"
    )]
    async fn test_is_thread_contention_monitoring_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_thread_contention_monitoring_enabled(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.isThreadCpuTimeEnabled()Z"
    )]
    async fn test_is_thread_cpu_time_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_thread_cpu_time_enabled(thread, Arguments::default()).await;
    }
}
