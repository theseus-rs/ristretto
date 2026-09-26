use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Between, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::{JavaObject, Parameters, Result};
use ristretto_types::{Thread, method_resolution};
use std::sync::Arc;

/// The legacy native entry point validates a public, concrete instance run method before
/// calling it. Use shared hierarchy resolution while preserving that JDK-specific validation.
async fn run_action<T: Thread + 'static>(thread: &T, object: Value) -> Result<Option<Value>> {
    let class = method_resolution::receiver_class(thread, &object).await?;
    let resolved = class.resolve_method("run", "()Ljava/lang/Object;")?;
    let method = &resolved.method;
    if !method.is_public() || method.is_static() || method.is_abstract() {
        let message = "No run method".to_object(thread).await?;
        let exception = thread
            .object("java/lang/InternalError", "Ljava/lang/String;", &[message])
            .await?;
        return Err(ristretto_types::Error::Throwable(exception));
    }
    thread
        .execute(&resolved.declaring_class, method, &[object])
        .await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
pub async fn do_privileged_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    run_action(thread.as_ref(), object).await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
pub async fn do_privileged_2<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop()?;
    let object = parameters.pop()?;
    run_action(thread.as_ref(), object).await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
pub async fn do_privileged_3<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    run_action(thread.as_ref(), object).await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
pub async fn do_privileged_4<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop()?;
    let object = parameters.pop()?;
    run_action(thread.as_ref(), object).await
}

#[intrinsic_method(
    "java/security/AccessController.ensureMaterializedForStackWalk(Ljava/lang/Object;)V",
    Between(JAVA_17, JAVA_21)
)]
pub fn ensure_materialized_for_stack_walk<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/security/AccessController.getInheritedAccessControlContext()Ljava/security/AccessControlContext;",
    LessThanOrEqual(JAVA_21)
)]
pub fn get_inherited_access_control_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "java/security/AccessController.getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;",
    Between(JAVA_17, JAVA_21)
)]
pub fn get_protection_domain<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "java/security/AccessController.getStackAccessControlContext()Ljava/security/AccessControlContext;",
    LessThanOrEqual(JAVA_21)
)]
pub fn get_stack_access_control_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ensure_materialized_for_stack_walk() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = ensure_materialized_for_stack_walk(thread, Parameters::default())?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_inherited_access_control_context() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = get_inherited_access_control_context(thread, Parameters::default())?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_protection_domain() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = get_protection_domain(thread, Parameters::new(vec![Value::Object(None)]))?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_stack_access_control_context() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = get_stack_access_control_context(thread, Parameters::default())?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
