use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Error::RuntimeError;
use crate::Result;
use ristretto_classloader::{Reference, Value};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for java.lang.Class.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Class";
    registry.register(
        class_name,
        "desiredAssertionStatus0",
        "(Ljava/lang/Class;)Z",
        desired_assertion_status_0,
    );
    registry.register(
        class_name,
        "getPrimitiveClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        get_primitive_class,
    );
    registry.register(class_name, "isPrimitive", "()Z", is_primitive);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
fn desired_assertion_status_0(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(Some(Value::Int(0))) })
}

fn get_primitive_class(
    call_stack: Arc<CallStack>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let Some(Reference::Object(primitive)) = arguments.pop_object()? else {
            return Err(RuntimeError("getPrimitiveClass: no arguments".to_string()));
        };

        let primitive = primitive.as_string()?;
        let class_name = match primitive.as_str() {
            "boolean" => "java/lang/Boolean",
            "byte" => "java/lang/Byte",
            "char" => "java/lang/Character",
            "double" => "java/lang/Double",
            "float" => "java/lang/Float",
            "int" => "java/lang/Integer",
            "long" => "java/lang/Long",
            "short" => "java/lang/Short",
            "void" => "java/lang/Void",
            _ => {
                return Err(RuntimeError(format!(
                    "getPrimitiveClass: unrecognized primitive: {primitive}"
                )));
            }
        };

        let vm = call_stack.vm()?;
        let class = vm.to_class_value(&call_stack, class_name).await?;
        Ok(Some(class))
    })
}

#[expect(clippy::needless_pass_by_value)]
fn is_primitive(
    _call_stack: Arc<CallStack>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let Some(Reference::Object(object)) = arguments.pop_object()? else {
            return Err(RuntimeError("isPrimitive: no arguments".to_string()));
        };
        let field = object.field("name")?;
        let class_name = field.value()?.as_string()?;
        match class_name.as_str() {
            "java/lang/Boolean"
            | "java/lang/Byte"
            | "java/lang/Character"
            | "java/lang/Double"
            | "java/lang/Float"
            | "java/lang/Integer"
            | "java/lang/Long"
            | "java/lang/Short"
            | "java/lang/Void" => Ok(Some(Value::Int(1))),
            _ => Ok(Some(Value::Int(0))),
        }
    })
}

#[expect(clippy::needless_pass_by_value)]
fn register_natives(
    _call_stack: Arc<CallStack>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(None) })
}
