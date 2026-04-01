use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::{JavaObject, Parameters, Result, Thread, VM};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};

/// Sentinel values representing macOS preferences user/host domains.
/// These correspond to `CFStringRef` pointers cast to `jlong` in the JDK; we use sentinel
/// integers because the actual preferences data is stored in an in-memory map.
const CURRENT_USER: i64 = 1;
const ANY_USER: i64 = 2;
const CURRENT_HOST: i64 = 3;
const ANY_HOST: i64 = 4;

/// In-memory preferences store that mimics macOS `CFPreferences` behaviour.
///
/// Keyed by `(user_domain, host_domain, app_name)`.  Within each domain the
/// flat key-space follows the macOS convention:
///   - node markers:      `path` (e.g. `"/a/b/"`) -> empty string
///   - preference values:  `path + key` (e.g. `"/a/b/myKey"`) -> value string
///   - child markers:      `path + child + "/"` -> empty string
type DomainKey = (i64, i64, String);

static PREFS: LazyLock<Mutex<HashMap<DomainKey, HashMap<String, String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// `addChildToNode(path, child, name, user, host) -> boolean`
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.addChildToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Z",
    Any
)]
#[async_method]
pub async fn add_child_to_node<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host = parameters.pop_long()?;
    let user = parameters.pop_long()?;
    let name = parameters.pop()?.as_string()?;
    let child = parameters.pop()?.as_string()?;
    let path = parameters.pop()?.as_string()?;

    let child_path = format!("{path}{child}/");
    let mut store = PREFS.lock().map_err(|e| InternalError(e.to_string()))?;
    let domain = store.entry((user, host, name)).or_default();
    let is_new = !domain.contains_key(&child_path);
    domain.entry(child_path).or_default();
    Ok(Some(Value::Int(i32::from(is_new))))
}

/// `addKeyToNode(path, key, value, name, user, host) -> void`
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.addKeyToNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
    Any
)]
#[async_method]
pub async fn add_key_to_node<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host = parameters.pop_long()?;
    let user = parameters.pop_long()?;
    let name = parameters.pop()?.as_string()?;
    let value = parameters.pop()?.as_string()?;
    let key = parameters.pop()?.as_string()?;
    let path = parameters.pop()?.as_string()?;

    let full_key = format!("{path}{key}");
    let mut store = PREFS.lock().map_err(|e| InternalError(e.to_string()))?;
    let domain = store.entry((user, host, name)).or_default();
    domain.insert(full_key, value);
    Ok(None)
}

/// `addNode(path, name, user, host) -> boolean`
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.addNode(Ljava/lang/String;Ljava/lang/String;JJ)Z",
    Any
)]
#[async_method]
pub async fn add_node<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host = parameters.pop_long()?;
    let user = parameters.pop_long()?;
    let name = parameters.pop()?.as_string()?;
    let path = parameters.pop()?.as_string()?;

    let mut store = PREFS.lock().map_err(|e| InternalError(e.to_string()))?;
    let domain = store.entry((user, host, name)).or_default();
    let is_new = !domain.contains_key(&path);
    domain.entry(path).or_default();
    Ok(Some(Value::Int(i32::from(is_new))))
}

/// `anyHost() -> long`
#[intrinsic_method("java/util/prefs/MacOSXPreferencesFile.anyHost()J", Any)]
#[async_method]
pub async fn any_host<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(ANY_HOST)))
}

/// `anyUser() -> long`
#[intrinsic_method("java/util/prefs/MacOSXPreferencesFile.anyUser()J", Any)]
#[async_method]
pub async fn any_user<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(ANY_USER)))
}

/// `currentHost() -> long`
#[intrinsic_method("java/util/prefs/MacOSXPreferencesFile.currentHost()J", Any)]
#[async_method]
pub async fn current_host<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(CURRENT_HOST)))
}

/// `currentUser() -> long`
#[intrinsic_method("java/util/prefs/MacOSXPreferencesFile.currentUser()J", Any)]
#[async_method]
pub async fn current_user<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(CURRENT_USER)))
}

/// `getChildrenForNode(path, name, user, host) -> String[]`
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.getChildrenForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_children_for_node<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host = parameters.pop_long()?;
    let user = parameters.pop_long()?;
    let name = parameters.pop()?.as_string()?;
    let path = parameters.pop()?.as_string()?;

    let children: Vec<String> = {
        let store = PREFS.lock().map_err(|e| InternalError(e.to_string()))?;
        if let Some(domain) = store.get(&(user, host, name)) {
            let mut child_set = HashSet::new();
            for key in domain.keys() {
                if let Some(suffix) = key.strip_prefix(path.as_str())
                    && let Some(slash_pos) = suffix.find('/')
                {
                    let child_name = &suffix[..slash_pos];
                    if !child_name.is_empty() {
                        child_set.insert(child_name.to_string());
                    }
                }
            }
            child_set.into_iter().collect()
        } else {
            Vec::new()
        }
    };

    let string_class = thread.class("java/lang/String").await?;
    let mut values: Vec<Value> = Vec::new();
    for child in &children {
        values.push(child.to_object(&thread).await?);
    }
    let reference = Reference::try_from((string_class, values))?;
    let result = Value::new_object(thread.vm()?.garbage_collector(), reference);
    Ok(Some(result))
}

