use crate::Result;
use ahash::AHashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockMappedWriteGuard, RwLockReadGuard, RwLockWriteGuard};

/// Handles for operating system resources, such as files or sockets.
#[derive(Debug, Default)]
pub struct HandleManager<K, V>
where
    K: Eq + Hash,
    V: Send + Sync,
{
    handles: Arc<RwLock<AHashMap<K, V>>>,
}

impl<K, V> HandleManager<K, V>
where
    K: Eq + Hash,
    V: Send + Sync,
{
    /// Creates a new `Handles` instance.
    #[must_use]
    pub fn new() -> Self {
        HandleManager {
            handles: Arc::new(RwLock::new(AHashMap::default())),
        }
    }

    /// Inserts a handle into the collection.
    ///
    /// # Errors
    /// Returns an error if the handle cannot be inserted.
    pub async fn insert(&self, key: K, handle: V) -> Result<()> {
        let mut handles = self.handles.write().await;
        handles.insert(key, handle);
        Ok(())
    }

    /// Retrieves a handle by its key.
    pub async fn get<Q>(&self, key: &Q) -> Option<RwLockReadGuard<'_, V>>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        let guard = self.handles.read().await;
        if guard.contains_key(key) {
            Some(RwLockReadGuard::map(guard, |handles| &handles[key]))
        } else {
            None
        }
    }

    /// Retrieves a mutable handle by its key.
    ///
    /// # Panics
    /// Panics if the key is not found after a successful containment check
    /// (should not happen in practice).
    pub async fn get_mut<Q>(&self, key: &Q) -> Option<RwLockMappedWriteGuard<'_, V>>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        let guard = self.handles.write().await;
        if guard.contains_key(key) {
            Some(RwLockWriteGuard::map(guard, |handles| {
                handles.get_mut(key).expect("Handle not found")
            }))
        } else {
            None
        }
    }

    /// Removes a handle by its key.
    pub async fn remove<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        let mut handles = self.handles.write().await;
        handles.remove(key)
    }

    /// Returns a read lock on the handles.
    pub async fn read(&self) -> RwLockReadGuard<'_, AHashMap<K, V>> {
        self.handles.read().await
    }

    /// Returns a write lock on the handles.
    pub async fn write(&self) -> RwLockWriteGuard<'_, AHashMap<K, V>> {
        self.handles.write().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[tokio::test]
    async fn test_new() {
        let manager: HandleManager<u64, String> = HandleManager::new();
        let guard = manager.read().await;
        assert!(guard.is_empty());
    }

    #[tokio::test]
    async fn test_default() {
        let manager: HandleManager<u64, String> = HandleManager::default();
        let guard = manager.read().await;
        assert!(guard.is_empty());
    }

    #[tokio::test]
    async fn test_insert_and_get() -> Result<()> {
        let manager = HandleManager::new();
        manager.insert(1u64, "hello".to_string()).await?;
        let value = manager.get(&1u64).await;
        assert!(value.is_some());
        assert_eq!(&*value.unwrap(), "hello");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_missing_key() {
        let manager: HandleManager<u64, String> = HandleManager::new();
        assert!(manager.get(&42u64).await.is_none());
    }

    #[tokio::test]
    async fn test_insert_and_get_mut() -> Result<()> {
        let manager = HandleManager::new();
        manager.insert(1u64, "hello".to_string()).await?;
        let mut value = manager.get_mut(&1u64).await.unwrap();
        *value = "world".to_string();
        drop(value);
        let value = manager.get(&1u64).await.unwrap();
        assert_eq!(&*value, "world");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_mut_missing_key() {
        let manager: HandleManager<u64, String> = HandleManager::new();
        assert!(manager.get_mut(&42u64).await.is_none());
    }

    #[tokio::test]
    async fn test_remove() -> Result<()> {
        let manager = HandleManager::new();
        manager.insert(1u64, "hello".to_string()).await?;
        let removed = manager.remove(&1u64).await;
        assert_eq!(removed, Some("hello".to_string()));
        assert!(manager.get(&1u64).await.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_missing_key() {
        let manager: HandleManager<u64, String> = HandleManager::new();
        assert!(manager.remove(&42u64).await.is_none());
    }

    #[tokio::test]
    async fn test_read() -> Result<()> {
        let manager = HandleManager::new();
        manager.insert(1u64, "hello".to_string()).await?;
        let guard = manager.read().await;
        assert!(guard.contains_key(&1u64));
        assert_eq!(guard.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_write() -> Result<()> {
        let manager: HandleManager<u64, String> = HandleManager::new();
        let mut guard = manager.write().await;
        guard.insert(1u64, "hello".to_string());
        assert!(guard.contains_key(&1u64));
        guard.remove(&1u64);
        assert!(!guard.contains_key(&1u64));
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_inserts() -> Result<()> {
        let manager = HandleManager::new();
        manager.insert("a".to_string(), 1u64).await?;
        manager.insert("b".to_string(), 2u64).await?;
        manager.insert("c".to_string(), 3u64).await?;
        assert_eq!(&*manager.get("a").await.unwrap(), &1u64);
        assert_eq!(&*manager.get("b").await.unwrap(), &2u64);
        assert_eq!(&*manager.get("c").await.unwrap(), &3u64);
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_overwrite() -> Result<()> {
        let manager = HandleManager::new();
        manager.insert(1u64, "first".to_string()).await?;
        manager.insert(1u64, "second".to_string()).await?;
        let value = manager.get(&1u64).await.unwrap();
        assert_eq!(&*value, "second");
        Ok(())
    }

    #[tokio::test]
    async fn test_debug() {
        let manager: HandleManager<u64, String> = HandleManager::new();
        let debug = format!("{manager:?}");
        assert!(debug.contains("HandleManager"));
    }
}
