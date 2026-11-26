use crate::Result;
use crate::function::activation::RequestKind;
use crate::instruction::field::{FieldKind, resolve_field_descriptor};
use crate::instruction::{
    RequestContinuation, ThrowContext, emit_bci, emit_pending_exception_check, single_inst_result,
};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::codegen::ir::{FuncRef, Value};
use cranelift::prelude::{FunctionBuilder, InstBuilder, types};
use ristretto_classfile::ConstantPool;

/// # References
///
/// - [JVMS §6.5.getstatic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.getstatic)
#[expect(
    clippy::too_many_arguments,
    reason = "field emission context and owning request"
)]
pub(crate) fn getstatic(
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
    let bci = emit_bci(function_builder, throw_context);
    let field_ref = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_field_ref_index));
    let helper: FuncRef = match kind {
        FieldKind::Int => helpers.getstatic_int,
        FieldKind::Long => helpers.getstatic_long,
        FieldKind::Float => helpers.getstatic_float,
        FieldKind::Double => helpers.getstatic_double,
        FieldKind::Object => helpers.getstatic_object,
    };
    let synchronous = |builder: &mut FunctionBuilder| {
        let call = builder
            .ins()
            .call(helper, &[context_pointer, bci, field_ref]);
        let value = single_inst_result(builder, call)?;
        Ok(kind.bits(builder, value))
    };
    let bits = if let Some(continuation) = continuation {
        continuation.emit_bits(
            function_builder,
            stack,
            (RequestKind::GetStatic, cp_field_ref_index),
            &[],
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
/// - [JVMS §6.5.putstatic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.putstatic)
#[expect(
    clippy::too_many_arguments,
    reason = "field emission context and owning request"
)]
pub(crate) fn putstatic(
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
    let field_ref = function_builder
        .ins()
        .iconst(types::I32, i64::from(cp_field_ref_index));
    let bci = emit_bci(function_builder, throw_context);
    let helper: FuncRef = match kind {
        FieldKind::Int => helpers.putstatic_int,
        FieldKind::Long => helpers.putstatic_long,
        FieldKind::Float => helpers.putstatic_float,
        FieldKind::Double => helpers.putstatic_double,
        FieldKind::Object => helpers.putstatic_object,
    };
    let synchronous = |builder: &mut FunctionBuilder| {
        builder
            .ins()
            .call(helper, &[context_pointer, bci, field_ref, value]);
        Ok(builder.ins().iconst(types::I64, 0))
    };
    if let Some(continuation) = continuation {
        let bits = kind.bits(function_builder, value);
        continuation.emit_bits(
            function_builder,
            stack,
            (RequestKind::PutStatic, cp_field_ref_index),
            &[bits],
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
