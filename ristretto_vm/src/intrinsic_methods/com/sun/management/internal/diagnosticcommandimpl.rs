use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/management/internal/DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn execute_diagnostic_command(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.management.internal.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;"
    )
}

#[intrinsic_method(
    "com/sun/management/internal/DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_diagnostic_command_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;"
    )
}

#[intrinsic_method(
    "com/sun/management/internal/DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_diagnostic_commands(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;"
    )
}

#[intrinsic_method(
    "com/sun/management/internal/DiagnosticCommandImpl.setNotificationEnabled(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn set_notification_enabled(
    _thread: Arc<Thread>,
    _parameters: Parameters,
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
        let _ = execute_diagnostic_command(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;"
    )]
    async fn test_get_diagnostic_command_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_diagnostic_command_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;"
    )]
    async fn test_get_diagnostic_commands() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_diagnostic_commands(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.management.internal.DiagnosticCommandImpl.setNotificationEnabled(Z)V"
    )]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_notification_enabled(thread, Parameters::default()).await;
    }
}
