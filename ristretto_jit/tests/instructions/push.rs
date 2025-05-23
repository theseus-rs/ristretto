use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn bipush() -> Result<()> {
    let instructions = vec![Instruction::Bipush(42), Instruction::Ireturn];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(42);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}

#[test]
fn sipush() -> Result<()> {
    let instructions = vec![Instruction::Sipush(42), Instruction::Ireturn];
    let function = create_function("()I", &instructions)?;
    let expected_value = Value::I32(42);
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, expected_value);
    Ok(())
}
