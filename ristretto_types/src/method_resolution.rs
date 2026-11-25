//! Receiver class loading for runtime calls. Hierarchy resolution is provided by `Class`.

use crate::{JavaError, Result, Thread};
use ristretto_classfile::attributes::Attribute;
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

/// Resolve and validate both nest hosts before allowing private member access.
///
/// # Errors
/// Returns an error if nest metadata cannot be resolved or the host does not list the member.
pub async fn are_nestmates<T: Thread + ?Sized>(
    thread: &T,
    left: &Arc<Class>,
    right: &Arc<Class>,
) -> Result<bool> {
    if Arc::ptr_eq(left, right) {
        return Ok(true);
    }
    if !left.same_runtime_package(right)? {
        return Ok(false);
    }
    Ok(Arc::ptr_eq(
        &nest_host(thread, left).await?,
        &nest_host(thread, right).await?,
    ))
}

/// Resolve a class's dynamically assigned or class-file nest host.
///
/// # Errors
/// Returns an error if a symbolic host cannot be loaded or does not list its member.
pub async fn nest_host<T: Thread + ?Sized>(thread: &T, class: &Arc<Class>) -> Result<Arc<Class>> {
    if let Some(host) = class.dynamic_nest_host() {
        return Ok(host.clone());
    }
    for attribute in &class.class_file().attributes {
        if let Attribute::NestHost {
            host_class_index, ..
        } = attribute
        {
            let host = thread
                .load_referenced_class(
                    class,
                    class.constant_pool().try_get_class(*host_class_index)?,
                )
                .await?;
            if class.same_runtime_package(&host)? {
                for attribute in &host.class_file().attributes {
                    if let Attribute::NestMembers { class_indexes, .. } = attribute {
                        for index in class_indexes {
                            if host.constant_pool().try_get_class(*index)? == class.java_name() {
                                return Ok(host);
                            }
                        }
                    }
                }
            }
            return Err(JavaError::IllegalAccessError(
                ["Invalid nest host for ", class.name()].concat(),
            )
            .into());
        }
    }
    Ok(class.clone())
}
