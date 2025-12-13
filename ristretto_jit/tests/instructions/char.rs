use crate::util::create_function;
use ristretto_classfile::attributes::{ArrayType, Instruction};
use ristretto_jit::Value;

#[test]
fn caload_castore() -> ristretto_jit::Result<()> {
    let instructions = vec![
        Instruction::Bipush(10),
        Instruction::Newarray(ArrayType::Char),
        Instruction::Dup,
        Instruction::Iconst_0,
        Instruction::Bipush(42),
        Instruction::Castore,
        Instruction::Iconst_0,
        Instruction::Caload,
        Instruction::Ireturn,
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}
