use crate::JavaError::{ClassNotFoundException, NullPointerException};
use crate::Result;
use crate::assignable::Assignable;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::rust_value::RustValue;
use crate::thread::Thread;
use async_recursion::async_recursion;
use byteorder::{BigEndian, WriteBytesExt};
use ristretto_classfile::VersionSpecification::{
    Any, GreaterThan, GreaterThanOrEqual, LessThanOrEqual,
};
use ristretto_classfile::attributes::{Attribute, InnerClass};
use ristretto_classfile::{
    ClassAccessFlags, FieldAccessFlags, JAVA_8, JAVA_11, JAVA_17, JAVA_21, MethodAccessFlags,
};
use ristretto_classloader::{Class, Method, Object, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

/// Get the class of an object, handling special cases for `java/lang/Class`.
pub async fn get_class(thread: &Thread, object: &Value) -> Result<Arc<Class>> {
    {
        let object = object.as_object_ref()?;
        let class = object.class();
        if class.name() != "java/lang/Class" {
            return Ok(Arc::clone(class));
        }
    }

    let class_name = {
        let object = object.as_object_ref()?;
        object.value("name")?.as_string()?
    };
    thread.class(class_name.as_str()).await
}

#[intrinsic_method("java/lang/Class.desiredAssertionStatus0(Ljava/lang/Class;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn desired_assertion_status_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "java/lang/Class.forName0(Ljava/lang/String;ZLjava/lang/ClassLoader;Ljava/lang/Class;)Ljava/lang/Class;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn for_name_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: Add support for unused parameters
    let _caller = parameters.pop_reference()?;
    let _class_loader = parameters.pop_reference()?;
    let _initialize = parameters.pop_bool()?;
    let object = parameters.pop()?;
    if object.is_null() {
        return Err(NullPointerException("className cannot be null".to_string()).into());
    }

    let class_name = object.as_string()?;
    let class = match thread.class(&class_name).await {
        Ok(class) => class,
        Err(_error) => {
            return Err(ClassNotFoundException(class_name).into());
        }
    };

    if class.is_primitive() {
        return Err(ClassNotFoundException(class_name).into());
    }

    let class_object = class.to_object(&thread).await?;
    Ok(Some(class_object))
}

#[intrinsic_method(
    "java/lang/Class.getClassAccessFlagsRaw0()I",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_class_access_flags_raw_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_file = class.class_file();
    let access_flags = &class_file.access_flags;
    #[expect(clippy::cast_lossless)]
    let class_access_flags = access_flags.bits() as i32;
    Ok(Some(Value::Int(class_access_flags)))
}

#[intrinsic_method("java/lang/Class.getClassFileVersion0()I", GreaterThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn get_class_file_version_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
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

#[intrinsic_method(
    "java/lang/Class.getComponentType()Ljava/lang/Class;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_component_type(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class_name = {
        let object = object.as_object_ref()?;
        object.value("name")?.as_string()?
    };
    let class = thread.class(&class_name).await?;

    if !class.is_array() {
        return Ok(Some(Value::Object(None)));
    }

    let class_name = class_name.trim_start_matches('[');
    let class = thread.class(class_name).await?;
    let class_object = class.to_object(&thread).await?;

    Ok(Some(class_object))
}

#[intrinsic_method(
    "java/lang/Class.getConstantPool()Lsun/reflect/ConstantPool;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_constant_pool_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getConstantPool()Lsun/reflect/ConstantPool;")
}

#[intrinsic_method(
    "java/lang/Class.getConstantPool()Ljdk/internal/reflect/ConstantPool;",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_constant_pool_1(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let class_object = parameters.pop()?;
    let class = thread.class("jdk.internal.reflect.ConstantPool").await?;
    let mut constant_pool = Object::new(class)?;
    constant_pool.set_value("constantPoolOop", class_object)?;
    Ok(Some(Value::from(constant_pool)))
}

#[intrinsic_method("java/lang/Class.getDeclaredClasses0()[Ljava/lang/Class;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_declared_classes_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let class = parameters.pop()?;
    let class_name = {
        let class = class.as_object_ref()?;
        class.value("name")?.as_string()?
    };
    let class = thread.class(&class_name).await?;
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
        let class = class.to_object(&thread).await?;
        declared_classes.push(class);
    }

    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let declared_classes = Value::try_from((class_array, declared_classes))?;
    Ok(Some(declared_classes))
}

