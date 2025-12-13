use crate::util::create_function;
use ristretto_classfile::attributes::{ArrayType, Instruction};
use ristretto_jit::Value;

#[test]
fn baload_bastore() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Bipush(10),
        Instruction::Newarray(ArrayType::Byte),
        Instruction::Dup,
        Instruction::Iconst_0,
        Instruction::Bipush(42),
        Instruction::Bastore,
        Instruction::Iconst_0,
        Instruction::Baload,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}
