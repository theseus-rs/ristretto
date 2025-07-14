use crate::thread::Thread;
use crate::{JavaObject, Result};
use ristretto_classloader::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A thread-safe string pool that allows for efficient storage and retrieval of string values
#[derive(Debug)]
pub struct StringPool {
    strings: Arc<RwLock<HashMap<String, Value>>>,
}

impl StringPool {
    /// Creates a new empty string pool
    pub fn new() -> Self {
        StringPool {
            strings: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Inserts a string into the pool and returns its associated value
    pub async fn intern(&self, thread: &Thread, string: &str) -> Result<Value> {
        {
            let strings = self.strings.read().await;
            if let Some(value) = strings.get(string) {
                return Ok(value.clone());
            }
        }
        let value = string.to_object(thread).await?;
        let mut strings = self.strings.write().await;
        let string = string.to_string();
        // Double-check in case another thread inserted it
        let entry = strings.entry(string).or_insert_with(|| value);
        Ok(entry.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::thread;

    #[tokio::test]
    async fn test_string_pool() -> Result<()> {
        let (_givm, thread) = thread().await?;
        let string_pool = StringPool::new();
        let value1 = string_pool.intern(&thread, "Hello").await?;
        let value2 = string_pool.intern(&thread, "Hello").await?;
        assert_eq!(value1, value2);
        Ok(())
    }
}
