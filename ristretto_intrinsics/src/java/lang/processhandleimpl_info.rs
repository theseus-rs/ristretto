use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
#[cfg(not(target_family = "wasm"))]
use ristretto_classloader::Reference;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(target_family = "wasm")]
use ristretto_types::JavaError;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use sysinfo::{Pid, ProcessesToUpdate, System};

#[intrinsic_method(
    "java/lang/ProcessHandleImpl$Info.info0(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn info_0<T: Thread + 'static>(
    thread: Arc<T>,
    #[cfg_attr(target_family = "wasm", allow(unused_mut))] mut parameters: Parameters,
) -> Result<Option<Value>> {
    #[cfg(not(target_family = "wasm"))]
    {
        let pid = parameters.pop_long()?;
        let Some(info_ref) = parameters.pop_reference()? else {
            return Ok(None);
        };

        let pid_usize = usize::try_from(pid)?;
        let sys_pid = Pid::from(pid_usize);
        let mut system = System::new_all();
        system.refresh_processes(ProcessesToUpdate::Some(&[sys_pid]), true);

        if let Some(process) = system.process(sys_pid) {
            let exe_path = process.exe().map(|p| p.to_string_lossy().to_string());
            let start_time = i64::try_from(process.start_time()).unwrap_or_default() * 1000;

            let command_value = if let Some(ref exe) = exe_path {
                Some(exe.as_str().to_object(&thread).await?)
            } else {
                None
            };

            let mut guard = info_ref.write();
            if let Reference::Object(object) = &mut *guard {
                if let Some(cmd) = command_value {
                    object.set_value("command", cmd)?;
                }
                object.set_value("startTime", Value::Long(start_time))?;
                object.set_value("totalTime", Value::Long(-1))?;
            }
        }

        Ok(None)
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = (thread, parameters);
        Err(JavaError::UnsatisfiedLinkError(
            "java.lang.ProcessHandleImpl$Info.info0(J)V".to_string(),
        )
        .into())
    }
}

#[intrinsic_method(
    "java/lang/ProcessHandleImpl$Info.initIDs()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Object;

    #[tokio::test]
    async fn test_info_0_empty_params() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = info_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_info_0_null_reference() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None));
        parameters.push(Value::Long(i64::from(std::process::id())));
        let result = info_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_info_0_nonexistent_pid() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None));
        parameters.push(Value::Long(999_999_999));
        let result = info_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_info_0_with_current_pid() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.ProcessHandleImpl$Info").await?;
        let info_object = Object::new(class)?;
        let vm = thread.vm()?;
        let info_value = Value::new_object(vm.garbage_collector(), Reference::Object(info_object));

        let mut parameters = Parameters::default();
        parameters.push(info_value);
        parameters.push(Value::Long(i64::from(std::process::id())));

        let result = info_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_info_0_sets_command_field() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.ProcessHandleImpl$Info").await?;
        let info_object = Object::new(class)?;
        let vm = thread.vm()?;
        let info_value = Value::new_object(vm.garbage_collector(), Reference::Object(info_object));
        let info_clone = info_value.clone();

        let mut parameters = Parameters::default();
        parameters.push(info_value);
        parameters.push(Value::Long(i64::from(std::process::id())));

        let result = info_0(thread, parameters).await?;
        assert_eq!(result, None);

        // Verify the command field was populated
        let guard = info_clone.as_reference()?;
        if let Reference::Object(ref obj) = *guard {
            let command = obj.value("command")?;
            assert_ne!(command, Value::Object(None), "command should be set");
        } else {
            panic!("expected Reference::Object");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_info_0_sets_start_time_field() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.ProcessHandleImpl$Info").await?;
        let info_object = Object::new(class)?;
        let vm = thread.vm()?;
        let info_value = Value::new_object(vm.garbage_collector(), Reference::Object(info_object));
        let info_clone = info_value.clone();

        let mut parameters = Parameters::default();
        parameters.push(info_value);
        parameters.push(Value::Long(i64::from(std::process::id())));

        info_0(thread, parameters).await?;

        // Verify the startTime field was populated with a positive epoch millis value
        let guard = info_clone.as_reference()?;
        if let Reference::Object(ref obj) = *guard {
            let start_time = obj.value("startTime")?;
            let millis = start_time.as_i64()?;
            assert!(millis > 0, "startTime should be positive epoch millis");
        } else {
            panic!("expected Reference::Object");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_info_0_sets_total_time_field() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.ProcessHandleImpl$Info").await?;
        let info_object = Object::new(class)?;
        let vm = thread.vm()?;
        let info_value = Value::new_object(vm.garbage_collector(), Reference::Object(info_object));
        let info_clone = info_value.clone();

        let mut parameters = Parameters::default();
        parameters.push(info_value);
        parameters.push(Value::Long(i64::from(std::process::id())));

        info_0(thread, parameters).await?;

        // Verify totalTime is set to -1 (unavailable)
        let guard = info_clone.as_reference()?;
        if let Reference::Object(ref obj) = *guard {
            let total_time = obj.value("totalTime")?;
            assert_eq!(total_time.as_i64()?, -1);
        } else {
            panic!("expected Reference::Object");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
