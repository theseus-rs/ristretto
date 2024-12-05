use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.KQueueArrayWrapper`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/KQueueArrayWrapper";
    registry.register(class_name, "init", "()I", init);
    registry.register(class_name, "initStructSizes", "()V", init_struct_sizes);
    registry.register(class_name, "interrupt", "(I)V", interrupt);
    registry.register(class_name, "kevent0", "(IJIJ)I", kevent_0);
    registry.register(class_name, "register0", "(IIII)V", register_0);
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.init()I");
}

#[async_recursion(?Send)]
async fn init_struct_sizes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.initStructSizes()V");
}

#[async_recursion(?Send)]
async fn interrupt(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.interrupt(I)V");
}

#[async_recursion(?Send)]
async fn kevent_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.kevent0(IJIJ)I");
}

#[async_recursion(?Send)]
async fn register_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueueArrayWrapper.register0(IIII)V");
}
