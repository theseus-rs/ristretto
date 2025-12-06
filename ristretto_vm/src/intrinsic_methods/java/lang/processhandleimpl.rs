use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use std::process;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.destroy0(JJZ)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn destroy_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let force = parameters.pop_bool()?;
    let _start_time = parameters.pop_long()?;
    let pid = parameters.pop_long()?;
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

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.getCurrentPid0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_current_pid_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let pid = i64::from(process::id());
    Ok(Some(Value::Long(pid)))
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.getProcessPids0(J[J[J[J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[expect(clippy::similar_names)]
#[async_recursion(?Send)]
pub(crate) async fn get_process_pids_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(start_times_ref) = parameters.pop_reference()? else {
        return Ok(Some(Value::Int(-1)));
    };
    let Some(ppids_ref) = parameters.pop_reference()? else {
        return Ok(Some(Value::Int(-1)));
    };
    let Some(pids_ref) = parameters.pop_reference()? else {
        return Ok(Some(Value::Int(-1)));
    };

    let mut start_times_guard = start_times_ref.write();
    let Reference::LongArray(start_times) = &mut *start_times_guard else {
        return Ok(Some(Value::Int(-1)));
    };

    let mut ppids_guard = ppids_ref.write();
    let Reference::LongArray(ppids) = &mut *ppids_guard else {
        return Ok(Some(Value::Int(-1)));
    };

    let mut pids_guard = pids_ref.write();
    let Reference::LongArray(pids) = &mut *pids_guard else {
        return Ok(Some(Value::Int(-1)));
    };

    let pid = parameters.pop_long()?;
    let mut system = System::new_all();

    let processes_length = if pid == 0 {
        system.refresh_processes(ProcessesToUpdate::All, true);
        let processes = system.processes();
        for (index, (pid, process)) in processes.iter().enumerate() {
            // Determine if we have enough space to store the next pid
            if index > pids.capacity() {
                break;
            }

            let pid = pid.as_u32();
            let pid = i64::from(pid);
            pids.push(pid);
            let parent = process.parent().map(Pid::as_u32).unwrap_or_default();
            let parent = i64::from(parent);
            ppids.push(parent);
            let run_time = Duration::from_secs(process.run_time());
            let duration = run_time.as_millis();
            let duration = i64::try_from(duration).unwrap_or_default();
            start_times.push(duration);
        }
        i32::try_from(processes.len())?
    } else {
        let pid = usize::try_from(pid)?;
        let pid = Pid::from(pid);
        system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
        if let Some(process) = system.process(pid) {
            let pid = pid.as_u32();
            let pid = i64::from(pid);
            pids.push(pid);
            let parent = process.parent().map(Pid::as_u32).unwrap_or_default();
            let parent = i64::from(parent);
            ppids.push(parent);
            let run_time = Duration::from_secs(process.run_time());
            let duration = run_time.as_millis();
            let duration = i64::try_from(duration).unwrap_or_default();
            start_times.push(duration);
            1
        } else {
            -1
        }
    };

    Ok(Some(Value::Int(processes_length)))
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.initNative()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.isAlive0(J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn is_alive_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let pid = parameters.pop_long()?;
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

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.parent0(JJ)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn parent_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _start_time = parameters.pop_long()?;
    let pid = parameters.pop_long()?;
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

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.waitForProcessExit0(JZ)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn wait_for_process_exit_0(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _start_time = parameters.pop_long()?;
    let pid = parameters.pop_long()?;
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
        let result = get_current_pid_0(thread, Parameters::default()).await?;
        let pid = i64::from(process::id());
        assert_eq!(result, Some(Value::Long(pid)));
        Ok(())
    }

    #[tokio::test]
    async fn test_init_native() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_native(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_alive_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let pid = Value::Long(i64::from(process::id()));
        let result = is_alive_0(thread, Parameters::new(vec![pid])).await?;
        let run_time = result.unwrap_or(Value::Long(0)).as_i64()?;
        assert!(run_time > 0);
        Ok(())
    }
}
