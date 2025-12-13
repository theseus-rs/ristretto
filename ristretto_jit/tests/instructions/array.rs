use crate::util::create_function;
use ristretto_classfile::attributes::{ArrayType, Instruction};
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
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(10));
    Ok(())
}