#[intrinsic_method(
    "java/lang/Class.getDeclaredConstructors0(Z)[Ljava/lang/reflect/Constructor;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_declared_constructors_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let public_only = parameters.pop_bool()?;
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_object = class.to_object(&thread).await?;

    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let mut constructors = Vec::new();
    for (slot, method) in class.methods().iter().enumerate() {
        if method.name() != "<init>" {
            continue;
        }

        let access_flags = method.access_flags();
        if public_only && !access_flags.contains(MethodAccessFlags::PUBLIC) {
            continue;
        }

        let mut parameters = Vec::new();
        for parameter in method.parameters() {
            let class_name = parameter.class_name();
            let class = thread.class(class_name).await?;
            parameters.push(class.to_object(&thread).await?);
        }
        let parameter_types = Value::try_from((class_array.clone(), parameters))?;
        let checked_exceptions = get_exceptions(&thread, &class, method).await?;
        let modifiers = Value::Int(i32::from(access_flags.bits()));
        let slot = Value::Int(i32::try_from(slot)?);

        let mut method_signature = Value::Object(None);
        let mut annotations = Value::Object(None);
        let mut parameter_annotations = Value::Object(None);
        for attribute in method.attributes() {
            match attribute {
                Attribute::Signature {
                    signature_index, ..
                } => {
                    let class_file = class.class_file();
                    let constant_pool = &class_file.constant_pool;
                    let signature = constant_pool.try_get_utf8(*signature_index)?;
                    method_signature = signature.to_object(&thread).await?;
                }
                Attribute::RuntimeVisibleAnnotations {
                    annotations: runtime_annotations,
                    ..
                } => {
                    let mut method_annotations = Vec::new();
                    method_annotations
                        .write_u16::<BigEndian>(u16::try_from(runtime_annotations.len())?)?;
                    for annotation in runtime_annotations {
                        annotation.to_bytes(&mut method_annotations)?;
                    }
                    annotations = Value::from(method_annotations);
                }
                Attribute::RuntimeVisibleParameterAnnotations {
                    parameter_annotations: runtime_parameter_annotations,
                    ..
                } => {
                    let mut method_parameter_annotations = Vec::new();
                    method_parameter_annotations.write_u16::<BigEndian>(u16::try_from(
                        runtime_parameter_annotations.len(),
                    )?)?;
                    for parameter_annotation in runtime_parameter_annotations {
                        parameter_annotation.to_bytes(&mut method_parameter_annotations)?;
                    }
                    parameter_annotations = Value::from(method_parameter_annotations);
                }
                _ => {}
            }
        }

        let constructor = thread
            .object(
                "java/lang/reflect/Constructor",
                "Ljava/lang/Class;[Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B",
                &[
                    class_object.clone(),
                    parameter_types,
                    checked_exceptions,
                    modifiers,
                    slot,
                    method_signature,
                    annotations,
                    parameter_annotations,
                ],
            )
            .await?;
        constructors.push(constructor);
    }
    let constructors_array_class = thread.class("[Ljava/lang/reflect/Constructor;").await?;
    let constructors = Value::try_from((constructors_array_class, constructors))?;
    Ok(Some(constructors))
}

#[intrinsic_method("java/lang/Class.getDeclaredFields0(Z)[Ljava/lang/reflect/Field;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_declared_fields_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let public_only = parameters.pop_bool()?;
    let object = parameters.pop()?;
    let vm = thread.vm()?;
    let class = get_class(&thread, &object).await?;
    let class_object = class.to_object(&thread).await?;

    let mut fields = Vec::new();
    for field in class.declared_fields() {
        let access_flags = field.access_flags();
        if public_only && !access_flags.contains(FieldAccessFlags::PUBLIC) {
            continue;
        }

        let field_name = field.name();
        let field_type_class_name = field.field_type().class_name();
        let field_type_class = thread.class(field_type_class_name).await?;
        let field_type = field_type_class.to_object(&thread).await?;
        let modifiers = Value::Int(i32::from(access_flags.bits()));
        let slot = &class.field_offset(field_name)?;
        let slot = Value::Int(i32::try_from(*slot)?);
        let field_name = field.name().to_value();

        let mut field_signature = Value::Object(None);
        let mut annotations = Value::Object(None);
        for attribute in field.attributes() {
            match attribute {
                Attribute::Signature {
                    signature_index, ..
                } => {
                    let class_file = class.class_file();
                    let constant_pool = &class_file.constant_pool;
                    let signature = constant_pool.try_get_utf8(*signature_index)?;
                    field_signature = signature.to_object(&thread).await?;
                }
                Attribute::RuntimeVisibleAnnotations {
                    annotations: runtime_annotations,
                    ..
                } => {
                    let mut field_annotations = Vec::new();
                    field_annotations
                        .write_u16::<BigEndian>(u16::try_from(runtime_annotations.len())?)?;
                    for annotation in runtime_annotations {
                        annotation.to_bytes(&mut field_annotations)?;
                    }
                    annotations = Value::from(field_annotations);
                }
                _ => {}
            }
        }

        let (descriptor, parameters) = if vm.java_major_version() <= JAVA_11.java() {
            (
                "Ljava/lang/Class;Ljava/lang/String;Ljava/lang/Class;IILjava/lang/String;[B",
                vec![
                    class_object.clone(),
                    field_name,
                    field_type,
                    modifiers,
                    slot,
                    field_signature,
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
                    field_signature,
                    annotations,
                ],
            )
        };
        let field = thread
            .object("java/lang/reflect/Field", descriptor, &parameters)
            .await?;
        fields.push(field);
    }
    let fields_array_class = thread.class("[Ljava/lang/reflect/Field;").await?;
    let fields = Value::try_from((fields_array_class, fields))?;
    Ok(Some(fields))
}

