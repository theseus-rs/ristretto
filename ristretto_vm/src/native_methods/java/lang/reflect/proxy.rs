use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.reflect.Proxy`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/reflect/Proxy";
    registry.register(
        class_name,
        "defineClass0",
        "(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;",
        define_class_0,
    );
}

#[async_recursion(?Send)]
async fn define_class_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.reflect.Proxy.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;")
}
