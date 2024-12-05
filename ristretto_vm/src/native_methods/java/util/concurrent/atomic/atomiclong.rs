use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::env::consts::ARCH;
use std::sync::Arc;

/// Register all native methods for `java.util.concurrent.atomic.AtomicLong`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/concurrent/atomic/AtomicLong";
    registry.register(class_name, "VMSupportsCS8", "()Z", vm_supports_cs_8);
}

#[async_recursion(?Send)]
async fn vm_supports_cs_8(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // See "Atomic accesses to read-only memory" in `core::sync::atomic` for more information.
    let atomic_8_bytes = matches!(
        ARCH,
        "x86_64"
            | "aarch64"
            | "loongarch64"
            | "mips64"
            | "mips64r6"
            | "powerpc64"
            | "riscv64"
            | "sparc64"
            | "s390x"
    );
    Ok(Some(Value::from(atomic_8_bytes)))
}
