use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Class, Value};
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `java.security.AccessController`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/security/AccessController";
    let java_version = registry.java_version();

    if java_version <= &JAVA_11 {
        registry.register(
            class_name,
            "doPrivileged",
            "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
            do_privileged_1,
        );
        registry.register(
            class_name,
            "doPrivileged",
            "(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
            do_privileged_2,
        );
        registry.register(
            class_name,
            "doPrivileged",
            "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
            do_privileged_3,
        );
        registry.register(class_name, "doPrivileged", "(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;", do_privileged_4);
    } else {
        registry.register(
            class_name,
            "ensureMaterializedForStackWalk",
            "(Ljava/lang/Object;)V",
            ensure_materialized_for_stack_walk,
        );
        registry.register(
            class_name,
            "getProtectionDomain",
            "(Ljava/lang/Class;)Ljava/security/ProtectionDomain;",
            get_protection_domain,
        );
    }

    registry.register(
        class_name,
        "getInheritedAccessControlContext",
        "()Ljava/security/AccessControlContext;",
        get_inherited_access_control_context,
    );
    registry.register(
        class_name,
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
        .execute(&class, &method, vec![Value::from(object)], true)
        .await
}

#[async_recursion(?Send)]
async fn do_privileged_2(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _context = arguments.pop_object()?;
    let object = arguments.pop_object()?;
    let class: Arc<Class> = object.class().clone();
    let method = class.try_get_method("run", "()Ljava/lang/Object;")?;
    thread
        .execute(&class, &method, vec![Value::from(object)], true)
        .await
}

#[async_recursion(?Send)]
async fn do_privileged_3(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class: Arc<Class> = object.class().clone();
    let method = class.try_get_method("run", "()Ljava/lang/Object;")?;
    thread
        .execute(&class, &method, vec![Value::from(object)], true)
        .await
}

#[async_recursion(?Send)]
async fn do_privileged_4(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _context = arguments.pop_object()?;
    let object = arguments.pop_object()?;
    let class: Arc<Class> = object.class().clone();
    let method = class.try_get_method("run", "()Ljava/lang/Object;")?;
    thread
        .execute(&class, &method, vec![Value::from(object)], true)
        .await
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn ensure_materialized_for_stack_walk(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_inherited_access_control_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_protection_domain(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_stack_access_control_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}
