use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn execute_diagnostic_command<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;"
    )
}

#[intrinsic_method(
    "sun/management/DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_diagnostic_command_info<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.management.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;"
    )
}

#[intrinsic_method(
    "sun/management/DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_diagnostic_commands<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.management.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/management/DiagnosticCommandImpl.setNotificationEnabled(Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_notification_enabled<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
