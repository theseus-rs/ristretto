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
        let key = string.to_java_string();
        let mut strings = self.strings.write().await;
        // Double-check in case another thread inserted it
        let entry = strings.entry(key).or_insert_with(|| value);
        Ok(entry.clone())
    }

    /// Interns a Rust `&str` into the pool.
    ///
    /// Borrows the lookup key when UTF-8 and MUTF-8 are identical, allocating only
    /// for a new entry or when null/supplementary characters require conversion.
    pub async fn intern(&self, thread: &Thread, string: &str) -> Result<Value> {
        let java_string = JavaStr::cow_from_str(string);
        {
            let strings = self.strings.read().await;
            if let Some(value) = strings.get(java_string.as_ref()) {
                return Ok(value.clone());
            }
        }
        let value = string.to_object(thread).await?;
        let key = java_string.into_owned();
        let mut strings = self.strings.write().await;
        // Double-check in case another thread inserted it
        let entry = strings.entry(key).or_insert_with(|| value);
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
        for input in ["", "Hello", "caf\u{e9}", "\u{100}", "a\0b", "\u{1f600}"] {
            let value1 = string_pool.intern(&thread, input).await?;
            let value2 = string_pool.intern(&thread, input).await?;
            let java_string = JavaString::from(input);
            let value3 = string_pool.intern_java_str(&thread, &java_string).await?;
            assert_same_string(&value1, &value2);
            assert_same_string(&value1, &value3);
            assert_eq!(input, value1.as_string()?);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_string_pool_java_str() -> Result<()> {
        let (_givm, thread) = thread().await?;
        let string_pool = StringPool::new();
        for input in ["", "Hello", "caf\u{e9}", "\u{100}", "a\0b", "\u{1f600}"] {
            let js = JavaString::from(input);
            let value1 = string_pool.intern_java_str(&thread, &js).await?;
            let value2 = string_pool.intern_java_str(&thread, &js).await?;
            let value3 = string_pool.intern(&thread, input).await?;
            assert_same_string(&value1, &value2);
            assert_same_string(&value1, &value3);
            assert_eq!(input, value1.as_string()?);
        }
        Ok(())
    }

    fn assert_same_string(left: &Value, right: &Value) {
        assert!(matches!(
            (left, right),
            (Value::Object(Some(left)), Value::Object(Some(right))) if left.ptr_eq(right)
        ));
    }
}
