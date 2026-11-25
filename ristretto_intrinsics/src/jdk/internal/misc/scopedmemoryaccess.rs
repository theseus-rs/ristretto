use ristretto_classfile::VersionSpecification::{Equal, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::{Parameters, Result};
use ristretto_types::{Thread, VM};
use std::sync::Arc;

fn session_id(parameters: &mut Parameters) -> Result<i64> {
    let session = parameters
        .pop_reference()?
        .ok_or(JavaError::NullPointerException(None))?;
    Ok(session.as_ptr_i64())
}

fn try_close<T: Thread + 'static>(
    thread: &T,
    parameters: &mut Parameters,
) -> Result<Option<Value>> {
    let active = thread
        .vm()?
        .native_memory()
        .scope_accesses(session_id(parameters)?);
    Ok(Some(Value::from(active.is_empty())))
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.closeScope0(Ljdk/internal/misc/ScopedMemoryAccess$Scope;Ljdk/internal/misc/ScopedMemoryAccess$Scope$ScopedAccessError;)Z",
    Equal(JAVA_17)
)]
pub fn close_scope_0_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    parameters.pop_reference()?; // scoped-access error supplied by the JDK
    try_close(thread.as_ref(), &mut parameters)
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;)Z",
    Equal(JAVA_21)
)]
pub fn close_scope_0_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    try_close(thread.as_ref(), &mut parameters)
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.closeScope0(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/ScopedMemoryAccess$ScopedAccessError;)V",
    GreaterThan(JAVA_21)
)]
pub async fn close_scope_0_2<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    parameters.pop_reference()?; // scoped-access error supplied by the JDK
    let active = thread
        .vm()?
        .native_memory()
        .scope_accesses(session_id(&mut parameters)?);
    // SharedSession publishes CLOSED before this call. Later accesses fail their Java check.
    for mut frame in active {
        let _ = frame.changed().await;
    }
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/misc/ScopedMemoryAccess.registerNatives()V",
    GreaterThanOrEqual(JAVA_17)
)]
pub fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close_scope_0_0() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let session = thread.object("java/lang/Object", "", &[]).await?;
        let args = Parameters::new(vec![session, Value::Object(None)]);
        assert_eq!(close_scope_0_0(thread, args)?, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_close_scope_0_1() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let session = thread.object("java/lang/Object", "", &[]).await?;
        let id = session_id(&mut Parameters::new(vec![session.clone()]))?;
        let active = vm.native_memory().begin_scoped_access([id].into_iter());
        let args = || Parameters::new(vec![session.clone()]);
        assert_eq!(
            close_scope_0_1(thread.clone(), args())?,
            Some(Value::from(false))
        );
        drop(active);
        assert_eq!(close_scope_0_1(thread, args())?, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn close_waits_for_existing_access_to_either_session() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let session = thread.object("java/lang/Object", "", &[]).await?;
        let id = session_id(&mut Parameters::new(vec![session.clone()]))?;
        let active = vm.native_memory().begin_scoped_access([1, id].into_iter());
        let _unrelated = vm.native_memory().begin_scoped_access([2].into_iter());
        let args = Parameters::new(vec![session, Value::Object(None)]);
        let closing = close_scope_0_2(thread, args);
        tokio::pin!(closing);
        let mut context = std::task::Context::from_waker(std::task::Waker::noop());
        assert!(closing.as_mut().poll(&mut context).is_pending());
        let _later = vm.native_memory().begin_scoped_access([id].into_iter());
        drop(active);
        assert_eq!(closing.await?, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = register_natives(thread, Parameters::default())?;
        assert_eq!(value, None);
        Ok(())
    }
}
