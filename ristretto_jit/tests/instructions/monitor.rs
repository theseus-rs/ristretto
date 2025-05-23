use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::Result;

#[test]
fn monitorenter() -> Result<()> {
    let instructions = vec![
        Instruction::Lconst_0, // TODO: Use a reference to an object when objects are supported
        Instruction::Monitorenter,
        Instruction::Return,
    ];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(vec![])?;
    assert_eq!(value, None);
    Ok(())
}

#[test]
fn monitorexit() -> Result<()> {
    let instructions = vec![
        Instruction::Lconst_0, // TODO: Use a reference to an object when objects are supported
        Instruction::Monitorexit,
        Instruction::Return,
    ];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(vec![])?;
    assert_eq!(value, None);
    Ok(())
}
