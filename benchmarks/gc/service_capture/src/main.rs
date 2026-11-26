//! Finite actual canonical Java capture. No throughput/endurance qualification.
mod export;
mod inputs;
use inputs::Inputs;
use ristretto_classfile::ClassFile;
use ristretto_vm::canonical::Program;
use ristretto_vm::canonical_telemetry::Telemetry;
use std::path::PathBuf;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    let [directory] = arguments.as_slice() else {
        return Err("expected capture directory".into());
    };
    let Inputs {
        admission,
        class_bytes,
        binding,
        identity,
    } = Inputs::read(&PathBuf::from(directory))?;
    let classes = class_bytes
        .iter()
        .map(|bytes| ClassFile::from_bytes(bytes))
        .collect::<Result<Vec<_>, _>>()?;
    let program = Program::from_classfiles(classes)?;
    drop(class_bytes);
    #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
    let compiled = if admission.tier == "compiled" {
        Some(program.compile()?)
    } else {
        None
    };
    let telemetry = Telemetry::new(admission.recorder_capacity, binding)?;
    let result = if admission.tier == "interpreted" {
        program
            .run_recorded(
                &admission.class,
                &admission.method,
                admission.argument,
                admission.configuration.value(),
                admission.limits.interpreted(),
                &telemetry,
            )
            .await
    } else {
        #[cfg(all(feature = "compiled", not(target_family = "wasm")))]
        {
            compiled
                .as_ref()
                .ok_or("compiled program missing")?
                .run_recorded(
                    &admission.class,
                    &admission.method,
                    admission.argument,
                    admission.configuration.value(),
                    admission.limits.compiled(),
                    &telemetry,
                )
                .await
        }
        #[cfg(not(all(feature = "compiled", not(target_family = "wasm"))))]
        {
            return Err("this artifact has no canonical compiled tier".into());
        }
    };
    let error = match &result {
        Err(error) => Some(error.to_string()),
        Ok(report)
            if report.result != admission.expected_result
                || report.payload_bytes_after_cleanup != 0
                || report.catalog_bytes_after_drop != 0
                || (admission.tier == "compiled"
                    && admission.require_compiled_backedges
                    && report.compiled_backedges == 0) =>
        {
            Some("finite correctness/cleanup oracle failed".to_owned())
        }
        Ok(_) => None,
    };
    let capture = telemetry
        .finish()
        .map_err(|_| "recorded driver retained telemetry owners")?;
    export::write(capture, &identity, result.as_ref().ok(), error.as_deref())?;
    if error.is_some() {
        return Err("finite canonical execution failed; raw capture retained".into());
    }
    Ok(())
}
