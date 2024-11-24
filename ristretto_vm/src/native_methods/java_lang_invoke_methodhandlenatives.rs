use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.invoke.MethodHandleNatives`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/invoke/MethodHandleNatives";
    registry.register(
        class_name,
        "expand",
        "(Ljava/lang/invoke/MemberName;)V",
        expand,
    );
    registry.register(class_name, "getConstant", "(I)I", get_constant);
    registry.register(
        class_name,
        "getMemberVMInfo",
        "(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
        get_member_vm_info,
    );
    registry.register(class_name, "getMembers", "(Ljava/lang/Class;Ljava/lang/String;Ljava/lang/String;ILjava/lang/Class;I[Ljava/lang/invoke/MemberName;)I", get_members);
    registry.register(
        class_name,
        "getNamedCon",
        "(I[Ljava/lang/Object;)I",
        get_named_con,
    );
    registry.register(
        class_name,
        "init",
        "(Ljava/lang/invoke/MemberName;Ljava/lang/Object;)V",
        init,
    );
    registry.register(
        class_name,
        "objectFieldOffset",
        "(Ljava/lang/invoke/MemberName;)J",
        object_field_offset,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
    registry.register(
        class_name,
        "resolve",
        "(Ljava/lang/invoke/MemberName;Ljava/lang/Class;)Ljava/lang/invoke/MemberName;",
        resolve,
    );
    registry.register(
        class_name,
        "setCallSiteTargetNormal",
        "(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V",
        set_call_site_target_normal,
    );
    registry.register(
        class_name,
        "setCallSiteTargetVolatile",
        "(Ljava/lang/invoke/CallSite;Ljava/lang/invoke/MethodHandle;)V",
        set_call_site_target_volatile,
    );
    registry.register(
        class_name,
        "staticFieldBase",
        "(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;",
        static_field_base,
    );
    registry.register(
        class_name,
        "staticFieldOffset",
        "(Ljava/lang/invoke/MemberName;)J",
        static_field_offset,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn expand(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_constant(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_member_vm_info(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_members(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_named_con(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn object_field_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn resolve(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_call_site_target_normal(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_call_site_target_volatile(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn static_field_base(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn static_field_offset(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