#[intrinsic_method(
    "java/lang/Class.getDeclaredMethods0(Z)[Ljava/lang/reflect/Method;",
    Any
)]
#[expect(clippy::too_many_lines)]
#[async_recursion(?Send)]
pub(crate) async fn get_declared_methods_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let public_only = parameters.pop_bool()?;
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_object = class.to_object(&thread).await?;

    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let mut methods = Vec::new();
    for (slot, method) in class.methods().iter().enumerate() {
        let method_name = method.name();
        if ["<clinit>", "<init>"].contains(&method_name) {
            continue;
        }

        let access_flags = method.access_flags();
        if public_only && !access_flags.contains(MethodAccessFlags::PUBLIC) {
            continue;
        }

        let method_name = method_name.to_object(&thread).await?;
        let mut parameters = Vec::new();
        for parameter in method.parameters() {
            let class_name = parameter.class_name();
            let class = thread.class(class_name).await?;
            parameters.push(class.to_object(&thread).await?);
        }
        let parameter_types = Value::try_from((class_array.clone(), parameters))?;
        let return_type = if let Some(return_type) = method.return_type() {
            let class_name = return_type.class_name();
            let class = thread.class(class_name).await?;
            class.to_object(&thread).await?
        } else {
            let class = thread.class("void").await?;
            class.to_object(&thread).await?
        };
        let checked_exceptions = get_exceptions(&thread, &class, method).await?;
        let modifiers = Value::Int(i32::from(access_flags.bits()));
        let slot = Value::Int(i32::try_from(slot)?);
        let mut method_signature = Value::Object(None);
        let mut annotations = Value::Object(None);
        let mut parameter_annotations = Value::Object(None);
        let mut annotation_default = Value::Object(None);
        for attribute in method.attributes() {
            match attribute {
                Attribute::Signature {
                    signature_index, ..
                } => {
                    let class_file = class.class_file();
                    let constant_pool = &class_file.constant_pool;
                    let signature = constant_pool.try_get_utf8(*signature_index)?;
                    method_signature = signature.to_object(&thread).await?;
                }
                Attribute::RuntimeVisibleAnnotations {
                    annotations: runtime_annotations,
                    ..
                } => {
                    let mut method_annotations = Vec::new();
                    method_annotations
                        .write_u16::<BigEndian>(u16::try_from(runtime_annotations.len())?)?;
                    for annotation in runtime_annotations {
                        annotation.to_bytes(&mut method_annotations)?;
                    }
                    annotations = Value::from(method_annotations);
                }
                Attribute::RuntimeVisibleParameterAnnotations {
                    parameter_annotations: runtime_parameter_annotations,
                    ..
                } => {
                    let mut method_parameter_annotations = Vec::new();
                    method_parameter_annotations.write_u16::<BigEndian>(u16::try_from(
                        runtime_parameter_annotations.len(),
                    )?)?;
                    for parameter_annotation in runtime_parameter_annotations {
                        parameter_annotation.to_bytes(&mut method_parameter_annotations)?;
                    }
                    parameter_annotations = Value::from(method_parameter_annotations);
                }
                Attribute::AnnotationDefault { element, .. } => {
                    let mut method_annotation_default = Vec::new();
                    element.to_bytes(&mut method_annotation_default)?;
                    annotation_default = Value::from(method_annotation_default);
                }
                _ => {}
            }
        }

        let method = thread
            .object(
                "java/lang/reflect/Method",
                "Ljava/lang/Class;Ljava/lang/String;[Ljava/lang/Class;Ljava/lang/Class;[Ljava/lang/Class;IILjava/lang/String;[B[B[B",
                &[
                    class_object.clone(),
                    method_name,
                    parameter_types,
                    return_type,
                    checked_exceptions,
                    modifiers,
                    slot,
                    method_signature,
                    annotations,
                    parameter_annotations,
                    annotation_default,
                ],
            )
            .await?;
        methods.push(method);
    }
    let methods_array_class = thread.class("[Ljava/lang/reflect/Method;").await?;
    let methods = Value::try_from((methods_array_class, methods))?;
    Ok(Some(methods))
}

#[intrinsic_method("java/lang/Class.getDeclaringClass0()Ljava/lang/Class;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_declaring_class_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
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
            let class = class.to_object(&thread).await?;
            Ok(Some(class))
        }
        None => Ok(Some(Value::Object(None))),
    }
}

