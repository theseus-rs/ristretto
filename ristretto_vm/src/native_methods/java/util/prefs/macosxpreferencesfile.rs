use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/util/prefs/MacOSXPreferencesFile";

/// Register all native methods for `java.util.prefs.MacOSXPreferencesFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "addChildToNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Z",
        add_child_to_node,
    );
    registry.register(
        CLASS_NAME,
        "addKeyToNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
        add_key_to_node,
    );
    registry.register(
        CLASS_NAME,
        "addNode",
        "(Ljava/lang/String;Ljava/lang/String;JJ)Z",
        add_node,
    );
    registry.register(CLASS_NAME, "anyHost", "()J", any_host);
    registry.register(CLASS_NAME, "anyUser", "()J", any_user);
    registry.register(CLASS_NAME, "currentHost", "()J", current_host);
    registry.register(CLASS_NAME, "currentUser", "()J", current_user);
    registry.register(
        CLASS_NAME,
        "getChildrenForNode",
        "(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;",
        get_children_for_node,
    );
    registry.register(
        CLASS_NAME,
        "getKeyFromNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Ljava/lang/String;",
        get_key_from_node,
    );
    registry.register(
        CLASS_NAME,
        "getKeysForNode",
        "(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;",
        get_keys_for_node,
    );
    registry.register(
        CLASS_NAME,
        "removeChildFromNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
        remove_child_from_node,
    );
    registry.register(
        CLASS_NAME,
        "removeKeyFromNode",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
        remove_key_from_node,
    );
    registry.register(
        CLASS_NAME,
        "removeNode",
        "(Ljava/lang/String;Ljava/lang/String;JJ)V",
        remove_node,
    );
    registry.register(
        CLASS_NAME,
        "synchronize",
        "(Ljava/lang/String;JJ)Z",
        synchronize,
    );
}

#[async_recursion(?Send)]
async fn add_child_to_node(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.addChildToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Z")
}

#[async_recursion(?Send)]
async fn add_key_to_node(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.addKeyToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V")
}

#[async_recursion(?Send)]
async fn add_node(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.addNode(Ljava/lang/String;Ljava/lang/String;JJ)Z")
}

#[async_recursion(?Send)]
async fn any_host(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.anyHost()J")
}

#[async_recursion(?Send)]
async fn any_user(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.anyUser()J")
}

#[async_recursion(?Send)]
async fn current_host(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.currentHost()J")
}

#[async_recursion(?Send)]
async fn current_user(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.currentUser()J")
}

#[async_recursion(?Send)]
async fn get_children_for_node(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.getChildrenForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_key_from_node(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.getKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_keys_for_node(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.getKeysForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn remove_child_from_node(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.removeChildFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V")
}

#[async_recursion(?Send)]
async fn remove_key_from_node(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.removeKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V")
}

#[async_recursion(?Send)]
async fn remove_node(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "java.util.prefs.MacOSXPreferencesFile.removeNode(Ljava/lang/String;Ljava/lang/String;JJ)V"
    )
}

#[async_recursion(?Send)]
async fn synchronize(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.util.prefs.MacOSXPreferencesFile.synchronize(Ljava/lang/String;JJ)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.addChildToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Z"
    )]
    async fn test_add_child_to_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_child_to_node(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.addKeyToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V"
    )]
    async fn test_add_key_to_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_key_to_node(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.addNode(Ljava/lang/String;Ljava/lang/String;JJ)Z"
    )]
    async fn test_add_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_node(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.anyHost()J"
    )]
    async fn test_any_host() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = any_host(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.anyUser()J"
    )]
    async fn test_any_user() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = any_user(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.currentHost()J"
    )]
    async fn test_current_host() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_host(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.currentUser()J"
    )]
    async fn test_current_user() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = current_user(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.getChildrenForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;"
    )]
    async fn test_get_children_for_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_children_for_node(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.getKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Ljava/lang/String;"
    )]
    async fn test_get_key_from_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_key_from_node(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.getKeysForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;"
    )]
    async fn test_get_keys_for_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_keys_for_node(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.removeChildFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V"
    )]
    async fn test_remove_child_from_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_child_from_node(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.removeKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V"
    )]
    async fn test_remove_key_from_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_key_from_node(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.removeNode(Ljava/lang/String;Ljava/lang/String;JJ)V"
    )]
    async fn test_remove_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_node(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.util.prefs.MacOSXPreferencesFile.synchronize(Ljava/lang/String;JJ)Z"
    )]
    async fn test_synchronize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = synchronize(thread, Parameters::default()).await;
    }
}
