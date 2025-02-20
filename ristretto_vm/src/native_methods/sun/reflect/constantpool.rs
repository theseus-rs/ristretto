use crate::Result;
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
async fn get_class_at_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getClassAt0(Ljava/lang/Object;I)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_class_at_if_loaded_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getClassAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_double_at_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getDoubleAt0(Ljava/lang/Object;I)D")
}

#[async_recursion(?Send)]
async fn get_field_at_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getFieldAt0(Ljava/lang/Object;I)Ljava/lang/reflect/Field;")
}

#[async_recursion(?Send)]
async fn get_field_at_if_loaded_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.reflect.ConstantPool.getFieldAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/reflect/Field;"
    )
}

#[async_recursion(?Send)]
async fn get_float_at_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getFloatAt0(Ljava/lang/Object;I)F")
}

#[async_recursion(?Send)]
async fn get_int_at_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getIntAt0(Ljava/lang/Object;I)I")
}

#[async_recursion(?Send)]
async fn get_long_at_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getLongAt0(Ljava/lang/Object;I)J")
}

#[async_recursion(?Send)]
async fn get_member_ref_info_at_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getMemberRefInfoAt0(Ljava/lang/Object;I)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_method_at_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getMethodAt0(Ljava/lang/Object;I)Ljava/lang/reflect/Member;")
}

#[async_recursion(?Send)]
async fn get_method_at_if_loaded_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.reflect.ConstantPool.getMethodAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/reflect/Member;"
    )
}

#[async_recursion(?Send)]
async fn get_size_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getSize0(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_string_at_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getStringAt0(Ljava/lang/Object;I)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_utf_8_at_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getUTF8At0(Ljava/lang/Object;I)Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getClassAt0(Ljava/lang/Object;I)Ljava/lang/Class;"
    )]
    async fn test_get_class_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_at_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getClassAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/Class;"
    )]
    async fn test_get_class_at_if_loaded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_class_at_if_loaded_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getDoubleAt0(Ljava/lang/Object;I)D"
    )]
    async fn test_get_double_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_double_at_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getFieldAt0(Ljava/lang/Object;I)Ljava/lang/reflect/Field;"
    )]
    async fn test_get_field_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_field_at_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getFieldAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/reflect/Field;"
    )]
    async fn test_get_field_at_if_loaded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_field_at_if_loaded_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getFloatAt0(Ljava/lang/Object;I)F"
    )]
    async fn test_get_float_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_float_at_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getIntAt0(Ljava/lang/Object;I)I"
    )]
    async fn test_get_int_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_int_at_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getLongAt0(Ljava/lang/Object;I)J"
    )]
    async fn test_get_long_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_long_at_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getMemberRefInfoAt0(Ljava/lang/Object;I)[Ljava/lang/String;"
    )]
    async fn test_get_member_ref_info_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_member_ref_info_at_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getMethodAt0(Ljava/lang/Object;I)Ljava/lang/reflect/Member;"
    )]
    async fn test_get_method_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_method_at_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getMethodAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/reflect/Member;"
    )]
    async fn test_get_method_at_if_loaded_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_method_at_if_loaded_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getSize0(Ljava/lang/Object;)I"
    )]
    async fn test_get_size_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_size_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getStringAt0(Ljava/lang/Object;I)Ljava/lang/String;"
    )]
    async fn test_get_string_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_string_at_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.reflect.ConstantPool.getUTF8At0(Ljava/lang/Object;I)Ljava/lang/String;"
    )]
    async fn test_get_utf_8_at_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_utf_8_at_0(thread, Parameters::default()).await;
    }
}
