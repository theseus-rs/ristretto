use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{Reference, Value};
use std::process;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

const CLASS_NAME: &str = "java/lang/ProcessHandleImpl";

/// Register all native methods for `java.lang.ProcessHandleImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "destroy0", "(JJZ)Z", destroy_0);
    registry.register(CLASS_NAME, "getCurrentPid0", "()J", get_current_pid_0);
    registry.register(
        CLASS_NAME,
        "getProcessPids0",
        "(J[J[J[J)I",
        get_process_pids_0,
    );
    registry.register(CLASS_NAME, "initNative", "()V", init_native);
    registry.register(CLASS_NAME, "isAlive0", "(J)J", is_alive_0);
    registry.register(CLASS_NAME, "parent0", "(JJ)J", parent_0);
    registry.register(
        CLASS_NAME,
        "waitForProcessExit0",
        "(JZ)I",
        wait_for_process_exit_0,
    );
}

#[async_recursion(?Send)]
async fn destroy_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let force = arguments.pop_int()? != 0;
    let _start_time = arguments.pop_long()?;
    let pid = arguments.pop_long()?;
    let pid = usize::try_from(pid)?;
    let pid = Pid::from(pid);
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    let Some(process) = system.process(pid) else {
        return Ok(Some(Value::from(false)));
    };

    let signal = if force { Signal::Kill } else { Signal::Term };
    let signal_without_error = process.kill_with(signal).unwrap_or(false);

    Ok(Some(Value::from(signal_without_error)))
}

#[async_recursion(?Send)]
async fn get_current_pid_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    let pid = i64::from(process::id());
    Ok(Some(Value::Long(pid)))
}

#[expect(clippy::similar_names)]
#[async_recursion(?Send)]
async fn get_process_pids_0(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::LongArray(start_times)) = arguments.pop_reference()? else {
        return Ok(Some(Value::Int(-1)));
    };
    let Some(Reference::LongArray(ppids)) = arguments.pop_reference()? else {
        return Ok(Some(Value::Int(-1)));
    };
    let Some(Reference::LongArray(pids)) = arguments.pop_reference()? else {
        return Ok(Some(Value::Int(-1)));
    };
    let pid = arguments.pop_long()?;
    let mut system = System::new_all();

    let processes_length = if pid == 0 {
        system.refresh_processes(ProcessesToUpdate::All, true);
        let processes = system.processes();
        for (index, (pid, process)) in processes.iter().enumerate() {
            // Determine if we have enough space to store the next pid
            if index > pids.capacity()? {
                break;
            }

            let pid = pid.as_u32();
            let pid = i64::from(pid);
            pids.push(pid)?;
            let parent = process.parent().map(Pid::as_u32).unwrap_or_default();
            let parent = i64::from(parent);
            ppids.push(parent)?;
            let run_time = Duration::from_secs(process.run_time());
            let duration = run_time.as_millis();
            let duration = i64::try_from(duration).unwrap_or_default();
            start_times.push(duration)?;
        }
        i32::try_from(processes.len())?
    } else {
        let pid = usize::try_from(pid)?;
        let pid = Pid::from(pid);
        system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
        if let Some(process) = system.process(pid) {
            let pid = pid.as_u32();
            let pid = i64::from(pid);
            pids.push(pid)?;
            let parent = process.parent().map(Pid::as_u32).unwrap_or_default();
            let parent = i64::from(parent);
            ppids.push(parent)?;
            let run_time = Duration::from_secs(process.run_time());
            let duration = run_time.as_millis();
            let duration = i64::try_from(duration).unwrap_or_default();
            start_times.push(duration)?;
            1
        } else {
            -1
        }
    };

    Ok(Some(Value::Int(processes_length)))
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_alive_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let pid = arguments.pop_long()?;
    let pid = usize::try_from(pid)?;
    let pid = Pid::from(pid);
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    let alive = if let Some(process) = system.process(pid) {
        let run_time = Duration::from_secs(process.run_time());
        let duration = run_time.as_millis();
        i64::try_from(duration).unwrap_or_default()
    } else {
        -1
    };

    Ok(Some(Value::Long(alive)))
}

#[async_recursion(?Send)]
async fn parent_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _start_time = arguments.pop_long()?;
    let pid = arguments.pop_long()?;
    let pid = usize::try_from(pid)?;
    let pid = Pid::from(pid);
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    let Some(process) = system.process(pid) else {
        return Ok(Some(Value::Long(-1)));
    };
    let Some(parent) = process.parent() else {
        return Ok(Some(Value::Long(-1)));
    };
    let parent_pid = parent.as_u32();
    let parent_pid = i64::from(parent_pid);

    Ok(Some(Value::Long(parent_pid)))
}

#[async_recursion(?Send)]
async fn wait_for_process_exit_0(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let _start_time = arguments.pop_long()?;
    let pid = arguments.pop_long()?;
    let pid = usize::try_from(pid)?;
    let pid = Pid::from(pid);
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    let Some(process) = system.process(pid) else {
        return Ok(Some(Value::Int(-1)));
    };

    process.wait();

    // TODO: evaluate expected return value
    Ok(Some(Value::Int(0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_current_pid_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_current_pid_0(thread, Arguments::default()).await?;
        let pid = i64::from(process::id());
        assert_eq!(result, Some(Value::Long(pid)));
        Ok(())
    }

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_native(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_alive_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let pid = Value::Long(i64::from(process::id()));
        let result = is_alive_0(thread, Arguments::new(vec![pid])).await?;
        let run_time = result.unwrap_or(Value::Long(0)).to_long()?;
        assert!(run_time > 0);
        Ok(())
    }
}
