use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::{InternalError, NullPointer};
use crate::{Result, VM};
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Class, Object, Reference, Value};
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

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
        "forName0",
        "(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;",
        for_name_0,
    );
    registry.register(
        class_name,
        "getClassAccessFlagsRaw0",
        "()I",
        get_class_access_flags_raw_0,
    );
    registry.register(
        class_name,
        "getClassFileVersion0",
        "()I",
        get_class_file_version_0,
    );

    if *registry.java_version() <= JAVA_8 {
        registry.register(
            class_name,
            "getComponentType",
            "()Ljava/lang/Class;",
            get_component_type,
        );
    }

    registry.register(
        class_name,
        "getDeclaringClass0",
        "()Ljava/lang/Class;",
        get_declaring_class_0,
    );
    registry.register(
        class_name,
        "getPermittedSubclasses0",
        "()[Ljava/lang/Class;",
        get_permitted_subclasses_0,
    );
    registry.register(
        class_name,
        "getPrimitiveClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        get_primitive_class,
    );
    registry.register(
        class_name,
        "getSigners",
        "()Ljava/lang/Object;",
        get_signers,
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
    registry.register(class_name, "isHidden", "()Z", is_hidden);
    registry.register(class_name, "isInterface", "()Z", is_interface);
    registry.register(class_name, "isPrimitive", "()Z", is_primitive);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "setSigners",
        "(Ljava/lang/Object;)V",
        set_signers,
    );
}

async fn get_class(vm: &VM, object: &Object) -> Result<Arc<Class>> {
    let class = object.class();
    if class.name() == "java/lang/Class" {
        let class_name: String = object.value("name")?.try_into()?;
        let class = vm.class(class_name.as_str()).await?;
        return Ok(class);
    }
    Ok(Arc::clone(class))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn desired_assertion_status_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(0)))
}

#[async_recursion(?Send)]
async fn get_permitted_subclasses_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError(
            "getPermittedSubclasses0: no arguments".to_string(),
        ));
    };
    let vm = thread.vm()?;
    let _class = get_class(&vm, &object).await?;
    // TODO: add support for sealed classes
    Ok(None)
}

#[async_recursion(?Send)]
async fn for_name_0(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    // TODO: Add support for unused arguments
    let _caller = arguments.pop_object()?;
    let _class_loader = arguments.pop_object()?;
    let _initialize = arguments.pop_int()? != 0;
    let Some(Reference::Object(class_name)) = arguments.pop_object()? else {
        return Err(InternalError(
            "forName0: no class_name argument".to_string(),
        ));
    };
    let class_name: String = class_name.try_into()?;
    let vm = thread.vm()?;
    let class = thread.class(class_name).await?;
    let class_object = class.to_object(&vm).await?;

    Ok(Some(class_object))
}

#[async_recursion(?Send)]
async fn get_class_access_flags_raw_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError(
            "getClassAccessFlagsRaw0: no arguments".to_string(),
        ));
    };
    let vm = thread.vm()?;
    let class = get_class(&vm, &object).await?;
    let class_file = class.class_file();
    let access_flags = &class_file.access_flags;
    #[expect(clippy::cast_lossless)]
    let class_access_flags = access_flags.bits() as i32;
    Ok(Some(Value::Int(class_access_flags)))
}

#[async_recursion(?Send)]
async fn get_class_file_version_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError(
            "getClassFileVersion0: no arguments".to_string(),
        ));
    };
    let vm = thread.vm()?;
    let class = get_class(&vm, &object).await?;
    let class_file = class.class_file();
    let version = &class_file.version;
    #[expect(clippy::cast_lossless)]
    let major = version.major() as i32;
    #[expect(clippy::cast_lossless)]
    let minor = version.minor() as i32;
    let class_file_version = (minor << 16) | major;
    Ok(Some(Value::Int(class_file_version)))
}

#[async_recursion(?Send)]
async fn get_component_type(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError(
            "getComponentType: no class reference".to_string(),
        ));
    };

    let class = object.class();
    if class.is_array() {
        return Ok(Some(Value::Object(None)));
    }

    let class_name: String = object.value("name")?.try_into()?;
    let class = thread.class(class_name).await?;
    let vm = thread.vm()?;
    let class_object = class.to_object(&vm).await?;

    Ok(Some(class_object))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_declaring_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn get_primitive_class(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(primitive)) = arguments.pop_object()? else {
        return Err(InternalError("getPrimitiveClass: no arguments".to_string()));
    };

    let class_name: String = primitive.try_into()?;
    let vm = thread.vm()?;
    let class = thread.class(class_name).await?;
    let class = class.to_object(&vm).await?;
    Ok(Some(class))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_signers(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: Implement get_signers
    Ok(None)
}

#[async_recursion(?Send)]
async fn get_super_class(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError("getSuperclass: no arguments".to_string()));
    };
    let class = object.class();
    match class.parent()? {
        Some(parent) => {
            let class_name = parent.name();
            let vm = thread.vm()?;
            let class = thread.class(class_name).await?;
            let class = class.to_object(&vm).await?;
            Ok(Some(class))
        }
        None => Ok(None),
    }
}

#[async_recursion(?Send)]
async fn is_array(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError("isArray: no arguments".to_string()));
    };
    let vm = thread.vm()?;
    let class = get_class(&vm, &object).await?;
    if class.is_array() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[async_recursion(?Send)]
async fn is_assignable_from(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let object_argument = match arguments.pop_object()? {
        Some(Reference::Object(object)) => object,
        None => return Err(NullPointer("object cannot be null".to_string())),
        _ => return Err(InternalError("isAssignableFrom: no arguments".to_string())),
    };
    let vm = thread.vm()?;
    let class_argument = get_class(&vm, &object_argument).await?;
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError("isAssignableFrom: no instance".to_string()));
    };
    let class = get_class(&vm, &object).await?;
    if class.is_assignable_from(&class_argument)? {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_hidden(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn is_interface(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError("isInterface: no arguments".to_string()));
    };
    let vm = thread.vm()?;
    let class = get_class(&vm, &object).await?;
    if class.is_interface() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[async_recursion(?Send)]
async fn is_primitive(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_object()? else {
        return Err(InternalError("isPrimitive: no arguments".to_string()));
    };
    let vm = thread.vm()?;
    let class = get_class(&vm, &object).await?;
    if class.is_primitive() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_signers(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: Implement set_signers
    Ok(None)
}
