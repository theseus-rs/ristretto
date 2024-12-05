use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.management.internal.DiagnosticCommandImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/management/internal/DiagnosticCommandImpl";
    registry.register(
        class_name,
        "executeDiagnosticCommand",
        "(Ljava/lang/String;)Ljava/lang/String;",
        execute_diagnostic_command,
    );
    registry.register(
        class_name,
        "getDiagnosticCommandInfo",
        "([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;",
        get_diagnostic_command_info,
    );
    registry.register(
        class_name,
        "getDiagnosticCommands",
        "()[Ljava/lang/String;",
        get_diagnostic_commands,
    );
    registry.register(
        class_name,
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
