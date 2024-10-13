use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Error::RuntimeError;
use crate::Result;
use ristretto_classloader::{Reference, Value};
use std::io::Write;
use std::sync::Arc;

/// Register all native methods for java.io.FileOutputStream.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/FileOutputStream";
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "writeBytes", "([BIIZ)V", write_bytes);
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn init_ids(_call_stack: &Arc<CallStack>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::cast_sign_loss)]
fn write_bytes(_call_stack: &Arc<CallStack>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _append = arguments.pop_int()? == 1;
    let length = usize::try_from(arguments.pop_int()?)?;
    let offset = usize::try_from(arguments.pop_int()?)?;
    let Some(Reference::ByteArray(bytes)) = arguments.pop_object()? else {
        return Err(RuntimeError(
            "Invalid argument type; expected byte[]".to_string(),
        ));
    };
    let bytes: Vec<u8> = bytes.to_vec()?.iter().map(|&x| x as u8).collect();
    let Some(Reference::Object(file_output_stream)) = arguments.pop_object()? else {
        return Err(RuntimeError(
            "Invalid argument type; expected object".to_string(),
        ));
    };
    let Value::Object(Some(Reference::Object(file_descriptor))) =
        file_output_stream.field("fd")?.value()?
    else {
        return Err(RuntimeError(
            "Invalid argument type; expected object".to_string(),
        ));
    };
    let Value::Long(handle) = file_descriptor.field("handle")?.value()? else {
        return Err(RuntimeError(
            "Invalid argument type; expected long".to_string(),
        ));
    };

    match handle {
        1 => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            stdout
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| RuntimeError(error.to_string()))?;
            stdout
                .flush()
                .map_err(|error| RuntimeError(error.to_string()))?;
        }
        2 => {
            let stderr = std::io::stderr();
            let mut stderr = stderr.lock();
            stderr
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| RuntimeError(error.to_string()))?;
            stderr
                .flush()
                .map_err(|error| RuntimeError(error.to_string()))?;
        }
        _ => {
            return Err(RuntimeError(format!("Invalid file handle: {handle}")));
        }
    }
    Ok(None)
}
