use crate::native_methods::registry::{MethodRegistry, JAVA_18, JAVA_21};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/misc/ScopedMemoryAccess";

/// Register all native methods for `jdk.internal.misc.ScopedMemoryAccess`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_18 {
        registry.register(CLASS_NAME, "closeScope0", "(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z", close_scope_0);
    } else if registry.java_major_version() <= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "closeScope0",
            "(Ljdk/internal/foreign/MemorySessionImpl;)Z",
            close_scope_0,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "closeScope0",
            "(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/ScopedMemoryAccess$ScopedAccessError;)V",
            close_scope_0,
        );
    }

    registry.register(CLASS_NAME, "registerNatives", "()V", register_natives);
}

#[async_recursion(?Send)]
async fn close_scope_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z"
    )]
    async fn test_close_scope_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_scope_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = register_natives(thread, Parameters::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }
}
