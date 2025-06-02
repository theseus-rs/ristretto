use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_17, JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/ThreadImpl";

/// Register all intrinsic methods for `sun.management.ThreadImpl`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_17 || registry.java_major_version() >= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "getTotalThreadAllocatedMemory",
            "()J",
            get_total_thread_allocated_memory,
        );
    }

    registry.register(
        CLASS_NAME,
        "dumpThreads0",
        "([JZZI)[Ljava/lang/management/ThreadInfo;",
        dump_threads_0,
    );
    registry.register(
        CLASS_NAME,
        "findDeadlockedThreads0",
        "()[Ljava/lang/Thread;",
        find_deadlocked_threads_0,
    );
    registry.register(
        CLASS_NAME,
        "findMonitorDeadlockedThreads0",
        "()[Ljava/lang/Thread;",
        find_monitor_deadlocked_threads_0,
    );
    registry.register(
        CLASS_NAME,
        "getThreadAllocatedMemory0",
        "(J)J",
        get_thread_allocated_memory_0,
    );
    registry.register(
        CLASS_NAME,
        "getThreadAllocatedMemory1",
        "([J[J)V",
        get_thread_allocated_memory_1,
    );
    registry.register(
        CLASS_NAME,
        "getThreadInfo1",
        "([JI[Ljava/lang/management/ThreadInfo;)V",
        get_thread_info_1,
    );
    registry.register(
        CLASS_NAME,
        "getThreadTotalCpuTime0",
        "(J)J",
        get_thread_total_cpu_time_0,
    );
    registry.register(
        CLASS_NAME,
        "getThreadTotalCpuTime1",
        "([J[J)V",
        get_thread_total_cpu_time_1,
    );
    registry.register(
        CLASS_NAME,
        "getThreadUserCpuTime0",
        "(J)J",
        get_thread_user_cpu_time_0,
    );
    registry.register(
        CLASS_NAME,
        "getThreadUserCpuTime1",
        "([J[J)V",
        get_thread_user_cpu_time_1,
    );
    registry.register(
        CLASS_NAME,
        "getThreads",
        "()[Ljava/lang/Thread;",
        get_threads,
    );
    registry.register(
        CLASS_NAME,
        "resetContentionTimes0",
        "(J)V",
        reset_contention_times_0,
    );
    registry.register(
        CLASS_NAME,
        "resetPeakThreadCount0",
        "()V",
        reset_peak_thread_count_0,
    );
    registry.register(
        CLASS_NAME,
        "setThreadAllocatedMemoryEnabled0",
        "(Z)V",
        set_thread_allocated_memory_enabled_0,
    );
    registry.register(
        CLASS_NAME,
        "setThreadContentionMonitoringEnabled0",
        "(Z)V",
        set_thread_contention_monitoring_enabled_0,
    );
    registry.register(
        CLASS_NAME,
        "setThreadCpuTimeEnabled0",
        "(Z)V",
        set_thread_cpu_time_enabled_0,
    );
}

#[async_recursion(?Send)]
async fn dump_threads_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.dumpThreads0([JZZI)[Ljava/lang/management/ThreadInfo;")
}

#[async_recursion(?Send)]
async fn find_deadlocked_threads_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.findDeadlockedThreads0()[Ljava/lang/Thread;")
}

#[async_recursion(?Send)]
async fn find_monitor_deadlocked_threads_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.findMonitorDeadlockedThreads0()[Ljava/lang/Thread;")
}

#[async_recursion(?Send)]
async fn get_thread_allocated_memory_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadAllocatedMemory0(J)J")
}

#[async_recursion(?Send)]
async fn get_thread_allocated_memory_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadAllocatedMemory1([J[J)V")
}

#[async_recursion(?Send)]
async fn get_thread_info_1(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadInfo1([JI[Ljava/lang/management/ThreadInfo;)V")
}

#[async_recursion(?Send)]
async fn get_thread_total_cpu_time_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadTotalCpuTime0(J)J")
}

#[async_recursion(?Send)]
async fn get_thread_total_cpu_time_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadTotalCpuTime1([J[J)V")
}

#[async_recursion(?Send)]
async fn get_thread_user_cpu_time_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadUserCpuTime0(J)J")
}

#[async_recursion(?Send)]
async fn get_thread_user_cpu_time_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadUserCpuTime1([J[J)V")
}

#[async_recursion(?Send)]
async fn get_threads(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreads()[Ljava/lang/Thread;")
}

#[async_recursion(?Send)]
async fn get_total_thread_allocated_memory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getTotalThreadAllocatedMemory()J")
}

#[async_recursion(?Send)]
async fn reset_contention_times_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.resetContentionTimes0(J)V")
}

#[async_recursion(?Send)]
async fn reset_peak_thread_count_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.resetPeakThreadCount0()V")
}

#[async_recursion(?Send)]
async fn set_thread_allocated_memory_enabled_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.setThreadAllocatedMemoryEnabled0(Z)V")
}

#[async_recursion(?Send)]
async fn set_thread_contention_monitoring_enabled_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.setThreadContentionMonitoringEnabled0(Z)V")
}

#[async_recursion(?Send)]
async fn set_thread_cpu_time_enabled_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.setThreadCpuTimeEnabled0(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.dumpThreads0([JZZI)[Ljava/lang/management/ThreadInfo;"
    )]
    async fn test_dump_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dump_threads_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.findDeadlockedThreads0()[Ljava/lang/Thread;"
    )]
    async fn test_find_deadlocked_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_deadlocked_threads_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.findMonitorDeadlockedThreads0()[Ljava/lang/Thread;"
    )]
    async fn test_find_monitor_deadlocked_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_monitor_deadlocked_threads_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.getThreadAllocatedMemory0(J)J"
    )]
    async fn test_get_thread_allocated_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_allocated_memory_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.getThreadAllocatedMemory1([J[J)V"
    )]
    async fn test_get_thread_allocated_memory_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_allocated_memory_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.getThreadInfo1([JI[Ljava/lang/management/ThreadInfo;)V"
    )]
    async fn test_get_thread_info_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_info_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.getThreadTotalCpuTime0(J)J"
    )]
    async fn test_get_thread_total_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_total_cpu_time_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.getThreadTotalCpuTime1([J[J)V"
    )]
    async fn test_get_thread_total_cpu_time_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_total_cpu_time_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.getThreadUserCpuTime0(J)J"
    )]
    async fn test_get_thread_user_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_user_cpu_time_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.getThreadUserCpuTime1([J[J)V"
    )]
    async fn test_get_thread_user_cpu_time_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_user_cpu_time_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.getThreads()[Ljava/lang/Thread;"
    )]
    async fn test_get_threads() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_threads(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.getTotalThreadAllocatedMemory()J"
    )]
    async fn test_get_total_thread_allocated_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_thread_allocated_memory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.resetContentionTimes0(J)V"
    )]
    async fn test_reset_contention_times_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_contention_times_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.resetPeakThreadCount0()V"
    )]
    async fn test_reset_peak_thread_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_peak_thread_count_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.setThreadAllocatedMemoryEnabled0(Z)V"
    )]
    async fn test_set_thread_allocated_memory_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_thread_allocated_memory_enabled_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.setThreadContentionMonitoringEnabled0(Z)V"
    )]
    async fn test_set_thread_contention_monitoring_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_thread_contention_monitoring_enabled_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.ThreadImpl.setThreadCpuTimeEnabled0(Z)V"
    )]
    async fn test_set_thread_cpu_time_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_thread_cpu_time_enabled_0(thread, Parameters::default()).await;
    }
}
