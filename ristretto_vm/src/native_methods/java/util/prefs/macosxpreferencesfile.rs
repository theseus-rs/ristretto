use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.util.prefs.MacOSXPreferencesFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/util/prefs/MacOSXPreferencesFile";
    registry.register(
        class_name,
        "addChildToNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Z",
        add_child_to_node,
    );
    registry.register(
        class_name,
        "addKeyToNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
        add_key_to_node,
    );
    registry.register(
        class_name,
        "addNode",
        "(Ljava/lang/String;Ljava/lang/String;JJ)Z",
        add_node,
    );
    registry.register(class_name, "anyHost", "()J", any_host);
    registry.register(class_name, "anyUser", "()J", any_user);
    registry.register(class_name, "currentHost", "()J", current_host);
    registry.register(class_name, "currentUser", "()J", current_user);
    registry.register(
        class_name,
        "getChildrenForNode",
        "(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;",
        get_children_for_node,
    );
    registry.register(
        class_name,
        "getKeyFromNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Ljava/lang/String;",
        get_key_from_node,
    );
    registry.register(
        class_name,
        "getKeysForNode",
        "(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;",
        get_keys_for_node,
    );
    registry.register(
        class_name,
        "removeChildFromNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
        remove_child_from_node,
    );
    registry.register(
        class_name,
        "removeKeyFromNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
        remove_key_from_node,
    );
    registry.register(
        class_name,
        "removeNode",
        "(Ljava/lang/String;Ljava/lang/String;JJ)V",
        remove_node,
    );
    registry.register(
        class_name,
        "synchronize",
        "(Ljava/lang/String;JJ)Z",
        synchronize,
    );
}

#[async_recursion(?Send)]
async fn add_child_to_node(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn add_key_to_node(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn add_node(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn any_host(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn any_user(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn current_host(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn current_user(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_children_for_node(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_key_from_node(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_keys_for_node(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn remove_child_from_node(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn remove_key_from_node(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn remove_node(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn synchronize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
