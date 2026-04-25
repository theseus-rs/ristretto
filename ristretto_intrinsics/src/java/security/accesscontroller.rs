use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Between, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn do_privileged_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class_name = {
        let object = object.as_object_ref()?;
        object.class().name().to_string()
    };
    thread
        .invoke(&class_name, "run()Ljava/lang/Object;", &[object])
        .await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn do_privileged_2<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop()?;
    let object = parameters.pop()?;
    let class_name = {
        let object = object.as_object_ref()?;
        object.class().name().to_string()
    };
    thread
        .invoke(&class_name, "run()Ljava/lang/Object;", &[object])
        .await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn do_privileged_3<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class_name = {
        let object = object.as_object_ref()?;
        object.class().name().to_string()
    };
    thread
        .invoke(&class_name, "run()Ljava/lang/Object;", &[object])
        .await
}

#[intrinsic_method(
    "java/security/AccessController.doPrivileged(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn do_privileged_4<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _context = parameters.pop()?;
    let object = parameters.pop()?;
    let class_name = {
        let object = object.as_object_ref()?;
        object.class().name().to_string()
    };
    thread
        .invoke(&class_name, "run()Ljava/lang/Object;", &[object])
        .await
}

#[intrinsic_method(
    "java/security/AccessController.ensureMaterializedForStackWalk(Ljava/lang/Object;)V",
    Between(JAVA_17, JAVA_21)
)]
#[async_method]
pub async fn ensure_materialized_for_stack_walk<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/security/AccessController.getInheritedAccessControlContext()Ljava/security/AccessControlContext;",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_inherited_access_control_context<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.security.AccessController.getInheritedAccessControlContext()Ljava/security/AccessControlContext;".to_string()).into())
}

#[intrinsic_method(
    "java/security/AccessController.getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;",
    Between(JAVA_17, JAVA_21)
)]
#[async_method]
pub async fn get_protection_domain<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java.security.AccessController.getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;".to_string()).into())
}

#[intrinsic_method(
    "java/security/AccessController.getStackAccessControlContext()Ljava/security/AccessControlContext;",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_stack_access_control_context<T: Thread + 'static>(
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
        let result = ensure_materialized_for_stack_walk(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_inherited_access_control_context() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = get_inherited_access_control_context(thread, Parameters::default()).await;
        assert_eq!(
            "java.security.AccessController.getInheritedAccessControlContext()Ljava/security/AccessControlContext;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_protection_domain() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result =
            get_protection_domain(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java.security.AccessController.getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_stack_access_control_context() -> Result<()> {
        let (_vm, thread) = crate::test::java21_thread().await?;
        let result = get_stack_access_control_context(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
