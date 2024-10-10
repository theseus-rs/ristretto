use crate::Error::PoisonedLock;
use crate::Result;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::{Arc, RwLock};

/// A concurrent vector.
pub struct ConcurrentVec<T: Clone + Debug + PartialEq> {
    inner: Arc<RwLock<Vec<T>>>,
}

impl<T: Clone + Debug + PartialEq> ConcurrentVec<T> {
    /// Create a new concurrent vector.
    #[must_use]
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    /// Create a new concurrent vector with the defined capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from(Vec::with_capacity(capacity))
    }

    /// Create a new concurrent vector from a vector.
    #[must_use]
    pub fn from(values: Vec<T>) -> Self {
        ConcurrentVec {
            inner: Arc::new(RwLock::new(values)),
        }
    }

    /// Push a value onto the vector.
    ///
    /// # Errors
    /// if the lock is poisoned.
    pub fn push(&self, value: T) -> Result<()> {
        let mut vec = self
            .inner
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        vec.push(value);
        Ok(())
    }

    /// Pop a value from the vector.
    ///
    /// # Errors
    /// if the lock is poisoned.
    pub fn pop(&self) -> Result<Option<T>> {
        let mut vec = self
            .inner
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(vec.pop())
    }

    /// Get a value from the vector.
    ///
    /// # Errors
    /// if the lock is poisoned.
    pub fn get(&self, index: usize) -> Result<Option<T>>
    where
        T: Clone,
    {
        let vec = self
            .inner
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(vec.get(index).cloned())
    }

    /// Set a value in the vector.
    ///
    /// # Errors
    /// if the lock is poisoned.
    pub fn set(&self, index: usize, value: T) -> Result<Option<T>> {
        let mut vec = self
            .inner
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        let value = if index < vec.len() {
            Some(std::mem::replace(&mut vec[index], value))
        } else {
            None
        };
        Ok(value)
    }

    /// Get the length of the vector.
    ///
    /// # Errors
    /// if the lock is poisoned.
    pub fn len(&self) -> Result<usize> {
        let vec = self
            .inner
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(vec.len())
    }

    /// Check if the vector is empty.
    ///
    /// # Errors
    /// if the lock is poisoned.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Get the capacity of the vector.
    ///
    /// # Errors
    /// if the lock is poisoned.
    pub fn capacity(&self) -> Result<usize> {
        let vec = self
            .inner
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(vec.capacity())
    }

    /// Remove a value from the vector.
    ///
    /// # Errors
    /// if the lock is poisoned.
    pub fn remove(&self, index: usize) -> Result<Option<T>> {
        let mut vec = self
            .inner
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        let value = if index < vec.len() {
            Some(vec.remove(index))
        } else {
            None
        };
        Ok(value)
    }

    /// Convert to a vector.
    ///
    /// # Errors
    /// if the lock is poisoned.
    pub fn to_vec(&self) -> Result<Vec<T>> {
        let vec = self
            .inner
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(vec.clone())
    }
}

impl<T: Clone + Debug + PartialEq> Clone for ConcurrentVec<T> {
    /// Clone the concurrent vector.
    fn clone(&self) -> Self {
        ConcurrentVec {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T: Clone + Debug + PartialEq> Debug for ConcurrentVec<T> {
    /// Debug the concurrent vector.
    #[expect(clippy::unwrap_in_result)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = self.inner.read().expect("poisoned lock");
        write!(f, "{:?}", &*vec)
    }
}

impl<T: Clone + Debug + PartialEq> Default for ConcurrentVec<T> {
    /// Create a default concurrent vector.
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Debug + PartialEq> Deref for ConcurrentVec<T> {
    type Target = RwLock<Vec<T>>;

    /// Get a reference to the inner vector.
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clone + Debug + PartialEq> Display for ConcurrentVec<T> {
    /// Display the concurrent vector.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = self.inner.read().map_err(|_| fmt::Error)?;
        write!(f, "{:?}", &*vec)
    }
}

impl<T: Clone + Debug + PartialEq> PartialEq for ConcurrentVec<T> {
    /// Compare two concurrent vectors.
    fn eq(&self, other: &Self) -> bool {
        let vec = self.inner.read().expect("poisoned lock");
        let other = other.inner.read().expect("poisoned lock");
        *vec == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        vec.push(2)?;
        vec.push(3)?;
        assert_eq!(vec.len()?, 3);
        Ok(())
    }

    #[test]
    fn test_pop() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        vec.push(2)?;
        vec.push(3)?;
        assert_eq!(vec.pop()?, Some(3));
        assert_eq!(vec.pop()?, Some(2));
        assert_eq!(vec.pop()?, Some(1));
        assert_eq!(vec.pop()?, None);
        Ok(())
    }

    #[test]
    fn test_get() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        vec.push(2)?;
        vec.push(3)?;
        assert_eq!(vec.get(0)?, Some(1));
        assert_eq!(vec.get(1)?, Some(2));
        assert_eq!(vec.get(2)?, Some(3));
        assert_eq!(vec.get(3)?, None);
        Ok(())
    }

    #[test]
    fn test_set() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        vec.push(2)?;
        vec.push(3)?;
        assert_eq!(vec.set(0, 4)?, Some(1));
        assert_eq!(vec.set(1, 5)?, Some(2));
        assert_eq!(vec.set(2, 6)?, Some(3));
        assert_eq!(vec.set(3, 7)?, None);
        assert_eq!(vec.get(0)?, Some(4));
        assert_eq!(vec.get(1)?, Some(5));
        assert_eq!(vec.get(2)?, Some(6));
        Ok(())
    }

    #[test]
    fn test_len() -> Result<()> {
        let vec = ConcurrentVec::new();
        assert_eq!(vec.len()?, 0);
        vec.push(1)?;
        assert_eq!(vec.len()?, 1);
        vec.push(2)?;
        assert_eq!(vec.len()?, 2);
        vec.push(3)?;
        assert_eq!(vec.len()?, 3);
        Ok(())
    }

    #[test]
    fn test_is_empty() -> Result<()> {
        let vec = ConcurrentVec::new();
        assert!(vec.is_empty()?);
        vec.push(1)?;
        assert!(!vec.is_empty()?);
        vec.push(2)?;
        assert!(!vec.is_empty()?);
        vec.push(3)?;
        assert!(!vec.is_empty()?);
        Ok(())
    }

    #[test]
    fn test_capacity() -> Result<()> {
        let vec: ConcurrentVec<u8> = ConcurrentVec::new();
        assert_eq!(vec.capacity()?, 0);
        let vec: ConcurrentVec<u8> = ConcurrentVec::with_capacity(10);
        assert_eq!(vec.capacity()?, 10);
        Ok(())
    }

    #[test]
    fn test_remove() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        vec.push(2)?;
        vec.push(3)?;
        assert_eq!(vec.remove(1)?, Some(2));
        assert_eq!(vec.remove(1)?, Some(3));
        assert_eq!(vec.remove(1)?, None);
        Ok(())
    }

    #[test]
    fn test_clone() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        vec.push(2)?;
        vec.push(3)?;
        let clone = vec.clone();
        assert_eq!(vec, clone);
        Ok(())
    }

    #[test]
    fn test_debug() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        vec.push(2)?;
        vec.push(3)?;
        let debug = format!("{vec:?}");
        assert_eq!("[1, 2, 3]", debug);
        Ok(())
    }
}
