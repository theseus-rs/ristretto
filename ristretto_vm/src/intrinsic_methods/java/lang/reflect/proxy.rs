use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/reflect/Proxy";

/// Register all intrinsic methods for `java.lang.reflect.Proxy`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "defineClass0",
        "(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;",
        define_class_0,
    );
}

#[async_recursion(?Send)]
async fn define_class_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.lang.reflect.Proxy.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.reflect.Proxy.defineClass0(Ljava/lang/ClassLoader;Ljava/lang/String;[BII)Ljava/lang/Class;"
    )]
    async fn test_define_class_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = define_class_0(thread, Parameters::default()).await;
    }
}
