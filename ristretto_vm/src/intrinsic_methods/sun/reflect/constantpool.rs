use crate::Result;
use crate::intrinsic_methods::jdk;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/reflect/ConstantPool";

/// Register all intrinsic methods for `sun.reflect.ConstantPool`.
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

    #[tokio::test]
    async fn test_get_class_at_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_class_at_test(get_class_at_0).await
    }

    #[tokio::test]
    async fn test_get_class_at_if_loaded_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_class_at_if_loaded_test(
            get_class_at_if_loaded_0,
        )
        .await
    }

    #[tokio::test]
    async fn test_get_double_at_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_double_at_test(get_double_at_0).await
    }

    #[tokio::test]
    async fn test_get_field_at_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_field_at_test(get_field_at_0).await
    }

    #[tokio::test]
    async fn test_get_field_at_if_loaded_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_field_at_if_loaded_test(
            get_field_at_if_loaded_0,
        )
        .await
    }

    #[tokio::test]
    async fn test_get_float_at_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_float_at_test(get_float_at_0).await
    }

    #[tokio::test]
    async fn test_get_int_at_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_int_at_test(get_int_at_0).await
    }

    #[tokio::test]
    async fn test_get_long_at_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_long_at_test(get_long_at_0).await
    }

    #[tokio::test]
    async fn test_get_member_ref_info_at_0_field() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_member_ref_info_at_field_test(
            get_member_ref_info_at_0,
        )
        .await
    }

    #[tokio::test]
    async fn test_get_member_ref_info_at_0_method() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_member_ref_info_at_method_test(
            get_member_ref_info_at_0,
        )
        .await
    }

    #[tokio::test]
    async fn test_get_method_at_0_constructor() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_method_at_constructor_test(get_method_at_0)
            .await
    }

    #[tokio::test]
    async fn test_get_method_at_0_method() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_method_at_method_test(get_method_at_0)
            .await
    }

    #[tokio::test]
    async fn test_get_method_at_if_loaded_0_constructor() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_method_at_if_loaded_constructor_test(
            get_method_at_if_loaded_0,
        )
        .await
    }

    #[tokio::test]
    async fn test_get_method_at_if_loaded_0_method() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_method_at_if_loaded_method_test(
            get_method_at_if_loaded_0,
        )
        .await
    }

    #[tokio::test]
    async fn test_get_size_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_size_test(get_size_0).await
    }

    #[tokio::test]
    async fn test_get_string_at_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_string_at_test(get_string_at_0).await
    }

    #[tokio::test]
    async fn test_get_utf_8_at_0() -> Result<()> {
        jdk::internal::reflect::constantpool::tests::get_utf_8_at_test(get_utf_8_at_0).await
    }
}
