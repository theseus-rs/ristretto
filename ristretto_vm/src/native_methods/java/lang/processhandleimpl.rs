use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.ProcessHandleImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ProcessHandleImpl";
    registry.register(class_name, "destroy0", "(JJZ)Z", destroy_0);
    registry.register(class_name, "getCurrentPid0", "()J", get_current_pid_0);
    registry.register(
        class_name,
        "getProcessPids0",
        "(J[J[J[J)I",
        get_process_pids_0,
    );
    registry.register(class_name, "initNative", "()V", init_native);
    registry.register(class_name, "isAlive0", "(J)J", is_alive_0);
    registry.register(class_name, "parent0", "(JJ)J", parent_0);
    registry.register(
        class_name,
        "waitForProcessExit0",
        "(JZ)I",
        wait_for_process_exit_0,
    );
}

#[async_recursion(?Send)]
async fn destroy_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ProcessHandleImpl.destroy0(JJZ)Z")
}

#[async_recursion(?Send)]
async fn get_current_pid_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ProcessHandleImpl.getCurrentPid0()J")
}

#[async_recursion(?Send)]
async fn get_process_pids_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ProcessHandleImpl.getProcessPids0(J[J[J[J)I")
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_alive_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ProcessHandleImpl.isAlive0(J)J")
}

#[async_recursion(?Send)]
async fn parent_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ProcessHandleImpl.parent0(JJ)J")
}

#[async_recursion(?Send)]
async fn wait_for_process_exit_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.ProcessHandleImpl.waitForProcessExit0(JZ)I")
}
