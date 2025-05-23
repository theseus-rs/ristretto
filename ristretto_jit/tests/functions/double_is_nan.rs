use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn double_is_nan() -> Result<()> {
    let instructions = vec![
        Instruction::Dload_0,
        Instruction::Dload_0,
        Instruction::Dcmpl,
        Instruction::Ifeq(6),
        Instruction::Iconst_1,
        Instruction::Goto(7),
        Instruction::Iconst_0,
        Instruction::Ireturn,
    ];
    let function = create_function("(D)Z", &instructions)?;
    let value = function.execute(vec![Value::F64(42.1)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    let value = function
        .execute(vec![Value::F64(f64::NAN)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}
