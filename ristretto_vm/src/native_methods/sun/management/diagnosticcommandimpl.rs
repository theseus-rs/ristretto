use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/management/DiagnosticCommandImpl";

/// Register all native methods for `sun.management.DiagnosticCommandImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "executeDiagnosticCommand",
        "(Ljava/lang/String;)Ljava/lang/String;",
        execute_diagnostic_command,
    );
    registry.register(
        CLASS_NAME,
        "getDiagnosticCommandInfo",
        "([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;",
        get_diagnostic_command_info,
    );
    registry.register(
        CLASS_NAME,
        "getDiagnosticCommands",
        "()[Ljava/lang/String;",
        get_diagnostic_commands,
    );
    registry.register(
        CLASS_NAME,
        "setNotificationEnabled",
        "(Z)V",
        set_notification_enabled,
    );
}

#[async_recursion(?Send)]
async fn execute_diagnostic_command(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_diagnostic_command_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;")
}

#[async_recursion(?Send)]
async fn get_diagnostic_commands(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn set_notification_enabled(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.DiagnosticCommandImpl.setNotificationEnabled(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_execute_diagnostic_command() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = execute_diagnostic_command(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;"
    )]
    async fn test_get_diagnostic_command_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_diagnostic_command_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;"
    )]
    async fn test_get_diagnostic_commands() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_diagnostic_commands(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.management.DiagnosticCommandImpl.setNotificationEnabled(Z)V"
    )]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_notification_enabled(thread, Parameters::default()).await;
    }
}
