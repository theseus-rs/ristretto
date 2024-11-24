use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::cmp::min;
use std::sync::Arc;
use sysinfo::System;

/// Register all native methods for `java.lang.Runtime`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Runtime";
    registry.register(
        class_name,
        "availableProcessors",
        "()I",
        available_processors,
    );
    registry.register(class_name, "freeMemory", "()J", free_memory);
    registry.register(class_name, "gc", "()V", gc);
    registry.register(class_name, "maxMemory", "()J", max_memory);
    registry.register(class_name, "runFinalization0", "()V", run_finalization_0);
    registry.register(class_name, "totalMemory", "()J", total_memory);
    registry.register(class_name, "traceInstructions", "(Z)V", trace_instructions);
    registry.register(class_name, "traceMethodCalls", "(Z)V", trace_method_calls);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn available_processors(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    let sys = System::new_all();
    let cpus = sys.physical_core_count().unwrap_or(1);
    let cpus = i32::try_from(cpus)?;
    Ok(Some(Value::Int(cpus)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn free_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let sys = System::new_all();
    let free_memory = sys.total_memory() - sys.used_memory();
    let free_memory = if free_memory > u64::try_from(i64::MAX)? {
        i64::MAX
    } else {
        i64::try_from(free_memory)?
    };
    Ok(Some(Value::Long(free_memory)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn max_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let sys = System::new_all();
    let total_memory = min(sys.total_memory(), u64::try_from(i64::MAX)?);
    let total_memory = i64::try_from(total_memory)?;
    Ok(Some(Value::Long(total_memory)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn run_finalization_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn total_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: This is not the correct implementation; should be the total memory of the JVM
    let sys = System::new_all();
    let used_memory = sys.used_memory();
    let free_memory = if used_memory > u64::try_from(i64::MAX)? {
        i64::MAX
    } else {
        i64::try_from(used_memory)?
    };
    Ok(Some(Value::Long(free_memory)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn trace_instructions(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn trace_method_calls(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
