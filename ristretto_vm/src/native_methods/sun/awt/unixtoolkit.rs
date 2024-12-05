use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.UNIXToolkit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/UNIXToolkit";
    registry.register(class_name, "check_gtk", "(I)Z", check_gtk);
    registry.register(class_name, "get_gtk_version", "()I", get_gtk_version);
    registry.register(
        class_name,
        "gtkCheckVersionImpl",
        "(III)Z",
        gtk_check_version_impl,
    );
    registry.register(class_name, "load_gtk", "(IZ)Z", load_gtk);
    registry.register(
        class_name,
        "load_gtk_icon",
        "(Ljava/lang/String;)Z",
        load_gtk_icon,
    );
    registry.register(
        class_name,
        "load_stock_icon",
        "(ILjava/lang/String;IILjava/lang/String;)Z",
        load_stock_icon,
    );
    registry.register(class_name, "nativeSync", "()V", native_sync);
    registry.register(class_name, "unload_gtk", "()Z", unload_gtk);
}

#[async_recursion(?Send)]
async fn check_gtk(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.check_gtk(I)Z")
}

#[async_recursion(?Send)]
async fn get_gtk_version(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.get_gtk_version()I")
}

#[async_recursion(?Send)]
async fn gtk_check_version_impl(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.gtkCheckVersionImpl(III)Z")
}

#[async_recursion(?Send)]
async fn load_gtk(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.load_gtk(IZ)Z")
}

#[async_recursion(?Send)]
async fn load_gtk_icon(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.load_gtk_icon(Ljava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn load_stock_icon(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.load_stock_icon(ILjava/lang/String;IILjava/lang/String;)Z")
}

#[async_recursion(?Send)]
async fn native_sync(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.nativeSync()V")
}

#[async_recursion(?Send)]
async fn unload_gtk(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.UNIXToolkit.unload_gtk()Z")
}
