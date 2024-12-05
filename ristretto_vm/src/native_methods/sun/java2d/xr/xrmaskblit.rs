use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.xr.XRMaskBlit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/xr/XRMaskBlit";
    registry.register(class_name, "maskBlit", "(JJIIIIIIIII[B)V", mask_blit);
}

#[async_recursion(?Send)]
async fn mask_blit(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRMaskBlit.maskBlit(JJIIIIIIIII[B)V");
}
