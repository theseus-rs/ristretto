//! Receiver class loading for runtime calls. Hierarchy resolution is provided by `Class`.

use crate::{JavaError, Result, Thread};
use ristretto_classloader::{Class, Reference, Value};
use std::sync::Arc;

/// Get the run-time class of an object or array receiver.
///
/// # Errors
/// Returns an error for null/non-reference values or if a primitive array class cannot load.
pub async fn receiver_class<T: Thread + ?Sized>(
    thread: &T,
    receiver: &Value,
) -> Result<Arc<Class>> {
    if receiver.is_null() {
        return Err(JavaError::NullPointerException(None).into());
    }
    let name = {
        let reference = receiver.as_reference()?;
        match &*reference {
            Reference::Object(object) => return Ok(object.class().clone()),
            Reference::Array(array) => return Ok(array.class.clone()),
            _ => reference.class_name()?,
        }
    };
    thread.class(&name).await
}
