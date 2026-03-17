use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/management/VMManagementImpl.getAvailableProcessors()I", Any)]
#[async_method]
pub async fn get_available_processors<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getAvailableProcessors()I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getClassInitializationTime()J", Any)]
#[async_method]
pub async fn get_class_initialization_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getClassInitializationTime()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getClassLoadingTime()J", Any)]
#[async_method]
pub async fn get_class_loading_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getClassLoadingTime()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getClassVerificationTime()J", Any)]
#[async_method]
pub async fn get_class_verification_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getClassVerificationTime()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getDaemonThreadCount()I", Any)]
#[async_method]
pub async fn get_daemon_thread_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getDaemonThreadCount()I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getInitializedClassCount()J", Any)]
#[async_method]
pub async fn get_initialized_class_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getInitializedClassCount()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getLiveThreadCount()I", Any)]
#[async_method]
pub async fn get_live_thread_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getLiveThreadCount()I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getLoadedClassSize()J", Any)]
#[async_method]
pub async fn get_loaded_class_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getLoadedClassSize()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getMethodDataSize()J", Any)]
#[async_method]
pub async fn get_method_data_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getMethodDataSize()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getPeakThreadCount()I", Any)]
#[async_method]
pub async fn get_peak_thread_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getPeakThreadCount()I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getProcessId()I", Any)]
#[async_method]
pub async fn get_process_id<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getProcessId()I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getSafepointCount()J", Any)]
#[async_method]
pub async fn get_safepoint_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getSafepointCount()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getSafepointSyncTime()J", Any)]
#[async_method]
pub async fn get_safepoint_sync_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getSafepointSyncTime()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getStartupTime()J", Any)]
#[async_method]
pub async fn get_startup_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getStartupTime()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.getTotalApplicationNonStoppedTime()J",
    Any
)]
#[async_method]
pub async fn get_total_application_non_stopped_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getTotalApplicationNonStoppedTime()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalClassCount()J", Any)]
#[async_method]
pub async fn get_total_class_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getTotalClassCount()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalCompileTime()J", Any)]
#[async_method]
pub async fn get_total_compile_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getTotalCompileTime()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalSafepointTime()J", Any)]
#[async_method]
pub async fn get_total_safepoint_time<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getTotalSafepointTime()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getTotalThreadCount()J", Any)]
#[async_method]
pub async fn get_total_thread_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getTotalThreadCount()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getUnloadedClassCount()J", Any)]
#[async_method]
pub async fn get_unloaded_class_count<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getUnloadedClassCount()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getUnloadedClassSize()J", Any)]
#[async_method]
pub async fn get_unloaded_class_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getUnloadedClassSize()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getUptime0()J", Any)]
#[async_method]
pub async fn get_uptime_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getUptime0()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getVerboseClass()Z", Any)]
#[async_method]
pub async fn get_verbose_class<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getVerboseClass()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getVerboseGC()Z", Any)]
#[async_method]
pub async fn get_verbose_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getVerboseGC()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.getVersion0()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_version_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getVersion0()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.getVmArguments0()[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_vm_arguments_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.getVmArguments0()[Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.initOptionalSupportFields()V", Any)]
#[async_method]
pub async fn init_optional_support_fields<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.initOptionalSupportFields()V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.isThreadAllocatedMemoryEnabled()Z",
    Any
)]
#[async_method]
pub async fn is_thread_allocated_memory_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.isThreadAllocatedMemoryEnabled()Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/VMManagementImpl.isThreadContentionMonitoringEnabled()Z",
    Any
)]
#[async_method]
pub async fn is_thread_contention_monitoring_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.isThreadContentionMonitoringEnabled()Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/VMManagementImpl.isThreadCpuTimeEnabled()Z", Any)]
#[async_method]
pub async fn is_thread_cpu_time_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.VMManagementImpl.isThreadCpuTimeEnabled()Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_available_processors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_available_processors(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_class_initialization_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_class_initialization_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_class_loading_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_class_loading_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_class_verification_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_class_verification_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_daemon_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_daemon_thread_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_initialized_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_initialized_class_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_live_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_live_thread_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_loaded_class_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_loaded_class_size(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_method_data_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_method_data_size(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_peak_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_peak_thread_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_process_id() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_process_id(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_safepoint_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_safepoint_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_safepoint_sync_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_safepoint_sync_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_startup_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_startup_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_total_application_non_stopped_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_application_non_stopped_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_total_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_class_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_total_compile_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_compile_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_total_safepoint_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_safepoint_time(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_total_thread_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_thread_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_unloaded_class_count() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_unloaded_class_count(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_unloaded_class_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_unloaded_class_size(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_uptime_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_uptime_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_verbose_class() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_verbose_class(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_verbose_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_verbose_gc(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_version_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_version_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_vm_arguments_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_vm_arguments_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_optional_support_fields() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_optional_support_fields(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_thread_allocated_memory_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_thread_allocated_memory_enabled(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_thread_contention_monitoring_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_thread_contention_monitoring_enabled(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_thread_cpu_time_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_thread_cpu_time_enabled(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
