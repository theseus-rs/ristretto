use crate::Error::InternalError;
use crate::intrinsic_methods::java::lang::class;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::{JavaObject, Result};
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{Constant, FieldType, JAVA_11};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getClassAt0(Ljava/lang/Object;I)Ljava/lang/Class;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_class_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let class_name = constant_pool.try_get_class(index)?;
    let class = thread.class(class_name).await?;
    let result = class.to_object(&thread).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getClassAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/Class;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_class_at_if_loaded_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let class_name = constant_pool.try_get_class(index)?;
    let class = thread.class(class_name).await?;
    let result = class.to_object(&thread).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getClassRefIndexAt0(Ljava/lang/Object;I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_class_ref_index_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let class_index = match constant_pool.try_get(index)? {
        Constant::FieldRef { class_index, .. }
        | Constant::InterfaceMethodRef { class_index, .. }
        | Constant::MethodRef { class_index, .. } => class_index,
        value => {
            return Err(InternalError(format!(
                "Expected class reference, found: {value:?}",
            )));
        }
    };
    let class_index = i32::from(*class_index);
    Ok(Some(Value::Int(class_index)))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getDoubleAt0(Ljava/lang/Object;I)D",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_double_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let double = constant_pool.try_get_double(index)?;
    Ok(Some(Value::Double(*double)))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getFieldAt0(Ljava/lang/Object;I)Ljava/lang/reflect/Field;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_field_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let class_object = class.to_object(&thread).await?;
    let (name_index, _descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let field_name = constant_pool.try_get_utf8(*name_index)?;
    let field_name = field_name.to_object(&thread).await?;

    let field = thread
        .invoke(
            "java.lang.Class",
            "getDeclaredField(Ljava/lang/String;)Ljava/lang/reflect/Field;",
            &[class_object, field_name],
        )
        .await?;
    Ok(field)
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getFieldAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/reflect/Field;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_field_at_if_loaded_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_field_at_0(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getFloatAt0(Ljava/lang/Object;I)F",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_float_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let float = constant_pool.try_get_float(index)?;
    Ok(Some(Value::Float(*float)))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getIntAt0(Ljava/lang/Object;I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_int_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let integer = constant_pool.try_get_integer(index)?;
    Ok(Some(Value::Int(*integer)))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getLongAt0(Ljava/lang/Object;I)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_long_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let long = constant_pool.try_get_long(index)?;
    Ok(Some(Value::Long(*long)))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getMemberRefInfoAt0(Ljava/lang/Object;I)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_member_ref_info_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let (class_index, name_and_type_index) = match constant_pool.try_get(index)? {
        Constant::FieldRef {
            class_index,
            name_and_type_index,
        }
        | Constant::InterfaceMethodRef {
            class_index,
            name_and_type_index,
        }
        | Constant::MethodRef {
            class_index,
            name_and_type_index,
        } => (class_index, name_and_type_index),
        value => {
            return Err(InternalError(format!(
                "Expected class, name and type reference, found: {value:?}",
            )));
        }
    };
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class_name = class_name.to_object(&thread).await?.to_reference()?;
    let (name_index, type_index) = constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let name = constant_pool.try_get_utf8(*name_index)?;
    let name = name.to_object(&thread).await?.to_reference()?;
    let descriptor = constant_pool.try_get_utf8(*type_index)?;
    let descriptor = descriptor.to_object(&thread).await?.to_reference()?;
    let string_class = thread.class("java/lang/String").await?;
    let string_array = vec![class_name, name, descriptor];
    let results = Reference::from((string_class, string_array));
    Ok(Some(Value::from(results)))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getMethodAt0(Ljava/lang/Object;I)Ljava/lang/reflect/Member;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_method_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_method_ref(index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let class_object = class.to_object(&thread).await?;
    let (name_index, descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let method_name = constant_pool.try_get_utf8(*name_index)?;
    let method_descriptor = constant_pool.try_get_utf8(*descriptor_index)?;

    let mut parameters_classes = Vec::new();
    let (parameter_types, _return_type) = FieldType::parse_method_descriptor(method_descriptor)?;
    for parameter_type in parameter_types {
        let parameter_type = parameter_type.class_name();
        let parameter_type = thread.class(&parameter_type).await?;
        let parameter_type = parameter_type.to_object(&thread).await?;
        parameters_classes.push(parameter_type);
    }
    let class = thread.class("java.lang.Class").await?;
    let class_parameters = Value::try_from((class, parameters_classes))?;

    let method = if method_name == "<init>" {
        thread
            .invoke(
                "java.lang.Class",
                "getDeclaredConstructor([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;",
                &[class_object, class_parameters],
            )
            .await?
    } else {
        let method_name = method_name.to_object(&thread).await?;
        thread
            .invoke(
                "java.lang.Class",
                "getDeclaredMethod(Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;",
                &[class_object, method_name, class_parameters],
            )
            .await?
    };

    Ok(method)
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getMethodAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/reflect/Member;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_method_at_if_loaded_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    get_method_at_0(thread, parameters).await
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getNameAndTypeRefIndexAt0(Ljava/lang/Object;I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_name_and_type_ref_index_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let name_and_type_index = match constant_pool.try_get(index)? {
        Constant::Dynamic {
            name_and_type_index,
            ..
        }
        | Constant::FieldRef {
            name_and_type_index,
            ..
        }
        | Constant::InterfaceMethodRef {
            name_and_type_index,
            ..
        }
        | Constant::InvokeDynamic {
            name_and_type_index,
            ..
        }
        | Constant::MethodRef {
            name_and_type_index,
            ..
        } => name_and_type_index,
        value => {
            return Err(InternalError(format!(
                "Expected name and type reference, found: {value:?}"
            )));
        }
    };
    let name_and_type_index = i32::from(*name_and_type_index);
    Ok(Some(Value::Int(name_and_type_index)))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getNameAndTypeRefInfoAt0(Ljava/lang/Object;I)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_name_and_type_ref_info_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let name_and_type_index = match constant_pool.try_get(index)? {
        Constant::Dynamic {
            name_and_type_index,
            ..
        }
        | Constant::FieldRef {
            name_and_type_index,
            ..
        }
        | Constant::InterfaceMethodRef {
            name_and_type_index,
            ..
        }
        | Constant::InvokeDynamic {
            name_and_type_index,
            ..
        }
        | Constant::MethodRef {
            name_and_type_index,
            ..
        } => name_and_type_index,
        value => {
            return Err(InternalError(format!(
                "Expected name and type reference, found: {value:?}"
            )));
        }
    };
    let (name_index, type_index) = constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let name = constant_pool.try_get_utf8(*name_index)?;
    let name = name.to_object(&thread).await?.to_reference()?;
    let descriptor = constant_pool.try_get_utf8(*type_index)?;
    let descriptor = descriptor.to_object(&thread).await?.to_reference()?;
    let string_class = thread.class("java/lang/String").await?;
    let string_array = vec![name, descriptor];
    let results = Reference::from((string_class, string_array));
    Ok(Some(Value::from(results)))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getSize0(Ljava/lang/Object;)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_size_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let size = i32::try_from(constant_pool.len())?;
    Ok(Some(Value::from(size)))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getStringAt0(Ljava/lang/Object;I)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_string_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let utf8 = constant_pool.try_get_string(index)?;
    let result = utf8.to_object(&thread).await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getTagAt0(Ljava/lang/Object;I)B",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_tag_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let constant = constant_pool.try_get(index)?;
    thread
        .invoke(
            "jdk.internal.reflect.ConstantPool$Tag",
            "valueOf(B)Ljdk/internal/reflect/ConstantPool$Tag;",
            &[constant.tag()],
        )
        .await
}

#[intrinsic_method(
    "jdk/internal/reflect/ConstantPool.getUTF8At0(Ljava/lang/Object;I)Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_utf_8_at_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = u16::try_from(parameters.pop_int()?)?;
    let object: Object = parameters.pop()?.try_into()?;
    let class = class::get_class(&thread, &object).await?;
    let constant_pool = class.constant_pool();
    let utf8 = constant_pool.try_get_utf8(index)?;
    let result = utf8.to_object(&thread).await?;
    Ok(Some(result))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::VM;
    use crate::intrinsic_methods::registry::IntrinsicMethod;
    use ristretto_classfile::attributes::{Attribute, Instruction};
    use ristretto_classfile::{
        BaseType, ClassAccessFlags, ClassFile, ConstantPool, FieldAccessFlags, FieldType,
        MethodAccessFlags,
    };
    use ristretto_classloader::Class;

    pub(crate) async fn test_object() -> Result<(Arc<VM>, Arc<Thread>, Value)> {
        let (vm, thread) = crate::test::thread().await.expect("thread");
        let mut constant_pool = ConstantPool::default();
        let super_class = constant_pool.add_class("java/lang/Object")?;
        let object_init = constant_pool.add_method_ref(super_class, "<init>", "()V")?;
        let this_class = constant_pool.add_class("TestClass")?;
        let _int = constant_pool.add_integer(1)?;
        let _long = constant_pool.add_long(2)?;
        let _float = constant_pool.add_float(3.0)?;
        let _double = constant_pool.add_double(4.0)?;
        let _string = constant_pool.add_string("foo")?;
        let x_field = constant_pool.add_field_ref(this_class, "x", "I")?;
        let code_index = constant_pool.add_utf8("Code")?;
        let main_method_index =
            constant_pool.add_method_ref(this_class, "main", "([Ljava/lang/String;)V")?;

        let mut fields = Vec::new();
        let (_class_index, name_and_type_index) = constant_pool.try_get_field_ref(x_field)?;
        let (name_index, descriptor_index) =
            constant_pool.try_get_name_and_type(*name_and_type_index)?;
        fields.push(ristretto_classfile::Field {
            access_flags: FieldAccessFlags::PUBLIC,
            name_index: *name_index,
            descriptor_index: *descriptor_index,
            field_type: FieldType::Base(BaseType::Int),
            attributes: Vec::new(),
        });

        let mut methods = Vec::new();
        let (_class_index, name_and_type_index) = constant_pool.try_get_method_ref(object_init)?;
        let (name_index, descriptor_index) =
            constant_pool.try_get_name_and_type(*name_and_type_index)?;
        let mut init_method = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: *name_index,
            descriptor_index: *descriptor_index,
            attributes: Vec::new(),
        };
        init_method.attributes.push(Attribute::Code {
            name_index: code_index,
            max_stack: 1,
            max_locals: 1,
            code: vec![
                Instruction::Aload_0,
                Instruction::Invokespecial(object_init),
                Instruction::Return,
            ],
            exception_table: Vec::new(),
            attributes: Vec::new(),
        });
        methods.push(init_method);

        let (_class_index, main_name_and_type_index) =
            constant_pool.try_get_method_ref(main_method_index)?;
        let (main_name_index, main_descriptor_index) =
            constant_pool.try_get_name_and_type(*main_name_and_type_index)?;
        let mut main_method = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: *main_name_index,
            descriptor_index: *main_descriptor_index,
            attributes: Vec::new(),
        };
        main_method.attributes.push(Attribute::Code {
            name_index: code_index,
            max_stack: 2,
            max_locals: 1,
            code: vec![Instruction::Return],
            exception_table: Vec::new(),
            attributes: Vec::new(),
        });
        methods.push(main_method);

        let class_file = ClassFile {
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
            constant_pool,
            this_class,
            super_class,
            fields,
            methods,
            ..Default::default()
        };
        let class = Class::from(class_file)?;
        let class_loader_lock = vm.class_loader();
        let class_loader = class_loader_lock.write().await;
        class_loader.register(class.clone()).await?;
        let object = Object::new(class)?;
        Ok((vm, thread, Value::from(object)))
    }

    pub(crate) async fn get_class_at_test(get_class_at: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(8)]);
        let result = get_class_at(thread, parameters).await?.expect("value");
        let object: Object = result.try_into()?;
        let class_name: String = object.value("name")?.try_into()?;
        assert_eq!("TestClass", class_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_class_at_0() -> Result<()> {
        get_class_at_test(get_class_at_0).await
    }

    pub(crate) async fn get_class_at_if_loaded_test(
        get_class_at_if_loaded: IntrinsicMethod,
    ) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(8)]);
        let result = get_class_at_if_loaded(thread, parameters)
            .await?
            .expect("value");
        let object: Object = result.try_into()?;
        let class_name: String = object.value("name")?.try_into()?;
        assert_eq!("TestClass", class_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_class_at_if_loaded_0() -> Result<()> {
        get_class_at_if_loaded_test(get_class_at_if_loaded_0).await
    }

    #[tokio::test]
    async fn test_get_class_ref_index_at_0_field() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(20)]);
        let result = get_class_ref_index_at_0(thread, parameters)
            .await?
            .expect("value");
        let class_index: u16 = result.try_into()?;
        assert_eq!(8, class_index);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_class_ref_index_at_0_method() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(6)]);
        let result = get_class_ref_index_at_0(thread, parameters)
            .await?
            .expect("value");
        let class_index: u16 = result.try_into()?;
        assert_eq!(2, class_index);
        Ok(())
    }

    pub(crate) async fn get_double_at_test(get_double_at: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(13)]);
        let result = get_double_at(thread, parameters).await?.expect("value");
        let value: f64 = result.try_into()?;
        let value = value - 4.0f64;
        assert!(value.abs() < 0.01f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_at_0() -> Result<()> {
        get_double_at_test(get_double_at_0).await
    }

    pub(crate) async fn get_field_at_test(get_field_at: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(20)]);
        let result = get_field_at(thread, parameters).await?.expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Field", class.name());
        let name: String = object.value("name")?.try_into()?;
        assert_eq!("x", name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_field_at_0() -> Result<()> {
        get_field_at_test(get_field_at_0).await
    }

    pub(crate) async fn get_field_at_if_loaded_test(
        get_field_at_if_loaded: IntrinsicMethod,
    ) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(20)]);
        let result = get_field_at_if_loaded(thread, parameters)
            .await?
            .expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Field", class.name());
        let name: String = object.value("name")?.try_into()?;
        assert_eq!("x", name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_field_at_if_loaded_0() -> Result<()> {
        get_field_at_if_loaded_test(get_field_at_if_loaded_0).await
    }

    pub(crate) async fn get_float_at_test(get_float_at: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(12)]);
        let result = get_float_at(thread, parameters).await?.expect("value");
        let value: f32 = result.try_into()?;
        let value = value - 3.0f32;
        assert!(value.abs() < 0.01f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_float_at_0() -> Result<()> {
        get_float_at_test(get_float_at_0).await
    }

    pub(crate) async fn get_int_at_test(get_int_at: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(9)]);
        let result = get_int_at(thread, parameters).await?.expect("value");
        let value: i32 = result.try_into()?;
        assert_eq!(1, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_int_at_0() -> Result<()> {
        get_int_at_test(get_int_at_0).await
    }

    pub(crate) async fn get_long_at_test(get_long_at: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(10)]);
        let result = get_long_at(thread, parameters).await?.expect("value");
        let value: i64 = result.try_into()?;
        assert_eq!(2, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_long_at_0() -> Result<()> {
        get_long_at_test(get_long_at_0).await
    }

    pub(crate) async fn get_member_ref_info_at_field_test(
        get_member_ref_info_at: IntrinsicMethod,
    ) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(20)]);
        let result = get_member_ref_info_at(thread, parameters)
            .await?
            .expect("value");
        let (class, values) = result.try_into()?;
        assert_eq!("java/lang/String", class.name());
        assert_eq!(3, values.len());
        let class_name: String = values
            .first()
            .expect("value")
            .clone()
            .expect("class name")
            .try_into()?;
        let name: String = values
            .get(1)
            .expect("value")
            .clone()
            .expect("name")
            .try_into()?;
        let descriptor: String = values
            .get(2)
            .expect("value")
            .clone()
            .expect("descriptor")
            .try_into()?;
        assert_eq!("TestClass", class_name);
        assert_eq!("x", name);
        assert_eq!("I", descriptor);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_member_ref_info_at_0_field() -> Result<()> {
        get_member_ref_info_at_field_test(get_member_ref_info_at_0).await
    }

    pub(crate) async fn get_member_ref_info_at_method_test(
        get_member_ref_info_at: IntrinsicMethod,
    ) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(6)]);
        let result = get_member_ref_info_at(thread, parameters)
            .await?
            .expect("value");
        let (class, values) = result.try_into()?;
        assert_eq!("java/lang/String", class.name());
        assert_eq!(3, values.len());
        let class_name: String = values
            .first()
            .expect("value")
            .clone()
            .expect("class name")
            .try_into()?;
        let name: String = values
            .get(1)
            .expect("value")
            .clone()
            .expect("name")
            .try_into()?;
        let descriptor: String = values
            .get(2)
            .expect("value")
            .clone()
            .expect("descriptor")
            .try_into()?;
        assert_eq!("java/lang/Object", class_name);
        assert_eq!("<init>", name);
        assert_eq!("()V", descriptor);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_member_ref_info_at_0_method() -> Result<()> {
        get_member_ref_info_at_method_test(get_member_ref_info_at_0).await
    }

    pub(crate) async fn get_method_at_constructor_test(
        get_method_at: IntrinsicMethod,
    ) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(6)]);
        let result = get_method_at(thread, parameters).await?.expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Constructor", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_method_at_0_constructor() -> Result<()> {
        get_method_at_constructor_test(get_method_at_0).await
    }

    pub(crate) async fn get_method_at_method_test(get_method_at: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(25)]);
        let result = get_method_at(thread, parameters).await?.expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Method", class.name());
        let method_name: String = object.value("name")?.try_into()?;
        assert_eq!("main", method_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_method_at_0_method() -> Result<()> {
        get_method_at_method_test(get_method_at_0).await
    }

    pub(crate) async fn get_method_at_if_loaded_constructor_test(
        get_method_at_if_loaded: IntrinsicMethod,
    ) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(6)]);
        let result = get_method_at_if_loaded(thread, parameters)
            .await?
            .expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Constructor", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_method_at_if_loaded_0_constructor() -> Result<()> {
        get_method_at_if_loaded_constructor_test(get_method_at_if_loaded_0).await
    }

    pub(crate) async fn get_method_at_if_loaded_method_test(
        get_method_at_if_loaded: IntrinsicMethod,
    ) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(25)]);
        let result = get_method_at_if_loaded(thread, parameters)
            .await?
            .expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Method", class.name());
        let method_name: String = object.value("name")?.try_into()?;
        assert_eq!("main", method_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_method_at_if_loaded_0_method() -> Result<()> {
        get_method_at_if_loaded_method_test(get_method_at_if_loaded_0).await
    }

    #[tokio::test]
    async fn test_get_name_and_type_ref_index_at_0_field() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(20)]);
        let result = get_name_and_type_ref_index_at_0(thread, parameters)
            .await?
            .expect("value");
        let class_index: u16 = result.try_into()?;
        assert_eq!(19, class_index);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_name_and_type_ref_index_at_0_method() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(6)]);
        let result = get_name_and_type_ref_index_at_0(thread, parameters)
            .await?
            .expect("value");
        let class_index: u16 = result.try_into()?;
        assert_eq!(5, class_index);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_name_and_type_ref_info_at_0_field() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(20)]);
        let result = get_name_and_type_ref_info_at_0(thread, parameters)
            .await?
            .expect("value");
        let (class, values) = result.try_into()?;
        assert_eq!("java/lang/String", class.name());
        assert_eq!(2, values.len());
        let name: String = values
            .first()
            .expect("value")
            .clone()
            .expect("name")
            .try_into()?;
        let descriptor: String = values
            .get(1)
            .expect("value")
            .clone()
            .expect("descriptor")
            .try_into()?;
        assert_eq!("x", name);
        assert_eq!("I", descriptor);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_name_and_type_ref_info_at_0_method() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(6)]);
        let result = get_name_and_type_ref_info_at_0(thread, parameters)
            .await?
            .expect("value");
        let (class, values) = result.try_into()?;
        assert_eq!("java/lang/String", class.name());
        assert_eq!(2, values.len());
        let name: String = values
            .first()
            .expect("value")
            .clone()
            .expect("name")
            .try_into()?;
        let descriptor: String = values
            .get(1)
            .expect("value")
            .clone()
            .expect("descriptor")
            .try_into()?;
        assert_eq!("<init>", name);
        assert_eq!("()V", descriptor);
        Ok(())
    }

    pub(crate) async fn get_size_test(get_size: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_size(thread, parameters).await?.expect("value");
        let class_index: u16 = result.try_into()?;
        assert_eq!(25, class_index);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_size_0() -> Result<()> {
        get_size_test(get_size_0).await
    }

    pub(crate) async fn get_string_at_test(get_string_at: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(16)]);
        let result = get_string_at(thread, parameters).await?.expect("value");
        let value: String = result.try_into()?;
        assert_eq!("foo", value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_string_at_0() -> Result<()> {
        get_string_at_test(get_string_at_0).await
    }

    #[tokio::test]
    async fn test_get_tag_at_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(16)]);
        let result = get_tag_at_0(thread, parameters).await?.expect("value");
        let value: Object = result.try_into()?;
        let class = value.class();
        assert_eq!("jdk/internal/reflect/ConstantPool$Tag", class.name());
        let name: String = value.value("name")?.try_into()?;
        assert_eq!("STRING", name);
        Ok(())
    }

    pub(crate) async fn get_utf_8_at_test(get_utf_8_at: IntrinsicMethod) -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(15)]);
        let result = get_utf_8_at(thread, parameters).await?.expect("value");
        let value: String = result.try_into()?;
        assert_eq!("foo", value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_utf_8_at_0() -> Result<()> {
        get_utf_8_at_test(get_utf_8_at_0).await
    }
}
