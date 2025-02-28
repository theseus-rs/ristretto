use crate::Result;
use crate::native_methods::jdk;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/reflect/ConstantPool";

/// Register all native methods for `sun.reflect.ConstantPool`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getClassAt0",
        "(Ljava/lang/Object;I)Ljava/lang/Class;",
        get_class_at_0,
    );
    registry.register(
        CLASS_NAME,
        "getClassAtIfLoaded0",
        "(Ljava/lang/Object;I)Ljava/lang/Class;",
        get_class_at_if_loaded_0,
    );
    registry.register(
        CLASS_NAME,
        "getDoubleAt0",
        "(Ljava/lang/Object;I)D",
        get_double_at_0,
    );
    registry.register(
        CLASS_NAME,
        "getFieldAt0",
        "(Ljava/lang/Object;I)Ljava/lang/reflect/Field;",
        get_field_at_0,
    );
    registry.register(
        CLASS_NAME,
        "getFieldAtIfLoaded0",
        "(Ljava/lang/Object;I)Ljava/lang/reflect/Field;",
        get_field_at_if_loaded_0,
    );
    registry.register(
        CLASS_NAME,
        "getFloatAt0",
        "(Ljava/lang/Object;I)F",
        get_float_at_0,
    );
    registry.register(
        CLASS_NAME,
        "getIntAt0",
        "(Ljava/lang/Object;I)I",
        get_int_at_0,
    );
    registry.register(
        CLASS_NAME,
        "getLongAt0",
        "(Ljava/lang/Object;I)J",
        get_long_at_0,
    );
    registry.register(
        CLASS_NAME,
        "getMemberRefInfoAt0",
        "(Ljava/lang/Object;I)[Ljava/lang/String;",
        get_member_ref_info_at_0,
    );
    registry.register(
        CLASS_NAME,
        "getMethodAt0",
        "(Ljava/lang/Object;I)Ljava/lang/reflect/Member;",
        get_method_at_0,
    );
    registry.register(
        CLASS_NAME,
        "getMethodAtIfLoaded0",
        "(Ljava/lang/Object;I)Ljava/lang/reflect/Member;",
        get_method_at_if_loaded_0,
    );
    registry.register(CLASS_NAME, "getSize0", "(Ljava/lang/Object;)I", get_size_0);
    registry.register(
        CLASS_NAME,
        "getStringAt0",
        "(Ljava/lang/Object;I)Ljava/lang/String;",
        get_string_at_0,
    );
    registry.register(
        CLASS_NAME,
        "getUTF8At0",
        "(Ljava/lang/Object;I)Ljava/lang/String;",
        get_utf_8_at_0,
    );
}

#[async_recursion(?Send)]
async fn get_class_at_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_class_at_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_class_at_if_loaded_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_class_at_if_loaded_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_double_at_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_double_at_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_field_at_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_field_at_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_field_at_if_loaded_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_field_at_if_loaded_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_float_at_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_float_at_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_int_at_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_int_at_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_long_at_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_long_at_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_member_ref_info_at_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_member_ref_info_at_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_method_at_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_method_at_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_method_at_if_loaded_0(
    thread: Arc<Thread>,
    parameters: Parameters,
) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_method_at_if_loaded_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_size_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_size_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_string_at_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_string_at_0(thread, parameters).await
}

#[async_recursion(?Send)]
async fn get_utf_8_at_0(thread: Arc<Thread>, parameters: Parameters) -> Result<Option<Value>> {
    jdk::internal::reflect::constantpool::get_utf_8_at_0(thread, parameters).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use jdk::internal::reflect::constantpool::tests::test_object;
    use ristretto_classloader::Object;

    #[tokio::test]
    async fn test_get_class_at_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(8)]);
        let result = get_class_at_0(thread, parameters).await?.expect("value");
        let object: Object = result.try_into()?;
        let class_name: String = object.value("name")?.try_into()?;
        assert_eq!("TestClass", class_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_class_at_if_loaded_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(8)]);
        let result = get_class_at_if_loaded_0(thread, parameters)
            .await?
            .expect("value");
        let object: Object = result.try_into()?;
        let class_name: String = object.value("name")?.try_into()?;
        assert_eq!("TestClass", class_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_double_at_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(13)]);
        let result = get_double_at_0(thread, parameters).await?.expect("value");
        let value: f64 = result.try_into()?;
        assert_eq!(4.0f64, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_field_at_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(20)]);
        let result = get_field_at_0(thread, parameters).await?.expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Field", class.name());
        let name: String = object.value("name")?.try_into()?;
        assert_eq!("x", name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_field_at_if_loaded_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(20)]);
        let result = get_field_at_if_loaded_0(thread, parameters)
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
    async fn test_get_float_at_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(12)]);
        let result = get_float_at_0(thread, parameters).await?.expect("value");
        let value: f32 = result.try_into()?;
        assert_eq!(3.0f32, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_int_at_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(9)]);
        let result = get_int_at_0(thread, parameters).await?.expect("value");
        let value: i32 = result.try_into()?;
        assert_eq!(1, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_long_at_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(10)]);
        let result = get_long_at_0(thread, parameters).await?.expect("value");
        let value: i64 = result.try_into()?;
        assert_eq!(2, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_member_ref_info_at_0_field() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(20)]);
        let result = get_member_ref_info_at_0(thread, parameters)
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
    async fn test_get_member_ref_info_at_0_method() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(6)]);
        let result = get_member_ref_info_at_0(thread, parameters)
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
    async fn test_get_method_at_0_constructor() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(6)]);
        let result = get_method_at_0(thread, parameters).await?.expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Constructor", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_method_at_0_method() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(25)]);
        let result = get_method_at_0(thread, parameters).await?.expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Method", class.name());
        let method_name: String = object.value("name")?.try_into()?;
        assert_eq!("main", method_name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_method_at_if_loaded_0_constructor() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(6)]);
        let result = get_method_at_if_loaded_0(thread, parameters)
            .await?
            .expect("value");
        let object: Object = result.try_into()?;
        let class = object.class();
        assert_eq!("java/lang/reflect/Constructor", class.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_method_at_if_loaded_0_method() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(25)]);
        let result = get_method_at_if_loaded_0(thread, parameters)
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
    async fn test_get_size_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object]);
        let result = get_size_0(thread, parameters).await?.expect("value");
        let class_index: u16 = result.try_into()?;
        assert_eq!(25, class_index);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_string_at_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(16)]);
        let result = get_string_at_0(thread, parameters).await?.expect("value");
        let value: String = result.try_into()?;
        assert_eq!("foo", value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_utf_8_at_0() -> Result<()> {
        let (_vm, thread, object) = test_object().await?;
        let parameters = Parameters::new(vec![object, Value::Int(15)]);
        let result = get_utf_8_at_0(thread, parameters).await?.expect("value");
        let value: String = result.try_into()?;
        assert_eq!("foo", value);
        Ok(())
    }
}
