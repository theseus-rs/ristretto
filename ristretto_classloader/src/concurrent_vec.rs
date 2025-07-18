use crate::Error::PoisonedLock;
use crate::Result;
use ristretto_gc::Gc;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// A concurrent vector.
pub struct ConcurrentVec<T: Clone + Debug + PartialEq + Send + Sync> {
    inner: Gc<RwLock<Vec<T>>>,
}

impl<T: Clone + Debug + PartialEq + Send + Sync> ConcurrentVec<T> {
    /// Create a new concurrent vector.
    #[must_use]
    pub fn new() -> Self {
        Self::from(Vec::new())
    }

    /// Create a new concurrent vector from a vector.
    #[must_use]
    pub fn from(values: Vec<T>) -> Self {
        ConcurrentVec {
            inner: Gc::new(RwLock::new(values)),
        }
    }

    /// Push a value onto the vector.
    ///
    /// # Errors
    ///
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
    ///
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
    ///
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
    ///
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
    ///
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
    ///
    /// if the lock is poisoned.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }

    /// Get the capacity of the vector.
    ///
    /// # Errors
    ///
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
    ///
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

    /// Get a read lock guard to the inner vector.
    ///
    /// # Errors
    ///
    /// if the lock is poisoned.
    pub fn as_ref(&self) -> Result<RwLockReadGuard<'_, Vec<T>>> {
        let guard = self
            .inner
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(guard)
    }

    /// Get a write lock guard to the inner vector.
    ///
    /// # Errors
    ///
    /// if the lock is poisoned.
    pub fn as_mut(&self) -> Result<RwLockWriteGuard<'_, Vec<T>>> {
        let guard = self
            .inner
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(guard)
    }

    /// Convert to a vector.
    ///
    /// # Errors
    ///
    /// if the lock is poisoned.
    pub fn to_vec(&self) -> Result<Vec<T>> {
        let vec = self
            .inner
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        Ok(vec.clone())
    }

    /// Returns hash code implementation based on memory address.
    #[must_use]
    pub fn hash_code(&self) -> usize {
        Gc::as_ptr(&self.inner).cast::<Vec<RwLock<T>>>() as usize
    }

    /// Check if two concurrent vectors point to the same memory location.
    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Gc::ptr_eq(&self.inner, &other.inner)
    }

    /// Deep clone the concurrent vector.
    ///
    /// # Errors
    ///
    /// if the lock is poisoned.
    pub fn deep_clone(&self) -> Result<Self> {
        let vec = self.to_vec()?;
        Ok(Self::from(vec))
    }
}

impl<T: Clone + Debug + PartialEq + Send + Sync> Clone for ConcurrentVec<T> {
    /// Clone the concurrent vector.
    fn clone(&self) -> Self {
        ConcurrentVec {
            inner: Gc::clone(&self.inner),
        }
    }
}

impl<T: Clone + Debug + PartialEq + Send + Sync> Debug for ConcurrentVec<T> {
    /// Debug the concurrent vector.
    #[expect(clippy::unwrap_in_result)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = self.inner.read().expect("poisoned lock");
        write!(f, "{:?}", &*vec)
    }
}

impl<T: Clone + Debug + PartialEq + Send + Sync> Default for ConcurrentVec<T> {
    /// Create a default concurrent vector.
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Debug + PartialEq + Send + Sync> Display for ConcurrentVec<T> {
    /// Display the concurrent vector.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let vec = self.inner.read().map_err(|_| fmt::Error)?;
        let mut values = Vec::new();
        for value in &*vec {
            let value = format!("{value:?}");
            if value.len() > 100 {
                values.push(format!("{}...", &value[..97]));
            } else {
                values.push(value);
            }
        }
        write!(f, "[{}]", values.join(", "))
    }
}

impl<T: Clone + Debug + PartialEq + Send + Sync> PartialEq for ConcurrentVec<T> {
    /// Compare two concurrent vectors.
    fn eq(&self, other: &Self) -> bool {
        // Quick reference equality check
        if Gc::ptr_eq(&self.inner, &other.inner) {
            return true;
        }
        // Use try_read to avoid blocking if locks are contended
        if let (Ok(inner), Ok(other_inner)) = (self.inner.try_read(), other.inner.try_read()) {
            *inner == *other_inner
        } else {
            let inner = self.inner.read().expect("poisoned lock");
            let other_inner = other.inner.read().expect("poisoned lock");
            *inner == *other_inner
        }
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
    fn test_as_ref() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        vec.push(2)?;
        vec.push(3)?;
        let guard = vec.as_ref()?;
        assert_eq!(*guard, vec![1, 2, 3]);
        Ok(())
    }

    #[test]
    fn test_as_mut() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        vec.push(2)?;
        vec.push(3)?;
        {
            let mut guard = vec.as_mut()?;
            guard.push(4);
            assert_eq!(*guard, vec![1, 2, 3, 4]);
        }
        assert_eq!(vec.len()?, 4);
        Ok(())
    }

    #[test]
    fn test_clone() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        let clone = vec.clone();
        assert_eq!(vec, clone);

        clone.set(0, 2)?;
        assert_eq!(vec, clone);
        Ok(())
    }

    #[test]
    fn test_hash_code() {
        let vec1 = ConcurrentVec::from(vec![1]);
        let vec2 = ConcurrentVec::from(vec![1]);
        assert_ne!(0, vec1.hash_code());
        assert_ne!(vec1.hash_code(), vec2.hash_code());
    }

    #[test]
    fn test_ptr_eq() -> Result<()> {
        let vec1 = ConcurrentVec::new();
        let vec2 = ConcurrentVec::new();
        assert!(!vec1.ptr_eq(&vec2));

        let vec3 = vec1.clone();
        assert!(vec1.ptr_eq(&vec3));

        vec1.push(1)?;
        assert!(!vec1.ptr_eq(&vec2));
        assert!(vec1.ptr_eq(&vec3));
        Ok(())
    }

    #[test]
    fn test_deep_clone() -> Result<()> {
        let vec = ConcurrentVec::new();
        vec.push(1)?;
        let clone = vec.deep_clone()?;
        assert_eq!(vec, clone);

        clone.set(0, 2)?;
        assert_ne!(vec, clone);
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

    #[test]
    fn test_to_string_large_values() -> Result<()> {
        // create a vector with 1024 sequential values
        let value = (0..1024).collect::<Vec<u16>>();
        let vec = ConcurrentVec::new();
        vec.push(value)?;
        assert_eq!(
            "[[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26...]",
            vec.to_string(),
        );
        Ok(())
    }
}
