use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.Shutdown`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Shutdown";
    registry.register(class_name, "beforeHalt", "()V", before_halt);
    registry.register(class_name, "halt0", "(I)V", halt_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn before_halt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn halt_0(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let code = arguments.pop_int()?;
    std::process::exit(code);
}
