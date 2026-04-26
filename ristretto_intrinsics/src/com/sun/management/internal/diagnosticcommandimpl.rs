use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/management/internal/DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn execute_diagnostic_command<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _command = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.management.internal.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;".to_string()).into())
}

#[intrinsic_method(
    "com/sun/management/internal/DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_diagnostic_command_info<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _commands = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;".to_string()).into())
}

#[intrinsic_method(
    "com/sun/management/internal/DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_diagnostic_commands<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;".to_string()).into())
}

#[intrinsic_method(
    "com/sun/management/internal/DiagnosticCommandImpl.setNotificationEnabled(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_notification_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enabled = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.management.internal.DiagnosticCommandImpl.setNotificationEnabled(Z)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_diagnostic_command() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            execute_diagnostic_command(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.sun.management.internal.DiagnosticCommandImpl.executeDiagnosticCommand(Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_diagnostic_command_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_diagnostic_command_info(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommandInfo([Ljava/lang/String;)[Lcom/sun/management/internal/DiagnosticCommandInfo;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_diagnostic_commands() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_diagnostic_commands(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.management.internal.DiagnosticCommandImpl.getDiagnosticCommands()[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_notification_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_notification_enabled(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "com.sun.management.internal.DiagnosticCommandImpl.setNotificationEnabled(Z)V",
            result.unwrap_err().to_string()
        );
    }
}
