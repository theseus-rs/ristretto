use crate::Error::{InvalidConstant, InvalidConstantIndex, UnsupportedType};
use crate::Result;
use cranelift::codegen::ir::Value;
use cranelift::prelude::{FunctionBuilder, InstBuilder, types};
use ristretto_classfile::{Constant, ConstantPool};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ldc>
pub(crate) fn ldc(
    constant_pool: &ConstantPool,
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u8,
) -> Result<()> {
    let index = u16::from(index);
    load_constant(constant_pool, function_builder, stack, index)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ldc_w>
pub(crate) fn ldc_w(
    constant_pool: &ConstantPool,
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u16,
) -> Result<()> {
    load_constant(constant_pool, function_builder, stack, index)
}

/// Load the constant at the specified index onto the stack
///
/// # Errors
/// if the constant is not an integer, float, string or class
fn load_constant(
    constant_pool: &ConstantPool,
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u16,
) -> Result<()> {
    let constant = constant_pool
        .get(index)
        .ok_or_else(|| InvalidConstantIndex(index))?;

    let value = match constant {
        Constant::Integer(value) => {
            let value = i64::from(*value);
            function_builder.ins().iconst(types::I32, value)
        }
        Constant::Float(value) => {
            let value = f64::from(*value);
            function_builder.ins().f64const(value)
        }
        Constant::String(utf8_index) => {
            let _utf8_value = constant_pool.try_get_utf8(*utf8_index)?;
            return Err(UnsupportedType(
                "loading string constants not supported".to_string(),
            ));
        }
        Constant::Class(class_index) => {
            let _class_name = constant_pool.try_get_utf8(*class_index)?;
            return Err(UnsupportedType(
                "loading class constants not supported".to_string(),
            ));
        }
        constant => {
            return Err(InvalidConstant {
                expected: "integer|float|string|class".to_string(),
                actual: format!("{constant:?}"),
            });
        }
    };
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ldc2_w>
pub(crate) fn ldc2_w(
    constant_pool: &ConstantPool,
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u16,
) -> Result<()> {
    let constant = constant_pool
        .get(index)
        .ok_or_else(|| InvalidConstantIndex(index))?;

    let value = match constant {
        Constant::Long(value) => function_builder.ins().iconst(types::I64, *value),
        Constant::Double(value) => function_builder.ins().f64const(*value),
        constant => {
            return Err(InvalidConstant {
                expected: "long|double".to_string(),
                actual: format!("{constant:?}"),
            });
        }
    };
    stack.push(value);
    Ok(())
}
