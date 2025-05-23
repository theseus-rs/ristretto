use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::Result;

#[test]
fn breakpoint() -> Result<()> {
    let instructions = vec![Instruction::Breakpoint, Instruction::Return];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(vec![])?;
    assert_eq!(value, None);
    Ok(())
}

#[test]
fn impdep1() -> Result<()> {
    let instructions = vec![Instruction::Impdep1, Instruction::Return];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(vec![])?;
    assert_eq!(value, None);
    Ok(())
}

#[test]
fn impdep2() -> Result<()> {
    let instructions = vec![Instruction::Impdep2, Instruction::Return];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(vec![])?;
    assert_eq!(value, None);
    Ok(())
}
