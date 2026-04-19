use crate::util::{TestContext, create_function_with_constant_pool};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

fn add_class(constant_pool: &mut ConstantPool, name: &str) -> Result<u16> {
    Ok(constant_pool.add_class(name)?)
}

#[test]
fn new_object_returns_nonzero_reference() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let class_index = add_class(&mut constant_pool, "java/lang/Object")?;
    let instructions = vec![Instruction::New(class_index), Instruction::Areturn];
    let function =
        create_function_with_constant_pool(constant_pool, "()Ljava/lang/Object;", &instructions)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?.expect("value");
    match value {
        Value::Ptr(v) => assert!(v != 0, "new should return a non-zero object id, got {v}"),
        other => panic!("expected object reference, got {other:?}"),
    }
    Ok(())
}

#[test]
fn anewarray_allocates_and_arraylength_reports_count() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let class_index = add_class(&mut constant_pool, "java/lang/Object")?;
    let instructions = vec![
        Instruction::Bipush(7),
        Instruction::Anewarray(class_index),
        Instruction::Arraylength,
        Instruction::Ireturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()I", &instructions)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?.expect("value");
    assert_eq!(value, Value::I32(7));
    Ok(())
}

#[test]
fn multianewarray_allocates_and_arraylength_reports_outermost() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let class_index = add_class(&mut constant_pool, "[[I")?;
    let instructions = vec![
        Instruction::Bipush(4),
        Instruction::Bipush(3),
        Instruction::Multianewarray(class_index, 2),
        Instruction::Arraylength,
        Instruction::Ireturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()I", &instructions)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?.expect("value");
    assert_eq!(value, Value::I32(4));
    Ok(())
}

#[test]
fn checkcast_success_preserves_reference() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let class_index = add_class(&mut constant_pool, "java/lang/Object")?;
    let instructions = vec![
        Instruction::Aload_0,
        Instruction::Checkcast(class_index),
        Instruction::Areturn,
    ];
    let function = create_function_with_constant_pool(
        constant_pool,
        "(Ljava/lang/Object;)Ljava/lang/Object;",
        &instructions,
    )?;
    let ctx = TestContext::new();
    let value = function
        .execute(&[Value::Ptr(1234)], ctx.as_ptr())?
        .expect("value");
    assert_eq!(value, Value::Ptr(1234));
    assert_eq!(ctx.pending_exception(), 0);
    Ok(())
}

#[test]
fn instanceof_nonnull_returns_one() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let class_index = add_class(&mut constant_pool, "java/lang/Object")?;
    let instructions = vec![
        Instruction::Aload_0,
        Instruction::Instanceof(class_index),
        Instruction::Ireturn,
    ];
    let function =
        create_function_with_constant_pool(constant_pool, "(Ljava/lang/Object;)I", &instructions)?;
    let ctx = TestContext::new();
    let value = function
        .execute(&[Value::Ptr(42)], ctx.as_ptr())?
        .expect("value");
    assert_eq!(value, Value::I32(1));
    Ok(())
}

#[test]
fn instanceof_null_returns_zero() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let class_index = add_class(&mut constant_pool, "java/lang/Object")?;
    let instructions = vec![
        Instruction::Aconst_null,
        Instruction::Instanceof(class_index),
        Instruction::Ireturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()I", &instructions)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?.expect("value");
    assert_eq!(value, Value::I32(0));
    Ok(())
}
