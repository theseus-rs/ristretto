use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_21: Version = Version::Java21 { minor: 0 };

/// Register all native methods for `sun.management.ThreadImpl`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/ThreadImpl";
    let java_version = registry.java_version().clone();

    if java_version == JAVA_17 || java_version >= JAVA_21 {
        registry.register(
            class_name,
            "getTotalThreadAllocatedMemory",
            "()J",
            get_total_thread_allocated_memory,
        );
    }

    registry.register(
        class_name,
        "dumpThreads0",
        "([JZZI)[Ljava/lang/management/ThreadInfo;",
        dump_threads_0,
    );
    registry.register(
        class_name,
        "findDeadlockedThreads0",
        "()[Ljava/lang/Thread;",
        find_deadlocked_threads_0,
    );
    registry.register(
        class_name,
        "findMonitorDeadlockedThreads0",
        "()[Ljava/lang/Thread;",
        find_monitor_deadlocked_threads_0,
    );
    registry.register(
        class_name,
        "getThreadAllocatedMemory0",
        "(J)J",
        get_thread_allocated_memory_0,
    );
    registry.register(
        class_name,
        "getThreadAllocatedMemory1",
        "([J[J)V",
        get_thread_allocated_memory_1,
    );
    registry.register(
        class_name,
        "getThreadInfo1",
        "([JI[Ljava/lang/management/ThreadInfo;)V",
        get_thread_info_1,
    );
    registry.register(
        class_name,
        "getThreadTotalCpuTime0",
        "(J)J",
        get_thread_total_cpu_time_0,
    );
    registry.register(
        class_name,
        "getThreadTotalCpuTime1",
        "([J[J)V",
        get_thread_total_cpu_time_1,
    );
    registry.register(
        class_name,
        "getThreadUserCpuTime0",
        "(J)J",
        get_thread_user_cpu_time_0,
    );
    registry.register(
        class_name,
        "getThreadUserCpuTime1",
        "([J[J)V",
        get_thread_user_cpu_time_1,
    );
    registry.register(
        class_name,
        "getThreads",
        "()[Ljava/lang/Thread;",
        get_threads,
    );
    registry.register(
        class_name,
        "resetContentionTimes0",
        "(J)V",
        reset_contention_times_0,
    );
    registry.register(
        class_name,
        "resetPeakThreadCount0",
        "()V",
        reset_peak_thread_count_0,
    );
    registry.register(
        class_name,
        "setThreadAllocatedMemoryEnabled0",
        "(Z)V",
        set_thread_allocated_memory_enabled_0,
    );
    registry.register(
        class_name,
        "setThreadContentionMonitoringEnabled0",
        "(Z)V",
        set_thread_contention_monitoring_enabled_0,
    );
    registry.register(
        class_name,
        "setThreadCpuTimeEnabled0",
        "(Z)V",
        set_thread_cpu_time_enabled_0,
    );
}

#[async_recursion(?Send)]
async fn dump_threads_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.dumpThreads0([JZZI)[Ljava/lang/management/ThreadInfo;")
}

#[async_recursion(?Send)]
async fn find_deadlocked_threads_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.findDeadlockedThreads0()[Ljava/lang/Thread;")
}

#[async_recursion(?Send)]
async fn find_monitor_deadlocked_threads_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.findMonitorDeadlockedThreads0()[Ljava/lang/Thread;")
}

#[async_recursion(?Send)]
async fn get_thread_allocated_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadAllocatedMemory0(J)J")
}

