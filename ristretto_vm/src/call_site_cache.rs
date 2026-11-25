//! Per-instruction linkage results for invokedynamic (JVMS §5.4.3.6).
use crate::{Error, JavaError, Result, Thread};
use dashmap::DashMap;
use ristretto_classfile::{CONSTRUCTOR_NAME, JavaString};
use ristretto_classloader::{Class, Method, Object, Value};
use ristretto_types::Assignable;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, OnceLock};

/// Unique identifier for an invokedynamic call site
#[derive(Clone)]
pub struct CallSiteKey {
    /// Retain the defining class so identities cannot be reused while a site is cached.
    pub class: Arc<Class>,
    method: Arc<Method>,
    instruction_index: usize,
}

impl fmt::Debug for CallSiteKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CallSiteKey")
            .field("class", &self.class.name())
            .field("method", &self.method.name())
            .field("instruction_index", &self.instruction_index)
            .finish()
    }
}

impl CallSiteKey {
    /// Create a new call site key
    pub fn new(class: Arc<Class>, method: Arc<Method>, instruction_index: usize) -> Self {
        Self {
            class,
            method,
            instruction_index,
        }
    }
}

impl PartialEq for CallSiteKey {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.class, &other.class)
            && Arc::ptr_eq(&self.method, &other.method)
            && self.instruction_index == other.instruction_index
    }
}

impl Eq for CallSiteKey {}

impl Hash for CallSiteKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(Arc::as_ptr(&self.class), state);
        std::ptr::hash(Arc::as_ptr(&self.method), state);
        self.instruction_index.hash(state);
    }
}

/// The JVM retains the error class and message for a failed indy instruction. The
/// original throwable (including its cause) escapes the first resolution attempt.
#[derive(Debug)]
struct LinkageFailure {
    class_name: JavaString,
    message: Value,
}

impl LinkageFailure {
    async fn replay(&self, thread: &Thread) -> Result<Value> {
        let (descriptor, mut arguments) = if self.message.is_null() {
            ("()V", vec![])
        } else {
            ("(Ljava/lang/String;)V", vec![self.message.clone()])
        };
        // Repeated linkage errors are recreated by the VM's bootstrap loader.
        let loader = thread.vm()?.class_loader().read().await.clone();
        let class = loader
            .bootstrap()
            .await?
            .load(&self.class_name)
            .await
            .map_err(|error| match error {
                ristretto_classloader::Error::ClassNotFound(name) => {
                    Error::from(JavaError::NoClassDefFoundError(name))
                }
                error => error.into(),
            })?;
        let constructor = class.method(CONSTRUCTOR_NAME, descriptor).ok_or_else(|| {
            JavaError::NoSuchMethodError(format!("{}.<init>{descriptor}", class.name()))
        })?;
        let throwable = Value::from_object(
            thread.vm()?.garbage_collector(),
            Object::new(class.clone())?,
        );
        arguments.insert(0, throwable.clone());
        Box::pin(thread.execute(&class, &constructor, &arguments)).await?;
        Err(Error::Throwable(throwable))
    }
}

type SiteEntry = OnceLock<Result<Value, LinkageFailure>>;

#[derive(Debug, Default)]
pub struct CallSiteCache {
    states: DashMap<CallSiteKey, Arc<SiteEntry>>,
}

