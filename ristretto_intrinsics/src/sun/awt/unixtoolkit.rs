use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/UNIXToolkit.check_gtk(I)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn check_gtk<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.awt.UNIXToolkit.check_gtk(I)Z".to_string()).into())
}

#[intrinsic_method("sun/awt/UNIXToolkit.get_gtk_version()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn get_gtk_version<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.UNIXToolkit.get_gtk_version()I".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/awt/UNIXToolkit.gtkCheckVersionImpl(III)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn gtk_check_version_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.UNIXToolkit.gtkCheckVersionImpl(III)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/UNIXToolkit.load_gtk(IZ)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn load_gtk<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.awt.UNIXToolkit.load_gtk(IZ)Z".to_string()).into())
}

#[intrinsic_method(
    "sun/awt/UNIXToolkit.load_gtk_icon(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn load_gtk_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.UNIXToolkit.load_gtk_icon(Ljava/lang/String;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/UNIXToolkit.load_stock_icon(ILjava/lang/String;IILjava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn load_stock_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.UNIXToolkit.load_stock_icon(ILjava/lang/String;IILjava/lang/String;)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/UNIXToolkit.nativeSync()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn native_sync<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.awt.UNIXToolkit.nativeSync()V".to_string()).into())
}

#[intrinsic_method("sun/awt/UNIXToolkit.unload_gtk()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn unload_gtk<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.awt.UNIXToolkit.unload_gtk()Z".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_gtk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_gtk(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_gtk_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_gtk_version(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_gtk_check_version_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = gtk_check_version_impl(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_gtk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_gtk(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_gtk_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_gtk_icon(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_stock_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_stock_icon(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_sync() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_sync(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unload_gtk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unload_gtk(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
