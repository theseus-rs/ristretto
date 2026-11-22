//! Protocol checks only; canonical collector ownership is tested by the VM bridge.
#![allow(unsafe_code)]
#![expect(
    clippy::cast_ptr_alignment,
    reason = "opaque pointers originate from aligned live Context owners"
)]
use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_jit::{Compiler, Error, ExecutionStep, Function, Result, Value};
use std::cell::{Cell, RefCell};
use std::num::NonZeroU32;

struct Context {
    roots: RefCell<Vec<i64>>,
    failed: Cell<bool>,
    rejected: Cell<i64>,
}
impl Context {
    fn new() -> Self {
        Self {
            roots: RefCell::new(Vec::with_capacity(64)),
            failed: Cell::new(false),
            rejected: Cell::new(-1),
        }
    }
    fn pointer(&self) -> *const u8 {
        std::ptr::from_ref(self).cast()
    }
}

extern "C" fn publish(pointer: *const u8, handles: *const i64, count: i32) -> i32 {
    // SAFETY: every test keeps this aligned Context alive for the entire invocation.
    let context = unsafe { &*pointer.cast::<Context>() };
    if context.failed.get() || !(0..=64).contains(&count) {
        context.failed.set(true);
        return 0;
    }
    let Ok(count) = usize::try_from(count) else {
        return 0;
    };
    let values = if count == 0 {
        &[]
    } else {
        // SAFETY: generated publication passes exactly count initialized i64 cells.
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    if values.contains(&context.rejected.get()) {
        context.failed.set(true);
        return 0;
    }
    let Ok(mut roots) = context.roots.try_borrow_mut() else {
        context.failed.set(true);
        return 0;
    };
    roots.clear();
    roots.extend_from_slice(values); // capacity was preallocated before native entry
    1
}

fn compile(descriptor: &str, code: &[Instruction]) -> Result<Function> {
    let class = super::continuation::fixture(descriptor, code)?;
    // No legacy helper symbol is registered, including unreachable fallbacks.
    Compiler::new()?.compile_requests(
        &class,
        &class.methods[0],
        &[("jit_publish_frame_roots", publish as *const u8)],
    )
}

#[test]
fn live_local_is_published_across_yield_and_reference_return() -> Result<()> {
    let function = compile(
        "(Ljava/lang/Object;I)Ljava/lang/Object;",
        &[
            Instruction::Aload_0,
            Instruction::Astore_2, // 0,1
            Instruction::Iload_1,
            Instruction::Ifle(9), // 2,3 ->12
            Instruction::Iinc(1, -1),
            Instruction::Goto(-7), //6,9 ->2
            Instruction::Aload_2,
            Instruction::Areturn, //12,13
        ],
    )?;
    assert!(function.cooperation_requires_context());
    let context = Context::new();
    let mut activation = function.activation(&[Value::Ptr(17), Value::I32(2)])?;
    assert!(activation.resume(NonZeroU32::MIN).is_err());
    assert_eq!(activation.suspended_bci(), None);
    for _ in 0..2 {
        assert_eq!(
            activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
            ExecutionStep::Yielded
        );
        assert_eq!(*context.roots.borrow(), [17]);
    }
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(Some(Value::Ptr(17)))
    );
    assert_eq!(
        *context.roots.borrow(),
        [17],
        "result remains published for host handoff"
    );
    Ok(())
}

#[test]
fn reference_operand_survives_yield_after_its_local_is_dead() -> Result<()> {
    let function = compile(
        "(Ljava/lang/Object;I)Ljava/lang/Object;",
        &[
            Instruction::Aload_0,
            Instruction::Iload_1,
            Instruction::Ifle(9), //0,1,2 ->11
            Instruction::Iinc(1, -1),
            Instruction::Goto(-7), //5,8 ->1
            Instruction::Areturn,  //11
        ],
    )?;
    let context = Context::new();
    let mut activation = function.activation(&[Value::Ptr(23), Value::I32(1)])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Yielded
    );
    assert_eq!(*context.roots.borrow(), [23]);
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(Some(Value::Ptr(23)))
    );
    Ok(())
}

#[test]
fn dead_argument_is_not_retained_during_a_scalar_loop() -> Result<()> {
    let function = compile(
        "(Ljava/lang/Object;I)I",
        &[
            Instruction::Aload_0,
            Instruction::Pop,
            Instruction::Iload_1,
            Instruction::Ifle(9),
            Instruction::Iinc(1, -1),
            Instruction::Goto(-7),
            Instruction::Iload_1,
            Instruction::Ireturn,
        ],
    )?;
    let context = Context::new();
    let mut activation = function.activation(&[Value::Ptr(17), Value::I32(1)])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Yielded
    );
    assert!(context.roots.borrow().is_empty());
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(Some(Value::I32(0)))
    );
    Ok(())
}

#[test]
fn failed_publication_returns_before_reference_code_continues() -> Result<()> {
    let function = compile(
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        &[Instruction::Aload_0, Instruction::Areturn],
    )?;
    let context = Context::new();
    context.rejected.set(17);
    let mut activation = function.activation(&[Value::Ptr(17)])?;
    assert_eq!(
        activation.resume_with_context(NonZeroU32::MIN, context.pointer())?,
        ExecutionStep::Returned(None)
    );
    assert!(context.failed.get());
    assert!(context.roots.borrow().is_empty());
    Ok(())
}

#[test]
fn oversized_root_publication_is_rejected() -> Result<()> {
    let mut class = super::continuation::fixture(
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        &[Instruction::Aload_0, Instruction::Areturn],
    )?;
    if let Attribute::Code { max_stack, .. } = &mut class.methods[0].attributes[0] {
        *max_stack = u16::try_from(ristretto_jit::FRAME_ROOT_CAPACITY)?;
    }
    assert!(
        matches!(Compiler::new()?.compile_requests(&class, &class.methods[0], &[]), Err(Error::UnsupportedMethod(message)) if message == "frame root spill capacity is not qualified")
    );
    Ok(())
}