#[intrinsic_method("java/lang/Class.getEnclosingMethod0()[Ljava/lang/Object;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_enclosing_method_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_file = class.class_file();
    for attribute in &class_file.attributes {
        if let Attribute::EnclosingMethod {
            class_index,
            method_index,
            ..
        } = attribute
        {
            let constant_pool = &class_file.constant_pool;
            let class_name = constant_pool.try_get_utf8(*class_index)?;
            let class = thread.class(class_name).await?;
            let class = class.to_object(&thread).await?;
            let (method_name, method_descriptor) = if *method_index == 0 {
                (Value::Object(None), Value::Object(None))
            } else {
                let (name_index, descriptor_index) =
                    constant_pool.try_get_name_and_type(*method_index)?;
                let method_name = constant_pool.try_get_utf8(*name_index)?;
                let method_name = method_name.to_object(&thread).await?;
                let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;
                let method_descriptor = method_descriptor.to_object(&thread).await?;
                (method_name, method_descriptor)
            };
            let object_array_class = thread.class("[Ljava/lang/Object;").await?;
            let enclosing_information = vec![class, method_name, method_descriptor];
            let enclosing_information_array =
                Value::try_from((object_array_class, enclosing_information))?;
            return Ok(Some(enclosing_information_array));
        }
    }

    Ok(Some(Value::Object(None)))
}

/// Get the exceptions declared by a method.
pub(crate) async fn get_exceptions(
    thread: &Arc<Thread>,
    class: &Arc<Class>,
    method: &Arc<Method>,
) -> Result<Value> {
    let constant_pool = class.constant_pool();
    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let mut exceptions = Vec::new();
    for attribute in method.attributes() {
        if let Attribute::Exceptions {
            exception_indexes, ..
        } = attribute
        {
            for exception_index in exception_indexes {
                let class_name = constant_pool.try_get_class(*exception_index)?;
                let exception = thread.class(class_name).await?;
                let exception = exception.to_object(thread).await?;
                exceptions.push(exception);
            }
            break;
        }
    }
    let exceptions = Value::try_from((class_array, exceptions))?;
    Ok(exceptions)
}

#[intrinsic_method("java/lang/Class.getGenericSignature0()Ljava/lang/String;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_generic_signature_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getGenericSignature0()Ljava/lang/String;")
}

#[intrinsic_method("java/lang/Class.getInterfaces0()[Ljava/lang/Class;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_interfaces_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let class = parameters.pop()?;
    let class_name = {
        let class = class.as_object_ref()?;
        class.value("name")?.as_string()?
    };
    let class = thread.class(class_name).await?;
    let mut interfaces = Vec::new();

    for interface in class.interfaces()? {
        let interface = interface.to_object(&thread).await?;
        interfaces.push(interface);
    }

    let class_array = thread.class("[Ljava/lang/Class;").await?;
    let interfaces = Value::try_from((class_array, interfaces))?;
    Ok(Some(interfaces))
}

#[intrinsic_method("java/lang/Class.getModifiers()I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_modifiers(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_file = class.class_file();
    let access_flags = &class_file.access_flags.bits();
    let excluded_flags =
        (ClassAccessFlags::MODULE | ClassAccessFlags::SUPER | ClassAccessFlags::SYNTHETIC).bits();
    let excluded_flags_mask = !excluded_flags;
    let modifiers = i32::from(access_flags & excluded_flags_mask);

    // TODO: correct the modifier values
    Ok(Some(Value::Int(modifiers)))
}

#[intrinsic_method(
    "java/lang/Class.getName0()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_name_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_name = class.name().replace('/', ".");
    let value = class_name.to_object(&thread).await?;
    Ok(Some(value))
}

#[intrinsic_method("java/lang/Class.getNestHost0()Ljava/lang/Class;", GreaterThan(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_nest_host_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getNestHost0()Ljava/lang/Class;")
}

#[intrinsic_method(
    "java/lang/Class.getNestMembers0()[Ljava/lang/Class;",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_nest_members_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getNestMembers0()[Ljava/lang/Class;")
}

#[intrinsic_method(
    "java/lang/Class.getPermittedSubclasses0()[Ljava/lang/Class;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_permitted_subclasses_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let _class = get_class(&thread, &object).await?;
    // TODO: add support for sealed classes
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "java/lang/Class.getPrimitiveClass(Ljava/lang/String;)Ljava/lang/Class;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_primitive_class(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let primitive = parameters.pop()?;
    let class_name = primitive.as_string()?;
    let class = thread.class(class_name).await?;
    let class = class.to_object(&thread).await?;
    Ok(Some(class))
}

#[intrinsic_method(
    "java/lang/Class.getProtectionDomain0()Ljava/security/ProtectionDomain;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_protection_domain_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getProtectionDomain0()Ljava/security/ProtectionDomain;")
}

