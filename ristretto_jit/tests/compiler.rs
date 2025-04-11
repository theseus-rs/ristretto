use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ConstantPool, MethodAccessFlags};
use ristretto_jit::{Compiler, Result, Value};

#[test]
fn test_compile() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let first_argument_index = constant_pool.add_long(4294967295)?;
    let code_index = constant_pool.add_utf8("Code")?;
    let test_name_index = constant_pool.add_utf8("multiplyHigh")?;
    let test_descriptor_index = constant_pool.add_utf8("(JJ)J")?;

    let mut test_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: test_name_index,
        descriptor_index: test_descriptor_index,
        attributes: Vec::new(),
    };
    let test_method_code = vec![
        Instruction::Lload_0,
        Instruction::Bipush(32),
        Instruction::Lshr,
        Instruction::Lstore(4),
        Instruction::Lload_0,
        Instruction::Ldc2_w(first_argument_index),
        Instruction::Land,
        Instruction::Lstore(6),
        Instruction::Lload_2,
        Instruction::Bipush(32),
        Instruction::Lshr,
        Instruction::Lstore(8),
        Instruction::Lload_2,
        Instruction::Ldc2_w(first_argument_index),
        Instruction::Land,
        Instruction::Lstore(10),
        Instruction::Lload(6),
        Instruction::Lload(10),
        Instruction::Lmul,
        Instruction::Lstore(12),
        Instruction::Lload(4),
        Instruction::Lload(10),
        Instruction::Lmul,
        Instruction::Lload(12),
        Instruction::Bipush(32),
        Instruction::Lushr,
        Instruction::Ladd,
        Instruction::Lstore(14),
        Instruction::Lload(14),
        Instruction::Ldc2_w(first_argument_index),
        Instruction::Land,
        Instruction::Lstore(16),
        Instruction::Lload(14),
        Instruction::Bipush(32),
        Instruction::Lshr,
        Instruction::Lstore(18),
        Instruction::Lload(16),
        Instruction::Lload(6),
        Instruction::Lload(8),
        Instruction::Lmul,
        Instruction::Ladd,
        Instruction::Lstore(16),
        Instruction::Lload(4),
        Instruction::Lload(8),
        Instruction::Lmul,
        Instruction::Lload(18),
        Instruction::Ladd,
        Instruction::Lload(16),
        Instruction::Bipush(32),
        Instruction::Lshr,
        Instruction::Ladd,
        Instruction::Lreturn,
    ];
    let test_max_stack = test_method_code.max_stack(&constant_pool)?;
    let test_max_locals = test_method_code.max_locals(&constant_pool, test_descriptor_index)?;
    test_method.attributes.push(Attribute::Code {
        name_index: code_index,
        max_stack: test_max_stack,
        max_locals: test_max_locals,
        code: test_method_code,
        exception_table: Vec::new(),
        attributes: Vec::new(),
    });

    let compiler = Compiler::new()?;
    let function = compiler.compile(&constant_pool, &test_method)?;
    let arguments = vec![Value::I64(4), Value::I64(8)];
    let value = function.execute(arguments)?.expect("value");
    assert_eq!(value, Value::I64(0));
    Ok(())
}
