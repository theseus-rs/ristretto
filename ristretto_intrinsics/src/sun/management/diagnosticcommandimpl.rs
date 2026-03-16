use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn execute_diagnostic_command<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.management.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;".to_string()).into())
}

#[intrinsic_method(
    "sun/management/DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_diagnostic_command_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.management.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lsun/management/DiagnosticCommandInfo;".to_string()).into())
}

#[intrinsic_method(
    "sun/management/DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_diagnostic_commands<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/management/DiagnosticCommandImpl.setNotificationEnabled(Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_notification_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.DiagnosticCommandImpl.setNotificationEnabled(Z)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_diagnostic_command() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = execute_diagnostic_command(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_diagnostic_command_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_diagnostic_command_info(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_diagnostic_commands() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_diagnostic_commands(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_notification_enabled(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
