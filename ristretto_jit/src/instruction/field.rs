use crate::Error::InternalError;
use crate::Result;
use crate::function::activation::RequestKind;
use crate::instruction::{
    RequestContinuation, ThrowContext, emit_bci, emit_pending_exception_check, single_inst_result,
};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::codegen::ir::{FuncRef, MemFlagsData, Value};
use cranelift::prelude::{FunctionBuilder, InstBuilder, types};
use ristretto_classfile::{ConstantPool, JavaStr};

pub(crate) fn field_metadata(
    instruction: &ristretto_classfile::attributes::Instruction,
) -> Option<(RequestKind, u16, usize)> {
    use ristretto_classfile::attributes::Instruction;
    Some(match instruction {
        Instruction::Getstatic(index) => (RequestKind::GetStatic, *index, 0),
        Instruction::Putstatic(index) => (RequestKind::PutStatic, *index, 1),
        Instruction::Getfield(index) => (RequestKind::GetField, *index, 1),
        Instruction::Putfield(index) => (RequestKind::PutField, *index, 2),
        _ => return None,
    })
}

/// Describes how a field value is represented in JIT code.
#[derive(Clone, Copy)]
pub(crate) enum FieldKind {
    Int,
    Long,
    Float,
    Double,
    Object,
}

impl FieldKind {
    pub(crate) fn bits(self, builder: &mut FunctionBuilder, value: Value) -> Value {
        match self {
            Self::Int => builder.ins().sextend(types::I64, value),
            Self::Float => {
                let value = builder
                    .ins()
                    .bitcast(types::I32, MemFlagsData::new(), value);
                builder.ins().uextend(types::I64, value)
            }
            Self::Double => builder
                .ins()
                .bitcast(types::I64, MemFlagsData::new(), value),
            Self::Long | Self::Object => value,
        }
    }

    pub(crate) fn value(self, builder: &mut FunctionBuilder, bits: Value) -> Value {
        match self {
            Self::Int => builder.ins().ireduce(types::I32, bits),
            Self::Float => {
                let value = builder.ins().ireduce(types::I32, bits);
                builder
                    .ins()
                    .bitcast(types::F32, MemFlagsData::new(), value)
            }
            Self::Double => builder.ins().bitcast(types::F64, MemFlagsData::new(), bits),
            Self::Long | Self::Object => bits,
        }
    }

    pub(crate) fn from_descriptor(descriptor: &JavaStr) -> Result<Self> {
        let bytes = descriptor.as_bytes();
        let first = bytes.first().copied().ok_or_else(|| {
            InternalError(["empty field descriptor: ", &descriptor.to_str_lossy()].concat())
        })?;
        Ok(match first {
            b'B' | b'C' | b'S' | b'Z' | b'I' => FieldKind::Int,
            b'J' => FieldKind::Long,
            b'F' => FieldKind::Float,
            b'D' => FieldKind::Double,
            b'L' | b'[' => FieldKind::Object,
            _ => {
                return Err(InternalError(
                    ["unsupported field descriptor: ", &descriptor.to_str_lossy()].concat(),
                ));
            }
        })
    }
}

/// Resolves the field descriptor string for a `CONSTANT_Fieldref_info` entry.
pub(crate) fn resolve_field_descriptor<'a>(
    constant_pool: &'a ConstantPool,
    cp_field_ref_index: u16,
) -> Result<&'a JavaStr> {
    let (_class_index, name_and_type_index) =
        constant_pool.try_get_field_ref(cp_field_ref_index)?;
    let (_name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    Ok(constant_pool.try_get_utf8(*descriptor_index)?)
}

/// # References
///
/// - [JVMS §6.5.getfield](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.getfield)
#[expect(
    clippy::too_many_arguments,
    reason = "field emission context and owning request"
)]
pub(crate) fn getfield(
    constant_pool: &ConstantPool,
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    cp_field_ref_index: u16,
    continuation: Option<RequestContinuation<'_>>,
) -> Result<()> {
    let descriptor = resolve_field_descriptor(constant_pool, cp_field_ref_index)?;
    let kind = FieldKind::from_descriptor(descriptor)?;
    let object = stack.pop_object(function_builder)?;
    let field_ref = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_field_ref_index));
    let bci = emit_bci(function_builder, throw_context);

    let helper: FuncRef = match kind {
        FieldKind::Int => helpers.getfield_int,
        FieldKind::Long => helpers.getfield_long,
        FieldKind::Float => helpers.getfield_float,
        FieldKind::Double => helpers.getfield_double,
        FieldKind::Object => helpers.getfield_object,
    };
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder
            .ins()
            .call(helper, &[context_pointer, bci, field_ref, object]);
        let value = single_inst_result(builder, call)?;
        Ok(kind.bits(builder, value))
    };
    let bits = if let Some(continuation) = continuation {
        continuation.emit_bits(
            function_builder,
            stack,
            (RequestKind::GetField, cp_field_ref_index),
            &[object],
            synchronous,
        )?
    } else {
        synchronous(function_builder)?
    };
    let result = kind.value(function_builder, bits);
    match kind {
        FieldKind::Int => stack.push_int(function_builder, result)?,
        FieldKind::Long => stack.push_long(function_builder, result)?,
        FieldKind::Float => stack.push_float(function_builder, result)?,
        FieldKind::Double => stack.push_double(function_builder, result)?,
        FieldKind::Object => stack.push_object(function_builder, result)?,
    }
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}

/// # References
///
/// - [JVMS §6.5.putfield](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.putfield)
#[expect(
    clippy::too_many_arguments,
    reason = "field emission context and owning request"
)]
pub(crate) fn putfield(
    constant_pool: &ConstantPool,
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
    cp_field_ref_index: u16,
    continuation: Option<RequestContinuation<'_>>,
) -> Result<()> {
    let descriptor = resolve_field_descriptor(constant_pool, cp_field_ref_index)?;
    let kind = FieldKind::from_descriptor(descriptor)?;
    let value = match kind {
        FieldKind::Int => stack.pop_int(function_builder)?,
        FieldKind::Long => stack.pop_long(function_builder)?,
        FieldKind::Float => stack.pop_float(function_builder)?,
        FieldKind::Double => stack.pop_double(function_builder)?,
        FieldKind::Object => stack.pop_object(function_builder)?,
    };
    let object = stack.pop_object(function_builder)?;
    let field_ref = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_field_ref_index));
    let bci = emit_bci(function_builder, throw_context);
    let helper: FuncRef = match kind {
        FieldKind::Int => helpers.putfield_int,
        FieldKind::Long => helpers.putfield_long,
        FieldKind::Float => helpers.putfield_float,
        FieldKind::Double => helpers.putfield_double,
        FieldKind::Object => helpers.putfield_object,
    };
    let synchronous = |builder: &mut FunctionBuilder| {
        builder
            .ins()
            .call(helper, &[context_pointer, bci, field_ref, object, value]);
        Ok(builder.ins().iconst(types::I64, 0))
    };
    if let Some(continuation) = continuation {
        let bits = kind.bits(function_builder, value);
        continuation.emit_bits(
            function_builder,
            stack,
            (RequestKind::PutField, cp_field_ref_index),
            &[object, bits],
            synchronous,
        )?;
    } else {
        synchronous(function_builder)?;
    }
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}
