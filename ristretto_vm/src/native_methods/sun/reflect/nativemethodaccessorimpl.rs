use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/reflect/NativeMethodAccessorImpl";

/// Register all native methods for `sun.reflect.NativeMethodAccessorImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "invoke0",
        "(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_0,
    );
}

#[async_recursion(?Send)]
async fn invoke_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.NativeMethodAccessorImpl.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.NativeMethodAccessorImpl.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;"
    )]
    async fn test_invoke_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = invoke_0(thread, Arguments::default()).await;
    }
}
