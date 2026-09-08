//! File-channel support for the in-memory WASI filesystem. Socket and pipe natives
//! remain unavailable; their operating-system implementations are separate.
use crate::java::io::socketfiledescriptor::get_fd;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::{async_method, intrinsic_method};
use ristretto_types::{Parameters, Result, Thread};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/IOUtil.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/IOUtil.iovMax()I", Any)]
#[async_method]
pub async fn iov_max<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(16)))
}

#[intrinsic_method("sun/nio/ch/IOUtil.writevMax()J", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn writev_max<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(i64::from(i32::MAX))))
}

#[intrinsic_method("sun/nio/ch/IOUtil.fdVal(Ljava/io/FileDescriptor;)I", Any)]
#[async_method]
pub async fn fd_val<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(get_fd(&parameters.pop()?)?)))
}

#[intrinsic_method("sun/nio/ch/IOUtil.setfdVal(Ljava/io/FileDescriptor;I)V", Any)]
#[async_method]
pub async fn set_fd_val<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    parameters
        .pop()?
        .as_object_mut()?
        .set_value("fd", Value::Int(fd))?;
    Ok(None)
}
