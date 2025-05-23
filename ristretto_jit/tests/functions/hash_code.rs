use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn compile_hash_code() -> Result<()> {
    let instructions = vec![
        Instruction::Lload_0,
        Instruction::Lload_0,
        Instruction::Bipush(32),
        Instruction::Lushr,
        Instruction::Lxor,
        Instruction::L2i,
        Instruction::Ireturn,
    ];
    let function = create_function("(J)I", &instructions)?;
    let arguments = vec![Value::I64(9_223_372_036_854_775_807)];
    let value = function.execute(arguments)?.expect("value");
    assert_eq!(value, Value::I32(-2_147_483_648));
    Ok(())
}