#[async_recursion(?Send)]
async fn get_thread_allocated_memory_1(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadAllocatedMemory1([J[J)V")
}

#[async_recursion(?Send)]
async fn get_thread_info_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadInfo1([JI[Ljava/lang/management/ThreadInfo;)V")
}

#[async_recursion(?Send)]
async fn get_thread_total_cpu_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadTotalCpuTime0(J)J")
}

#[async_recursion(?Send)]
async fn get_thread_total_cpu_time_1(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadTotalCpuTime1([J[J)V")
}

#[async_recursion(?Send)]
async fn get_thread_user_cpu_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadUserCpuTime0(J)J")
}

#[async_recursion(?Send)]
async fn get_thread_user_cpu_time_1(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreadUserCpuTime1([J[J)V")
}

#[async_recursion(?Send)]
async fn get_threads(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getThreads()[Ljava/lang/Thread;")
}

#[async_recursion(?Send)]
async fn get_total_thread_allocated_memory(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.getTotalThreadAllocatedMemory()J")
}

#[async_recursion(?Send)]
async fn reset_contention_times_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.resetContentionTimes0(J)V")
}

#[async_recursion(?Send)]
async fn reset_peak_thread_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.resetPeakThreadCount0()V")
}

#[async_recursion(?Send)]
async fn set_thread_allocated_memory_enabled_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.setThreadAllocatedMemoryEnabled0(Z)V")
}

#[async_recursion(?Send)]
async fn set_thread_contention_monitoring_enabled_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.setThreadContentionMonitoringEnabled0(Z)V")
}

#[async_recursion(?Send)]
async fn set_thread_cpu_time_enabled_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.ThreadImpl.setThreadCpuTimeEnabled0(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/management/ThreadImpl";
        assert!(registry
            .method(
                class_name,
                "dumpThreads0",
                "([JZZI)[Ljava/lang/management/ThreadInfo;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "findDeadlockedThreads0",
                "()[Ljava/lang/Thread;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "findMonitorDeadlockedThreads0",
                "()[Ljava/lang/Thread;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getThreadAllocatedMemory0", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "getThreadAllocatedMemory1", "([J[J)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getThreadInfo1",
                "([JI[Ljava/lang/management/ThreadInfo;)V"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getThreadTotalCpuTime0", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "getThreadTotalCpuTime1", "([J[J)V")
            .is_some());
        assert!(registry
            .method(class_name, "getThreadUserCpuTime0", "(J)J")
            .is_some());
        assert!(registry
            .method(class_name, "getThreadUserCpuTime1", "([J[J)V")
            .is_some());
        assert!(registry
            .method(class_name, "getThreads", "()[Ljava/lang/Thread;")
            .is_some());
        assert!(registry
            .method(class_name, "resetContentionTimes0", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "resetPeakThreadCount0", "()V")
            .is_some());
        assert!(registry
            .method(class_name, "setThreadAllocatedMemoryEnabled0", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "setThreadContentionMonitoringEnabled0", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "setThreadCpuTimeEnabled0", "(Z)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.ThreadImpl.dumpThreads0([JZZI)[Ljava/lang/management/ThreadInfo;"
    )]
    async fn test_dump_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dump_threads_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.ThreadImpl.findDeadlockedThreads0()[Ljava/lang/Thread;"
    )]
    async fn test_find_deadlocked_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_deadlocked_threads_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.ThreadImpl.findMonitorDeadlockedThreads0()[Ljava/lang/Thread;"
    )]
    async fn test_find_monitor_deadlocked_threads_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = find_monitor_deadlocked_threads_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.getThreadAllocatedMemory0(J)J")]
    async fn test_get_thread_allocated_memory_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_allocated_memory_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.getThreadAllocatedMemory1([J[J)V")]
    async fn test_get_thread_allocated_memory_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_allocated_memory_1(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.ThreadImpl.getThreadInfo1([JI[Ljava/lang/management/ThreadInfo;)V"
    )]
    async fn test_get_thread_info_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_info_1(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.getThreadTotalCpuTime0(J)J")]
    async fn test_get_thread_total_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_total_cpu_time_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.getThreadTotalCpuTime1([J[J)V")]
    async fn test_get_thread_total_cpu_time_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_total_cpu_time_1(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.getThreadUserCpuTime0(J)J")]
    async fn test_get_thread_user_cpu_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_user_cpu_time_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.getThreadUserCpuTime1([J[J)V")]
    async fn test_get_thread_user_cpu_time_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_thread_user_cpu_time_1(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.getThreads()[Ljava/lang/Thread;")]
    async fn test_get_threads() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_threads(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.getTotalThreadAllocatedMemory()J")]
    async fn test_get_total_thread_allocated_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_total_thread_allocated_memory(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.resetContentionTimes0(J)V")]
    async fn test_reset_contention_times_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_contention_times_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.resetPeakThreadCount0()V")]
    async fn test_reset_peak_thread_count_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = reset_peak_thread_count_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.setThreadAllocatedMemoryEnabled0(Z)V")]
    async fn test_set_thread_allocated_memory_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_thread_allocated_memory_enabled_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.ThreadImpl.setThreadContentionMonitoringEnabled0(Z)V"
    )]
    async fn test_set_thread_contention_monitoring_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_thread_contention_monitoring_enabled_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.ThreadImpl.setThreadCpuTimeEnabled0(Z)V")]
    async fn test_set_thread_cpu_time_enabled_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_thread_cpu_time_enabled_0(thread, Arguments::default()).await;
    }
}
