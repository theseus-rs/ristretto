use crate::Result;
use crate::handle::Handle;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockMappedWriteGuard, RwLockReadGuard, RwLockWriteGuard};

/// Handles for operating system resources, such as files or sockets.
#[derive(Debug, Default)]
pub(crate) struct Handles {
    handles: Arc<RwLock<HashMap<String, Handle>>>,
}

impl Handles {
    /// Creates a new `Handles` instance.
    pub fn new() -> Self {
        Handles {
            handles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Inserts a handle into the collection.
    pub async fn insert(&self, key: String, handle: Handle) -> Result<()> {
        let mut handles = self.handles.write().await;
        handles.insert(key, handle);
        Ok(())
    }

    /// Retrieves a handle by its key.
    pub async fn get(&self, key: &str) -> Option<RwLockReadGuard<'_, Handle>> {
        let guard = self.handles.read().await;
        if guard.contains_key(key) {
            Some(RwLockReadGuard::map(guard, |handles| &handles[key]))
        } else {
            None
        }
    }

    /// Retrieves a mutable handle by its key.
    pub async fn get_mut(&self, key: &str) -> Option<RwLockMappedWriteGuard<'_, Handle>> {
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
    pub async fn remove(&self, key: &str) -> Option<Handle> {
        let mut handles = self.handles.write().await;
        handles.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handle::Handle;
    use tokio::fs::{File, remove_file};

    #[tokio::test]
    async fn test_handles_insert_and_get() -> Result<()> {
        let handles = Handles::new();
        let file_name = "test_handles_insert_and_get.txt";
        let file = File::create(file_name).await?;
        let handle: Handle = (file, false).into();

        handles.insert("test_handle".to_string(), handle).await?;
        assert!(handles.get("test_handle").await.is_some());
        remove_file(file_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_handles_insert_and_get_mut() -> Result<()> {
        let handles = Handles::new();
        let file_name = "test_handles_insert_and_get_mut.txt";
        let file = File::create(file_name).await?;
        let handle: Handle = (file, false).into();

        handles.insert("test_handle".to_string(), handle).await?;
        assert!(handles.get_mut("test_handle").await.is_some());
        remove_file(file_name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_handles_remove() -> Result<()> {
        let handles = Handles::new();
        let file_name = "test_handles_remove.txt";
        let file = File::create(file_name).await?;
        let handle: Handle = (file, false).into();

        handles.insert("test_handle".to_string(), handle).await?;
        assert!(handles.remove("test_handle").await.is_some());
        assert!(handles.get("test_handle").await.is_none());
        remove_file(file_name).await?;
        Ok(())
    }
}
