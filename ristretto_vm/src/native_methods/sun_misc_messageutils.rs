use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.misc.MessageUtils`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/misc/MessageUtils";
    registry.register(class_name, "toStderr", "(Ljava/lang/String;)V", to_stderr);
    registry.register(class_name, "toStdout", "(Ljava/lang/String;)V", to_stdout);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn to_stderr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn to_stdout(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
