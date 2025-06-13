use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z",
    Equal(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn close_scope_0_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z"
    )
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;)Z",
    Equal(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn close_scope_0_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;)Z"
    )
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/ScopedMemoryAccess$ScopedAccessError;)V",
    GreaterThan(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn close_scope_0_2(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/ScopedMemoryAccess$ScopedAccessError;)V"
    )
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.registerNatives()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z"
    )]
    async fn test_close_scope_0_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_scope_0_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;)Z"
    )]
    async fn test_close_scope_0_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_scope_0_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/ScopedMemoryAccess$ScopedAccessError;)V"
    )]
    async fn test_close_scope_0_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_scope_0_2(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = register_natives(thread, Parameters::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }
}
