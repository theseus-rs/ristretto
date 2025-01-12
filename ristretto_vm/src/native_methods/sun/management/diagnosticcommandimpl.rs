use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.management.DiagnosticCommandImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/management/DiagnosticCommandImpl";
    registry.register(
        class_name,
        "executeDiagnosticCommand",
        "(Ljava/lang/String;)Ljava/lang/String;",
        execute_diagnostic_command,
    );
    registry.register(
        class_name,
        "getDiagnosticCommandInfo",
        "([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;",
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
    todo!("sun.management.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_diagnostic_command_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;")
}

#[async_recursion(?Send)]
async fn get_diagnostic_commands(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn set_notification_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.management.DiagnosticCommandImpl.setNotificationEnabled(Z)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/management/DiagnosticCommandImpl";
        assert!(registry
            .method(
                class_name,
                "executeDiagnosticCommand",
                "(Ljava/lang/String;)Ljava/lang/String;"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "getDiagnosticCommandInfo",
                "([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "getDiagnosticCommands", "()[Ljava/lang/String;")
            .is_some());
        assert!(registry
            .method(class_name, "setNotificationEnabled", "(Z)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;"
    )]
    async fn test_execute_diagnostic_command() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = execute_diagnostic_command(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;"
    )]
    async fn test_get_diagnostic_command_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_diagnostic_command_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.management.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;"
    )]
    async fn test_get_diagnostic_commands() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_diagnostic_commands(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.management.DiagnosticCommandImpl.setNotificationEnabled(Z)V")]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_notification_enabled(thread, Arguments::default()).await;
    }
}
