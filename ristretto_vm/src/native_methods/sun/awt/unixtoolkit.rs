use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/UNIXToolkit";

/// Register all native methods for `sun.awt.UNIXToolkit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "check_gtk", "(I)Z", check_gtk);
    registry.register(CLASS_NAME, "get_gtk_version", "()I", get_gtk_version);
    registry.register(
        CLASS_NAME,
        "gtkCheckVersionImpl",
        "(III)Z",
        gtk_check_version_impl,
    );
    registry.register(CLASS_NAME, "load_gtk", "(IZ)Z", load_gtk);
    registry.register(
        CLASS_NAME,
        "load_gtk_icon",
        "(Ljava/lang/String;)Z",
        load_gtk_icon,
    );
    registry.register(
        CLASS_NAME,
        "load_stock_icon",
        "(ILjava/lang/String;IILjava/lang/String;)Z",
        load_stock_icon,
    );
    registry.register(CLASS_NAME, "nativeSync", "()V", native_sync);
    registry.register(CLASS_NAME, "unload_gtk", "()Z", unload_gtk);
}

#[async_recursion(?Send)]
async fn check_gtk(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.check_gtk(I)Z")
}

#[async_recursion(?Send)]
async fn get_gtk_version(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.get_gtk_version()I")
}

#[async_recursion(?Send)]
async fn gtk_check_version_impl(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.gtkCheckVersionImpl(III)Z")
}

#[async_recursion(?Send)]
async fn load_gtk(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.load_gtk(IZ)Z")
}

#[async_recursion(?Send)]
async fn load_gtk_icon(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.load_gtk_icon(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn load_stock_icon(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.load_stock_icon(ILjava/lang/String;IILjava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn native_sync(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.nativeSync()V")
}

#[async_recursion(?Send)]
async fn unload_gtk(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
