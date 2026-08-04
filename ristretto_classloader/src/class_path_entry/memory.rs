use crate::Error::ClassNotFound;
use crate::Result;
use ahash::AHashMap;
use ristretto_classfile::ClassFile;
use std::ffi::{OsStr, OsString};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::RwLock;

/// An in-memory class path entry.
#[derive(Clone, Default)]
pub struct Memory {
    name: OsString,
    classes: Arc<RwLock<AHashMap<String, MemoryClass>>>,
}

/// A class definition retained by an in-memory class path entry.
#[derive(Clone, Debug)]
struct MemoryClass {
    bytes: Vec<u8>,
    class_file: ClassFile<'static>,
}

impl Memory {
    /// Create a new in-memory class path entry.
    pub fn new<S: AsRef<OsStr>>(name: S) -> Self {
        Self {
            name: name.as_ref().to_os_string(),
            classes: Arc::new(RwLock::new(AHashMap::default())),
        }
    }

    /// Get the name of the in-memory class path entry.
    #[must_use]
    pub fn name(&self) -> &OsString {
        &self.name
    }

    /// Add or replace a class definition.
    ///
    /// # Errors
    ///
    /// if `bytes` does not contain a valid class file or its class name cannot be read.
    pub async fn add_class(&self, bytes: &[u8]) -> Result<()> {
        let class_file = ClassFile::from_bytes(bytes)?;
        let class_name = class_file.class_name()?.to_rust_string();
        let memory_class = MemoryClass {
            bytes: bytes.to_vec(),
            class_file,
        };
        self.classes.write().await.insert(class_name, memory_class);
        Ok(())
    }

    /// Remove a class definition.
    pub async fn remove_class<S: AsRef<str>>(&self, name: S) -> bool {
        let name = normalize_class_name(name.as_ref());
        self.classes.write().await.remove(&name).is_some()
    }

    /// Read a class from memory.
    ///
    /// # Errors
    ///
    /// if the requested class is not in this entry.
    pub async fn read_class<S: AsRef<str>>(&self, name: S) -> Result<ClassFile<'static>> {
        let name = normalize_class_name(name.as_ref());
        let classes = self.classes.read().await;
        classes
            .get(&name)
            .map(|memory_class| memory_class.class_file.clone())
            .ok_or(ClassNotFound(name))
    }

    /// Read a class resource from memory.
    ///
    /// # Errors
    ///
    /// if the resource cannot be read.
    pub async fn read_resource<S: AsRef<str>>(&self, name: S) -> Result<Option<Vec<u8>>> {
        let Some(name) = name.as_ref().strip_suffix(".class") else {
            return Ok(None);
        };
        let name = normalize_class_name(name.trim_start_matches('/'));
        let classes = self.classes.read().await;
        Ok(classes
            .get(&name)
            .map(|memory_class| memory_class.bytes.clone()))
    }

    /// Get the class names in memory.
    ///
    /// # Errors
    ///
    /// if the class names cannot be read.
    pub async fn class_names(&self) -> Result<Vec<String>> {
        let classes = self.classes.read().await;
        Ok(classes.keys().cloned().collect())
    }
}

impl PartialEq for Memory {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && Arc::ptr_eq(&self.classes, &other.classes)
    }
}

