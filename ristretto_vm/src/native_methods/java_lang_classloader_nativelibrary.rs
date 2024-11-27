use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.lang.ClassLoader$NativeLibrary`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ClassLoader$NativeLibrary";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "find", "(Ljava/lang/String;)J", find);
        registry.register(class_name, "load", "(Ljava/lang/String;Z)V", load);
    } else {
        registry.register(class_name, "findEntry", "(Ljava/lang/String;)J", find_entry);
        registry.register(class_name, "load0", "(Ljava/lang/String;ZZ)Z", load_0);
    }

    registry.register(class_name, "unload", "(Ljava/lang/String;ZJ)V", unload);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find_entry(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unload(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
