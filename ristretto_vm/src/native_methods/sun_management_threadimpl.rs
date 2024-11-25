use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `sun.management.ThreadImpl`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/ThreadImpl";
    let java_version = registry.java_version();

    if java_version >= &JAVA_17 {
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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn dump_threads_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_deadlocked_threads_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_monitor_deadlocked_threads_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_thread_allocated_memory_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_thread_allocated_memory_1(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_thread_info_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_thread_total_cpu_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_thread_total_cpu_time_1(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_thread_user_cpu_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_thread_user_cpu_time_1(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_threads(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_total_thread_allocated_memory(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn reset_contention_times_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn reset_peak_thread_count_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_thread_allocated_memory_enabled_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_thread_contention_monitoring_enabled_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_thread_cpu_time_enabled_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
