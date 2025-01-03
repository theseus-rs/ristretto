use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.reflect.ConstantPool`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/reflect/ConstantPool";
    registry.register(
        class_name,
        "getClassAt0",
        "(Ljava/lang/Object;I)Ljava/lang/Class;",
        get_class_at_0,
    );
    registry.register(
        class_name,
        "getClassAtIfLoaded0",
        "(Ljava/lang/Object;I)Ljava/lang/Class;",
        get_class_at_if_loaded_0,
    );
    registry.register(
        class_name,
        "getDoubleAt0",
        "(Ljava/lang/Object;I)D",
        get_double_at_0,
    );
    registry.register(
        class_name,
        "getFieldAt0",
        "(Ljava/lang/Object;I)Ljava/lang/reflect/Field;",
        get_field_at_0,
    );
    registry.register(
        class_name,
        "getFieldAtIfLoaded0",
        "(Ljava/lang/Object;I)Ljava/lang/reflect/Field;",
        get_field_at_if_loaded_0,
    );
    registry.register(
        class_name,
        "getFloatAt0",
        "(Ljava/lang/Object;I)F",
        get_float_at_0,
    );
    registry.register(
        class_name,
        "getIntAt0",
        "(Ljava/lang/Object;I)I",
        get_int_at_0,
    );
    registry.register(
        class_name,
        "getLongAt0",
        "(Ljava/lang/Object;I)J",
        get_long_at_0,
    );
    registry.register(
        class_name,
        "getMemberRefInfoAt0",
        "(Ljava/lang/Object;I)[Ljava/lang/String;",
        get_member_ref_info_at_0,
    );
    registry.register(
        class_name,
        "getMethodAt0",
        "(Ljava/lang/Object;I)Ljava/lang/reflect/Member;",
        get_method_at_0,
    );
    registry.register(
        class_name,
        "getMethodAtIfLoaded0",
        "(Ljava/lang/Object;I)Ljava/lang/reflect/Member;",
        get_method_at_if_loaded_0,
    );
    registry.register(class_name, "getSize0", "(Ljava/lang/Object;)I", get_size_0);
    registry.register(
        class_name,
        "getStringAt0",
        "(Ljava/lang/Object;I)Ljava/lang/String;",
        get_string_at_0,
    );
    registry.register(
        class_name,
        "getUTF8At0",
        "(Ljava/lang/Object;I)Ljava/lang/String;",
        get_utf_8_at_0,
    );
}

#[async_recursion(?Send)]
async fn get_class_at_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getClassAt0(Ljava/lang/Object;I)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_class_at_if_loaded_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getClassAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/Class;")
}

#[async_recursion(?Send)]
async fn get_double_at_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getDoubleAt0(Ljava/lang/Object;I)D")
}

#[async_recursion(?Send)]
async fn get_field_at_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getFieldAt0(Ljava/lang/Object;I)Ljava/lang/reflect/Field;")
}

#[async_recursion(?Send)]
async fn get_field_at_if_loaded_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getFieldAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/reflect/Field;")
}

#[async_recursion(?Send)]
async fn get_float_at_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getFloatAt0(Ljava/lang/Object;I)F")
}

#[async_recursion(?Send)]
async fn get_int_at_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getIntAt0(Ljava/lang/Object;I)I")
}

#[async_recursion(?Send)]
async fn get_long_at_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getLongAt0(Ljava/lang/Object;I)J")
}

#[async_recursion(?Send)]
async fn get_member_ref_info_at_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getMemberRefInfoAt0(Ljava/lang/Object;I)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_method_at_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getMethodAt0(Ljava/lang/Object;I)Ljava/lang/reflect/Member;")
}

#[async_recursion(?Send)]
async fn get_method_at_if_loaded_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getMethodAtIfLoaded0(Ljava/lang/Object;I)Ljava/lang/reflect/Member;")
}

#[async_recursion(?Send)]
async fn get_size_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getSize0(Ljava/lang/Object;)I")
}

#[async_recursion(?Send)]
async fn get_string_at_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getStringAt0(Ljava/lang/Object;I)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_utf_8_at_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.reflect.ConstantPool.getUTF8At0(Ljava/lang/Object;I)Ljava/lang/String;")
}
