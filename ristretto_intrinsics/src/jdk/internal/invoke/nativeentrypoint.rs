use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

// VMStorage classes used by the JDK 17 foreign linker. These values are part of the
// private contract between the architecture-specific Java code and HotSpot.
const INTEGER_TYPE: i32 = 0;
const VECTOR_TYPE: i32 = 1;
const STACK_TYPE: i32 = 3;
const BAD_VM_REG: i64 = -1;

/// Encode a JDK `VMStorage` as the numeric value of `HotSpot`'s `VMRegImpl*`.
///
/// A `VMReg` identifies a four-byte slot. Consequently, a physical register can consume
/// multiple consecutive `VMRegs`, and stack indexes (which the foreign linker expresses in
/// eight-byte slots on 64-bit targets) must be doubled. `stack0` is `HotSpot`'s register count
/// rounded up to an eight-slot boundary.
#[cfg(target_arch = "x86_64")]
fn encode_vm_storage(type_: i32, index: i32) -> i64 {
    // 16 GPRs * 2 slots + 8 x87 registers * 2 slots.
    const MAX_FPR: i64 = 48;
    // 577 concrete register slots, rounded up to a multiple of eight.
    const STACK_0: i64 = 584;
    let index = i64::from(index);

    match type_ {
        INTEGER_TYPE if (0..16).contains(&index) => index * 2,
        VECTOR_TYPE if (0..32).contains(&index) => MAX_FPR + index * 16,
        STACK_TYPE if index >= 0 => STACK_0 + index * 2,
        _ => BAD_VM_REG,
    }
}

#[cfg(target_arch = "x86")]
fn encode_vm_storage(type_: i32, index: i32) -> i64 {
    // 8 GPR slots + 8 x87 registers * 2 slots.
    const MAX_FPR: i64 = 24;
    // 177 concrete register slots, rounded up to a multiple of eight.
    const STACK_0: i64 = 184;
    let index = i64::from(index);

    match type_ {
        INTEGER_TYPE if (0..8).contains(&index) => index,
        VECTOR_TYPE if (0..8).contains(&index) => MAX_FPR + index * 16,
        STACK_TYPE if index >= 0 => STACK_0 + index,
        _ => BAD_VM_REG,
    }
}

#[cfg(target_arch = "aarch64")]
fn encode_vm_storage(type_: i32, index: i32) -> i64 {
    // 32 GPRs * 2 slots.
    const MAX_GPR: i64 = 64;
    // 337 concrete register slots, rounded up to a multiple of eight.
    const STACK_0: i64 = 344;
    let index = i64::from(index);

    match type_ {
        INTEGER_TYPE if (0..32).contains(&index) => index * 2,
        VECTOR_TYPE if (0..32).contains(&index) => MAX_GPR + index * 8,
        STACK_TYPE if index >= 0 => STACK_0 + index * 2,
        _ => BAD_VM_REG,
    }
}

// JDK 17 only supplies foreign-linker ABIs for x86-64 and AArch64. The x86 HotSpot
// implementation also defines an encoding, but all other HotSpot ports implement this operation
// as `VMReg::Bad` (or reject the foreign ABI before reaching it).
#[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
fn encode_vm_storage(_type_: i32, _index: i32) -> i64 {
    BAD_VM_REG
}

