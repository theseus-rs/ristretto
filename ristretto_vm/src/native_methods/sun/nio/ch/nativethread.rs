use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `sun.nio.ch.NativeThread`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/NativeThread";
    let java_version = registry.java_version();

    if java_version <= &JAVA_18 {
        registry.register(class_name, "current", "()J", current);
        registry.register(class_name, "signal", "(J)V", signal);
    } else {
        registry.register(class_name, "current0", "()J", current_0);
        registry.register(class_name, "signal0", "(J)V", signal_0);
    }

    registry.register(class_name, "init", "()V", init);
}

#[async_recursion(?Send)]
async fn current(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeThread.current()J");
}

#[async_recursion(?Send)]
async fn current_0(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    current(thread, arguments).await
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn signal(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.NativeThread.signal(J)V");
}

#[async_recursion(?Send)]
async fn signal_0(thread: Arc<Thread>, arguments: Arguments) -> Result<Option<Value>> {
    signal(thread, arguments).await
}
