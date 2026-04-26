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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _max_depth = parameters.pop_int()?;
    let _locked_synchronizers = parameters.pop_bool()?;
    let _locked_monitors = parameters.pop_bool()?;
    let _ids = parameters.pop_reference()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadAllocatedMemory0(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadAllocatedMemory1([J[J)V", Any)]
#[async_method]
pub async fn get_thread_allocated_memory_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _result = parameters.pop_reference()?;
    let _ids = parameters.pop_reference()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _result = parameters.pop_reference()?;
    let _max_depth = parameters.pop_int()?;
    let _ids = parameters.pop_reference()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadTotalCpuTime0(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadTotalCpuTime1([J[J)V", Any)]
#[async_method]
pub async fn get_thread_total_cpu_time_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _result = parameters.pop_reference()?;
    let _ids = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadTotalCpuTime1([J[J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadUserCpuTime0(J)J", Any)]
#[async_method]
pub async fn get_thread_user_cpu_time_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.getThreadUserCpuTime0(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.getThreadUserCpuTime1([J[J)V", Any)]
#[async_method]
pub async fn get_thread_user_cpu_time_1<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _result = parameters.pop_reference()?;
    let _ids = parameters.pop_reference()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _tid = parameters.pop_long()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enable = parameters.pop_bool()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enable = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.ThreadImpl.setThreadContentionMonitoringEnabled0(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/management/ThreadImpl.setThreadCpuTimeEnabled0(Z)V", Any)]
#[async_method]
pub async fn set_thread_cpu_time_enabled_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enable = parameters.pop_bool()?;
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
        let result = dump_threads_0(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::from(false),
                Value::from(false),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.management.ThreadImpl.dumpThreads0([JZZI)[Ljava/lang/management/ThreadInfo;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_find_deadlocked_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_deadlocked_threads_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.ThreadImpl.findDeadlockedThreads0()[Ljava/lang/Thread;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_find_monitor_deadlocked_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = find_monitor_deadlocked_threads_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.ThreadImpl.findMonitorDeadlockedThreads0()[Ljava/lang/Thread;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_thread_allocated_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_thread_allocated_memory_0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.management.ThreadImpl.getThreadAllocatedMemory0(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_thread_allocated_memory_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_allocated_memory_1(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.management.ThreadImpl.getThreadAllocatedMemory1([J[J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_thread_info_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_info_1(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.management.ThreadImpl.getThreadInfo1([JI[Ljava/lang/management/ThreadInfo;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_thread_total_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_thread_total_cpu_time_0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.management.ThreadImpl.getThreadTotalCpuTime0(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_thread_total_cpu_time_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_total_cpu_time_1(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.management.ThreadImpl.getThreadTotalCpuTime1([J[J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_thread_user_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_thread_user_cpu_time_0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.management.ThreadImpl.getThreadUserCpuTime0(J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_thread_user_cpu_time_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_thread_user_cpu_time_1(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.management.ThreadImpl.getThreadUserCpuTime1([J[J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_threads() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_threads(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.ThreadImpl.getThreads()[Ljava/lang/Thread;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_total_thread_allocated_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_total_thread_allocated_memory(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.ThreadImpl.getTotalThreadAllocatedMemory()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_reset_contention_times_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_contention_times_0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.management.ThreadImpl.resetContentionTimes0(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_reset_peak_thread_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reset_peak_thread_count_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.management.ThreadImpl.resetPeakThreadCount0()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_thread_allocated_memory_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_thread_allocated_memory_enabled_0(
            thread,
            Parameters::new(vec![Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.management.ThreadImpl.setThreadAllocatedMemoryEnabled0(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_thread_contention_monitoring_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_thread_contention_monitoring_enabled_0(
            thread,
            Parameters::new(vec![Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.management.ThreadImpl.setThreadContentionMonitoringEnabled0(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_thread_cpu_time_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_thread_cpu_time_enabled_0(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun.management.ThreadImpl.setThreadCpuTimeEnabled0(Z)V",
            result.unwrap_err().to_string()
        );
    }
}
