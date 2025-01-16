use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{Class, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "java/security/AccessController";

/// Register all native methods for `java.security.AccessController`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "doPrivileged",
            "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
            do_privileged_1,
        );
        registry.register(
            CLASS_NAME,
            "doPrivileged",
            "(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
            do_privileged_2,
        );
        registry.register(
            CLASS_NAME,
            "doPrivileged",
            "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
            do_privileged_3,
        );
        registry.register(CLASS_NAME, "doPrivileged", "(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;", do_privileged_4);
    } else {
        registry.register(
            CLASS_NAME,
            "ensureMaterializedForStackWalk",
            "(Ljava/lang/Object;)V",
            ensure_materialized_for_stack_walk,
        );
        registry.register(
            CLASS_NAME,
            "getProtectionDomain",
            "(Ljava/lang/Class;)Ljava/security/ProtectionDomain;",
            get_protection_domain,
        );
    }

    registry.register(
        CLASS_NAME,
        "getInheritedAccessControlContext",
        "()Ljava/security/AccessControlContext;",
        get_inherited_access_control_context,
    );
    registry.register(
        CLASS_NAME,
        "getStackAccessControlContext",
        "()Ljava/security/AccessControlContext;",
        get_stack_access_control_context,
    );
}

#[async_recursion(?Send)]
async fn do_privileged_1(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class: Arc<Class> = object.class().clone();
    let method = class.try_get_method("run", "()Ljava/lang/Object;")?;
    thread
        .execute(&class, &method, vec![Value::from(object)])
        .await
}

#[async_recursion(?Send)]
async fn do_privileged_2(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _context = arguments.pop_object()?;
    let object = arguments.pop_object()?;
    let class: Arc<Class> = object.class().clone();
    let method = class.try_get_method("run", "()Ljava/lang/Object;")?;
    thread
        .execute(&class, &method, vec![Value::from(object)])
        .await
}

#[async_recursion(?Send)]
async fn do_privileged_3(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class: Arc<Class> = object.class().clone();
    let method = class.try_get_method("run", "()Ljava/lang/Object;")?;
    thread
        .execute(&class, &method, vec![Value::from(object)])
        .await
}

#[async_recursion(?Send)]
async fn do_privileged_4(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _context = arguments.pop_object()?;
    let object = arguments.pop_object()?;
    let class: Arc<Class> = object.class().clone();
    let method = class.try_get_method("run", "()Ljava/lang/Object;")?;
    thread
        .execute(&class, &method, vec![Value::from(object)])
        .await
}

#[async_recursion(?Send)]
async fn ensure_materialized_for_stack_walk(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn get_inherited_access_control_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.security.AccessController.getInheritedAccessControlContext()Ljava/security/AccessControlContext;")
}

#[async_recursion(?Send)]
async fn get_protection_domain(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.security.AccessController.getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;")
}

#[async_recursion(?Send)]
async fn get_stack_access_control_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ensure_materialized_for_stack_walk() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = ensure_materialized_for_stack_walk(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.security.AccessController.getInheritedAccessControlContext()Ljava/security/AccessControlContext;"
    )]
    async fn test_get_inherited_access_control_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_inherited_access_control_context(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.security.AccessController.getProtectionDomain(Ljava/lang/Class;)Ljava/security/ProtectionDomain;"
    )]
    async fn test_get_protection_domain() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_protection_domain(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_get_stack_access_control_context() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_stack_access_control_context(thread, Arguments::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }
}
