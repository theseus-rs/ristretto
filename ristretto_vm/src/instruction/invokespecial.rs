use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::instruction::resolve_method;
use crate::operand_stack::OperandStack;
use ristretto_classfile::Constant;
use ristretto_classfile::Error::InvalidConstantPoolIndexType;

/// # References
///
/// - [JVMS §6.5.invokespecial](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.invokespecial)
#[inline]
pub(crate) async fn invokespecial(
    frame: &Frame,
    stack: &mut OperandStack,
    method_index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (Constant::MethodRef {
        class_index,
        name_and_type_index,
    }
    | Constant::InterfaceMethodRef {
        class_index,
        name_and_type_index,
    }) = constant_pool.try_get(method_index)?
    else {
        return Err(InvalidConstantPoolIndexType(method_index).into());
    };
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
    let (resolved_class, resolved_method) = resolve_method(&class, method_name, method_descriptor)?;

    let parameters = stack.drain_last(resolved_method.parameters().len() + 1);
    let result = Box::pin(thread.execute(&resolved_class, &resolved_method, &parameters)).await?;
    if let Some(result) = result {
        stack.push(result)?;
    }
    Ok(Continue)
}
