use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "com/sun/management/internal/DiagnosticCommandImpl";

/// Register all native methods for `com.sun.management.internal.DiagnosticCommandImpl`.
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
        "([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;",
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
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_diagnostic_command_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;")
}

#[async_recursion(?Send)]
async fn get_diagnostic_commands(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn set_notification_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.management.internal.DiagnosticCommandImpl.setNotificationEnabled(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_execute_diagnostic_command() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = execute_diagnostic_command(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;"
    )]
    async fn test_get_diagnostic_command_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_diagnostic_command_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;"
    )]
    async fn test_get_diagnostic_commands() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_diagnostic_commands(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.DiagnosticCommandImpl.setNotificationEnabled(Z)V"
    )]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_notification_enabled(thread, Arguments::default()).await;
    }
}