/// `getKeyFromNode(path, key, name, user, host) -> String`
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.getKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_key_from_node<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host = parameters.pop_long()?;
    let user = parameters.pop_long()?;
    let name = parameters.pop()?.as_string()?;
    let key = parameters.pop()?.as_string()?;
    let path = parameters.pop()?.as_string()?;

    let full_key = format!("{path}{key}");
    let value = {
        let store = PREFS.lock().map_err(|e| InternalError(e.to_string()))?;
        store
            .get(&(user, host, name))
            .and_then(|domain| domain.get(&full_key).cloned())
    };

    match value {
        Some(v) => Ok(Some(v.to_object(&thread).await?)),
        None => Ok(Some(Value::Object(None))),
    }
}

/// `getKeysForNode(path, name, user, host) -> String[]`
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.getKeysForNode(Ljava/lang/String;Ljava/lang/String;JJ)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_keys_for_node<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host = parameters.pop_long()?;
    let user = parameters.pop_long()?;
    let name = parameters.pop()?.as_string()?;
    let path = parameters.pop()?.as_string()?;

    let keys: Vec<String> = {
        let store = PREFS.lock().map_err(|e| InternalError(e.to_string()))?;
        if let Some(domain) = store.get(&(user, host, name)) {
            domain
                .keys()
                .filter_map(|k| {
                    k.strip_prefix(path.as_str()).and_then(|suffix| {
                        if !suffix.is_empty() && !suffix.contains('/') {
                            Some(suffix.to_string())
                        } else {
                            None
                        }
                    })
                })
                .collect()
        } else {
            Vec::new()
        }
    };

    let string_class = thread.class("java/lang/String").await?;
    let mut values: Vec<Value> = Vec::new();
    for key in &keys {
        values.push(key.to_object(&thread).await?);
    }
    let reference = Reference::try_from((string_class, values))?;
    let result = Value::new_object(thread.vm()?.garbage_collector(), reference);
    Ok(Some(result))
}

/// `removeChildFromNode(path, child, name, user, host) -> void`
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.removeChildFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
    Any
)]
#[async_method]
pub async fn remove_child_from_node<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host = parameters.pop_long()?;
    let user = parameters.pop_long()?;
    let name = parameters.pop()?.as_string()?;
    let child = parameters.pop()?.as_string()?;
    let path = parameters.pop()?.as_string()?;

    let child_path = format!("{path}{child}/");
    let mut store = PREFS.lock().map_err(|e| InternalError(e.to_string()))?;
    if let Some(domain) = store.get_mut(&(user, host, name)) {
        domain.remove(&child_path);
    }
    Ok(None)
}

/// `removeKeyFromNode(path, key, name, user, host) -> void`
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.removeKeyFromNode(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JJ)V",
    Any
)]
#[async_method]
pub async fn remove_key_from_node<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host = parameters.pop_long()?;
    let user = parameters.pop_long()?;
    let name = parameters.pop()?.as_string()?;
    let key = parameters.pop()?.as_string()?;
    let path = parameters.pop()?.as_string()?;

    let full_key = format!("{path}{key}");
    let mut store = PREFS.lock().map_err(|e| InternalError(e.to_string()))?;
    if let Some(domain) = store.get_mut(&(user, host, name)) {
        domain.remove(&full_key);
    }
    Ok(None)
}

/// `removeNode(path, name, user, host) -> void`
///
/// Removes all keys that belong to the given node (keys starting with `path`).
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.removeNode(Ljava/lang/String;Ljava/lang/String;JJ)V",
    Any
)]
#[async_method]
pub async fn remove_node<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let host = parameters.pop_long()?;
    let user = parameters.pop_long()?;
    let name = parameters.pop()?.as_string()?;
    let path = parameters.pop()?.as_string()?;

    let mut store = PREFS.lock().map_err(|e| InternalError(e.to_string()))?;
    if let Some(domain) = store.get_mut(&(user, host, name)) {
        domain.retain(|k, _| !k.starts_with(&path));
    }
    Ok(None)
}

