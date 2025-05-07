use cranelift::codegen::ir::Value;
use cranelift::frontend::FunctionBuilder;

/// Operand stack for the JIT compiler.
pub struct OperandStack {
    stack: Vec<Value>,
}

impl OperandStack {
    /// Creates a new operand stack with the specified capacity.
    pub fn with_capacity(_function_builder: &mut FunctionBuilder, capacity: u16) -> Self {
        OperandStack {
            stack: Vec::with_capacity(usize::from(capacity)),
        }
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, _function_builder: &mut FunctionBuilder, value: Value) {
        self.stack.push(value);
    }

    /// Pops a value from the stack.
    pub fn pop(&mut self, _function_builder: &mut FunctionBuilder) -> Value {
        self.stack.pop().expect("stack underflow")
    }
}
