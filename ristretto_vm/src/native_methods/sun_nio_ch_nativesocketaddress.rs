use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.nio.ch.NativeSocketAddress`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/nio/ch/NativeSocketAddress";
    registry.register(class_name, "AFINET", "()I", afinet);
    registry.register(class_name, "AFINET6", "()I", afinet_6);
    registry.register(class_name, "offsetFamily", "()I", offset_family);
    registry.register(class_name, "offsetSin4Addr", "()I", offset_sin_4_addr);
    registry.register(class_name, "offsetSin4Port", "()I", offset_sin_4_port);
    registry.register(class_name, "offsetSin6Addr", "()I", offset_sin_6_addr);
    registry.register(
        class_name,
        "offsetSin6FlowInfo",
        "()I",
        offset_sin_6_flow_info,
    );
    registry.register(class_name, "offsetSin6Port", "()I", offset_sin_6_port);
    registry.register(
        class_name,
        "offsetSin6ScopeId",
        "()I",
        offset_sin_6_scope_id,
    );
    registry.register(class_name, "sizeofFamily", "()I", sizeof_family);
    registry.register(class_name, "sizeofSockAddr4", "()I", sizeof_sock_addr_4);
    registry.register(class_name, "sizeofSockAddr6", "()I", sizeof_sock_addr_6);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn afinet(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn afinet_6(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn offset_family(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn offset_sin_4_addr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn offset_sin_4_port(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn offset_sin_6_addr(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn offset_sin_6_flow_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn offset_sin_6_port(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn offset_sin_6_scope_id(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn sizeof_family(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn sizeof_sock_addr_4(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn sizeof_sock_addr_6(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
