use ristretto_classfile::VersionSpecification::{Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn close_scope_0_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;)Z",
    Equal(JAVA_21)
)]
#[async_method]
pub async fn close_scope_0_1<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;)Z".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/ScopedMemoryAccess$ScopedAccessError;)V",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn close_scope_0_2<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.misc.ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/ScopedMemoryAccess$ScopedAccessError;)V".to_string()).into())
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.registerNatives()V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close_scope_0_0() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = close_scope_0_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_close_scope_0_1() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = close_scope_0_1(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_close_scope_0_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_scope_0_2(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = register_natives(thread, Parameters::default()).await?;
        assert_eq!(value, None);
        Ok(())
    }
}
