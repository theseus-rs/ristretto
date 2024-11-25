use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `java.lang.StackStreamFactory$AbstractStackWalker`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StackStreamFactory$AbstractStackWalker";
    let java_version = registry.java_version();

    if java_version <= &JAVA_18 {
        registry.register(
            class_name,
            "callStackWalk",
            "(JIII[Ljava/lang/Object;)Ljava/lang/Object;",
            call_stack_walk,
        );
    } else {
        registry.register(class_name, "callStackWalk", "(JILjdk/internal/vm/ContinuationScope;Ljdk/internal/vm/Continuation;II[Ljava/lang/Object;)Ljava/lang/Object;", call_stack_walk);
        registry.register(
            class_name,
            "setContinuation",
            "(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V",
            set_continuation,
        );
    }

    registry.register(
        class_name,
        "fetchStackFrames",
        "(JJII[Ljava/lang/Object;)I",
        fetch_stack_frames,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn call_stack_walk(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn fetch_stack_frames(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_continuation(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
