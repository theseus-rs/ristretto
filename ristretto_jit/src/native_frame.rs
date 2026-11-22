//! Extract the backend's actual frame span using a null stack-map entry.
//!
//! These maps are size metadata, not the VM's GC root ownership protocol.
use crate::{Error, Result};
use cranelift::codegen::Context;
use cranelift::codegen::ir::{Opcode, StackSlot, UserStackMapEntry};
use cranelift::jit::JITModule;
use cranelift::module::{Linkage, Module};
use cranelift::prelude::*;

pub(super) const PROBE_SYMBOL: &str = "ristretto_jit_native_frame_probe";

const FRAME_SETUP_BYTES: usize = 16;
const WORD_ALIGN_SHIFT: u8 = 3; // Eight-byte stack slot alignment.

pub(super) fn word_stack_slot(builder: &mut FunctionBuilder, words: usize) -> Result<StackSlot> {
    let bytes = u32::try_from(words)?
        .checked_mul(types::I64.bytes())
        .ok_or_else(|| Error::InternalError("stack slot size overflow".to_owned()))?;
    Ok(builder.create_sized_stack_slot(StackSlotData::new(
        StackSlotKind::ExplicitSlot,
        bytes,
        WORD_ALIGN_SHIFT,
    )))
}

pub(super) fn word_offset(word: usize) -> Result<i32> {
    Ok(i32::try_from(
        word.checked_mul(size_of::<u64>())
            .ok_or_else(|| Error::InternalError("continuation offset overflow".to_owned()))?,
    )?)
}

pub(super) extern "C" fn probe() {}

pub(super) struct SizeProbe(StackSlot);

impl SizeProbe {
    pub(super) fn new(builder: &mut FunctionBuilder) -> Self {
        Self(builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            types::I64.bytes(),
            WORD_ALIGN_SHIFT,
        )))
    }

    pub(super) fn initialize(&self, builder: &mut FunctionBuilder) {
        let null = builder.ins().iconst(types::I64, 0);
        builder.ins().stack_store(types::I64, null, self.0, 0);
    }

    /// A Rust slice of nonzero-sized Values cannot have `usize::MAX` elements.
    /// Keeping this ABI-input branch opaque to the backend retains a safepoint
    /// for helper-free methods, without executing an extra helper on valid calls.
    pub(super) fn emit_cold_path(
        &self,
        module: &mut JITModule,
        builder: &mut FunctionBuilder,
        arguments_length: Value,
        body: Block,
        failure: Block,
    ) -> Result<()> {
        let cold = builder.create_block();
        builder.set_cold_block(cold);
        let impossible = builder.ins().icmp_imm_s(IntCC::Equal, arguments_length, -1);
        builder.ins().brif(impossible, cold, &[], body, &[]);
        builder.switch_to_block(cold);
        self.initialize(builder);
        let signature = module.make_signature();
        let id = module.declare_function(PROBE_SYMBOL, Linkage::Import, &signature)?;
        let function = module.declare_func_in_func(id, builder.func);
        let call = builder.ins().call(function, &[]);
        builder.func.dfg.append_user_stack_map_entry(
            call,
            UserStackMapEntry {
                ty: types::I64,
                slot: self.0,
                offset: 0,
            },
        );
        builder.ins().jump(failure, &[]);
        Ok(())
    }
}

pub(super) fn stack_bytes(context: &Context) -> Result<usize> {
    if !context.func.dynamic_stack_slots.is_empty()
        || context
            .func
            .layout
            .blocks()
            .flat_map(|block| context.func.layout.block_insts(block))
            .any(|inst| {
                matches!(
                    context.func.dfg.insts[inst].opcode(),
                    Opcode::ReturnCall | Opcode::ReturnCallIndirect
                )
            })
    {
        return Err(Error::UnsupportedMethod(
            "dynamic or tail-call frame sizing is not supported".to_owned(),
        ));
    }
    let code = context
        .compiled_code()
        .ok_or_else(|| Error::InternalError("missing compiled frame metadata".to_owned()))?;
    let maps = code.buffer.user_stack_maps();
    let (_, span, _) = maps
        .first()
        .ok_or_else(|| Error::UnsupportedMethod("native frame size unavailable".to_owned()))?;
    if maps.iter().any(|(_, other, _)| other != span) {
        return Err(Error::UnsupportedMethod(
            "nonuniform native frame size".to_owned(),
        ));
    }
    // Cranelift x64, aarch64 and riscv64 use at most 16 bytes for
    // FP/return-address setup. active_size covers clobbers, fixed slots/spills
    // and outgoing args. This compiler emits no tail calls or dynamic slots.
    usize::try_from(*span)?
        .checked_add(FRAME_SETUP_BYTES)
        .ok_or_else(|| Error::InternalError("native frame size overflow".to_owned()))
}

