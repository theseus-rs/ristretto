use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.addChildToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Z",
    Any
)]
#[async_method]
pub async fn add_child_to_node<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.util.prefs.MacOSXPreferencesFile.addChildToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Z".to_string()).into())
}

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.addKeyToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
    Any
)]
#[async_method]
pub async fn add_key_to_node<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.util.prefs.MacOSXPreferencesFile.addKeyToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V".to_string()).into())
}

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.addNode(Ljava/lang/String;Ljava/lang/String;JJ)Z",
    Any
)]
#[async_method]
pub async fn add_node<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.MacOSXPreferencesFile.addNode(Ljava/lang/String;Ljava/lang/String;JJ)Z"
            .to_string(),
    )
    .into())
}

#[intrinsic_method("java/util/prefs/MacOSXPreferencesFile.anyHost()J", Any)]
#[async_method]
pub async fn any_host<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.MacOSXPreferencesFile.anyHost()J".to_string(),
    )
    .into())
}

#[intrinsic_method("java/util/prefs/MacOSXPreferencesFile.anyUser()J", Any)]
#[async_method]
pub async fn any_user<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.MacOSXPreferencesFile.anyUser()J".to_string(),
    )
    .into())
}

#[intrinsic_method("java/util/prefs/MacOSXPreferencesFile.currentHost()J", Any)]
#[async_method]
pub async fn current_host<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.MacOSXPreferencesFile.currentHost()J".to_string(),
    )
    .into())
}

#[intrinsic_method("java/util/prefs/MacOSXPreferencesFile.currentUser()J", Any)]
#[async_method]
pub async fn current_user<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.MacOSXPreferencesFile.currentUser()J".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.getChildrenForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_children_for_node<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.util.prefs.MacOSXPreferencesFile.getChildrenForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;".to_string()).into())
}

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.getKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_key_from_node<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.util.prefs.MacOSXPreferencesFile.getKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Ljava/lang/String;".to_string()).into())
}

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.getKeysForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_keys_for_node<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.util.prefs.MacOSXPreferencesFile.getKeysForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;".to_string()).into())
}

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.removeChildFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
    Any
)]
#[async_method]
pub async fn remove_child_from_node<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.util.prefs.MacOSXPreferencesFile.removeChildFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V".to_string()).into())
}

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.removeKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
    Any
)]
#[async_method]
pub async fn remove_key_from_node<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.util.prefs.MacOSXPreferencesFile.removeKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V".to_string()).into())
}

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.removeNode(Ljava/lang/String;Ljava/lang/String;JJ)V",
    Any
)]
#[async_method]
pub async fn remove_node<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.MacOSXPreferencesFile.removeNode(Ljava/lang/String;Ljava/lang/String;JJ)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.synchronize(Ljava/lang/String;JJ)Z",
    Any
)]
#[async_method]
pub async fn synchronize<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.util.prefs.MacOSXPreferencesFile.synchronize(Ljava/lang/String;JJ)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_child_to_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_child_to_node(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_key_to_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_key_to_node(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_node(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_any_host() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = any_host(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_any_user() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = any_user(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_current_host() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = current_host(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_current_user() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = current_user(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_children_for_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_children_for_node(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_key_from_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_key_from_node(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_keys_for_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_keys_for_node(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_child_from_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_child_from_node(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_key_from_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_key_from_node(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_node(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_synchronize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = synchronize(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
