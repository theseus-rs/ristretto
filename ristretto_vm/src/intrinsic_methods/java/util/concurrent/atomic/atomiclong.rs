use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::env::consts::ARCH;
use std::sync::Arc;

#[intrinsic_method(
    "java/util/concurrent/atomic/AtomicLong.VMSupportsCS8()Z",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub(crate) async fn vm_supports_cs_8(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vm_supports_cs_8() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = vm_supports_cs_8(thread, Parameters::default()).await?;
        assert_eq!(value, Some(Value::from(true)));
        Ok(())
    }
}
