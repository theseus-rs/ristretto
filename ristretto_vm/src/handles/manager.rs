use crate::Result;
use ahash::AHashMap;
use std::borrow::Borrow;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockMappedWriteGuard, RwLockReadGuard, RwLockWriteGuard};

/// Handles for operating system resources, such as files or sockets.
#[derive(Debug, Default)]
pub(crate) struct HandleManager<K, V>
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
    pub fn new() -> Self {
        HandleManager {
            handles: Arc::new(RwLock::new(AHashMap::default())),
        }
    }

    /// Inserts a handle into the collection.
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
    use crate::handles::ThreadHandle;

    #[tokio::test]
    async fn test_insert_and_get() -> Result<()> {
        let handle_manager = HandleManager::new();
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let id = thread.id();
        let thread_handle: ThreadHandle = thread.clone().into();
        handle_manager.insert(id, thread_handle).await?;
        assert!(handle_manager.get(&id).await.is_some());
        assert!(handle_manager.get(&u64::MAX).await.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_and_get_mut() -> Result<()> {
        let handle_manager = HandleManager::new();
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let id = thread.id();
        let thread_handle: ThreadHandle = thread.clone().into();

        handle_manager.insert(id, thread_handle).await?;
        assert!(handle_manager.get_mut(&id).await.is_some());
        assert!(handle_manager.get_mut(&u64::MAX).await.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_remove() -> Result<()> {
        let handle_manager = HandleManager::new();
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let id = thread.id();
        let thread_handle: ThreadHandle = thread.clone().into();

        handle_manager.insert(id, thread_handle).await?;
        assert!(handle_manager.remove(&id).await.is_some());
        assert!(handle_manager.get(&id).await.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_handles_read() -> Result<()> {
        let handle_manager = HandleManager::new();
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let id = thread.id();
        let thread_handle: ThreadHandle = thread.clone().into();

        handle_manager.insert(id, thread_handle).await?;
        let read_guard = handle_manager.read().await;
        assert!(read_guard.contains_key(&id));
        Ok(())
    }

    #[tokio::test]
    async fn test_handles_write() -> Result<()> {
        let handle_manager = HandleManager::new();
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let id = thread.id();
        let thread_handle: ThreadHandle = thread.clone().into();

        let mut write_guard = handle_manager.write().await;
        write_guard.insert(id, thread_handle);
        assert!(write_guard.contains_key(&id));
        write_guard.remove(&id);
        assert!(!write_guard.contains_key(&id));
        Ok(())
    }
}