/// `synchronize(name, user, host) -> boolean`
///
/// In-memory store is always in sync; returns `true`.
#[intrinsic_method(
    "java/util/prefs/MacOSXPreferencesFile.synchronize(Ljava/lang/String;JJ)Z",
    Any
)]
#[async_method]
pub async fn synchronize<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _host = parameters.pop_long()?;
    let _user = parameters.pop_long()?;
    let _name = parameters.pop()?.as_string()?;
    Ok(Some(Value::Int(1))) // true
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;

    #[tokio::test]
    async fn test_current_user() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = current_user(thread, Parameters::default())
            .await
            .expect("result");
        assert_eq!(Some(Value::Long(CURRENT_USER)), result);
    }

    #[tokio::test]
    async fn test_any_user() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = any_user(thread, Parameters::default())
            .await
            .expect("result");
        assert_eq!(Some(Value::Long(ANY_USER)), result);
    }

    #[tokio::test]
    async fn test_current_host() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = current_host(thread, Parameters::default())
            .await
            .expect("result");
        assert_eq!(Some(Value::Long(CURRENT_HOST)), result);
    }

    #[tokio::test]
    async fn test_any_host() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = any_host(thread, Parameters::default())
            .await
            .expect("result");
        assert_eq!(Some(Value::Long(ANY_HOST)), result);
    }

    #[tokio::test]
    async fn test_add_and_get_key() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let path = "/test/add_and_get_key/";
        let app = "com.test.prefs";

        // add key: static native order is (path, key, value, appName, user, host)
        let mut params = Parameters::default();
        params.push(path.to_object(&thread).await.expect("value"));
        params.push("myKey".to_object(&thread).await.expect("value"));
        params.push("myValue".to_object(&thread).await.expect("value"));
        params.push(app.to_object(&thread).await.expect("value"));
        params.push_long(CURRENT_USER);
        params.push_long(ANY_HOST);
        let result = add_key_to_node(thread.clone(), params).await.expect("ok");
        assert_eq!(None, result); // void

        // get key: static native order is (path, key, appName, user, host)
        let mut params = Parameters::default();
        params.push(path.to_object(&thread).await.expect("value"));
        params.push("myKey".to_object(&thread).await.expect("value"));
        params.push(app.to_object(&thread).await.expect("value"));
        params.push_long(CURRENT_USER);
        params.push_long(ANY_HOST);
        let result = get_key_from_node(thread.clone(), params)
            .await
            .expect("ok")
            .expect("value");
        assert_eq!("myValue", result.as_string().expect("string"));

        // clean up
        let mut params = Parameters::default();
        params.push(path.to_object(&thread).await.expect("value"));
        params.push(app.to_object(&thread).await.expect("value"));
        params.push_long(CURRENT_USER);
        params.push_long(ANY_HOST);
        let _ = remove_node(thread, params).await;
    }

    #[tokio::test]
    async fn test_get_missing_key_returns_null() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let path = "/test/missing_key/";
        let app = "com.test.prefs.missing";

        // static native order is (path, key, appName, user, host)
        let mut params = Parameters::default();
        params.push(path.to_object(&thread).await.expect("value"));
        params.push("noSuchKey".to_object(&thread).await.expect("value"));
        params.push(app.to_object(&thread).await.expect("value"));
        params.push_long(CURRENT_USER);
        params.push_long(ANY_HOST);
        let result = get_key_from_node(thread, params)
            .await
            .expect("ok")
            .expect("value");
        assert_eq!(Value::Object(None), result);
    }

    #[tokio::test]
    async fn test_add_node() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let path = "/test/add_node/";
        let app = "com.test.prefs.addnode";

        let mut params = Parameters::default();
        params.push(path.to_object(&thread).await.expect("value"));
        params.push(app.to_object(&thread).await.expect("value"));
        params.push_long(CURRENT_USER);
        params.push_long(ANY_HOST);
        let result = add_node(thread.clone(), params)
            .await
            .expect("ok")
            .expect("value");
        assert_eq!(Value::Int(1), result); // new node

        // adding again should return false
        let mut params = Parameters::default();
        params.push(path.to_object(&thread).await.expect("value"));
        params.push(app.to_object(&thread).await.expect("value"));
        params.push_long(CURRENT_USER);
        params.push_long(ANY_HOST);
        let result = add_node(thread.clone(), params)
            .await
            .expect("ok")
            .expect("value");
        assert_eq!(Value::Int(0), result); // already exists

        // clean up
        let mut params = Parameters::default();
        params.push(path.to_object(&thread).await.expect("value"));
        params.push(app.to_object(&thread).await.expect("value"));
        params.push_long(CURRENT_USER);
        params.push_long(ANY_HOST);
        let _ = remove_node(thread, params).await;
    }

    #[tokio::test]
    async fn test_synchronize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");

        let mut params = Parameters::default();
        params.push("com.test.sync".to_object(&thread).await.expect("value"));
        params.push_long(CURRENT_USER);
        params.push_long(ANY_HOST);
        let result = synchronize(thread, params)
            .await
            .expect("ok")
            .expect("value");
        assert_eq!(Value::Int(1), result); // true
    }
}
