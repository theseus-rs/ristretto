use crate::ArrayAccessKind;
use crate::Result;
use crate::function::activation::RequestKind;
use crate::instruction::FieldKind;
use crate::instruction::{
    RequestContinuation, ThrowContext, emit_bci, emit_pending_exception_check, single_inst_result,
};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::codegen::ir::Value;
use cranelift::prelude::{FunctionBuilder, InstBuilder, types};
use ristretto_classfile::attributes::ArrayType;
use ristretto_classfile::attributes::Instruction;

/// # References
///
/// - [JVMS §6.5.newarray](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.newarray)
pub(crate) fn newarray(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    atype: &ArrayType,
    context_pointer: Value,
    helpers: &RuntimeHelpers,
    throw_context: &ThrowContext<'_>,
    continuation: Option<RequestContinuation<'_>>,
) -> Result<()> {
    let count = stack.pop_int(function_builder)?;

    let helper = match atype {
        ArrayType::Boolean => helpers.new_bool_array,
        ArrayType::Byte => helpers.new_byte_array,
        ArrayType::Char => helpers.new_char_array,
        ArrayType::Short => helpers.new_short_array,
        ArrayType::Int => helpers.new_int_array,
        ArrayType::Long => helpers.new_long_array,
        ArrayType::Float => helpers.new_float_array,
        ArrayType::Double => helpers.new_double_array,
    };

    let bci = emit_bci(function_builder, throw_context);
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder.ins().call(helper, &[context_pointer, bci, count]);
        single_inst_result(builder, call)
    };
    let array_ptr = if let Some(continuation) = continuation {
        continuation.emit(
            function_builder,
            stack,
            (RequestKind::NewPrimitiveArray, u16::from(atype.code())),
            &[count],
            synchronous,
        )?
    } else {
        synchronous(function_builder)?
    };

    // Push as object reference (Gc pointer encoded as i64)
    stack.push_object(function_builder, array_ptr)?;
    emit_pending_exception_check(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
    )
}

/// Request kind, private opcode and argument arity share one instruction mapping.
pub(crate) fn array_access_metadata(instruction: &Instruction) -> Option<(ArrayAccessKind, u16)> {
    let kind = ArrayAccessKind::from_instruction(instruction)?;
    Some((kind, u16::from(instruction.code())))
}

/// Save arguments before any lock wait or throwable allocation, preserving raw float bits.
pub(crate) fn array_access(
    builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context: Value,
    throw_context: &ThrowContext<'_>,
    access: (ArrayAccessKind, u16),
    continuation: Option<RequestContinuation<'_>>,
) -> Result<()> {
    use ArrayAccessKind as A;
    let (access, code) = access;
    let (helper, kind) = match access {
        A::Length => (helpers.arraylength, FieldKind::Int),
        A::IntLoad => (helpers.iaload, FieldKind::Int),
        A::LongLoad => (helpers.laload, FieldKind::Long),
        A::FloatLoad => (helpers.faload, FieldKind::Float),
        A::DoubleLoad => (helpers.daload, FieldKind::Double),
        A::ReferenceLoad => (helpers.aaload, FieldKind::Object),
        A::ByteLoad => (helpers.baload, FieldKind::Int),
        A::CharLoad => (helpers.caload, FieldKind::Int),
        A::ShortLoad => (helpers.saload, FieldKind::Int),
        A::IntStore => (helpers.iastore, FieldKind::Int),
        A::LongStore => (helpers.lastore, FieldKind::Long),
        A::FloatStore => (helpers.fastore, FieldKind::Float),
        A::DoubleStore => (helpers.dastore, FieldKind::Double),
        A::ByteStore => (helpers.bastore, FieldKind::Int),
        A::CharStore => (helpers.castore, FieldKind::Int),
        A::ShortStore => (helpers.sastore, FieldKind::Int),
    };
    let store = access.argument_count() == 3;
    let value = if store {
        Some(match kind {
            FieldKind::Int => stack.pop_int(builder)?,
            FieldKind::Long => stack.pop_long(builder)?,
            FieldKind::Float => stack.pop_float(builder)?,
            FieldKind::Double => stack.pop_double(builder)?,
            FieldKind::Object => stack.pop_object(builder)?,
        })
    } else {
        None
    };
    let index = if access == A::Length {
        None
    } else {
        Some(stack.pop_int(builder)?)
    };
    let array = stack.pop_object(builder)?;
    let bci = emit_bci(builder, throw_context);
    let mut arguments = vec![context, bci, array];
    arguments.extend(index);
    arguments.extend(value);
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder.ins().call(helper, &arguments);
        if store {
            Ok(builder.ins().iconst(types::I64, 0))
        } else {
            let result = single_inst_result(builder, call)?;
            Ok(kind.bits(builder, result))
        }
    };
    let bits = if let Some(continuation) = continuation {
        let mut cells = vec![array];
        if let Some(index) = index {
            cells.push(builder.ins().sextend(types::I64, index));
        }
        if let Some(value) = value {
            cells.push(kind.bits(builder, value));
        }
        continuation.emit_bits(
            builder,
            stack,
            (RequestKind::ArrayAccess, code),
            &cells,
            synchronous,
        )?
    } else {
        synchronous(builder)?
    };
    if !store {
        let value = kind.value(builder, bits);
        match kind {
            FieldKind::Int => stack.push_int(builder, value)?,
            FieldKind::Long => stack.push_long(builder, value)?,
            FieldKind::Float => stack.push_float(builder, value)?,
            FieldKind::Double => stack.push_double(builder, value)?,
            FieldKind::Object => stack.push_object(builder, value)?,
        }
    }
    emit_pending_exception_check(builder, stack, helpers, context, throw_context)
}
