use crate::thread::Thread;
use crate::{JavaObject, Result};
use ahash::AHashMap;
use ristretto_classfile::{JavaStr, JavaString};
use ristretto_classloader::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A thread-safe string pool that allows for efficient storage and retrieval of string values
#[derive(Debug)]
pub struct StringPool {
    strings: Arc<RwLock<AHashMap<JavaString, Value>>>,
}

impl StringPool {
    /// Creates a new empty string pool
    pub fn new() -> Self {
        StringPool {
            strings: Arc::new(RwLock::new(AHashMap::default())),
        }
    }

    /// Interns a Java string from MUTF-8 bytes, preserving lone surrogates losslessly.
    ///
    /// This is the preferred method for interning constant pool string values, as it
    /// converts MUTF-8 directly to Java's internal UTF-16 representation without going
    /// through standard UTF-8 (which would replace lone surrogates with U+FFFD).
    pub async fn intern_java_str(&self, thread: &Thread, string: &JavaStr) -> Result<Value> {
        {
            let strings = self.strings.read().await;
            if let Some(value) = strings.get(string) {
                return Ok(value.clone());
            }
        }
        let value = string.to_object(thread).await?;
        let mut strings = self.strings.write().await;
        let key = string.to_java_string();
        // Double-check in case another thread inserted it
        let entry = strings.entry(key).or_insert_with(|| value);
        Ok(entry.clone())
    }

    /// Interns a Rust `&str` into the pool.
    ///
    /// This converts the `&str` to a `JavaString` key. Since `&str` is valid UTF-8,
    /// no lone surrogates can be present and the conversion is lossless.
    pub async fn intern(&self, thread: &Thread, string: &str) -> Result<Value> {
        let java_string = JavaString::from(string);
        let java_str: &JavaStr = &java_string;
        {
            let strings = self.strings.read().await;
            if let Some(value) = strings.get(java_str) {
                return Ok(value.clone());
            }
        }
        let value = string.to_object(thread).await?;
        let mut strings = self.strings.write().await;
        // Double-check in case another thread inserted it
        let entry = strings.entry(java_string).or_insert_with(|| value);
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

    #[tokio::test]
    async fn test_string_pool_java_str() -> Result<()> {
        let (_givm, thread) = thread().await?;
        let string_pool = StringPool::new();
        let js = JavaString::from("Hello");
        let value1 = string_pool.intern_java_str(&thread, &js).await?;
        let value2 = string_pool.intern_java_str(&thread, &js).await?;
        assert_eq!(value1, value2);
        Ok(())
    }
}
