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
    "sun/security/pkcs11/Secmod.nssGetLibraryHandle(Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn nss_get_library_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_lib_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.Secmod.nssGetLibraryHandle(Ljava/lang/String;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/Secmod.nssGetModuleList(JLjava/lang/String;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn nss_get_module_list<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_lib_dir = parameters.pop_reference()?;
    let _j_handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.Secmod.nssGetModuleList(JLjava/lang/String;)Ljava/lang/Object;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/Secmod.nssInitialize(Ljava/lang/String;JLjava/lang/String;Z)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn nss_initialize<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_nss_optimize_space = parameters.pop_bool()?;
    let _j_config_dir = parameters.pop_reference()?;
    let _j_handle = parameters.pop_long()?;
    let _j_function_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.Secmod.nssInitialize(Ljava/lang/String;JLjava/lang/String;Z)Z"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/Secmod.nssLoadLibrary(Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn nss_load_library<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_name = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.Secmod.nssLoadLibrary(Ljava/lang/String;)J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/security/pkcs11/Secmod.nssVersionCheck(JLjava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn nss_version_check<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_version = parameters.pop_reference()?;
    let _j_handle = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.pkcs11.Secmod.nssVersionCheck(JLjava/lang/String;)Z".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nss_get_library_handle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            nss_get_library_handle(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.security.pkcs11.Secmod.nssGetLibraryHandle(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_nss_get_module_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = nss_get_module_list(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.Secmod.nssGetModuleList(JLjava/lang/String;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_nss_initialize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = nss_initialize(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.Secmod.nssInitialize(Ljava/lang/String;JLjava/lang/String;Z)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_nss_load_library() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = nss_load_library(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.security.pkcs11.Secmod.nssLoadLibrary(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_nss_version_check() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = nss_version_check(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.security.pkcs11.Secmod.nssVersionCheck(JLjava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }
}
