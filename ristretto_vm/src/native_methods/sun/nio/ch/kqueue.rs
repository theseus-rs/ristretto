use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.nio.ch.KQueue`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/KQueue";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "keventPoll", "(IJI)I", kevent_poll);
        registry.register(class_name, "keventRegister", "(IIII)I", kevent_register);
        registry.register(class_name, "kqueue", "()I", kqueue);
    } else {
        registry.register(class_name, "create", "()I", create);
        registry.register(class_name, "poll", "(IJIJ)I", poll);
        registry.register(class_name, "register", "(IIII)I", register_0);
    }

    registry.register(class_name, "filterOffset", "()I", filter_offset);
    registry.register(class_name, "flagsOffset", "()I", flags_offset);
    registry.register(class_name, "identOffset", "()I", ident_offset);
    registry.register(class_name, "keventSize", "()I", kevent_size);
}

#[async_recursion(?Send)]
async fn create(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.create()I");
}

#[async_recursion(?Send)]
async fn filter_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.filterOffset()I");
}

#[async_recursion(?Send)]
async fn flags_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.flagsOffset()I");
}

#[async_recursion(?Send)]
async fn ident_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.identOffset()I");
}

#[async_recursion(?Send)]
async fn kevent_poll(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventPoll(IJI)I");
}

#[async_recursion(?Send)]
async fn kevent_register(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventRegister(IIII)I");
}

#[async_recursion(?Send)]
async fn kevent_size(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.keventSize()I");
}

#[async_recursion(?Send)]
async fn kqueue(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.kqueue()I");
}

#[async_recursion(?Send)]
async fn poll(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.poll(IJIJ)I");
}

#[async_recursion(?Send)]
async fn register_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.nio.ch.KQueue.register(IIII)I");
}
