use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, Value};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.return>
pub(crate) fn r#return(function_builder: &mut FunctionBuilder, _return_pointer: Value) {
    // This optimization relies on the fact that when returning from a "void" method, the default
    // return pointer values should be initialized to 0 which is the same as the following code:
    // let value = function_builder.ins().iconst(types::I64, 0);
    // let discriminate = i64::from(jit_value::NONE);
    // let discriminate = function_builder.ins().iconst(types::I8, discriminate);
    // function_builder
    //     .ins()
    //     .store(MemFlags::new(), discriminate, return_pointer, 0);
    // function_builder
    //     .ins()
    //     .store(MemFlags::new(), value, return_pointer, 8);
    function_builder.ins().return_(&[]);
}
