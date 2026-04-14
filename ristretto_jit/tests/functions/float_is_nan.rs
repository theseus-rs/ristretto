use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn float_is_nan() -> Result<()> {
    let instructions = vec![
        Instruction::Fload_0,
        Instruction::Fload_0,
        Instruction::Fcmpl,
        Instruction::Ifeq(6),
        Instruction::Iconst_1,
        Instruction::Goto(7),
        Instruction::Iconst_0,
        Instruction::Ireturn,
    ];
    let function = create_function("(F)Z", &instructions)?;
    let value = function
        .execute(&[Value::F32(42.1)], std::ptr::null())?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    let value = function
        .execute(&[Value::F32(f32::NAN)], std::ptr::null())?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}