#[intrinsic_method("java/lang/Class.getRawAnnotations()[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_raw_annotations(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_file = class.class_file();
    let mut bytes = Vec::new();
    let annotations = class_file
        .attributes
        .iter()
        .filter_map(|attribute| {
            if let Attribute::RuntimeVisibleAnnotations { annotations, .. } = attribute {
                Some(annotations)
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    let annotations_length = u16::try_from(annotations.len())?;
    bytes.write_u16::<BigEndian>(annotations_length)?;
    for annotation in annotations {
        annotation.to_bytes(&mut bytes)?;
    }
    Ok(Some(Value::from(bytes)))
}

#[intrinsic_method("java/lang/Class.getRawTypeAnnotations()[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_raw_type_annotations(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_file = class.class_file();
    let mut bytes = Vec::new();
    let annotations = class_file
        .attributes
        .iter()
        .filter_map(|attribute| {
            if let Attribute::RuntimeVisibleTypeAnnotations {
                type_annotations, ..
            } = attribute
            {
                Some(type_annotations)
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    let annotations_length = u16::try_from(annotations.len())?;
    bytes.write_u16::<BigEndian>(annotations_length)?;
    for annotation in annotations {
        annotation.to_bytes(&mut bytes)?;
    }
    Ok(Some(Value::from(bytes)))
}

#[intrinsic_method(
    "java/lang/Class.getRecordComponents0()[Ljava/lang/reflect/RecordComponent;",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_record_components_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Class.getRecordComponents0()[Ljava/lang/reflect/RecordComponent;")
}

#[intrinsic_method(
    "java/lang/Class.getSigners()[Ljava/lang/Object;",
    LessThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_signers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: Implement get_signers
    Ok(Some(Value::Object(None)))
}

#[intrinsic_method(
    "java/lang/Class.getSimpleBinaryName0()Ljava/lang/String;",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_simple_binary_name_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_name = class.name();
    let class_name_parts = class_name.split('$').collect::<Vec<&str>>();

    if class_name_parts.len() <= 1 {
        return Ok(Some(Value::Object(None)));
    }

    let binary_name = class_name_parts[class_name_parts.len() - 1];
    let value: Value = binary_name.to_string().to_object(&thread).await?;
    Ok(Some(value))
}

#[intrinsic_method("java/lang/Class.getSuperclass()Ljava/lang/Class;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_superclass(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    if class.is_primitive() || class.is_interface() {
        return Ok(Some(Value::Object(None)));
    }

    match class.parent()? {
        Some(parent) => {
            let class_name = parent.name();
            let class = thread.class(class_name).await?;
            let class = class.to_object(&thread).await?;
            Ok(Some(class))
        }
        None => Ok(Some(Value::Object(None))),
    }
}

#[intrinsic_method(
    "java/lang/Class.initClassName()Ljava/lang/String;",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_class_name(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: implement support for hidden classes
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    let class_name = class.name().replace('/', ".");
    let value = class_name.to_object(&thread).await?;
    Ok(Some(value))
}

