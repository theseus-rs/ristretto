use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.ClassLoader`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ClassLoader";
    registry.register(
        class_name,
        "defineClass0",
        "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;)Ljava/lang/Class;",
        define_class_0,
    );
    registry.register(class_name, "defineClass1", "(Ljava/lang/String;[BIILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_1);
    registry.register(class_name, "defineClass2", "(Ljava/lang/String;Ljava/nio/ByteBuffer;IILjava/security/ProtectionDomain;Ljava/lang/String;)Ljava/lang/Class;", define_class_2);
    registry.register(
        class_name,
        "findBootstrapClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        find_bootstrap_class,
    );
    registry.register(
        class_name,
        "findBuiltinLib",
        "(Ljava/lang/String;)Ljava/lang/String;",
        find_builtin_lib,
    );
    registry.register(
        class_name,
        "findLoadedClass0",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        find_loaded_class_0,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "resolveClass0",
        "(Ljava/lang/Class;)V",
        resolve_class_0,
    );
    registry.register(
        class_name,
        "retrieveDirectives",
        "()Ljava/lang/AssertionStatusDirectives;",
        retrieve_directives,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn define_class_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn define_class_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn define_class_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_bootstrap_class(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_builtin_lib(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_loaded_class_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn resolve_class_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn retrieve_directives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
