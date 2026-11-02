use crate::util::{TestContext, create_function, create_function_with_exception_table};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{ArrayType, ExceptionTableEntry, Instruction};
use ristretto_jit::{Result, Value};

#[test]
fn newarray_and_arraylength() -> Result<()> {
    let instructions = vec![
        Instruction::Bipush(10),
        Instruction::Newarray(ArrayType::Int),
        Instruction::Arraylength,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(&[], std::ptr::null())?.expect("value");
    assert_eq!(value, Value::I32(10));
    Ok(())
}

#[test]
fn newarray_allocation_failure_is_caught() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Newarray(ArrayType::Long),
        Instruction::Pop,
        Instruction::Iconst_0,
        Instruction::Ireturn,
        Instruction::Pop,
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function_with_exception_table(
        ConstantPool::default(),
        "(I)I",
        &instructions,
        vec![ExceptionTableEntry {
            range_pc: 0..3,
            handler_pc: 6,
            catch_type: 0,
        }],
    )?;
    let ctx = TestContext::new();
    // The test allocator rejects MAX without attempting a large host allocation.
    for count in [i32::MAX, -1] {
        let value = function.execute(&[Value::I32(count)], ctx.as_ptr())?;
        assert_eq!(value, Some(Value::I32(1)));
        assert_eq!(ctx.pending_exception(), 0);
    }
    assert_eq!(
        function.execute(&[Value::I32(1)], ctx.as_ptr())?,
        Some(Value::I32(0))
    );
    Ok(())
}
