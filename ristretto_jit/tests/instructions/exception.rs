use crate::util::{
    TestContext, create_function_with_constant_pool, create_function_with_exception_table,
};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{ExceptionTableEntry, Instruction};
use ristretto_jit::{Result, Value};

#[test]
fn athrow_sets_pending_exception() -> Result<()> {
    let constant_pool = ConstantPool::default();
    // Method takes a Throwable argument and throws it.
    let instructions = vec![Instruction::Aload_0, Instruction::Athrow];
    let function = create_function_with_constant_pool(
        constant_pool,
        "(Ljava/lang/Throwable;)V",
        &instructions,
    )?;
    let ctx = TestContext::new();
    let value = function.execute(&[Value::Ptr(777)], ctx.as_ptr())?;
    // Because the JIT sets NONE on pending exception, athrow returns None.
    assert!(value.is_none());
    assert_eq!(ctx.pending_exception(), 777);
    Ok(())
}

fn exception_entry(
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
) -> ExceptionTableEntry {
    ExceptionTableEntry {
        range_pc: start_pc..end_pc,
        handler_pc,
        catch_type,
    }
}

/// A thrown exception is caught by a catch-all handler covering its PC; the handler runs
/// and returns a sentinel int, and the pending exception slot is cleared.
#[test]
fn athrow_caught_by_catch_all_handler() -> Result<()> {
    let constant_pool = ConstantPool::default();
    let instructions = vec![
        Instruction::Aload_0,    // 0
        Instruction::Athrow,     // 1
        Instruction::Pop,        // 2 (handler: discards exception)
        Instruction::Bipush(42), // 3
        Instruction::Ireturn,    // 4
    ];
    let exception_table = vec![exception_entry(0, 2, 2, 0)];
    let function = create_function_with_exception_table(
        constant_pool,
        "(Ljava/lang/Throwable;)I",
        &instructions,
        exception_table,
    )?;
    let ctx = TestContext::new();
    let value = function
        .execute(&[Value::Ptr(77)], ctx.as_ptr())?
        .expect("value");
    assert_eq!(value, Value::I32(42));
    // Handler consumed the exception, so the slot should be cleared.
    assert_eq!(ctx.pending_exception(), 0);
    Ok(())
}

/// A thrown exception matching a typed handler's `catch_type` is caught; the handler stores
/// the exception reference in a local and returns it as the result.
#[test]
fn athrow_caught_by_typed_handler_returns_exception() -> Result<()> {
    let constant_pool = ConstantPool::default();
    // Stub convention: `jit_exception_matches` returns true when
    // `pending_exception == cp_class_index`. Test passes Ptr(7) and catch_type = 7.
    let instructions = vec![
        Instruction::Aload_0, // 0
        Instruction::Athrow,  // 1
        Instruction::Areturn, // 2 (handler: returns the caught exception)
    ];
    let exception_table = vec![exception_entry(0, 2, 2, 7)];
    let function = create_function_with_exception_table(
        constant_pool,
        "(Ljava/lang/Throwable;)Ljava/lang/Throwable;",
        &instructions,
        exception_table,
    )?;
    let ctx = TestContext::new();
    let value = function
        .execute(&[Value::Ptr(7)], ctx.as_ptr())?
        .expect("value");
    assert_eq!(value, Value::Ptr(7));
    assert_eq!(ctx.pending_exception(), 0);
    Ok(())
}

/// A thrown exception that doesn't match any typed handler is propagated (function returns
/// `None` and the pending exception slot remains set).
#[test]
fn athrow_propagates_when_no_handler_matches() -> Result<()> {
    let constant_pool = ConstantPool::default();
    let instructions = vec![
        Instruction::Aload_0,    // 0
        Instruction::Athrow,     // 1
        Instruction::Bipush(99), // 2 (handler: returns 99 if caught)
        Instruction::Ireturn,    // 3
    ];
    // Catch type 123, but we throw Ptr(7) which does not match.
    let exception_table = vec![exception_entry(0, 2, 2, 123)];
    let function = create_function_with_exception_table(
        constant_pool,
        "(Ljava/lang/Throwable;)I",
        &instructions,
        exception_table,
    )?;
    let ctx = TestContext::new();
    let value = function.execute(&[Value::Ptr(7)], ctx.as_ptr())?;
    assert!(value.is_none(), "expected None (propagated), got {value:?}");
    assert_eq!(ctx.pending_exception(), 7);
    Ok(())
}