#[cfg(test)]
#[path = "native_frame_spill_tests.rs"]
mod spill_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Compiler;
    use crate::JitValue;
    use cranelift::codegen::ir::MemFlagsData;

    #[test]
    fn missing_backend_frame_metadata_is_rejected() -> Result<()> {
        let compiler = Compiler::new()?;
        let mut module = compiler.jit_module(&[]);
        let signature = Compiler::signature(&module);
        let id = module.declare_function("missing_frame_metadata", Linkage::Local, &signature)?;
        let mut context = module.make_context();
        context.func.signature = signature;
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let entry = builder.create_block();
        builder.switch_to_block(entry);
        builder.append_block_params_for_function_params(entry);
        builder.ins().return_(&[]);
        builder.seal_all_blocks();
        builder.finalize(module.target_config());
        module.define_function(id, &mut context)?;
        assert!(matches!(
            stack_bytes(&context),
            Err(Error::UnsupportedMethod(_))
        ));
        // Safety: the rejected fixture never yields an executable function owner.
        unsafe { module.free_memory() };
        Ok(())
    }

    fn large_frame(bytes: u32, argument_count: usize) -> Result<crate::Function> {
        let compiler = Compiler::new()?;
        let mut module = compiler.jit_module(&[]);
        let signature = Compiler::signature(&module);
        let id = module.declare_function("large_native_frame", Linkage::Local, &signature)?;
        let mut context = module.make_context();
        context.func.signature = signature;
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut context.func, &mut builder_context);
        let entry = builder.create_block();
        let body = builder.create_block();
        let failure = builder.create_block();
        builder.switch_to_block(entry);
        builder.append_block_params_for_function_params(entry);
        let (_, count, _, result) = Compiler::function_pointers(&mut builder, entry)?;
        let probe = SizeProbe::new(&mut builder);
        probe.emit_cold_path(&mut module, &mut builder, count, body, failure)?;
        builder.switch_to_block(body);
        let slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            bytes,
            4,
        ));
        let value = builder.ins().iconst(types::I64, 42);
        builder.ins().stack_store(types::I64, value, slot, 0);
        builder.ins().stack_store(
            types::I64,
            value,
            slot,
            i32::try_from(bytes - types::I64.bytes())?,
        );
        let value = builder.ins().stack_load(
            types::I64,
            types::I64,
            slot,
            i32::try_from(bytes - types::I64.bytes())?,
        );
        builder
            .ins()
            .store(MemFlagsData::trusted(), value, result, 0);
        builder.ins().return_(&[]);
        builder.switch_to_block(failure);
        builder.ins().return_(&[]);
        builder.seal_all_blocks();
        builder.finalize(module.target_config());
        module.define_function(id, &mut context)?;
        let stack_bytes = stack_bytes(&context)?;
        assert!(stack_bytes >= usize::try_from(bytes)? + FRAME_SETUP_BYTES);
        assert_eq!(
            context
                .compiled_code()
                .expect("compiled code")
                .buffer
                .user_stack_maps()
                .len(),
            1
        );
        module.finalize_definitions()?;
        let entry = module.get_finalized_function(id);
        // Safety: the generated fixture uses the same four-argument ABI as Function.
        let entry = unsafe {
            std::mem::transmute::<
                *const u8,
                unsafe extern "C" fn(*const JitValue, usize, *mut JitValue, *const u8),
            >(entry)
        };
        Ok(crate::Function::with_module(
            entry,
            module,
            stack_bytes,
            vec![crate::jit_value::I32; argument_count],
        ))
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn oversized_native_frames_are_rejected_before_entry() -> std::io::Result<()> {
        let mut command = std::process::Command::new("sh");
        command
            .args(["-c", "ulimit -c 0; exec \"$@\"", "frame-size-test"])
            .arg(std::env::current_exe()?)
            .args([
                "--exact",
                "compiler::native_frame::tests::large_frame_child",
                "--ignored",
                "--nocapture",
            ]);
        command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        let mut child = command.spawn()?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        loop {
            if child.try_wait()?.is_some() {
                break;
            }
            if std::time::Instant::now() >= deadline {
                child.kill()?;
                let _ = child.wait()?;
                return Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "native frame child timed out",
                ));
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        let output = child.wait_with_output()?;
        assert!(output.status.success(), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stdout).contains("large native frame recovery passed")
        );
        Ok(())
    }

    #[test]
    #[ignore = "bounded subprocess fixture for generated-frame stack exhaustion"]
    fn large_frame_child() -> Result<()> {
        let mut observed = 0_u64;
        let context = (&raw mut observed).cast::<u8>().cast_const();
        for arguments in [vec![], vec![crate::Value::I32(0); 9]] {
            let function = large_frame(1024 * 1024, arguments.len())?;
            observed = 0;
            let error = stacker::grow(512 * 1024, || function.execute(&arguments, context));
            assert!(
                matches!(error, Err(Error::NativeStackOverflow { required, available }) if required > available && available > crate::Function::STACK_RESERVE_BYTES)
            );
            assert_eq!(observed, 0, "rejected function entered native code");
            assert_eq!(
                stacker::grow(4 * 1024 * 1024, || function.execute(&arguments, context))?,
                None
            );
            assert_eq!(observed, 42);
            println!(
                "large native frame recovery passed: {} bytes",
                function.native_stack_bytes()
            );
        }
        Ok(())
    }
}