impl CallSiteCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Publish a single outcome. Bootstrap code can re-enter the same instruction or
    /// synchronize with a concurrent bootstrap, so execute it outside the cache lock.
    pub async fn resolve_with_cache<F, Fut>(
        &self,
        thread: &Thread,
        key: CallSiteKey,
        resolver: F,
    ) -> Result<Value>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<Value>>,
    {
        let entry = self.states.entry(key).or_default().clone();
        if entry.get().is_none() {
            match resolver().await {
                Ok(site) => {
                    let _ = entry.set(Ok(site));
                }
                Err(error) => {
                    let throwable =
                        crate::instruction::convert_error_to_throwable(thread, error).await?;
                    let class = throwable.as_object_ref()?.class().clone();
                    if !thread
                        .class("java/lang/LinkageError")
                        .await?
                        .is_assignable_from(thread, &class)
                        .await?
                    {
                        return Err(Error::Throwable(throwable));
                    }
                    let message = throwable.as_object_ref()?.value("detailMessage")?;
                    if entry
                        .set(Err(LinkageFailure {
                            class_name: class.java_name().to_owned(),
                            message,
                        }))
                        .is_ok()
                    {
                        return Err(Error::Throwable(throwable));
                    }
                }
            }
        }
        Self::published(&entry, thread).await
    }

    async fn published(entry: &SiteEntry, thread: &Thread) -> Result<Value> {
        match entry
            .get()
            .ok_or_else(|| Error::InternalError("Missing invokedynamic linkage result".into()))?
        {
            Ok(site) => Ok(site.clone()),
            Err(error) => error.replay(thread).await,
        }
    }

    /// Clear all cached call sites
    pub fn clear(&self) {
        self.states.clear();
    }

    /// Get the number of cached call sites
    pub fn len(&self) -> usize {
        self.states.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn test_class() -> Result<Arc<Class>> {
        let mut constant_pool = ristretto_classfile::ConstantPool::new();
        let this_class = constant_pool.add_class("TestClass")?;
        Ok(Class::from(
            None,
            ristretto_classfile::ClassFile {
                constant_pool,
                this_class,
                ..Default::default()
            },
        )?)
    }

    #[tokio::test]
    async fn test_same_named_definitions_have_separate_call_sites() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let method = thread
            .class("java/lang/Object")
            .await?
            .try_get_method("<init>", "()V")?;
        let cache = CallSiteCache::new();
        let first = CallSiteKey::new(test_class()?, method.clone(), 42);
        let second = CallSiteKey::new(test_class()?, method.clone(), 42);
        assert_eq!(first.class.name(), second.class.name());
        assert_ne!(first, second);
        cache
            .resolve_with_cache(&thread, first.clone(), || async { Ok(Value::Int(1)) })
            .await?;
        let result = cache
            .resolve_with_cache(&thread, second, || async { Ok(Value::Int(2)) })
            .await?;
        assert_eq!(result, Value::Int(2));
        let cached = cache
            .resolve_with_cache(&thread, first, || async { Ok(Value::Int(3)) })
            .await?;
        assert_eq!(cached, Value::Int(1));
        Ok(())
    }

    #[tokio::test]
    async fn test_call_site_cache_basic_resolution() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let method = thread
            .class("java/lang/Object")
            .await?
            .try_get_method("<init>", "()V")?;
        let cache = CallSiteCache::new();
        let key = CallSiteKey::new(test_class()?, method.clone(), 42);
        let expected_value = Value::Object(None);

        let result = cache
            .resolve_with_cache(&thread, key.clone(), || async {
                Ok(expected_value.clone())
            })
            .await?;

        assert_eq!(result, expected_value);
        Ok(())
    }

    #[tokio::test]
    async fn test_call_site_cache_concurrent_resolution() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let method = thread
            .class("java/lang/Object")
            .await?
            .try_get_method("<init>", "()V")?;
        let cache = CallSiteCache::new();
        let key = CallSiteKey::new(test_class()?, method, 42);
        let barrier = tokio::sync::Barrier::new(2);
        let attempt = |value| {
            let barrier = &barrier;
            move || async move {
                barrier.wait().await;
                Ok(Value::Int(value))
            }
        };
        let (first, second) = tokio::join!(
            cache.resolve_with_cache(&thread, key.clone(), attempt(1)),
            cache.resolve_with_cache(&thread, key, attempt(2)),
        );
        let winner = first?;
        assert!(matches!(winner, Value::Int(1 | 2)));
        assert_eq!(winner, second?);
        Ok(())
    }

    #[tokio::test]
    async fn test_call_site_cache_returns_cached_result() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let method = thread
            .class("java/lang/Object")
            .await?
            .try_get_method("<init>", "()V")?;
        let cache = CallSiteCache::new();
        let key = CallSiteKey::new(test_class()?, method.clone(), 42);
        let expected_value = Value::Object(None);

        // First resolution
        let result1 = cache
            .resolve_with_cache(&thread, key.clone(), || async {
                Ok(expected_value.clone())
            })
            .await?;

        // Second resolution should return cached result
        let result2 = cache
            .resolve_with_cache(&thread, key, || async {
                panic!("Resolver should not be called for cached result")
            })
            .await?;

        assert_eq!(result1, result2);
        Ok(())
    }
}
