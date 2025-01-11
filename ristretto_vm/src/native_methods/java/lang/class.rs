use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::registry::MethodRegistry;
use crate::rust_value::RustValue;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::JavaError::NullPointerException;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::attributes::{Attribute, InnerClass};
use ristretto_classfile::{ClassAccessFlags, FieldAccessFlags, Version};
use ristretto_classloader::{Class, Object, Reference, Value};
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };
const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_20: Version = Version::Java20 { minor: 0 };

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

    if java_version >= JAVA_20 {
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
    let class_name: String = arguments.pop_object()?.try_into()?;
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
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;
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
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;
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
    let object = arguments.pop_object()?;
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

#[async_recursion(?Send)]
async fn get_constant_pool(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Class.getConstantPool()Lsun/reflect/ConstantPool;")
}

#[async_recursion(?Send)]
async fn get_declared_classes_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let class = arguments.pop_object()?;
    let class_name: String = class.value("name")?.try_into()?;
    let class = thread.class(&class_name).await?;
    let vm = thread.vm()?;
    let mut declared_classes = Vec::new();

    let class_file = class.class_file();
    let constant_pool = &class_file.constant_pool;
    let inner_classes = class_file
        .attributes
        .iter()
        .filter_map(|attribute| match attribute {
            Attribute::InnerClasses {
                name_index: _,
                classes,
            } => Some(classes.clone()),
            _ => None,
        })
        .flatten()
        .collect::<Vec<InnerClass>>();
    let class_name = class.name();
    for inner_class in inner_classes {
        let outer_class_name = constant_pool.try_get_class(inner_class.outer_class_info_index)?;
        if outer_class_name != class_name {
            continue;
        }
        let inner_class_name = constant_pool.try_get_class(inner_class.class_info_index)?;
        let class = thread.class(inner_class_name).await?;
        let class = class.to_object(&vm).await?;
        declared_classes.push(class);
    }

    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let declared_classes = Value::try_from((class_array, declared_classes))?;
    Ok(Some(declared_classes))
}

#[async_recursion(?Send)]
async fn get_declared_constructors_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getDeclaredConstructors0(Z)[Ljava/lang/reflect/Constructor;")
}

#[async_recursion(?Send)]
async fn get_declared_fields_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let public_only = arguments.pop_int()? != 0;
    let object = arguments.pop_object()?;
    let vm = thread.vm()?;
    let class = get_class(&thread, &object).await?;
    let class_object = class.to_object(&vm).await?;

    let mut fields = Vec::new();
    for field in class.fields() {
        let access_flags = field.access_flags();
        if public_only && !access_flags.contains(FieldAccessFlags::PUBLIC) {
            continue;
        }

        let field_name = field.name();
        let field_type_class_name = field.field_type().class_name();
        let field_type_class = thread.class(field_type_class_name).await?;
        let field_type = field_type_class.to_object(&vm).await?;
        let modifiers = Value::Int(i32::from(access_flags.bits()));
        let slot = &class.field_offset(field_name)?;
        let slot = Value::Int(i32::try_from(*slot)?);
        let field_name = field.name().to_value();
        // TODO: Add support for generic signature
        let signature = Value::Object(None);
        // TODO: Add support for annotations
        let annotations = Value::Object(None);
        let (descriptor, arguments) = if vm.java_class_file_version() <= &JAVA_11 {
            (
                "Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IILjava/lang/String;[B",
                vec![
                    class_object.clone(),
                    field_name,
                    field_type,
                    modifiers,
                    slot,
                    signature,
                    annotations,
                ],
            )
        } else {
            let trusted_final =
                access_flags.contains(FieldAccessFlags::FINAL | FieldAccessFlags::STATIC);
            (
                "Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IZILjava/lang/String;[B",
                vec![
                    class_object.clone(),
                    field_name,
                    field_type,
                    modifiers,
                    Value::from(trusted_final),
                    slot,
                    signature,
                    annotations,
                ],
            )
        };
        let field = thread
            .object("java/lang/reflect/Field", descriptor, arguments)
            .await?;
        fields.push(field);
    }
    let fields_array_class = thread.class("[Ljava/lang/reflect/Field;").await?;
    let fields = Value::try_from((fields_array_class, fields))?;
    Ok(Some(fields))
}

#[async_recursion(?Send)]
async fn get_declared_methods_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getDeclaredMethods0(Z)[Ljava/lang/reflect/Method;")
}

#[async_recursion(?Send)]
async fn get_declaring_class_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;

    if class.is_array() || class.is_primitive() {
        return Ok(Some(Value::Object(None)));
    }

    let class_name = class.name();
    match class_name
        .rsplit_once('$')
        .map(|(class_name, _)| class_name)
    {
        Some(class_name) => {
            let class = thread.class(class_name).await?;
            let vm = thread.vm()?;
            let class = class.to_object(&vm).await?;
            Ok(Some(class))
        }
        None => Ok(Some(Value::Object(None))),
    }
}

