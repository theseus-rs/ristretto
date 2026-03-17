use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/ThreadImpl.dumpThreads0([JZZI)[Ljava/lang/management/ThreadInfo;",
    Any
)]
#[async_method]
pub async fn dump_threads_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.dumpThreads0([JZZI)[Ljava/lang/management/ThreadInfo;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/ThreadImpl.findDeadlockedThreads0()[Ljava/lang/Thread;",
    Any
)]
#[async_method]
pub async fn find_deadlocked_threads_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.findDeadlockedThreads0()[Ljava/lang/Thread;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/ThreadImpl.findMonitorDeadlockedThreads0()[Ljava/lang/Thread;",
    Any
)]
#[async_method]
pub async fn find_monitor_deadlocked_threads_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.findMonitorDeadlockedThreads0()[Ljava/lang/Thread;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadAllocatedMemory0(J)J", Any)]
#[async_method]
pub async fn get_thread_allocated_memory_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadAllocatedMemory0(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadAllocatedMemory1([J[J)V", Any)]
#[async_method]
pub async fn get_thread_allocated_memory_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadAllocatedMemory1([J[J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/ThreadImpl.getThreadInfo1([JI[Ljava/lang/management/ThreadInfo;)V",
    Any
)]
#[async_method]
pub async fn get_thread_info_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadInfo1([JI[Ljava/lang/management/ThreadInfo;)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadTotalCpuTime0(J)J", Any)]
#[async_method]
pub async fn get_thread_total_cpu_time_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadTotalCpuTime0(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadTotalCpuTime1([J[J)V", Any)]
#[async_method]
pub async fn get_thread_total_cpu_time_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadTotalCpuTime1([J[J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadUserCpuTime0(J)J", Any)]
#[async_method]
pub async fn get_thread_user_cpu_time_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadUserCpuTime0(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadUserCpuTime1([J[J)V", Any)]
#[async_method]
pub async fn get_thread_user_cpu_time_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadUserCpuTime1([J[J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreads()[Ljava/lang/Thread;", Any)]
#[async_method]
pub async fn get_threads<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreads()[Ljava/lang/Thread;".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getTotalThreadAllocatedMemory()J", Any)]
#[async_method]
pub async fn get_total_thread_allocated_memory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getTotalThreadAllocatedMemory()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.resetContentionTimes0(J)V", Any)]
#[async_method]
pub async fn reset_contention_times_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.resetContentionTimes0(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.resetPeakThreadCount0()V", Any)]
#[async_method]
pub async fn reset_peak_thread_count_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.resetPeakThreadCount0()V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.setThreadAllocatedMemoryEnabled0(Z)V", Any)]
#[async_method]
pub async fn set_thread_allocated_memory_enabled_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.setThreadAllocatedMemoryEnabled0(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/ThreadImpl.setThreadContentionMonitoringEnabled0(Z)V",
    Any
)]
#[async_method]
pub async fn set_thread_contention_monitoring_enabled_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.setThreadContentionMonitoringEnabled0(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.setThreadCpuTimeEnabled0(Z)V", Any)]
#[async_method]
pub async fn set_thread_cpu_time_enabled_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.setThreadCpuTimeEnabled0(Z)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dump_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dump_threads_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_deadlocked_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_deadlocked_threads_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_monitor_deadlocked_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_monitor_deadlocked_threads_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_thread_allocated_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_allocated_memory_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_thread_allocated_memory_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_allocated_memory_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_thread_info_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_info_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_thread_total_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_total_cpu_time_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_thread_total_cpu_time_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_total_cpu_time_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_thread_user_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_user_cpu_time_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_thread_user_cpu_time_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_user_cpu_time_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_threads() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_threads(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_total_thread_allocated_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_thread_allocated_memory(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_contention_times_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_contention_times_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_peak_thread_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_peak_thread_count_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_thread_allocated_memory_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_thread_allocated_memory_enabled_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_thread_contention_monitoring_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_thread_contention_monitoring_enabled_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_thread_cpu_time_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_thread_cpu_time_enabled_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
