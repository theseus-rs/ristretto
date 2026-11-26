use crate::compiler::continuation::CallContinuation;
use crate::function::activation::RequestKind;
use crate::instruction::{
    ThrowContext, emit_bci, emit_pending_exception_check, single_inst_result,
};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use crate::{Error, Result};
use cranelift::codegen::ir::{MemFlagsData, StackSlot, Value};
use cranelift::prelude::{FunctionBuilder, InstBuilder, types};
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{BaseType, Constant, ConstantPool, FieldType};

#[derive(Clone, Copy)]
pub(crate) enum CallTarget {
    Static(u16),
    Special(u16),
    Virtual(u16),
    Interface(u16, u8),
}

impl CallTarget {
    pub(crate) fn from_instruction(instruction: &Instruction) -> Option<Self> {
        Some(match instruction {
            Instruction::Invokestatic(index) => Self::Static(*index),
            Instruction::Invokespecial(index) => Self::Special(*index),
            Instruction::Invokevirtual(index) => Self::Virtual(*index),
            Instruction::Invokeinterface(index, count) => Self::Interface(*index, *count),
            _ => return None,
        })
    }
    pub(crate) fn metadata(self) -> (u16, usize, RequestKind) {
        match self {
            Self::Static(index) => (index, 0, RequestKind::InvokeStatic),
            Self::Special(index) => (index, 1, RequestKind::InvokeSpecial),
            Self::Virtual(index) => (index, 1, RequestKind::InvokeVirtual),
            Self::Interface(index, _) => (index, 1, RequestKind::InvokeInterface),
        }
    }
}

pub(crate) fn call_signature(
    pool: &ConstantPool,
    target: CallTarget,
) -> Result<(Vec<FieldType>, Option<FieldType>)> {
    let index = target.metadata().0;
    let ((
        CallTarget::Interface(..),
        Constant::InterfaceMethodRef {
            name_and_type_index,
            ..
        },
    )
    | (
        CallTarget::Virtual(_),
        Constant::MethodRef {
            name_and_type_index,
            ..
        },
    )
    | (
        CallTarget::Static(_) | CallTarget::Special(_),
        Constant::MethodRef {
            name_and_type_index,
            ..
        }
        | Constant::InterfaceMethodRef {
            name_and_type_index,
            ..
        },
    )) = (target, pool.try_get(index)?)
    else {
        return Err(ristretto_classfile::Error::InvalidConstantPoolIndexType(index).into());
    };
    let (_, descriptor) = pool.try_get_name_and_type(*name_and_type_index)?;
    Ok(FieldType::parse_method_descriptor(
        pool.try_get_utf8(*descriptor)?,
    )?)
}

/// Serialize one logical value per eight-byte cell. References remain stable IDs;
/// the caller's frame snapshot owns them while the runtime resolves and invokes.
#[expect(
    clippy::too_many_arguments,
    reason = "instruction emission context plus the shared outgoing call buffer"
)]
pub(crate) fn invoke(
    pool: &ConstantPool,
    builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context: Value,
    throw: &ThrowContext<'_>,
    arguments: StackSlot,
    target: CallTarget,
    continuation: Option<CallContinuation<'_>>,
) -> Result<()> {
    let (index, receiver, helper) = match target {
        CallTarget::Static(index) => (index, 0, helpers.invokestatic),
        CallTarget::Special(index) => (index, 1, helpers.invokespecial),
        CallTarget::Virtual(index) => (index, 1, helpers.invokevirtual),
        CallTarget::Interface(index, _) => (index, 1, helpers.invokeinterface),
    };
    let (parameters, result) = call_signature(pool, target)?;
    let slots: usize = parameters
        .iter()
        .map(|kind| {
            if matches!(kind, FieldType::Base(BaseType::Long | BaseType::Double)) {
                2
            } else {
                1
            }
        })
        .sum();
    if slots + receiver > crate::CALL_ARGUMENT_CAPACITY {
        return Err(Error::UnsupportedMethod(
            "compiled call exceeds the Java parameter-slot limit".to_owned(),
        ));
    }
    if let CallTarget::Interface(_, count) = target
        && usize::from(count) != slots + receiver
    {
        return Err(Error::UnsupportedMethod(
            "interface call count disagrees with the Java parameter-slot count".to_owned(),
        ));
    }
    let pointer_type = builder.func.dfg.value_type(context);
    for (position, kind) in parameters.iter().enumerate().rev() {
        let bits = argument_bits(builder, stack, kind)?;
        builder.ins().stack_store(
            pointer_type,
            bits,
            arguments,
            i32::try_from((position + receiver) * 8)?,
        );
    }
    if receiver != 0 {
        let object = stack.pop_object(builder)?;
        builder
            .ins()
            .stack_store(pointer_type, object, arguments, 0);
    }
    let pointer = builder.ins().stack_addr(pointer_type, arguments, 0);
    let count = builder
        .ins()
        .iconst(types::I32, i64::try_from(parameters.len() + receiver)?);
    let index = builder.ins().iconst(types::I32, i64::from(index));
    let bci = emit_bci(builder, throw);
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder
            .ins()
            .call(helper, &[context, bci, index, pointer, count]);
        single_inst_result(builder, call)
    };
    let bits = if let Some(continuation) = continuation {
        continuation.emit(
            builder,
            stack,
            arguments,
            parameters.len() + receiver,
            throw.program_counter,
            (target.metadata().2, target.metadata().0),
            synchronous,
        )?
    } else {
        synchronous(builder)?
    };
    // Do not make a failed call's placeholder result part of the operand state.
    emit_pending_exception_check(builder, stack, helpers, context, throw)?;
    if let Some(kind) = result {
        push_result(builder, stack, &kind, bits)?;
    }
    Ok(())
}

fn argument_bits(
    builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    kind: &FieldType,
) -> Result<Value> {
    Ok(match kind {
        FieldType::Object(_) | FieldType::Array(_) => stack.pop_object(builder)?,
        FieldType::Base(BaseType::Long) => stack.pop_long(builder)?,
        FieldType::Base(BaseType::Double) => {
            let value = stack.pop_double(builder)?;
            builder
                .ins()
                .bitcast(types::I64, MemFlagsData::new(), value)
        }
        FieldType::Base(BaseType::Float) => {
            let value = stack.pop_float(builder)?;
            let bits = builder
                .ins()
                .bitcast(types::I32, MemFlagsData::new(), value);
            builder.ins().uextend(types::I64, bits)
        }
        FieldType::Base(_) => {
            let value = stack.pop_int(builder)?;
            builder.ins().sextend(types::I64, value)
        }
    })
}

fn push_result(
    builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    kind: &FieldType,
    bits: Value,
) -> Result<()> {
    match kind {
        FieldType::Object(_) | FieldType::Array(_) => stack.push_object(builder, bits),
        FieldType::Base(BaseType::Long) => stack.push_long(builder, bits),
        FieldType::Base(BaseType::Double) => {
            let value = builder.ins().bitcast(types::F64, MemFlagsData::new(), bits);
            stack.push_double(builder, value)
        }
        FieldType::Base(BaseType::Float) => {
            let bits = builder.ins().ireduce(types::I32, bits);
            let value = builder.ins().bitcast(types::F32, MemFlagsData::new(), bits);
            stack.push_float(builder, value)
        }
        FieldType::Base(_) => {
            let value = builder.ins().ireduce(types::I32, bits);
            stack.push_int(builder, value)
        }
    }
}