#[intrinsic_method("java/lang/Class.isArray()Z", LessThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn is_array(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    if class.is_array() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[intrinsic_method("java/lang/Class.isAssignableFrom(Ljava/lang/Class;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn is_assignable_from(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object_parameter = parameters.pop()?;
    if object_parameter.is_null() {
        return Err(NullPointerException("object cannot be null".to_string()).into());
    }
    let class_parameter = get_class(&thread, &object_parameter).await?;
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    if class.is_assignable_from(&thread, &class_parameter).await? {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[intrinsic_method("java/lang/Class.isHidden()Z", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn is_hidden(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: implement support for hidden classes
    Ok(Some(Value::from(false)))
}

#[intrinsic_method("java/lang/Class.isInstance(Ljava/lang/Object;)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn is_instance(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let compare_object = parameters.pop()?;
    let self_object = parameters.pop()?;
    let self_class = get_class(&thread, &self_object).await?;

    if compare_object.is_null() {
        return Ok(Some(Value::from(false)));
    }
    let compare_object_class = {
        let compare_object = compare_object.as_object_ref()?;
        compare_object.class().clone()
    };
    if self_class
        .is_assignable_from(&thread, &compare_object_class)
        .await?
    {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[intrinsic_method("java/lang/Class.isInterface()Z", LessThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn is_interface(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    if class.is_interface() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[intrinsic_method("java/lang/Class.isPrimitive()Z", LessThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn is_primitive(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object = parameters.pop()?;
    let class = get_class(&thread, &object).await?;
    if class.is_primitive() {
        Ok(Some(Value::from(true)))
    } else {
        Ok(Some(Value::from(false)))
    }
}

#[intrinsic_method("java/lang/Class.isRecord0()Z", GreaterThanOrEqual(JAVA_17))]
#[async_recursion(?Send)]
pub(crate) async fn is_record_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop()?;
    let class = get_class(&thread, &value).await?;
    let Some(parent_class) = class.parent()? else {
        return Ok(Some(Value::from(false)));
    };
    let parent_class_name = parent_class.name();
    let is_record = parent_class_name == "java/lang/Record";
    Ok(Some(Value::from(is_record)))
}

#[intrinsic_method("java/lang/Class.registerNatives()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn register_natives(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/Class.setSigners([Ljava/lang/Object;)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_signers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    // TODO: Implement set_signers
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::JavaError;
    use ristretto_classfile::Version;
    use ristretto_classloader::Reference;

    #[tokio::test]
    async fn test_desired_assertion_status_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = desired_assertion_status_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_for_name_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "java.lang.String".to_object(&thread).await?;
        let parameters = Parameters::new(vec![
            object,
            Value::from(true),
            Value::Object(None),
            Value::Object(None),
        ]);
        let result = for_name_0(thread, parameters).await?;
        let class_object = result.expect("class");
        let class_object = class_object.as_object_ref()?;
        let class_name = class_object.value("name")?.as_string()?;
        assert_eq!(class_name.as_str(), "java.lang.String");
        Ok(())
    }

    #[tokio::test]
    async fn test_for_name_0_npe() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = Value::Object(None);
        let parameters = Parameters::new(vec![
            object,
            Value::from(true),
            Value::Object(None),
            Value::Object(None),
        ]);
        let result = for_name_0(thread, parameters).await;
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_for_name_0_class_not_found() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![
            object,
            Value::from(true),
            Value::Object(None),
            Value::Object(None),
        ]);
        let result = for_name_0(thread, parameters).await;
        assert!(matches!(result, Err(JavaError(ClassNotFoundException(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_for_name_0_primitive_class_not_found() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "int".to_object(&thread).await?;
        let parameters = Parameters::new(vec![
            object,
            Value::from(true),
            Value::Object(None),
            Value::Object(None),
        ]);
        let result = for_name_0(thread, parameters).await;
        assert!(matches!(result, Err(JavaError(ClassNotFoundException(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_class_access_flags_raw_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.String").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let result = get_class_access_flags_raw_0(thread, parameters).await?;
        let access_flags = result.expect("access_flags").as_i32()?;
        assert_eq!(access_flags, 49);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_class_file_version_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.Object").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let result = get_class_file_version_0(thread, parameters).await?;
        let version = result.expect("version").as_i32()?;
        assert_eq!(version, i32::from(Version::Java25 { minor: 0 }.major()));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_component_type() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("[int").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let result = get_component_type(thread, parameters).await?;
        let class_object = result.expect("class");
        let class_object = class_object.as_object_ref()?;
        let class_name = class_object.value("name")?.as_string()?;
        assert_eq!(class_name.as_str(), "int");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_component_type_null() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.String").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let result = get_component_type(thread, parameters).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Class.getConstantPool()Lsun/reflect/ConstantPool;"
    )]
    async fn test_get_constant_pool_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_constant_pool_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_constant_pool_1() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java.lang.String").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let constant_pool = get_constant_pool_1(thread, parameters)
            .await?
            .expect("constant pool");
        let constant_pool = constant_pool.as_object_ref()?;
        let class_object = constant_pool.value("constantPoolOop")?;
        let class_object = class_object.as_object_ref()?;
        let class_name = class_object.value("name")?.as_string()?;
        assert_eq!(class_name, "java.lang.String");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_declared_classes_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.String").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let value = get_declared_classes_0(thread, parameters)
            .await?
            .expect("interfaces");
        let (class, values) = value.as_class_vec_ref()?;
        assert_eq!(class.name(), "[Ljava/lang/Class;");
        let mut class_names = Vec::new();
        for reference in values.iter().cloned() {
            let reference = reference.expect("interfaces");
            let object = reference.as_object_ref()?;
            let class_name = object.value("name")?;
            let class_name = class_name.as_string()?;
            class_names.push(class_name);
        }
        assert_eq!(
            class_names,
            vec!["java.lang.String$CaseInsensitiveComparator",]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_declared_constructors_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.Integer").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object, Value::from(false)]);
        let value = get_declared_constructors_0(thread, parameters)
            .await?
            .expect("constructors");
        let (class, values) = value.as_class_vec_ref()?;
        assert_eq!(class.name(), "[Ljava/lang/reflect/Constructor;");
        assert_eq!(2, values.len());
        // TODO: Enable test assertions when invokedynamic is implemented
        // let mut signatures = Vec::new();
        // for reference in values.into_iter().flatten() {
        //     let constructor = Value::from(reference);
        //     let result = vm
        //         .invoke(
        //             "java.lang.reflect.Constructor",
        //             "toString",
        //             "()Ljava/lang/String;",
        //             vec![constructor],
        //         )
        //         .await?;
        //     let signature = result.expect("string").as_string()?;
        //     signatures.push(signature);
        // }
        // signatures.sort()
        // assert_eq!(
        //     signatures,
        //     vec![
        //         "public java.lang.Integer(int)",
        //         "public java.lang.Integer(java.lang.String) throws java.lang.NumberFormatException",
        //     ],
        // );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_declared_fields_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.Integer").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object, Value::from(false)]);
        let value = get_declared_fields_0(thread, parameters)
            .await?
            .expect("fields");
        let (class, references) = {
            let (class, values) = value.as_class_vec_ref()?;
            let references = values
                .iter()
                .cloned()
                .map(|reference| reference.expect("reference"))
                .collect::<Vec<Reference>>();
            (class, references)
        };
        assert_eq!(class.name(), "[Ljava/lang/reflect/Field;");
        let mut signatures = Vec::new();
        for reference in references {
            let value = Value::from(reference);
            let result = vm
                .invoke(
                    "java.lang.reflect.Field",
                    "toString()Ljava/lang/String;",
                    &[value],
                )
                .await?;
            let signature = result.expect("string").as_string()?;
            signatures.push(signature);
        }
        signatures.sort();
        assert_eq!(
            signatures,
            vec![
                "private final int java.lang.Integer.value",
                "private static final long java.lang.Integer.serialVersionUID",
                "public static final int java.lang.Integer.BYTES",
                "public static final int java.lang.Integer.MAX_VALUE",
                "public static final int java.lang.Integer.MIN_VALUE",
                "public static final int java.lang.Integer.SIZE",
                "public static final java.lang.Class java.lang.Integer.TYPE",
                "static final byte[] java.lang.Integer.digits",
            ]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_declared_methods_0() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.Boolean").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object, Value::from(false)]);
        let value = get_declared_methods_0(thread, parameters)
            .await?
            .expect("methods");
        let (class, references) = {
            let (class, values) = value.as_class_vec_ref()?;
            let references = values
                .iter()
                .cloned()
                .map(|reference| reference.expect("reference"))
                .collect::<Vec<Reference>>();
            (class, references)
        };
        assert_eq!(class.name(), "[Ljava/lang/reflect/Method;");
        let mut method_names = Vec::new();
        for reference in references {
            let value = Value::from(reference);
            let result = vm
                .invoke(
                    "java.lang.reflect.Method",
                    "getName()Ljava/lang/String;",
                    &[value],
                )
                .await?;
            let method_name = result.expect("string").as_string()?;
            method_names.push(method_name);
        }
        method_names.sort();
        assert_eq!(
            method_names,
            vec![
                "booleanValue",
                "compare",
                "compareTo",
                "compareTo",
                "describeConstable",
                "equals",
                "getBoolean",
                "hashCode",
                "hashCode",
                "logicalAnd",
                "logicalOr",
                "logicalXor",
                "parseBoolean",
                "toString",
                "toString",
                "valueOf",
                "valueOf",
            ]
        );
        // TODO: Enable test assertions when invokedynamic is implemented
        // let mut signatures = Vec::new();
        // for reference in values.into_iter().flatten() {
        //     let result = vm
        //         .invoke(
        //             "java.lang.reflect.Method",
        //             "toString",
        //             "()Ljava/lang/String;",
        //             vec![Value::from(reference.clone())],
        //         )
        //         .await?;
        //     let signature = result.expect("string").as_string()?;
        //     signatures.push(signature);
        // }
        // signatures.sort();
        // assert_eq!(
        //     signatures,
        //     vec![
        //         "public boolean java.lang.Boolean.booleanValue()",
        //         "public boolean java.lang.Boolean.equals(java.lang.Object)",
        //         "public int java.lang.Boolean.compareTo(java.lang.Boolean)",
        //         "public int java.lang.Boolean.compareTo(java.lang.Object)",
        //         "public int java.lang.Boolean.hashCode()",
        //         "public java.lang.String java.lang.Boolean.toString()",
        //         "public java.util.Optional java.lang.Boolean.describeConstable()",
        //         "public static boolean java.lang.Boolean.getBoolean(java.lang.String)",
        //         "public static boolean java.lang.Boolean.logicalAnd(boolean,boolean)",
        //         "public static boolean java.lang.Boolean.logicalOr(boolean,boolean)",
        //         "public static boolean java.lang.Boolean.logicalXor(boolean,boolean)",
        //         "public static boolean java.lang.Boolean.parseBoolean(java.lang.String)",
        //         "public static int java.lang.Boolean.compare(boolean,boolean)",
        //         "public static int java.lang.Boolean.hashCode(boolean)",
        //         "public static java.lang.Boolean java.lang.Boolean.valueOf(boolean)",
        //         "public static java.lang.Boolean java.lang.Boolean.valueOf(java.lang.String)",
        //         "public static java.lang.String java.lang.Boolean.toString(boolean)",
        //     ]
        // );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_declaring_class_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = thread
            .object(
                "java.util.HashMap$Node",
                "ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;",
                &[
                    Value::Int(0),
                    Value::Object(None),
                    Value::Object(None),
                    Value::Object(None),
                ],
            )
            .await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_declaring_class_0(thread, parameters).await?;
        let class_object = result.expect("class");
        let class_object = class_object.as_object_ref()?;
        let class_name = class_object.value("name")?.as_string()?;
        assert_eq!(class_name.as_str(), "java.util.HashMap");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_declaring_class_0_primitive() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("int").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let result = get_declaring_class_0(thread, parameters).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_declaring_class_0_non_inner() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_declaring_class_0(thread, parameters).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Class.getGenericSignature0()Ljava/lang/String;"
    )]
    async fn test_get_generic_signature_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_generic_signature_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_interfaces_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.String").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let value = get_interfaces_0(thread, parameters)
            .await?
            .expect("interfaces");
        let (class, values) = value.as_class_vec_ref()?;
        assert_eq!(class.name(), "[Ljava/lang/Class;");
        let mut class_names = Vec::new();
        for reference in values.iter().cloned() {
            let reference = reference.expect("reference");
            let object = reference.as_object_ref()?;
            let class_name = object.value("name")?;
            let class_name = class_name.as_string()?;
            class_names.push(class_name);
        }
        assert_eq!(
            class_names,
            vec![
                "java.io.Serializable",
                "java.lang.Comparable",
                "java.lang.CharSequence",
                "java.lang.constant.Constable",
                "java.lang.constant.ConstantDesc"
            ]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_modifiers() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_modifiers(thread, parameters).await?;
        let modifiers = result.expect("modifiers").as_i32()?;
        assert_eq!(modifiers, 17);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_name_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_name_0(thread, parameters).await?;
        let class_name = result.expect("object").as_string()?;
        assert_eq!(class_name.as_str(), "java.lang.String");
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Class.getNestHost0()Ljava/lang/Class;"
    )]
    async fn test_get_nest_host_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_nest_host_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Class.getNestMembers0()[Ljava/lang/Class;"
    )]
    async fn test_get_nest_members_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_nest_members_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_permitted_subclasses_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_permitted_subclasses_0(thread, parameters).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_primitive_class() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "int".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_primitive_class(thread, parameters).await?;
        let class_object = result.expect("class");
        let class_object = class_object.as_object_ref()?;
        let class_name = class_object.value("name")?.as_string()?;
        assert_eq!(class_name.as_str(), "int");
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Class.getProtectionDomain0()Ljava/security/ProtectionDomain;"
    )]
    async fn test_get_protection_domain_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_protection_domain_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_raw_annotations() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java.lang.String").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let value = get_raw_annotations(thread, parameters)
            .await?
            .expect("bytes");
        let bytes = value.as_byte_vec_ref()?;
        let bytes = bytes.as_slice();
        assert_eq!(bytes, vec![0, 0]);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_raw_type_annotations() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java.lang.String").await?;
        let class_object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![class_object]);
        let value = get_raw_type_annotations(thread, parameters)
            .await?
            .expect("bytes");
        let bytes = value.as_byte_vec_ref()?;
        let bytes = bytes.as_slice();
        assert_eq!(bytes, vec![0, 0]);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Class.getRecordComponents0()[Ljava/lang/reflect/RecordComponent;"
    )]
    async fn test_get_record_components_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_record_components_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_get_signers() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_signers(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_simple_binary_name_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = thread
            .object(
                "java.util.HashMap$Node",
                "ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Node;",
                &[
                    Value::Int(0),
                    Value::Object(None),
                    Value::Object(None),
                    Value::Object(None),
                ],
            )
            .await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_simple_binary_name_0(thread, parameters).await?;
        let result_object = result.expect("string").as_string()?;
        assert_eq!(result_object.as_str(), "Node");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_simple_binary_name_0_non_inner_class_returns_none() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_simple_binary_name_0(thread, parameters).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_superclass() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_superclass(thread, parameters).await?.expect("result");
        let result_object = result.as_object_ref()?;
        let class_name = result_object.value("name")?.as_string()?;
        assert_eq!(class_name, "java.lang.Object");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_superclass_primitive() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("int").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_superclass(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_superclass_void() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("void").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_superclass(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_superclass_interface() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.io.Serializable").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_superclass(thread, parameters).await?;
        assert_eq!(Some(Value::Object(None)), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_class_name() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = init_class_name(thread, parameters).await?.expect("result");
        let result_object = result.as_string()?;
        assert_eq!(result_object, "java.lang.String");
        Ok(())
    }

    #[tokio::test]
    async fn test_is_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("[int").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = is_array(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_array_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("int").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = is_array(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_assignable_from() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = thread
            .object("java.lang.Object", "", &[] as &[Value])
            .await?;
        let string_object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object, string_object]);
        let result = is_assignable_from(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_assignable_from_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = thread
            .object("java.lang.Object", "", &[] as &[Value])
            .await?;
        let string_object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![string_object, object]);
        let result = is_assignable_from(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_assignable_from_npe() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = thread
            .object("java.lang.Object", "", &[] as &[Value])
            .await?;
        let null_object = Value::Object(None);
        let parameters = Parameters::new(vec![object, null_object]);
        let result = is_assignable_from(thread, parameters).await;
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_hidden() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = is_hidden(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_instance() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = thread
            .object("java.lang.Object", "", &[] as &[Value])
            .await?;
        let string_object = "foo".to_object(&thread).await?;
        let parameters = Parameters::new(vec![object, string_object]);
        let result = is_instance(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_instance_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object = thread
            .object("java.lang.Object", "", &[] as &[Value])
            .await?;
        let null_object = Value::Object(None);
        let parameters = Parameters::new(vec![object, null_object]);
        let result = is_instance(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_interface() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.Cloneable").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = is_interface(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_interface_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.Integer").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = is_interface(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_primitive() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("int").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = is_primitive(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_primitive_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.Integer").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = is_primitive(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_is_record_0_false() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.Integer").await?;
        let object = class.to_object(&thread).await?;
        let parameters = Parameters::new(vec![object]);
        let result = is_record_0(thread, parameters).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_signers() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = set_signers(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
