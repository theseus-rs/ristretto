use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_20: Version = Version::Java20 { minor: 0 };

/// Register all native methods for `java.io.Console`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/Console";
    let java_version = registry.java_version();

    if java_version <= &JAVA_20 {
        registry.register(class_name, "echo", "(Z)Z", echo);
    }

    registry.register(class_name, "encoding", "()Ljava/lang/String;", encoding);
    registry.register(class_name, "istty", "()Z", istty);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn echo(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn encoding(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn istty(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
