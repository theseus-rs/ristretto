use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_20: Version = Version::Java20 { minor: 0 };

/// Register all native methods for `jdk.internal.vm.Continuation`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/vm/Continuation";
    let java_version = registry.java_version();

    if java_version >= &JAVA_20 {
        registry.register(class_name, "doYield", "()I", do_yield);
    }

    registry.register(
        class_name,
        "enterSpecial",
        "(Ljdk/internal/vm/Continuation;ZZ)V",
        enter_special,
    );
    registry.register(
        class_name,
        "isPinned0",
        "(Ljdk/internal/vm/ContinuationScope;)I",
        is_pinned_0,
    );
    registry.register(class_name, "pin", "()V", pin);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(class_name, "unpin", "()V", unpin);
}

#[async_recursion(?Send)]
async fn do_yield(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn enter_special(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_pinned_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn pin(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn unpin(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