#[intrinsic_method(
    "jdk/internal/invoke/NativeEntryPoint.registerNatives()V",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/invoke/NativeEntryPoint.vmStorageToVMReg(II)J",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn vm_storage_to_vm_reg<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let type_ = parameters.pop_int()?;
    let vm_reg = encode_vm_storage(type_, index);
    Ok(Some(Value::Long(vm_reg)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_vm_storage_to_vm_reg() -> Result<()> {
        let (_vm, thread) = crate::test::java17_thread().await?;
        let result = vm_storage_to_vm_reg(
            thread,
            Parameters::new(vec![Value::Int(INTEGER_TYPE), Value::Int(0)]),
        )
        .await?;
        assert_eq!(
            result,
            Some(Value::Long(encode_vm_storage(INTEGER_TYPE, 0)))
        );
        Ok(())
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_encode_vm_storage_x86_64() {
        assert_eq!(0, encode_vm_storage(INTEGER_TYPE, 0));
        assert_eq!(30, encode_vm_storage(INTEGER_TYPE, 15));
        assert_eq!(48, encode_vm_storage(VECTOR_TYPE, 0));
        assert_eq!(544, encode_vm_storage(VECTOR_TYPE, 31));
        assert_eq!(584, encode_vm_storage(STACK_TYPE, 0));
        assert_eq!(590, encode_vm_storage(STACK_TYPE, 3));
        assert_eq!(BAD_VM_REG, encode_vm_storage(2, 0));
        assert_eq!(BAD_VM_REG, encode_vm_storage(4, 0));
        assert_eq!(BAD_VM_REG, encode_vm_storage(INTEGER_TYPE, -1));
        assert_eq!(BAD_VM_REG, encode_vm_storage(INTEGER_TYPE, 16));
        assert_eq!(BAD_VM_REG, encode_vm_storage(VECTOR_TYPE, -1));
        assert_eq!(BAD_VM_REG, encode_vm_storage(VECTOR_TYPE, 32));
        assert_eq!(BAD_VM_REG, encode_vm_storage(STACK_TYPE, -1));
    }

    #[cfg(target_arch = "x86")]
    #[test]
    fn test_encode_vm_storage_x86() {
        assert_eq!(0, encode_vm_storage(INTEGER_TYPE, 0));
        assert_eq!(7, encode_vm_storage(INTEGER_TYPE, 7));
        assert_eq!(24, encode_vm_storage(VECTOR_TYPE, 0));
        assert_eq!(136, encode_vm_storage(VECTOR_TYPE, 7));
        assert_eq!(184, encode_vm_storage(STACK_TYPE, 0));
        assert_eq!(187, encode_vm_storage(STACK_TYPE, 3));
        assert_eq!(BAD_VM_REG, encode_vm_storage(2, 0));
        assert_eq!(BAD_VM_REG, encode_vm_storage(4, 0));
        assert_eq!(BAD_VM_REG, encode_vm_storage(INTEGER_TYPE, -1));
        assert_eq!(BAD_VM_REG, encode_vm_storage(INTEGER_TYPE, 8));
        assert_eq!(BAD_VM_REG, encode_vm_storage(VECTOR_TYPE, -1));
        assert_eq!(BAD_VM_REG, encode_vm_storage(VECTOR_TYPE, 8));
        assert_eq!(BAD_VM_REG, encode_vm_storage(STACK_TYPE, -1));
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn test_encode_vm_storage_aarch64() {
        assert_eq!(0, encode_vm_storage(INTEGER_TYPE, 0));
        assert_eq!(62, encode_vm_storage(INTEGER_TYPE, 31));
        assert_eq!(64, encode_vm_storage(VECTOR_TYPE, 0));
        assert_eq!(312, encode_vm_storage(VECTOR_TYPE, 31));
        assert_eq!(344, encode_vm_storage(STACK_TYPE, 0));
        assert_eq!(350, encode_vm_storage(STACK_TYPE, 3));
        assert_eq!(BAD_VM_REG, encode_vm_storage(2, 0));
        assert_eq!(BAD_VM_REG, encode_vm_storage(4, 0));
        assert_eq!(BAD_VM_REG, encode_vm_storage(INTEGER_TYPE, -1));
        assert_eq!(BAD_VM_REG, encode_vm_storage(INTEGER_TYPE, 32));
        assert_eq!(BAD_VM_REG, encode_vm_storage(VECTOR_TYPE, -1));
        assert_eq!(BAD_VM_REG, encode_vm_storage(VECTOR_TYPE, 32));
        assert_eq!(BAD_VM_REG, encode_vm_storage(STACK_TYPE, -1));
    }

    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
    #[test]
    fn test_encode_vm_storage_unsupported_architecture() {
        assert_eq!(BAD_VM_REG, encode_vm_storage(INTEGER_TYPE, 0));
        assert_eq!(BAD_VM_REG, encode_vm_storage(VECTOR_TYPE, 0));
        assert_eq!(BAD_VM_REG, encode_vm_storage(STACK_TYPE, 0));
    }
}
