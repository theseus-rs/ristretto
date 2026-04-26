use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/jpackage/internal/WindowsRegistry.closeRegistryKey(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn close_registry_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _l_key = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/WindowsRegistry.closeRegistryKey(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/jpackage/internal/WindowsRegistry.comparePaths(Ljava/lang/String;Ljava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn compare_paths<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_path2 = parameters.pop_reference()?;
    let _j_path1 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/WindowsRegistry.comparePaths(Ljava/lang/String;Ljava/lang/String;)Z"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/jpackage/internal/WindowsRegistry.enumRegistryValue(JI)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn enum_registry_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_index = parameters.pop_int()?;
    let _l_key = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/WindowsRegistry.enumRegistryValue(JI)Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/jpackage/internal/WindowsRegistry.openRegistryKey(ILjava/lang/String;)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn open_registry_key<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_sub_key = parameters.pop_reference()?;
    let _key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/WindowsRegistry.openRegistryKey(ILjava/lang/String;)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/jpackage/internal/WindowsRegistry.readDwordValue(ILjava/lang/String;Ljava/lang/String;I)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn read_dword_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _default_value = parameters.pop_int()?;
    let _j_value = parameters.pop_reference()?;
    let _j_sub_key = parameters.pop_reference()?;
    let _key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError("jdk/jpackage/internal/WindowsRegistry.readDwordValue(ILjava/lang/String;Ljava/lang/String;I)I".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_close_registry_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_registry_key(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "jdk/jpackage/internal/WindowsRegistry.closeRegistryKey(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_compare_paths() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = compare_paths(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk/jpackage/internal/WindowsRegistry.comparePaths(Ljava/lang/String;Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enum_registry_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            enum_registry_value(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "jdk/jpackage/internal/WindowsRegistry.enumRegistryValue(JI)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_open_registry_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_registry_key(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk/jpackage/internal/WindowsRegistry.openRegistryKey(ILjava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_read_dword_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = read_dword_value(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "jdk/jpackage/internal/WindowsRegistry.readDwordValue(ILjava/lang/String;Ljava/lang/String;I)I",
            result.unwrap_err().to_string()
        );
    }
}