impl Debug for Memory {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Memory")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

fn normalize_class_name(name: &str) -> String {
    name.replace('.', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::{ClassAccessFlags, ConstantPool, JAVA_21};

    fn class_bytes(name: &str) -> Result<Vec<u8>> {
        let mut constant_pool = ConstantPool::default();
        let this_class = constant_pool.add_class(name)?;
        let super_class = constant_pool.add_class("java/lang/Object")?;
        let class_file = ClassFile {
            version: JAVA_21,
            constant_pool,
            access_flags: ClassAccessFlags::PUBLIC,
            this_class,
            super_class,
            ..Default::default()
        };
        let mut bytes = Vec::new();
        class_file.to_bytes(&mut bytes)?;
        Ok(bytes)
    }

    #[tokio::test]
    async fn test_new_is_empty() -> Result<()> {
        let memory = Memory::new("test");

        assert_eq!(OsStr::new("test"), memory.name());
        assert!(memory.class_names().await?.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_default_is_empty_and_unnamed() -> Result<()> {
        let memory = Memory::default();

        assert_eq!(OsStr::new(""), memory.name());
        assert!(memory.class_names().await?.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_add_and_read_class_with_normalized_name() -> Result<()> {
        let memory = Memory::new("test");
        let bytes = class_bytes("com/example/Test")?;
        memory.add_class(&bytes).await?;

        let class_file = memory.read_class("com.example.Test").await?;
        assert_eq!("com/example/Test", class_file.class_name()?);
        assert_eq!(
            "com/example/Test",
            memory.read_class("com/example/Test").await?.class_name()?
        );
        assert_eq!(
            Some(bytes.clone()),
            memory.read_resource("/com/example/Test.class").await?
        );
        assert_eq!(
            Some(bytes),
            memory.read_resource("com.example.Test.class").await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_read_class_returns_independent_clone() -> Result<()> {
        let memory = Memory::new("test");
        memory.add_class(&class_bytes("MemoryTest")?).await?;

        let mut class_file = memory.read_class("MemoryTest").await?;
        class_file.access_flags |= ClassAccessFlags::FINAL;

        let class_file = memory.read_class("MemoryTest").await?;
        assert!(!class_file.access_flags.contains(ClassAccessFlags::FINAL));
        Ok(())
    }

    #[tokio::test]
    async fn test_clone_shares_classes() -> Result<()> {
        let memory = Memory::new("test");
        let clone = memory.clone();
        clone.add_class(&class_bytes("MemoryTest")?).await?;

        assert_eq!(memory, clone);
        assert_eq!(
            "MemoryTest",
            memory.read_class("MemoryTest").await?.class_name()?
        );
        assert!(memory.remove_class("MemoryTest").await);
        assert!(matches!(
            clone.read_class("MemoryTest").await,
            Err(ClassNotFound(name)) if name == "MemoryTest"
        ));
        Ok(())
    }

    #[test]
    fn test_independent_entries_are_not_equal() {
        let memory = Memory::new("test");

        assert_ne!(memory, Memory::new("test"));
        assert_ne!(memory, Memory::new("other"));
    }

    #[tokio::test]
    async fn test_remove_class_normalizes_name() -> Result<()> {
        let memory = Memory::new("test");
        memory.add_class(&class_bytes("com/example/Test")?).await?;

        assert!(memory.remove_class("com.example.Test").await);
        assert!(!memory.remove_class("com/example/Test").await);
        assert!(matches!(
            memory.read_class("com/example/Test").await,
            Err(ClassNotFound(name)) if name == "com/example/Test"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_class_bytes_do_not_change_classes() -> Result<()> {
        let memory = Memory::new("test");
        memory.add_class(&class_bytes("MemoryTest")?).await?;

        assert!(memory.add_class(&[0, 1, 2, 3]).await.is_err());
        assert_eq!(["MemoryTest"], memory.class_names().await?.as_slice());
        Ok(())
    }

    #[tokio::test]
    async fn test_replace_unloaded_class() -> Result<()> {
        let memory = Memory::new("test");
        let bytes = class_bytes("MemoryTest")?;
        memory.add_class(&bytes).await?;

        let mut replacement = ClassFile::from_bytes(&bytes)?;
        replacement.access_flags |= ClassAccessFlags::FINAL;
        let mut replacement_bytes = Vec::new();
        replacement.to_bytes(&mut replacement_bytes)?;
        memory.add_class(&replacement_bytes).await?;

        let class_file = memory.read_class("MemoryTest").await?;
        assert!(class_file.access_flags.contains(ClassAccessFlags::FINAL));
        assert_eq!(
            Some(replacement_bytes),
            memory.read_resource("MemoryTest.class").await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_read_missing_class_returns_normalized_name() {
        let memory = Memory::new("test");

        assert!(matches!(
            memory.read_class("com.example.Missing").await,
            Err(ClassNotFound(name)) if name == "com/example/Missing"
        ));
    }

    #[tokio::test]
    async fn test_read_resource_rejects_non_class_and_missing_resources() -> Result<()> {
        let memory = Memory::new("test");
        memory.add_class(&class_bytes("MemoryTest")?).await?;

        assert!(memory.read_resource("MemoryTest").await?.is_none());
        assert!(memory.read_resource("MemoryTest.CLASS").await?.is_none());
        assert!(memory.read_resource("Missing.class").await?.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_class_names() -> Result<()> {
        let memory = Memory::new("test");
        for name in ["z/Last", "a/First", "m/Middle"] {
            memory.add_class(&class_bytes(name)?).await?;
        }

        let class_names = memory.class_names().await?;
        assert_eq!(3, class_names.len());
        assert!(class_names.iter().any(|name| name == "a/First"));
        assert!(class_names.iter().any(|name| name == "m/Middle"));
        assert!(class_names.iter().any(|name| name == "z/Last"));
        Ok(())
    }

    #[test]
    fn test_debug_does_not_include_classes() {
        let memory = Memory::new("test");
        assert_eq!("Memory { name: \"test\", .. }", format!("{memory:?}"));
    }
}
