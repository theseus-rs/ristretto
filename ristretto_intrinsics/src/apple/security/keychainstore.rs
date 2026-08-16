use portable_atomic::{AtomicI64, Ordering};
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result, Thread, VM as _};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
struct KeychainItem {
    alias: String,
    certificate: bool,
    data: Vec<i8>,
    password: Vec<u16>,
}

#[derive(Debug)]
struct KeychainState {
    items: Mutex<HashMap<i64, KeychainItem>>,
    released: Mutex<HashSet<i64>>,
    next_handle: AtomicI64,
}

impl Default for KeychainState {
    fn default() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
            released: Mutex::new(HashSet::new()),
            next_handle: AtomicI64::new(1),
        }
    }
}

fn state<T: Thread + 'static>(thread: &T) -> Result<Arc<KeychainState>> {
    thread
        .vm()?
        .resource_manager()
        .get_or_init(KeychainState::default)
}

#[intrinsic_method(
    "apple/security/KeychainStore._addItemToKeychain(Ljava/lang/String;Z[B[C)J",
    Any
)]
#[async_method]
pub async fn add_item_to_keychain<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let password = Value::Object(parameters.pop_reference()?);
    let data = Value::Object(parameters.pop_reference()?);
    let certificate = parameters.pop_bool()?;
    let alias = Value::Object(parameters.pop_reference()?);
    let alias = if alias.is_null() {
        String::new()
    } else {
        alias.as_string()?
    };
    let data = if data.is_null() {
        Vec::new()
    } else {
        data.as_byte_vec_ref()?.to_vec()
    };
    let password = if password.is_null() {
        Vec::new()
    } else {
        password.as_char_vec_ref()?.to_vec()
    };
    let keychain = state(thread.as_ref())?;
    let handle = keychain.next_handle.fetch_add(1, Ordering::Relaxed);
    keychain
        .items
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .insert(
            handle,
            KeychainItem {
                alias,
                certificate,
                data,
                password,
            },
        );
    Ok(Some(Value::Long(handle)))
}

#[intrinsic_method("apple/security/KeychainStore._getEncodedKeyData(J[C)[B", Any)]
#[async_method]
pub async fn get_encoded_key_data<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _export_password = parameters.pop_reference()?;
    let handle = parameters.pop_long()?;
    let keychain = state(thread.as_ref())?;
    if keychain
        .released
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .contains(&handle)
    {
        return Ok(Some(Value::Object(None)));
    }
    let item = keychain
        .items
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&handle)
        .cloned();
    let Some(item) = item.filter(|item| !item.certificate) else {
        return Ok(Some(Value::Object(None)));
    };
    let vm = thread.vm()?;
    Ok(Some(Value::new_object(
        vm.garbage_collector(),
        Reference::from(item.data),
    )))
}

#[intrinsic_method("apple/security/KeychainStore._releaseKeychainItemRef(J)V", Any)]
#[async_method]
pub async fn release_keychain_item_ref<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    if state(thread.as_ref())?
        .items
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .contains_key(&handle)
    {
        state(thread.as_ref())?
            .released
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(handle);
    }
    Ok(None)
}

#[intrinsic_method("apple/security/KeychainStore._removeItemFromKeychain(J)I", Any)]
#[async_method]
pub async fn remove_item_from_keychain<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let handle = parameters.pop_long()?;
    let keychain = state(thread.as_ref())?;
    let removed = keychain
        .items
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .remove(&handle)
        .map(|mut item| {
            item.data.fill(0);
            item.password.fill(0);
        })
        .is_some();
    keychain
        .released
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .remove(&handle);
    // errSecItemNotFound is -25300. Security.framework uses zero for success.
    Ok(Some(Value::Int(if removed { 0 } else { -25_300 })))
}

#[intrinsic_method(
    "apple/security/KeychainStore._scanKeychain()V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn scan_keychain_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _receiver = parameters.pop().unwrap_or(Value::Object(None));
    let _keychain = state(thread.as_ref())?;
    Ok(None)
}

#[intrinsic_method(
    "apple/security/KeychainStore._scanKeychain(Ljava/lang/String;)V",
    GreaterThan(JAVA_21)
)]
#[async_method]
pub async fn scan_keychain_1<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _store_name = parameters.pop_reference()?;
    let _receiver = parameters.pop().unwrap_or(Value::Object(None));
    let _keychain = state(thread.as_ref())?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::JavaObject;

    #[tokio::test]
    async fn key_data_lifecycle() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let alias = "test-key".to_object(&thread).await?;
        let data = Value::new_object(vm.garbage_collector(), Reference::from(vec![1_i8, 2, 3]));
        let password = Value::new_object(
            vm.garbage_collector(),
            Reference::CharArray(vec![112_u16, 119].into_boxed_slice()),
        );
        let handle = add_item_to_keychain(
            thread.clone(),
            Parameters::new(vec![alias, Value::from(false), data, password]),
        )
        .await?
        .expect("handle")
        .as_i64()?;
        assert!(handle > 0);

        let encoded = get_encoded_key_data(
            thread.clone(),
            Parameters::new(vec![Value::Long(handle), Value::Object(None)]),
        )
        .await?
        .expect("encoded key");
        assert_eq!(&[1_i8, 2, 3], &*encoded.as_byte_vec_ref()?);

        release_keychain_item_ref(thread.clone(), Parameters::new(vec![Value::Long(handle)]))
            .await?;
        assert!(
            get_encoded_key_data(
                thread.clone(),
                Parameters::new(vec![Value::Long(handle), Value::Object(None)]),
            )
            .await?
            .expect("released key")
            .is_null()
        );
        assert_eq!(
            Some(Value::Int(0)),
            remove_item_from_keychain(thread, Parameters::new(vec![Value::Long(handle)])).await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn consecutive_items_have_distinct_handles() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let password = || {
            Value::new_object(
                vm.garbage_collector(),
                Reference::CharArray(vec![112_u16, 119].into_boxed_slice()),
            )
        };
        let add = |alias: Value, data: Vec<i8>, password: Value| {
            add_item_to_keychain(
                thread.clone(),
                Parameters::new(vec![
                    alias,
                    Value::from(false),
                    Value::new_object(vm.garbage_collector(), Reference::from(data)),
                    password,
                ]),
            )
        };
        let first = add("first".to_object(&thread).await?, vec![1], password())
            .await?
            .expect("first handle")
            .as_i64()?;
        let second = add("second".to_object(&thread).await?, vec![2], password())
            .await?
            .expect("second handle")
            .as_i64()?;
        assert_ne!(first, second);
        let encoded = get_encoded_key_data(
            thread,
            Parameters::new(vec![Value::Long(first), Value::Object(None)]),
        )
        .await?
        .expect("first item");
        assert_eq!(&[1], &*encoded.as_byte_vec_ref()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_scan_keychain_0() {
        let (vm, thread) = crate::test::java21_thread().await.expect("thread");
        let alias = "passwordless-key".to_object(&thread).await.expect("alias");
        let data = Value::new_object(vm.garbage_collector(), Reference::from(vec![1_i8]));
        add_item_to_keychain(
            thread.clone(),
            Parameters::new(vec![alias, Value::from(false), data, Value::Object(None)]),
        )
        .await
        .expect("add passwordless item");
        let result = scan_keychain_0(thread, Parameters::default()).await;
        assert_eq!(None, result.expect("result"));
    }

    #[tokio::test]
    async fn test_scan_keychain_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = scan_keychain_1(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(None, result.expect("result"));
    }
}