/// Multiple handlers are tried in order; the first non-matching typed handler is skipped and
/// the second (matching) handler catches the exception.
#[test]
fn athrow_dispatches_to_second_matching_handler() -> Result<()> {
    let constant_pool = ConstantPool::default();
    let instructions = vec![
        Instruction::Aload_0,    // 0
        Instruction::Athrow,     // 1
        Instruction::Pop,        // 2 (handler A; doesn't match)
        Instruction::Bipush(10), // 3
        Instruction::Ireturn,    // 4
        Instruction::Pop,        // 5 (handler B; matches catch-all)
        Instruction::Bipush(20), // 6
        Instruction::Ireturn,    // 7
    ];
    // Two entries covering the same PC range. First has catch_type 99 (won't match); second is
    // catch-all.
    let exception_table = vec![exception_entry(0, 2, 2, 99), exception_entry(0, 2, 5, 0)];
    let function = create_function_with_exception_table(
        constant_pool,
        "(Ljava/lang/Throwable;)I",
        &instructions,
        exception_table,
    )?;
    let ctx = TestContext::new();
    let value = function
        .execute(&[Value::Ptr(7)], ctx.as_ptr())?
        .expect("value");
    assert_eq!(value, Value::I32(20));
    assert_eq!(ctx.pending_exception(), 0);
    Ok(())
}

/// Exception thrown by a helper (here, checkcast with the test sentinel) is caught by a
/// covering catch-all handler.
#[test]
fn checkcast_failure_caught_by_handler() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    // Stub convention for `jit_checkcast`: cp_class_index == 0xFFFF triggers the pending
    // exception path. We build up the CP until a class index of 0xFFFF is reached is
    // impractical; instead use the class index from the test stub's general case: the stub
    // actually returns success for non-sentinel class indexes (see tests/util.rs).
    //
    // Since the test stub only fails on cp_class_index == 0xFFFF, we rely on that exact
    // sentinel. We fabricate it directly in the instruction.
    let class_index = constant_pool.add_class("Owner")?;
    let _ = class_index;
    let instructions = vec![
        Instruction::Aload_0,           // 0
        Instruction::Checkcast(0xFFFF), // 1 (triggers pending exception in stub)
        Instruction::Areturn,           // 2 (if checkcast succeeded - should not reach)
        Instruction::Pop,               // 3 (handler)
        Instruction::Bipush(55),        // 4
        Instruction::Ireturn,           // 5
    ];
    let exception_table = vec![exception_entry(0, 3, 3, 0)];
    let function = create_function_with_exception_table(
        constant_pool,
        "(Ljava/lang/Object;)I",
        &instructions,
        exception_table,
    )?;
    let ctx = TestContext::new();
    let value = function
        .execute(&[Value::Ptr(1)], ctx.as_ptr())?
        .expect("value");
    assert_eq!(value, Value::I32(55));
    assert_eq!(ctx.pending_exception(), 0);
    Ok(())
}

/// A `getfield` on a `null` reference triggers a pending exception (in the stub) that is
/// caught by a covering catch-all handler, demonstrating the dispatch path applies to
/// pending exception checks as well as explicit `athrow`.
#[test]
fn getfield_npe_caught_by_handler() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let class_index = constant_pool.add_class("Owner")?;
    let field_ref = constant_pool.add_field_ref(class_index, "value", "I")?;
    let instructions = vec![
        Instruction::Aconst_null,         // 0
        Instruction::Getfield(field_ref), // 1 (NPE-like failure)
        Instruction::Ireturn,             // 2 (would return the field if successful)
        Instruction::Pop,                 // 3 (handler)
        Instruction::Bipush(99),          // 4
        Instruction::Ireturn,             // 5
    ];
    let exception_table = vec![exception_entry(0, 3, 3, 0)];
    let function =
        create_function_with_exception_table(constant_pool, "()I", &instructions, exception_table)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?.expect("value");
    assert_eq!(value, Value::I32(99));
    assert_eq!(ctx.pending_exception(), 0);
    Ok(())
}