#[async_recursion(?Send)]
async fn get_enclosing_method_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;
    let class_file = class.class_file();
    for attribute in &class_file.attributes {
        if let Attribute::EnclosingMethod {
            class_index,
            method_index,
            ..
        } = attribute
        {
            let vm = thread.vm()?;
            let constant_pool = &class_file.constant_pool;
            let class_name = constant_pool.try_get_utf8(*class_index)?;
            let class = thread.class(class_name).await?;
            let class = class.to_object(&vm).await?;
            let (method_name, method_descriptor) = if *method_index == 0 {
                (Value::Object(None), Value::Object(None))
            } else {
                let (name_index, descriptor_index) =
                    constant_pool.try_get_name_and_type(*method_index)?;
                let method_name = constant_pool.try_get_utf8(*name_index)?;
                let method_name = method_name.to_object(&vm).await?;
                let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
                let method_descriptor = method_descriptor.to_object(&vm).await?;
                (method_name, method_descriptor)
            };
            let object_array_class = thread.class("[Ljava/lang/Object;").await?;
            let enclosing_information = vec![class, method_name, method_descriptor];
            let enclosing_information_array =
                Reference::try_from((object_array_class, enclosing_information))?;
            return Ok(Some(Value::from(enclosing_information_array)));
        }
    }

    Ok(Some(Value::Object(None)))
}

#[async_recursion(?Send)]
async fn get_generic_signature_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getGenericSignature0()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_interfaces_0(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let class = arguments.pop_object()?;
    let class_name: String = class.value("name")?.try_into()?;
    let class = thread.class(class_name).await?;
    let vm = thread.vm()?;
    let mut interfaces = Vec::new();

    for interface in class.interfaces()? {
        let interface = interface.to_object(&vm).await?;
        interfaces.push(interface);
    }

    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let interfaces = Value::try_from((class_array, interfaces))?;
    Ok(Some(interfaces))
}

#[async_recursion(?Send)]
async fn get_modifiers(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;
    let class_file = class.class_file();
    let access_flags = &class_file.access_flags.bits();
    let excluded_flags = (ClassAccessFlags::ANNOTATION
        | ClassAccessFlags::ENUM
        | ClassAccessFlags::MODULE
        | ClassAccessFlags::SUPER
        | ClassAccessFlags::SYNTHETIC)
        .bits();
    let excluded_flags_mask = !excluded_flags;
    let modifiers = i32::from(access_flags & excluded_flags_mask);

    // TODO: correct the modifier values
    Ok(Some(Value::Int(modifiers)))
}

#[async_recursion(?Send)]
async fn get_name_0(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;
    let class_name = class.name().replace('/', ".");
    let value = class_name.to_value();
    Ok(Some(value))
}

#[async_recursion(?Send)]
async fn get_nest_host_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Class.getNestHost0()Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_nest_members_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Class.getNestMembers0()[Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_permitted_subclasses_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let _class = get_class(&thread, &object).await?;
    // TODO: add support for sealed classes
    Ok(None)
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

#[async_recursion(?Send)]
async fn get_protection_domain_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getProtectionDomain0()Ljava/security/ProtectionDomain;")
}

#[async_recursion(?Send)]
async fn get_raw_annotations(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Class.getRawAnnotations()[B")
}

#[async_recursion(?Send)]
async fn get_raw_type_annotations(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getRawTypeAnnotations()[B")
}

#[async_recursion(?Send)]
async fn get_record_components_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getRecordComponents0()[Ljava/lang/reflect/RecordComponent;")
}

#[async_recursion(?Send)]
async fn get_signers(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: Implement get_signers
    Ok(None)
}

#[async_recursion(?Send)]
async fn get_simple_binary_name_0(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;
    let class_name = class.name();
    let class_name_parts = class_name.split('$').collect::<Vec<&str>>();

    if class_name_parts.len() <= 1 {
        return Ok(Some(Value::Object(None)));
    }

    let vm = thread.vm()?;
    let binary_name = class_name_parts[class_name_parts.len() - 1];
    let value: Value = binary_name.to_string().to_object(&vm).await?;
    Ok(Some(value))
}

#[async_recursion(?Send)]
async fn get_superclass(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
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
async fn init_class_name(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    // TODO: implement support for hidden classes
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;
    let class_name = class.name().replace('/', ".");
    let value = class_name.to_value();
    Ok(Some(value))
}

#[async_recursion(?Send)]
async fn is_array(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
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

#[async_recursion(?Send)]
async fn is_hidden(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: implement support for hidden classes
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn is_instance(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let Ok(compare_object) = arguments.pop_object() else {
        return Ok(Some(Value::from(false)));
    };
    let self_object = arguments.pop_object()?;
    let self_class = get_class(&thread, &self_object).await?;

    if compare_object.instance_of(&self_class)? {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[async_recursion(?Send)]
async fn is_interface(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;
    if class.is_interface() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[async_recursion(?Send)]
async fn is_primitive(thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let object = arguments.pop_object()?;
    let class = get_class(&thread, &object).await?;
    if class.is_primitive() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[async_recursion(?Send)]
async fn is_record_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.Class.isRecord0()Z")
}

#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn set_signers(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    // TODO: Implement set_signers
    Ok(None)
}
