use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/management/VMManagementImpl.getAvailableProcessors()I", Any)]
#[async_method]
pub async fn get_available_processors<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getAvailableProcessors()I")
}

#[intrinsic_method("sun/management/VMManagementImpl.getClassInitializationTime()J", Any)]
#[async_method]
pub async fn get_class_initialization_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getClassInitializationTime()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getClassLoadingTime()J", Any)]
#[async_method]
pub async fn get_class_loading_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getClassLoadingTime()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getClassVerificationTime()J", Any)]
#[async_method]
pub async fn get_class_verification_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getClassVerificationTime()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getDaemonThreadCount()I", Any)]
#[async_method]
pub async fn get_daemon_thread_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getDaemonThreadCount()I")
}

#[intrinsic_method("sun/management/VMManagementImpl.getInitializedClassCount()J", Any)]
#[async_method]
pub async fn get_initialized_class_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getInitializedClassCount()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getLiveThreadCount()I", Any)]
#[async_method]
pub async fn get_live_thread_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getLiveThreadCount()I")
}

#[intrinsic_method("sun/management/VMManagementImpl.getLoadedClassSize()J", Any)]
#[async_method]
pub async fn get_loaded_class_size<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getLoadedClassSize()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getMethodDataSize()J", Any)]
#[async_method]
pub async fn get_method_data_size<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getMethodDataSize()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getPeakThreadCount()I", Any)]
#[async_method]
pub async fn get_peak_thread_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getPeakThreadCount()I")
}

#[intrinsic_method("sun/management/VMManagementImpl.getProcessId()I", Any)]
#[async_method]
pub async fn get_process_id<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getProcessId()I")
}

#[intrinsic_method("sun/management/VMManagementImpl.getSafepointCount()J", Any)]
#[async_method]
pub async fn get_safepoint_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getSafepointCount()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getSafepointSyncTime()J", Any)]
#[async_method]
pub async fn get_safepoint_sync_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getSafepointSyncTime()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getStartupTime()J", Any)]
#[async_method]
pub async fn get_startup_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getStartupTime()J")
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.getTotalApplicationNonStoppedTime()J",
    Any
)]
#[async_method]
pub async fn get_total_application_non_stopped_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalApplicationNonStoppedTime()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalClassCount()J", Any)]
#[async_method]
pub async fn get_total_class_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalClassCount()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalCompileTime()J", Any)]
#[async_method]
pub async fn get_total_compile_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalCompileTime()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalSafepointTime()J", Any)]
#[async_method]
pub async fn get_total_safepoint_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalSafepointTime()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalThreadCount()J", Any)]
#[async_method]
pub async fn get_total_thread_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getTotalThreadCount()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getUnloadedClassCount()J", Any)]
#[async_method]
pub async fn get_unloaded_class_count<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getUnloadedClassCount()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getUnloadedClassSize()J", Any)]
#[async_method]
pub async fn get_unloaded_class_size<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getUnloadedClassSize()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getUptime0()J", Any)]
#[async_method]
pub async fn get_uptime_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getUptime0()J")
}

#[intrinsic_method("sun/management/VMManagementImpl.getVerboseClass()Z", Any)]
#[async_method]
pub async fn get_verbose_class<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getVerboseClass()Z")
}

#[intrinsic_method("sun/management/VMManagementImpl.getVerboseGC()Z", Any)]
#[async_method]
pub async fn get_verbose_gc<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getVerboseGC()Z")
}

#[intrinsic_method("sun/management/VMManagementImpl.getVersion0()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_version_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getVersion0()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.getVmArguments0()[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_vm_arguments_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.getVmArguments0()[Ljava/lang/String;")
}

#[intrinsic_method("sun/management/VMManagementImpl.initOptionalSupportFields()V", Any)]
#[async_method]
pub async fn init_optional_support_fields<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.initOptionalSupportFields()V")
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.isThreadAllocatedMemoryEnabled()Z",
    Any
)]
#[async_method]
pub async fn is_thread_allocated_memory_enabled<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.isThreadAllocatedMemoryEnabled()Z")
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.isThreadContentionMonitoringEnabled()Z",
    Any
)]
#[async_method]
pub async fn is_thread_contention_monitoring_enabled<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.VMManagementImpl.isThreadContentionMonitoringEnabled()Z")
}

#[intrinsic_method("sun/management/VMManagementImpl.isThreadCpuTimeEnabled()Z", Any)]
#[async_method]
pub async fn is_thread_cpu_time_enabled<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
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
        let _ = get_available_processors(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getClassInitializationTime()J"
    )]
    async fn test_get_class_initialization_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_initialization_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getClassLoadingTime()J"
    )]
    async fn test_get_class_loading_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_loading_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getClassVerificationTime()J"
    )]
    async fn test_get_class_verification_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_verification_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getDaemonThreadCount()I"
    )]
    async fn test_get_daemon_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_daemon_thread_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getInitializedClassCount()J"
    )]
    async fn test_get_initialized_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_initialized_class_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getLiveThreadCount()I"
    )]
    async fn test_get_live_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_live_thread_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getLoadedClassSize()J"
    )]
    async fn test_get_loaded_class_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_loaded_class_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getMethodDataSize()J"
    )]
    async fn test_get_method_data_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_method_data_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getPeakThreadCount()I"
    )]
    async fn test_get_peak_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_peak_thread_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getProcessId()I"
    )]
    async fn test_get_process_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_process_id(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getSafepointCount()J"
    )]
    async fn test_get_safepoint_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_safepoint_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getSafepointSyncTime()J"
    )]
    async fn test_get_safepoint_sync_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_safepoint_sync_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getStartupTime()J"
    )]
    async fn test_get_startup_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_startup_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalApplicationNonStoppedTime()J"
    )]
    async fn test_get_total_application_non_stopped_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_application_non_stopped_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalClassCount()J"
    )]
    async fn test_get_total_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_class_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalCompileTime()J"
    )]
    async fn test_get_total_compile_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_compile_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalSafepointTime()J"
    )]
    async fn test_get_total_safepoint_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_safepoint_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getTotalThreadCount()J"
    )]
    async fn test_get_total_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_thread_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getUnloadedClassCount()J"
    )]
    async fn test_get_unloaded_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unloaded_class_count(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getUnloadedClassSize()J"
    )]
    async fn test_get_unloaded_class_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_unloaded_class_size(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.management.VMManagementImpl.getUptime0()J")]
    async fn test_get_uptime_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_uptime_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getVerboseClass()Z"
    )]
    async fn test_get_verbose_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_verbose_class(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getVerboseGC()Z"
    )]
    async fn test_get_verbose_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_verbose_gc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getVersion0()Ljava/lang/String;"
    )]
    async fn test_get_version_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_version_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.getVmArguments0()[Ljava/lang/String;"
    )]
    async fn test_get_vm_arguments_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_vm_arguments_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.initOptionalSupportFields()V"
    )]
    async fn test_init_optional_support_fields() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_optional_support_fields(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.isThreadAllocatedMemoryEnabled()Z"
    )]
    async fn test_is_thread_allocated_memory_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_thread_allocated_memory_enabled(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.isThreadContentionMonitoringEnabled()Z"
    )]
    async fn test_is_thread_contention_monitoring_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_thread_contention_monitoring_enabled(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.VMManagementImpl.isThreadCpuTimeEnabled()Z"
    )]
    async fn test_is_thread_cpu_time_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_thread_cpu_time_enabled(thread, Parameters::default()).await;
    }
}
