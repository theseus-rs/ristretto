use crate::operand_stack::OperandStack;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, types};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.i2l>
pub(crate) fn i2l(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().sextend(types::I64, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.i2f>
pub(crate) fn i2f(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fcvt_from_sint(types::F32, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.i2d>
pub(crate) fn i2d(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fcvt_from_sint(types::F64, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.l2i>
pub(crate) fn l2i(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().ireduce(types::I32, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.l2f>
pub(crate) fn l2f(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fcvt_from_sint(types::F32, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.l2d>
pub(crate) fn l2d(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fcvt_from_sint(types::F64, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.f2i>
pub(crate) fn f2i(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fcvt_to_sint(types::I32, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.f2l>
pub(crate) fn f2l(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fcvt_to_sint(types::I64, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.f2d>
pub(crate) fn f2d(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fpromote(types::F64, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.d2i>
pub(crate) fn d2i(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fcvt_to_sint(types::I32, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.d2l>
pub(crate) fn d2l(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fcvt_to_sint(types::I64, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.d2f>
pub(crate) fn d2f(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().fdemote(types::F32, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.i2b>
pub(crate) fn i2b(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().ireduce(types::I8, value);
    let value = function_builder.ins().sextend(types::I32, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.i2c>
pub(crate) fn i2c(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    // TODO: verify i2c unsigned conversion
    let value = function_builder.ins().ireduce(types::I16, value);
    let value = function_builder.ins().sextend(types::I32, value);
    stack.push(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.i2s>
pub(crate) fn i2s(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop(function_builder);
    let value = function_builder.ins().ireduce(types::I16, value);
    let value = function_builder.ins().sextend(types::I32, value);
    stack.push(function_builder, value);
}
