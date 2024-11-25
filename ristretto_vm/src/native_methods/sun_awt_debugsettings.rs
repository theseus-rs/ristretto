use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.awt.DebugSettings`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/DebugSettings";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "setCTracingOn", "(Z)V", set_c_tracing_on_1);
    }

    registry.register(
        class_name,
        "setCTracingOn",
        "(ZLjava/lang/String;)V",
        set_c_tracing_on_2,
    );
    registry.register(
        class_name,
        "setCTracingOn",
        "(ZLjava/lang/String;I)V",
        set_c_tracing_on_3,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_c_tracing_on_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_c_tracing_on_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_c_tracing_on_3(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
