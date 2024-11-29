use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Reference, Value};
use std::io::Write;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `java.io.FileOutputStream`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/FileOutputStream";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(class_name, "close0", "()V", close_0);
    }

    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "open0", "(Ljava/lang/String;Z)V", open_0);
    registry.register(class_name, "write", "(IZ)V", write);
    registry.register(class_name, "writeBytes", "([BIIZ)V", write_bytes);
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn write(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::cast_sign_loss)]
#[async_recursion(?Send)]
async fn write_bytes(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _append = arguments.pop_int()? != 0;
    let length = usize::try_from(arguments.pop_int()?)?;
    let offset = usize::try_from(arguments.pop_int()?)?;
    let Some(Reference::ByteArray(bytes)) = arguments.pop_reference()? else {
        return Err(InternalError(
            "Invalid argument type; expected byte[]".to_string(),
        ));
    };
    let bytes: Vec<u8> = bytes.to_vec()?.iter().map(|&x| x as u8).collect();
    let Some(Reference::Object(file_output_stream)) = arguments.pop_reference()? else {
        return Err(InternalError(
            "Invalid argument type; expected object".to_string(),
        ));
    };
    let Value::Object(Some(Reference::Object(file_descriptor))) = file_output_stream.value("fd")?
    else {
        return Err(InternalError(
            "Invalid argument type; expected object".to_string(),
        ));
    };
    let Value::Long(handle) = file_descriptor.value("handle")? else {
        return Err(InternalError(
            "Invalid argument type; expected long".to_string(),
        ));
    };

    match handle {
        1 => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            stdout
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| InternalError(error.to_string()))?;
            stdout
                .flush()
                .map_err(|error| InternalError(error.to_string()))?;
        }
        2 => {
            let stderr = std::io::stderr();
            let mut stderr = stderr.lock();
            stderr
                .write_all(&bytes[offset..offset + length])
                .map_err(|error| InternalError(error.to_string()))?;
            stderr
                .flush()
                .map_err(|error| InternalError(error.to_string()))?;
        }
        _ => {
            return Err(InternalError(format!("Invalid file handle: {handle}")));
        }
    }
    Ok(None)
}
