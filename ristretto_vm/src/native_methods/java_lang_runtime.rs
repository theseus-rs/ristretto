use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Result;
use ristretto_classloader::Value;
use std::cmp::min;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use sysinfo::System;

/// Register all native methods for java.lang.Runtime.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Runtime";
    registry.register(
        class_name,
        "availableProcessors",
        "()I",
        available_processors,
    );
    registry.register(class_name, "freeMemory", "()J", free_memory);
    registry.register(class_name, "totalMemory", "()J", total_memory);
    registry.register(class_name, "maxMemory", "()J", max_memory);
    registry.register(class_name, "gc", "()V", gc);
}

#[expect(clippy::needless_pass_by_value)]
fn available_processors(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let sys = System::new_all();
        let cpus = sys.physical_core_count().unwrap_or(1);
        let cpus = i32::try_from(cpus)?;
        Ok(Some(Value::Int(cpus)))
    })
}

#[expect(clippy::needless_pass_by_value)]
fn free_memory(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let sys = System::new_all();
        let free_memory = sys.total_memory() - sys.used_memory();
        let free_memory = if free_memory > u64::try_from(i64::MAX)? {
            i64::MAX
        } else {
            i64::try_from(free_memory)?
        };
        Ok(Some(Value::Long(free_memory)))
    })
}

#[expect(clippy::needless_pass_by_value)]
fn total_memory(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        // TODO: This is not the correct implementation; should be the total memory of the JVM
        let sys = System::new_all();
        let used_memory = sys.used_memory();
        let free_memory = if used_memory > u64::try_from(i64::MAX)? {
            i64::MAX
        } else {
            i64::try_from(used_memory)?
        };
        Ok(Some(Value::Long(free_memory)))
    })
}

#[expect(clippy::needless_pass_by_value)]
fn max_memory(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let sys = System::new_all();
        let total_memory = min(sys.total_memory(), u64::try_from(i64::MAX)?);
        let total_memory = i64::try_from(total_memory)?;
        Ok(Some(Value::Long(total_memory)))
    })
}

#[expect(clippy::needless_pass_by_value)]
fn gc(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(None) })
}
