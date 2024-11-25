use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.System`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/System";
    registry.register(
        class_name,
        "arraycopy",
        "(Ljava/lang/Object;ILjava/lang/Object;II)V",
        arraycopy,
    );
    registry.register(class_name, "currentTimeMillis", "()J", current_time_millis);
    registry.register(
        class_name,
        "identityHashCode",
        "(Ljava/lang/Object;)I",
        identity_hash_code,
    );
    registry.register(
        class_name,
        "mapLibraryName",
        "(Ljava/lang/String;)Ljava/lang/String;",
        map_library_name,
    );
    registry.register(class_name, "nanoTime", "()J", nano_time);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(class_name, "setErr0", "(Ljava/io/PrintStream;)V", set_err_0);
    registry.register(class_name, "setIn0", "(Ljava/io/InputStream;)V", set_in_0);
    registry.register(class_name, "setOut0", "(Ljava/io/PrintStream;)V", set_out_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn arraycopy(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn current_time_millis(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn identity_hash_code(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn map_library_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn nano_time(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_err_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_in_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_out_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
