use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.net.spi.DefaultProxySelector`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/net/spi/DefaultProxySelector";
    registry.register(
        class_name,
        "getSystemProxy",
        "(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;",
        get_system_proxy,
    );
    registry.register(class_name, "init", "()Z", init);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_system_proxy(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
