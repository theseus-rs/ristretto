use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
#[cfg(not(target_family = "wasm"))]
use ristretto_classloader::Reference;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(target_family = "wasm")]
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
#[cfg(not(target_family = "wasm"))]
use std::process;
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.destroy0(JJZ)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn destroy_0<T: Thread + 'static>(
    _thread: Arc<T>,
    #[cfg_attr(target_family = "wasm", expect(unused_mut))] mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
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
    #[cfg(target_family = "wasm")]
    {
        let _ = parameters;
        Err(JavaError::UnsatisfiedLinkError(
            "java.lang.ProcessHandleImpl.destroy0(JJZ)Z".to_string(),
        )
        .into())
    }
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.getCurrentPid0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_current_pid_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let pid = i64::from(process::id());
        Ok(Some(Value::Long(pid)))
    }
    #[cfg(target_family = "wasm")]
    {
        Err(JavaError::UnsatisfiedLinkError(
            "java.lang.ProcessHandleImpl.getCurrentPid0()J".to_string(),
        )
        .into())
    }
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.getProcessPids0(J[J[J[J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[cfg_attr(not(target_family = "wasm"), expect(clippy::similar_names))]
#[async_method]
pub async fn get_process_pids_0<T: Thread + 'static>(
    _thread: Arc<T>,
    #[cfg_attr(target_family = "wasm", expect(unused_mut))] mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
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
                if index >= pids.len() {
                    break;
                }

                let pid = pid.as_u32();
                let pid = i64::from(pid);
                pids[index] = pid;
                let parent = process.parent().map(Pid::as_u32).unwrap_or_default();
                let parent = i64::from(parent);
                ppids[index] = parent;
                let start_time = i64::try_from(process.start_time()).unwrap_or_default() * 1000;
                start_times[index] = start_time;
            }
            i32::try_from(processes.len())?
        } else {
            let pid = usize::try_from(pid)?;
            let pid = Pid::from(pid);
            system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
            if let Some(process) = system.process(pid) {
                if !pids.is_empty() {
                    let pid = pid.as_u32();
                    let pid = i64::from(pid);
                    pids[0] = pid;
                    let parent = process.parent().map(Pid::as_u32).unwrap_or_default();
                    let parent = i64::from(parent);
                    ppids[0] = parent;
                    let start_time = i64::try_from(process.start_time()).unwrap_or_default() * 1000;
                    start_times[0] = start_time;
                }
                1
            } else {
                -1
            }
        };

        Ok(Some(Value::Int(processes_length)))
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = parameters;
        Err(JavaError::UnsatisfiedLinkError(
            "java.lang.ProcessHandleImpl.getProcessPids0(J[J[J[J)I".to_string(),
        )
        .into())
    }
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.initNative()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.isAlive0(J)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_alive_0<T: Thread + 'static>(
    _thread: Arc<T>,
    #[cfg_attr(target_family = "wasm", expect(unused_mut))] mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let pid = parameters.pop_long()?;
        let pid = usize::try_from(pid)?;
        let pid = Pid::from(pid);
        let mut system = System::new_all();
        system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

        let start_time = if let Some(process) = system.process(pid) {
            i64::try_from(process.start_time()).unwrap_or_default() * 1000
        } else {
            -1
        };

        Ok(Some(Value::Long(start_time)))
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = parameters;
        Err(
            JavaError::UnsatisfiedLinkError("java.lang.ProcessHandleImpl.isAlive0(J)J".to_string())
                .into(),
        )
    }
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.parent0(JJ)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn parent_0<T: Thread + 'static>(
    _thread: Arc<T>,
    #[cfg_attr(target_family = "wasm", expect(unused_mut))] mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
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
    #[cfg(target_family = "wasm")]
    {
        let _ = parameters;
        Err(
            JavaError::UnsatisfiedLinkError("java.lang.ProcessHandleImpl.parent0(JJ)J".to_string())
                .into(),
        )
    }
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl.waitForProcessExit0(JZ)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn wait_for_process_exit_0<T: Thread + 'static>(
    _thread: Arc<T>,
    #[cfg_attr(target_family = "wasm", expect(unused_mut))] mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let _reap_status = parameters.pop_bool()?;
        let pid = parameters.pop_long()?;
        let pid_usize = usize::try_from(pid)?;

        let exit_code = tokio::task::spawn_blocking(move || {
            let sys_pid = Pid::from(pid_usize);
            let mut system = System::new_all();
            system.refresh_processes(ProcessesToUpdate::Some(&[sys_pid]), true);

            let Some(process) = system.process(sys_pid) else {
                return 0;
            };

            process.wait();
            0
        })
        .await
        .map_err(|e| ristretto_types::Error::InternalError(e.to_string()))?;

        Ok(Some(Value::Int(exit_code)))
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = parameters;
        Err(JavaError::UnsatisfiedLinkError(
            "java.lang.ProcessHandleImpl.waitForProcessExit0(JZ)I".to_string(),
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_family = "wasm"))]
    use std::process;

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
        let start_time = result.unwrap_or(Value::Long(0)).as_i64()?;
        assert!(start_time > 0);
        Ok(())
    }
}
