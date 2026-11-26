//! Save allocation and field requests before the runtime may suspend their work.
use crate::Result;
use crate::compiler::continuation::CallContinuation;
use crate::function::activation::RequestKind;
use crate::operand_stack::OperandStack;
use cranelift::codegen::ir::{StackSlot, Value};
use cranelift::prelude::{FunctionBuilder, InstBuilder, types};
use ristretto_classfile::attributes::Instruction;

/// Private request tag, class/type operand and number of Java integer dimensions.
pub(crate) fn allocation_metadata(instruction: &Instruction) -> Option<(RequestKind, u16, usize)> {
    Some(match instruction {
        Instruction::New(index) => (RequestKind::NewObject, *index, 0),
        Instruction::Newarray(kind) => (RequestKind::NewPrimitiveArray, u16::from(kind.code()), 1),
        Instruction::Anewarray(index) => (RequestKind::NewReferenceArray, *index, 1),
        Instruction::Multianewarray(index, dimensions) => {
            (RequestKind::NewMultiArray, *index, usize::from(*dimensions))
        }
        _ => return None,
    })
}

pub(crate) struct RequestContinuation<'a> {
    pub call: CallContinuation<'a>,
    pub arguments: StackSlot,
    pub bci: usize,
}

impl RequestContinuation<'_> {
    /// Integer dimension cells are copied into owning state before native return.
    pub fn emit(
        &self,
        builder: &mut FunctionBuilder,
        stack: &mut OperandStack,
        metadata: (RequestKind, u16),
        dimensions: &[Value],
        synchronous: impl FnOnce(&mut FunctionBuilder) -> Result<Value>,
    ) -> Result<Value> {
        let bits = dimensions
            .iter()
            .map(|dimension| builder.ins().sextend(types::I64, *dimension))
            .collect::<Vec<_>>();
        self.emit_bits(builder, stack, metadata, &bits, synchronous)
    }

    /// Store one raw eight-byte cell per logical argument, preserving float bits
    /// and stable identities owned by the incoming frame snapshot.
    pub fn emit_bits(
        &self,
        builder: &mut FunctionBuilder,
        stack: &mut OperandStack,
        metadata: (RequestKind, u16),
        bits: &[Value],
        synchronous: impl FnOnce(&mut FunctionBuilder) -> Result<Value>,
    ) -> Result<Value> {
        let pointer_type = builder.func.dfg.value_type(self.call.state);
        for (index, bits) in bits.iter().enumerate() {
            builder.ins().stack_store(
                pointer_type,
                *bits,
                self.arguments,
                i32::try_from(index * 8)?,
            );
        }
        self.call.emit(
            builder,
            stack,
            self.arguments,
            bits.len(),
            self.bci,
            metadata,
            synchronous,
        )
    }
}
