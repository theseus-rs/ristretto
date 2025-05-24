use crate::util::create_function;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

#[test]
fn ifeq() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifeq(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if not equal to zero
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn ifne() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifne(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if not equal to zero
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn iflt() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iflt(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if less than zero
    let value = function.execute(vec![Value::I32(-1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if equal to or greater than zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn ifge() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifge(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if greater than or equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(1));
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if less than zero
    let value = function.execute(vec![Value::I32(-1)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn ifgt() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifgt(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if greater than zero
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if less than or equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn ifle() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Ifle(4),
        Instruction::Iconst_0,
        Instruction::Goto(5),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(I)Z", &instructions)?;

    // return true if less than or equal to zero
    let value = function.execute(vec![Value::I32(0)])?.expect("value");
    assert_eq!(value, Value::I32(1));
    let value = function.execute(vec![Value::I32(-1)])?.expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if greater than zero
    let value = function.execute(vec![Value::I32(1)])?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmpeq() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmpeq(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if values are equal
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if values are not equal
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmpne() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmpne(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if values are not equal
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if values are equal
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmplt() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmplt(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if the first value is less than the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if the first value is greater than or equal to the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmpge() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmpge(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if the first value is greater than or equal to the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    let value = function
        .execute(vec![Value::I32(1), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if the first value is less than the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmpgt() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmpgt(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if the first value is greater than the second
    let value = function
        .execute(vec![Value::I32(1), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if the first value is less than or equal to the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn if_icmple() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Iload_1,
        Instruction::If_icmple(5),
        Instruction::Iconst_0,
        Instruction::Goto(6),
        Instruction::Iconst_1,
        Instruction::Ireturn,
    ];
    let function = create_function("(II)I", &instructions)?;

    // return true if the first value is less than or equal to the second
    let value = function
        .execute(vec![Value::I32(0), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    let value = function
        .execute(vec![Value::I32(0), Value::I32(1)])?
        .expect("value");
    assert_eq!(value, Value::I32(1));

    // return false if the first value is greater than the second
    let value = function
        .execute(vec![Value::I32(1), Value::I32(0)])?
        .expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}

#[test]
fn goto() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Goto(2),
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn goto_w() -> Result<()> {
    let instructions = vec![
        Instruction::Iload_0,
        Instruction::Goto_w(2),
        Instruction::Ireturn,
    ];
    let function = create_function("(I)I", &instructions)?;
    let value = function.execute(vec![Value::I32(42)])?.expect("value");
    assert_eq!(value, Value::I32(42));
    Ok(())
}

#[test]
fn jsr() -> Result<()> {
    let instructions = vec![
        Instruction::Iconst_0,
        Instruction::Jsr(5),
        Instruction::Istore_1,
        Instruction::Iload_1,
        Instruction::Ireturn,
        Instruction::Goto(2), // Note: this should be Instruction::Ret(2)
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(2));
    Ok(())
}

#[test]
fn jsr_w() -> Result<()> {
    let instructions = vec![
        Instruction::Iconst_0,
        Instruction::Jsr_w(5),
        Instruction::Istore_1,
        Instruction::Iload_1,
        Instruction::Ireturn,
        Instruction::Goto(2), // Note: this should be Instruction::Ret_w(2)
    ];
    let function = create_function("()I", &instructions)?;
    let value = function.execute(vec![])?.expect("value");
    assert_eq!(value, Value::I32(2));
    Ok(())
}

#[test]
fn r#return() -> Result<()> {
    let instructions = vec![Instruction::Return];
    let function = create_function("()V", &instructions)?;
    let value = function.execute(vec![])?;
    assert_eq!(value, None);
    Ok(())
}
