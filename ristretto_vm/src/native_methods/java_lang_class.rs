use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::JavaError::NullPointerException;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Class, Object, Reference, Value};
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `java.lang.Class`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Class";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_8 {
        registry.register(
            class_name,
            "getComponentType",
            "()Ljava/lang/Class;",
            get_component_type,
        );
        registry.register(
            class_name,
            "getConstantPool",
            "()Lsun/reflect/ConstantPool;",
            get_constant_pool,
        );
        registry.register(class_name, "getName0", "()Ljava/lang/String;", get_name_0);
    } else {
        registry.register(
            class_name,
            "getConstantPool",
            "()Ljdk/internal/reflect/ConstantPool;",
            get_constant_pool,
        );
        registry.register(
            class_name,
            "getNestHost0",
            "()Ljava/lang/Class;",
            get_nest_host_0,
        );
        registry.register(
            class_name,
            "getNestMembers0",
            "()[Ljava/lang/Class;",
            get_nest_members_0,
        );
        registry.register(
            class_name,
            "getSimpleBinaryName0",
            "()Ljava/lang/String;",
            get_simple_binary_name_0,
        );
        registry.register(
            class_name,
            "initClassName",
            "()Ljava/lang/String;",
            init_class_name,
        );
    }

    if java_version >= JAVA_17 {
        registry.register(
            class_name,
            "getPermittedSubclasses0",
            "()[Ljava/lang/Class;",
            get_permitted_subclasses_0,
        );
        registry.register(
            class_name,
            "getRecordComponents0",
            "()[Ljava/lang/reflect/RecordComponent;",
            get_record_components_0,
        );
        registry.register(class_name, "isHidden", "()Z", is_hidden);
        registry.register(class_name, "isRecord0", "()Z", is_record_0);
    }

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
        "getConstantPool",
        "()Ljdk/internal/reflect/ConstantPool;",
        get_constant_pool,
    );
    registry.register(
        class_name,
        "getDeclaredClasses0",
        "()[Ljava/lang/Class;",
        get_declared_classes_0,
    );
    registry.register(
        class_name,
        "getDeclaredConstructors0",
        "(Z)[Ljava/lang/reflect/Constructor;",
        get_declared_constructors_0,
    );
    registry.register(
        class_name,
        "getDeclaredFields0",
        "(Z)[Ljava/lang/reflect/Field;",
        get_declared_fields_0,
    );
    registry.register(
        class_name,
        "getDeclaredMethods0",
        "(Z)[Ljava/lang/reflect/Method;",
        get_declared_methods_0,
    );
    registry.register(
        class_name,
        "getDeclaringClass0",
        "()Ljava/lang/Class;",
        get_declaring_class_0,
    );
    registry.register(
        class_name,
        "getEnclosingMethod0",
        "()[Ljava/lang/Object;",
        get_enclosing_method_0,
    );
    registry.register(
        class_name,
        "getGenericSignature0",
        "()Ljava/lang/String;",
        get_generic_signature_0,
    );
    registry.register(
        class_name,
        "getInterfaces0",
        "()[Ljava/lang/Class;",
        get_interfaces_0,
    );
    registry.register(class_name, "getModifiers", "()I", get_modifiers);
    registry.register(
        class_name,
        "getNestHost0",
        "()Ljava/lang/Class;",
        get_nest_host_0,
    );
    registry.register(
        class_name,
        "getNestMembers0",
        "()[Ljava/lang/Class;",
        get_nest_members_0,
    );
    registry.register(
        class_name,
        "getPrimitiveClass",
        "(Ljava/lang/String;)Ljava/lang/Class;",
        get_primitive_class,
    );
    registry.register(
        class_name,
        "getProtectionDomain0",
        "()Ljava/security/ProtectionDomain;",
        get_protection_domain_0,
    );
    registry.register(class_name, "getRawAnnotations", "()[B", get_raw_annotations);
    registry.register(
        class_name,
        "getRawTypeAnnotations",
        "()[B",
        get_raw_type_annotations,
    );
    registry.register(
        class_name,
        "getSigners",
        "()[Ljava/lang/Object;",
        get_signers,
    );
    registry.register(
        class_name,
        "getSimpleBinaryName0",
        "()Ljava/lang/String;",
        get_simple_binary_name_0,
    );
    registry.register(
        class_name,
        "getSuperclass",
        "()Ljava/lang/Class;",
        get_superclass,
    );
    registry.register(
        class_name,
        "initClassName",
        "()Ljava/lang/String;",
        init_class_name,
    );
    registry.register(class_name, "isArray", "()Z", is_array);
    registry.register(
        class_name,
        "isAssignableFrom",
        "(Ljava/lang/Class;)Z",
        is_assignable_from,
    );
    registry.register(
        class_name,
        "isInstance",
        "(Ljava/lang/Object;)Z",
        is_instance,
    );
    registry.register(class_name, "isInterface", "()Z", is_interface);
    registry.register(class_name, "isPrimitive", "()Z", is_primitive);
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "setSigners",
        "([Ljava/lang/Object;)V",
        set_signers,
    );
}

async fn get_class(thread: &Thread, object: &Object) -> Result<Arc<Class>> {
    let class = object.class();
    if class.name() == "java/lang/Class" {
        let class_name: String = object.value("name")?.try_into()?;
        let class = thread.class(class_name.as_str()).await?;
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
async fn for_name_0(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    // TODO: Add support for unused arguments
    let _caller = arguments.pop_reference()?;
    let _class_loader = arguments.pop_reference()?;
    let _initialize = arguments.pop_int()? != 0;
    let Some(Reference::Object(class_name)) = arguments.pop_reference()? else {
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
async fn get_component_type(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_reference()? else {
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
async fn get_constant_pool(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_declared_classes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_declared_constructors_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_declared_fields_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_declared_methods_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_declaring_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_enclosing_method_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_generic_signature_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_interfaces_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_modifiers(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_name_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_nest_host_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_nest_members_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_permitted_subclasses_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_primitive_class(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let Some(Reference::Object(primitive)) = arguments.pop_reference()? else {
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
async fn get_protection_domain_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_raw_annotations(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_raw_type_annotations(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_record_components_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_signers(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: Implement get_signers
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_simple_binary_name_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_superclass(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_reference()? else {
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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_class_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_array(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_reference()? else {
        return Err(InternalError("isArray: no arguments".to_string()));
    };
    let class = get_class(&thread, &object).await?;
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
    let object_argument = match arguments.pop_reference()? {
        Some(Reference::Object(object)) => object,
        None => return Err(NullPointerException("object cannot be null".to_string()).into()),
        _ => return Err(InternalError("isAssignableFrom: no arguments".to_string())),
    };
    let class_argument = get_class(&thread, &object_argument).await?;
    let Some(Reference::Object(object)) = arguments.pop_reference()? else {
        return Err(InternalError("isAssignableFrom: no instance".to_string()));
    };
    let class = get_class(&thread, &object).await?;
    if class.is_assignable_from(&class_argument)? {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_hidden(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_instance(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn is_interface(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_reference()? else {
        return Err(InternalError("isInterface: no arguments".to_string()));
    };
    let class = get_class(&thread, &object).await?;
    if class.is_interface() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[async_recursion(?Send)]
async fn is_primitive(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Some(Reference::Object(object)) = arguments.pop_reference()? else {
        return Err(InternalError("isPrimitive: no arguments".to_string()));
    };
    let class = get_class(&thread, &object).await?;
    if class.is_primitive() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_record_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
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
