use crate::Error::InternalError;
use crate::Result;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use ristretto_classfile::attributes::{Attribute, BootstrapMethod};
use ristretto_classfile::{Constant, ConstantPool};
use ristretto_classloader::Value;
use tracing::debug;

/// Get the bootstrap method attribute name and type for the specified method index.
fn get_bootstrap_method_attribute_name_and_type(
    constant_pool: &ConstantPool,
    method_index: u16,
) -> Result<(u16, u16)> {
    let (bootstrap_method_attr_index, name_and_type_index) =
        match constant_pool.try_get(method_index)? {
            Constant::Dynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                // Field descriptor: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.2
                (*bootstrap_method_attr_index, *name_and_type_index)
            }
            Constant::InvokeDynamic {
                bootstrap_method_attr_index,
                name_and_type_index,
            } => {
                // Method descriptor: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.3.3
                (*bootstrap_method_attr_index, *name_and_type_index)
            } // Method
            _ => {
                return Err(InternalError(format!(
                    "Invalid constant pool index for invokedynamic: {method_index}"
                )));
            }
        };
    Ok((bootstrap_method_attr_index, name_and_type_index))
}

/// Get the bootstrap method definition for the specified class.
///
/// # Errors
/// if the bootstrap method cannot be found.
fn get_bootstrap_method_attribute(
    frame: &Frame,
    bootstrap_method_attr_index: u16,
) -> Result<BootstrapMethod> {
    let class = frame.class();
    class
        .class_file()
        .attributes
        .iter()
        .find_map(|attribute| {
            if let Attribute::BootstrapMethods { methods, .. } = attribute {
                methods.get(bootstrap_method_attr_index as usize).cloned()
            } else {
                None
            }
        })
        .ok_or_else(|| {
            InternalError(format!(
                "No bootstrap methods found for class {} at index {}",
                class.name(),
                bootstrap_method_attr_index
            ))
        })
}

/// Resolves the invokedynamic call site from the frame and constant pool.
pub async fn get_call_site(frame: &Frame, method_index: u16) -> Result<Value> {
    let thread = frame.thread()?;
    let current_class = frame.class();
    let constant_pool = current_class.constant_pool();

    // Get the bootstrap method attribute name and type
    let (bootstrap_method_attr_index, name_and_type_index) =
        get_bootstrap_method_attribute_name_and_type(constant_pool, method_index)?;
    let (bootstrap_method_attr_name_index, bootstrap_method_attr_descriptor_index) =
        constant_pool.try_get_name_and_type(name_and_type_index)?;
    let _bootstrap_method_attr_name =
        constant_pool.try_get_utf8(*bootstrap_method_attr_name_index)?;
    let _bootstrap_method_attr_descriptor =
        constant_pool.try_get_utf8(*bootstrap_method_attr_descriptor_index)?;

    // Get the bootstrap method
    let bootstrap_method = get_bootstrap_method_attribute(frame, bootstrap_method_attr_index)?;
    let (reference_kind, method_ref) =
        constant_pool.try_get_method_handle(bootstrap_method.bootstrap_method_ref)?;
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(*method_ref)?;
    let bootstrap_class_name = constant_pool.try_get_class(*class_index)?;
    let bootstrap_class = thread.class(bootstrap_class_name).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let bootstrap_method_name = constant_pool.try_get_utf8(*name_index)?;
    let bootstrap_method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let bootstrap_method =
        bootstrap_class.try_get_method(bootstrap_method_name, bootstrap_method_descriptor)?;

    debug!(
        "invokedynamic: bootstrap method({reference_kind}): {}.{}{}",
        bootstrap_class.name(),
        bootstrap_method.name(),
        bootstrap_method.descriptor()
    );
    todo!("invokedynamic get_call_site")
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.invokedynamic>
#[inline]
pub(crate) async fn invokedynamic(
    frame: &Frame,
    _stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    let _call_site = get_call_site(frame, method_index).await?;
    todo!("invokedynamic")
}
