use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/awt/UNIXToolkit.check_gtk(I)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn check_gtk(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.check_gtk(I)Z")
}

#[intrinsic_method("sun/awt/UNIXToolkit.get_gtk_version()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn get_gtk_version(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.get_gtk_version()I")
}

#[intrinsic_method(
    "sun/awt/UNIXToolkit.gtkCheckVersionImpl(III)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn gtk_check_version_impl(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.gtkCheckVersionImpl(III)Z")
}

#[intrinsic_method("sun/awt/UNIXToolkit.load_gtk(IZ)Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn load_gtk(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.load_gtk(IZ)Z")
}

#[intrinsic_method(
    "sun/awt/UNIXToolkit.load_gtk_icon(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn load_gtk_icon(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.load_gtk_icon(Ljava/lang/String;)Z")
}

#[intrinsic_method(
    "sun/awt/UNIXToolkit.load_stock_icon(ILjava/lang/String;IILjava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn load_stock_icon(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.load_stock_icon(ILjava/lang/String;IILjava/lang/String;)Z")
}

#[intrinsic_method("sun/awt/UNIXToolkit.nativeSync()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn native_sync(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.nativeSync()V")
}

#[intrinsic_method("sun/awt/UNIXToolkit.unload_gtk()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn unload_gtk(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.unload_gtk()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.UNIXToolkit.check_gtk(I)Z")]
    async fn test_check_gtk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_gtk(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.UNIXToolkit.get_gtk_version()I")]
    async fn test_get_gtk_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_gtk_version(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.UNIXToolkit.gtkCheckVersionImpl(III)Z")]
    async fn test_gtk_check_version_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = gtk_check_version_impl(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.UNIXToolkit.load_gtk(IZ)Z")]
    async fn test_load_gtk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_gtk(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.UNIXToolkit.load_gtk_icon(Ljava/lang/String;)Z"
    )]
    async fn test_load_gtk_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_gtk_icon(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.UNIXToolkit.load_stock_icon(ILjava/lang/String;IILjava/lang/String;)Z"
    )]
    async fn test_load_stock_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_stock_icon(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.UNIXToolkit.nativeSync()V")]
    async fn test_native_sync() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_sync(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.UNIXToolkit.unload_gtk()Z")]
    async fn test_unload_gtk() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = unload_gtk(thread, Parameters::default()).await;
    }
}
