use crate::Error::{InvalidConstant, InvalidConstantIndex};
use crate::Result;
use crate::function::activation::RequestKind;
use crate::instruction::{
    RequestContinuation, ThrowContext, emit_bci, emit_pending_exception_check, single_inst_result,
};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::prelude::{FunctionBuilder, InstBuilder, types};
use ristretto_classfile::{Constant, ConstantPool, attributes::Instruction};

/// Only reference constants require an owning runtime request. Numeric constants
/// remain immediate values; unsupported constant kinds still fail compilation.
pub(crate) fn reference_constant_index(
    instruction: &Instruction,
    pool: &ConstantPool,
) -> Option<u16> {
    let index = match instruction {
        Instruction::Ldc(index) => u16::from(*index),
        Instruction::Ldc_w(index) => *index,
        _ => return None,
    };
    matches!(
        pool.get(index),
        Some(Constant::String(_) | Constant::Class(_))
    )
    .then_some(index)
}

/// # References
/// - [JVMS §6.5.ldc](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ldc)
#[expect(
    clippy::too_many_arguments,
    reason = "constant metadata and runtime continuation state"
)]
pub(crate) fn ldc(
    constant_pool: &ConstantPool,
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    index: u16,
    helpers: &RuntimeHelpers,
    context_pointer: cranelift::prelude::Value,
    throw_context: &ThrowContext<'_>,
    continuation: Option<RequestContinuation<'_>>,
) -> Result<()> {
    let Some(Constant::String(name) | Constant::Class(name)) = constant_pool.get(index) else {
        return load_constant(constant_pool, function_builder, stack, index);
    };
    constant_pool.try_get_utf8(*name)?;
    let bci = emit_bci(function_builder, throw_context);
    let constant_index = function_builder.ins().iconst(types::I32, i64::from(index));
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder.ins().call(
            helpers.load_constant,
            &[context_pointer, bci, constant_index],
        );
        single_inst_result(builder, call)
    };
    let result = if let Some(continuation) = continuation {
        continuation.emit_bits(
            function_builder,
            stack,
            (RequestKind::Constant, index),
            &[],
            synchronous,
        )?
    } else {
        synchronous(function_builder)?
    };
    stack.push_object(function_builder, result)?;
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}

/// Load the constant at the specified index onto the stack
///
/// # Errors
///
/// if the constant is not an integer, float, string or class
fn load_constant(
    constant_pool: &ConstantPool,
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let constant = constant_pool
        .get(index)
        .ok_or_else(|| InvalidConstantIndex(index))?;

    match constant {
        Constant::Integer(value) => {
            let value = i64::from(*value);
            let value = function_builder.ins().iconst(types::I32, value);
            stack.push_int(function_builder, value)?;
        }
        Constant::Float(value) => {
            let value = function_builder.ins().f32const(*value);
            stack.push_float(function_builder, value)?;
        }
        constant => {
            return Err(InvalidConstant {
                expected: "integer|float|string|class".to_string(),
                actual: format!("{constant:?}"),
            });
        }
    }
    Ok(())
}

/// # References
/// - [JVMS §6.5.ldc2_w](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ldc2_w)
pub(crate) fn ldc2_w(
    constant_pool: &ConstantPool,
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let constant = constant_pool
        .get(index)
        .ok_or_else(|| InvalidConstantIndex(index))?;

    match constant {
        Constant::Long(value) => {
            let value = function_builder.ins().iconst(types::I64, *value);
            stack.push(value)?;
        }
        Constant::Double(value) => {
            let value = function_builder.ins().f64const(*value);
            stack.push(value)?;
        }
        constant => {
            return Err(InvalidConstant {
                expected: "long|double".to_string(),
                actual: format!("{constant:?}"),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Compiler;
    use cranelift::frontend::FunctionBuilderContext;
    use cranelift::module::Module;

    #[test]
    fn test_load_constant_error() -> Result<()> {
        let compiler = Compiler::new()?;
        let jit_module = compiler.jit_module(&[]);
        let mut module_context = jit_module.make_context();
        let mut function_context = FunctionBuilderContext::new();
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);

        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_long(42)?;
        let mut stack = OperandStack::with_capacity(1);

        let result = load_constant(&constant_pool, &mut function_builder, &mut stack, index);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_ldc2_w_constant_error() -> Result<()> {
        let compiler = Compiler::new()?;
        let jit_module = compiler.jit_module(&[]);
        let mut module_context = jit_module.make_context();
        let mut function_context = FunctionBuilderContext::new();
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);

        let mut constant_pool = ConstantPool::new();
        let index = constant_pool.add_integer(42)?;
        let mut stack = OperandStack::with_capacity(1);

        let result = ldc2_w(&constant_pool, &mut function_builder, &mut stack, index);
        assert!(result.is_err());
        Ok(())
    }
}
