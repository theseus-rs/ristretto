use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::{InternalError, NullPointer};
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
    registry.register(
        class_name,
        "getSuperclass",
        "()Ljava/lang/Class;",
        get_super_class,
    );
    registry.register(class_name, "isArray", "()Z", is_array);
    registry.register(
        class_name,
        "isAssignableFrom",
        "(Ljava/lang/Class;)Z",
        is_assignable_from,
    );
    registry.register(class_name, "isPrimitive", "()Z", is_primitive);
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
fn desired_assertion_status_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(Some(Value::Int(0))) })
}

fn get_primitive_class(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let Some(Reference::Object(primitive)) = arguments.pop_object()? else {
            return Err(InternalError("getPrimitiveClass: no arguments".to_string()));
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
                return Err(InternalError(format!(
                    "getPrimitiveClass: unrecognized primitive: {primitive}"
                )));
            }
        };

        let vm = thread.vm()?;
        let class = vm.to_class_value(&thread, class_name).await?;
        Ok(Some(class))
    })
}

fn get_super_class(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let Some(Reference::Object(object)) = arguments.pop_object()? else {
            return Err(InternalError("getSuperclass: no arguments".to_string()));
        };
        let class = object.class();
        match class.parent()? {
            Some(parent) => {
                let class_name = parent.name();
                let vm = thread.vm()?;
                let class = vm.to_class_value(&thread, class_name).await?;
                Ok(Some(class))
            }
            None => Ok(None),
        }
    })
}

#[expect(clippy::needless_pass_by_value)]
fn is_array(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let Some(Reference::Object(object)) = arguments.pop_object()? else {
            return Err(InternalError("isArray: no arguments".to_string()));
        };
        let class = object.class();
        if class.is_array() {
            Ok(Some(Value::Int(1)))
        } else {
            Ok(Some(Value::Int(0)))
        }
    })
}

#[expect(clippy::needless_pass_by_value)]
fn is_assignable_from(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let object_argument = match arguments.pop_object()? {
            Some(Reference::Object(object)) => object,
            None => return Err(NullPointer("object cannot be null".to_string())),
            _ => return Err(InternalError("isAssignableFrom: no arguments".to_string())),
        };
        let class_argument = object_argument.class();
        let Some(Reference::Object(object)) = arguments.pop_object()? else {
            return Err(InternalError("isAssignableFrom: no instance".to_string()));
        };
        let class = object.class();
        if class.is_assignable_from(class_argument.name())? {
            Ok(Some(Value::Int(1)))
        } else {
            Ok(Some(Value::Int(0)))
        }
    })
}

#[expect(clippy::needless_pass_by_value)]
fn is_primitive(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let Some(Reference::Object(object)) = arguments.pop_object()? else {
            return Err(InternalError("isPrimitive: no arguments".to_string()));
        };
        let class_name = object.value("name")?.as_string()?;
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
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move { Ok(None) })
}
