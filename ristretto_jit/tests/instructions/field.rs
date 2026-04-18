use crate::util::{TestContext, create_function_with_constant_pool};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::Instruction;
use ristretto_jit::{Result, Value};

/// Adds a `CONSTANT_Fieldref_info` entry for class `Owner`, field `name` with field descriptor
/// `descriptor`, returning its constant pool index.
fn add_field_ref(
    constant_pool: &mut ConstantPool,
    class_name: &str,
    field_name: &str,
    descriptor: &str,
) -> Result<u16> {
    let class_index = constant_pool.add_class(class_name)?;
    let index = constant_pool.add_field_ref(class_index, field_name, descriptor)?;
    Ok(index)
}

#[test]
fn putstatic_getstatic_int() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let field_ref = add_field_ref(&mut constant_pool, "Owner", "value", "I")?;
    let instructions = vec![
        Instruction::Sipush(12345),
        Instruction::Putstatic(field_ref),
        Instruction::Getstatic(field_ref),
        Instruction::Ireturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()I", &instructions)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?.expect("value");
    assert_eq!(value, Value::I32(12345));
    Ok(())
}

#[test]
fn putstatic_getstatic_long() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let field_ref = add_field_ref(&mut constant_pool, "Owner", "value", "J")?;
    let long_index = constant_pool.add_long(0x1234_5678_9ABC_DEF0)?;
    let instructions = vec![
        Instruction::Ldc2_w(long_index),
        Instruction::Putstatic(field_ref),
        Instruction::Getstatic(field_ref),
        Instruction::Lreturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()J", &instructions)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?.expect("value");
    assert_eq!(value, Value::I64(0x1234_5678_9ABC_DEF0));
    Ok(())
}

#[test]
fn putstatic_getstatic_float() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let field_ref = add_field_ref(&mut constant_pool, "Owner", "value", "F")?;
    let float_index = constant_pool.add_float(2.5)?;
    let instructions = vec![
        Instruction::Ldc_w(float_index),
        Instruction::Putstatic(field_ref),
        Instruction::Getstatic(field_ref),
        Instruction::Freturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()F", &instructions)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?.expect("value");
    assert_eq!(value, Value::F32(2.5));
    Ok(())
}

#[test]
fn putstatic_getstatic_double() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let field_ref = add_field_ref(&mut constant_pool, "Owner", "value", "D")?;
    let double_index = constant_pool.add_double(3.5)?;
    let instructions = vec![
        Instruction::Ldc2_w(double_index),
        Instruction::Putstatic(field_ref),
        Instruction::Getstatic(field_ref),
        Instruction::Dreturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()D", &instructions)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?.expect("value");
    assert_eq!(value, Value::F64(3.5));
    Ok(())
}

#[test]
fn putstatic_getstatic_object() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let field_ref = add_field_ref(&mut constant_pool, "Owner", "value", "Ljava/lang/Object;")?;
    let instructions = vec![
        Instruction::Aconst_null,
        Instruction::Putstatic(field_ref),
        Instruction::Getstatic(field_ref),
        Instruction::Areturn,
    ];
    let function =
        create_function_with_constant_pool(constant_pool, "()Ljava/lang/Object;", &instructions)?;
    let ctx = TestContext::new();
    let value = function.execute(&[], ctx.as_ptr())?;
    // Object return: null is modelled as Ptr(0).
    assert_eq!(value, Some(Value::Ptr(0)));
    Ok(())
}

#[test]
fn putfield_getfield_int() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let field_ref = add_field_ref(&mut constant_pool, "Owner", "value", "I")?;
    // Takes `this` as arg 0, stores arg 1 into this.value, returns this.value.
    let instructions = vec![
        // putfield this, arg
        Instruction::Aload_0,
        Instruction::Iload_1,
        Instruction::Putfield(field_ref),
        // getfield this
        Instruction::Aload_0,
        Instruction::Getfield(field_ref),
        Instruction::Ireturn,
    ];
    let function =
        create_function_with_constant_pool(constant_pool, "(Ljava/lang/Object;I)I", &instructions)?;
    let ctx = TestContext::new();
    let value = function
        .execute(&[Value::Ptr(42), Value::I32(987)], ctx.as_ptr())?
        .expect("value");
    assert_eq!(value, Value::I32(987));
    Ok(())
}

#[test]
fn getfield_null_raises_pending_exception() -> Result<()> {
    let mut constant_pool = ConstantPool::default();
    let field_ref = add_field_ref(&mut constant_pool, "Owner", "value", "I")?;
    let instructions = vec![
        Instruction::Aconst_null,
        Instruction::Getfield(field_ref),
        Instruction::Ireturn,
    ];
    let function = create_function_with_constant_pool(constant_pool, "()I", &instructions)?;
    let ctx = TestContext::new();
    // On pending exception, JIT returns None (the NONE discriminant).
    let value = function.execute(&[], ctx.as_ptr())?;
    assert!(value.is_none());
    assert_ne!(ctx.pending_exception(), 0);
    Ok(())
}
