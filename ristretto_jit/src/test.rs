use crate::Error::UnsupportedTargetISA;
use crate::Result;
use cranelift::codegen::settings::Flags;
use cranelift::codegen::{Context, settings};
use cranelift::frontend::FunctionBuilderContext;
use cranelift::jit::{JITBuilder, JITModule};
use cranelift::module::{Module, default_libcall_names};

/// Returns the objects needed to construct a `FunctionBuilder`.
pub(crate) fn create_function_builder_contexts() -> Result<(Context, FunctionBuilderContext)> {
    let isa_builder = cranelift::native::builder().map_err(UnsupportedTargetISA)?;
    let flag_builder = settings::builder();
    let flags = Flags::new(flag_builder);
    let target_isa = isa_builder.finish(flags)?;
    let jit_builder = JITBuilder::with_isa(target_isa, default_libcall_names());
    let jit_module = JITModule::new(jit_builder);
    let module_context = jit_module.make_context();
    let function_context = FunctionBuilderContext::new();
    Ok((module_context, function_context))
}
