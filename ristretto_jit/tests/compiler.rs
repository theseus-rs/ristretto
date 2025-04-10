use ristretto_classfile::attributes::{Attribute, Instruction, MaxLocals, MaxStack};
use ristretto_classfile::{ConstantPool, MethodAccessFlags};
use ristretto_jit::{Compiler, Result, Value};

#[test]
fn test_compile() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let code_index = constant_pool.add_utf8("Code")?;
    let test_name_index = constant_pool.add_utf8("test")?;
    let test_descriptor_index = constant_pool.add_utf8("(II)I")?;

    let mut test_method = ristretto_classfile::Method {
        access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        name_index: test_name_index,
        descriptor_index: test_descriptor_index,
        attributes: Vec::new(),
    };
    let test_method_code = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::Iadd,
        Instruction::Ireturn,
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
    let arguments = vec![4i32.into(), 8i32.into()];
    let value = function.execute(arguments)?.expect("value");
    assert_eq!(value, Value::I32(12));
    Ok(())
}
