use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn math_max() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmplt(5),
        Instruction::Iload_0,
        Instruction::Goto(6),
        Instruction::Iload_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;
    let value = function
        .execute(&[Value::I32(3), Value::I32(42)], std::ptr::null())?
        .expect("value");
    assert_eq!(value, Value::I32(42));
    let value = function
        .execute(&[Value::I32(42), Value::I32(3)], std::ptr::null())?
        .expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}
