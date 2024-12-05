use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.PollArrayWrapper`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/PollArrayWrapper";
    registry.register(class_name, "interrupt", "(I)V", interrupt);
    registry.register(class_name, "poll0", "(JIJ)I", poll_0);
}

#[async_recursion(?Send)]
async fn interrupt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.PollArrayWrapper.interrupt(I)V")
}

#[async_recursion(?Send)]
async fn poll_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.PollArrayWrapper.poll0(JIJ)I")
}
