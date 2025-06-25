use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Between, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn do_privileged_1(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop_object()?;
    let class_name = object.class().name().to_string();
    thread
        .invoke(
            &class_name,
            "run()Ljava/lang/Object;",
            &[Value::from(object)],
        )
        .await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn do_privileged_2(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop_object()?;
    let object = parameters.pop_object()?;
    let class_name = object.class().name().to_string();
    thread
        .invoke(
            &class_name,
            "run()Ljava/lang/Object;",
            &[Value::from(object)],
        )
        .await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn do_privileged_3(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop_object()?;
    let class_name = object.class().name().to_string();
    thread
        .invoke(
            &class_name,
            "run()Ljava/lang/Object;",
            &[Value::from(object)],
        )
        .await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn do_privileged_4(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop_object()?;
    let object = parameters.pop_object()?;
    let class_name = object.class().name().to_string();
    thread
        .invoke(
            &class_name,
            "run()Ljava/lang/Object;",
            &[Value::from(object)],
        )
        .await
}

#[intrinsic_method(
    "java/security/AccessController.ensureMaterializedForStackWalk(Ljava/lang/Object;)V",
    Between(JAVA_11, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ensure_materialized_for_stack_walk(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/security/AccessController.getInheritedAccessControlContext()Ljava/security/AccessControlContext;",
    LessThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_inherited_access_control_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.security.AccessController.getInheritedAccessControlContext()Ljava/security/AccessControlContext;"
    )
}

#[intrinsic_method(
    "java/security/AccessController.getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;",
    Between(JAVA_11, JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_protection_domain(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.security.AccessController.getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;"
    )
}

#[intrinsic_method(
    "java/security/AccessController.getStackAccessControlContext()Ljava/security/AccessControlContext;",
    LessThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_stack_access_control_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ensure_materialized_for_stack_walk() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = ensure_materialized_for_stack_walk(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.security.AccessController.getInheritedAccessControlContext()Ljava/security/AccessControlContext;"
    )]
    async fn test_get_inherited_access_control_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_inherited_access_control_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.security.AccessController.getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;"
    )]
    async fn test_get_protection_domain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_protection_domain(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_stack_access_control_context() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_stack_access_control_context(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
