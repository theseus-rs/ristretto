use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::Result;

#[test]
fn nop() -> Result<()> {
    let instructions = vec![Instruction::Nop, Instruction::Return];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(vec![])?;
    assert_eq!(value, None);
    Ok(())
}
