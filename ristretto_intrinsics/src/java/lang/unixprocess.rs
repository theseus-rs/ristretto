use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, Thread};
use std::sync::Arc;
use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

#[intrinsic_method("java/lang/UNIXProcess.destroyProcess(IZ)V", Equal(JAVA_8))]
#[async_method]
pub async fn destroy_process<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let force = parameters.pop_bool()?;
    let pid = parameters.pop_int()?;
    let pid = usize::try_from(pid)?;
    let pid = Pid::from(pid);
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    if let Some(process) = system.process(pid) {
        let signal = if force { Signal::Kill } else { Signal::Term };
        process.kill_with(signal);
    }

    Ok(None)
}

/// Delegates to `ProcessImpl.forkAndExec` which contains the primary implementation.
/// In Java 8 and earlier, `UNIXProcess` was the process class on Unix platforms; both share
/// the same parameter layout.
#[intrinsic_method("java/lang/UNIXProcess.forkAndExec(I[B[B[BI[BI[B[IZ)I", Equal(JAVA_8))]
#[async_method]
pub async fn fork_and_exec<T: Thread + 'static>(
    thread: Arc<T>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    super::processimpl::fork_and_exec(thread, parameters).await
}

#[intrinsic_method("java/lang/UNIXProcess.init()V", Equal(JAVA_8))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/lang/UNIXProcess.waitForProcessExit(I)I", Equal(JAVA_8))]
#[async_method]
pub async fn wait_for_process_exit<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let pid = parameters.pop_int()?;
    let pid = usize::try_from(pid)?;
    let pid = Pid::from(pid);
    let mut system = System::new_all();
    system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);

    let Some(process) = system.process(pid) else {
        return Ok(Some(Value::Int(-1)));
    };

    process.wait();

    Ok(Some(Value::Int(0)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;
    use ristretto_gc::GarbageCollector;

    #[tokio::test]
    async fn test_destroy_process_nonexistent() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_int(999_999);
        parameters.push_bool(false);
        let result = destroy_process(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_destroy_process_force() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_int(999_999);
        parameters.push_bool(true);
        let result = destroy_process(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_fork_and_exec_empty_params() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = fork_and_exec(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fork_and_exec() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let gc = GarbageCollector::new();

        let (file_bytes, arg_bytes, argc) = if cfg!(target_os = "windows") {
            #[expect(clippy::cast_possible_wrap)]
            let file: Vec<i8> = b"cmd.exe\0".iter().map(|&b| b as i8).collect();
            #[expect(clippy::cast_possible_wrap)]
            let args: Vec<i8> = b"cmd.exe\0/C\0echo\0hello\0"
                .iter()
                .map(|&b| b as i8)
                .collect();
            (file, args, 4)
        } else {
            #[expect(clippy::cast_possible_wrap)]
            let file: Vec<i8> = b"/bin/echo\0".iter().map(|&b| b as i8).collect();
            #[expect(clippy::cast_possible_wrap)]
            let args: Vec<i8> = b"/bin/echo\0hello\0".iter().map(|&b| b as i8).collect();
            (file, args, 2)
        };

        let file_ref = Value::new_object(&gc, Reference::from(file_bytes));
        let arg_ref = Value::new_object(&gc, Reference::from(arg_bytes));
        let fds = Value::new_object(&gc, Reference::from(vec![0i32; 3]));

        let mut parameters = Parameters::default();
        parameters.push_int(0); // mode
        parameters.push(Value::Object(None)); // helperMethod
        parameters.push(file_ref); // file
        parameters.push(arg_ref); // argBlock
        parameters.push_int(argc); // argc
        parameters.push(Value::Object(None)); // envBlock
        parameters.push_int(0); // envc
        parameters.push(Value::Object(None)); // dir
        parameters.push(fds); // fds
        parameters.push_bool(false); // redirectErrorStream

        let result = fork_and_exec(thread, parameters).await?;
        let pid = result.expect("expected pid").as_i32()?;
        assert!(pid > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_for_process_exit_nonexistent() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let mut parameters = Parameters::default();
        parameters.push_int(999_999);
        let result = wait_for_process_exit(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(-1)));
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_for_process_exit_completed_process() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");

        let noop_command = if cfg!(target_os = "windows") {
            "cmd.exe"
        } else {
            "/usr/bin/true"
        };

        let child = tokio::process::Command::new(noop_command)
            .args(if cfg!(target_os = "windows") {
                vec!["/C", "exit", "0"]
            } else {
                vec![]
            })
            .spawn()
            .expect("failed to spawn noop command");
        let child_pid = child.id().expect("expected child pid");
        let child_pid = i32::try_from(child_pid)?;

        let mut parameters = Parameters::default();
        parameters.push_int(child_pid);
        let result = wait_for_process_exit(thread, parameters).await?;
        assert!(result == Some(Value::Int(0)) || result == Some(Value::Int(-1)));
        Ok(())
    }
}
