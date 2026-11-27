use crate::Result;
use crate::thread::Thread;
use ristretto_classloader::Class;
use std::sync::Arc;

pub trait Assignable {
    /// Returns true if the value can be assigned to the given type.
    async fn is_assignable_from(&self, thread: &Thread, other: &Arc<Class>) -> Result<bool>;
}

impl Assignable for Arc<Class> {
    /// Determine if this class is assignable from the given class.
    ///
    /// # Errors
    ///
    /// if classes or interfaces cannot be accessed.
    async fn is_assignable_from(&self, thread: &Thread, other: &Arc<Class>) -> Result<bool> {
        if Arc::ptr_eq(self, other) {
            return Ok(true);
        }

        let self_name = self.name();
        if self_name == "java/lang/Object" {
            return Ok(true);
        }

        // Array covariance is recursive over resolved component identities. Rank
        // comparisons and descriptor suffixes cannot distinguish, for example,
        // Cloneable[] <- int[][], and name lookup loses hidden/defining-loader types.
        if other.is_array() && self.is_array() {
            let target = thread.array_component_class(self).await?;
            let source = thread.array_component_class(other).await?;
            if target.is_primitive() || source.is_primitive() {
                return Ok(Arc::ptr_eq(&target, &source));
            }
            return Box::pin(target.is_assignable_from(thread, &source)).await;
        }

        // Check inheritance hierarchy
        if let Some(parent) = other.parent()?
            && Box::pin(self.is_assignable_from(thread, &parent)).await?
        {
            return Ok(true);
        }

        // Check interfaces
        for interface in other.interfaces()? {
            if Box::pin(self.is_assignable_from(thread, &interface)).await? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_object_is_assignable_from_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_class = thread.class("java.lang.Object").await?;
        assert!(
            object_class
                .is_assignable_from(&thread, &object_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_object_is_assignable_from_string() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let string_class = thread.class("java.lang.String").await?;
        let object_class = thread.class("java.lang.Object").await?;
        assert!(
            object_class
                .is_assignable_from(&thread, &string_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_string_not_is_assignable_from_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let string_class = thread.class("java.lang.String").await?;
        let object_class = thread.class("java.lang.Object").await?;
        assert!(
            !string_class
                .is_assignable_from(&thread, &object_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_string_not_is_assignable_from_serializable() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let string_class = thread.class("java.lang.String").await?;
        let serializable_class = thread.class("java.io.Serializable").await?;
        assert!(
            !string_class
                .is_assignable_from(&thread, &serializable_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_object_is_assignable_from_serializable_class() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_class = thread.class("java.lang.Object").await?;
        let serializable_class = thread.class("java.io.Serializable").await?;
        assert!(
            object_class
                .is_assignable_from(&thread, &serializable_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_not_is_assignable_from_object() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let object_class = thread.class("java.lang.Object").await?;
        assert!(
            !object_array_class
                .is_assignable_from(&thread, &object_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_object_is_assignable_from_int_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_class = thread.class("java.lang.Object").await?;
        let int_array_class = thread.class("[I").await?;
        assert!(
            object_class
                .is_assignable_from(&thread, &int_array_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_int_array_is_assignable_from_int_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let int_array_class = thread.class("[I").await?;
        assert!(
            int_array_class
                .is_assignable_from(&thread, &int_array_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_object_is_assignable_from_object_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_class = thread.class("java.lang.Object").await?;
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        assert!(
            object_class
                .is_assignable_from(&thread, &object_array_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_is_assignable_from_object_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        assert!(
            object_array_class
                .is_assignable_from(&thread, &object_array_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_not_is_assignable_from_int_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let int_array_class = thread.class("[I").await?;
        assert!(
            !object_array_class
                .is_assignable_from(&thread, &int_array_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_int_array_not_is_assignable_from_object_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let int_array_class = thread.class("[I").await?;
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        assert!(
            !int_array_class
                .is_assignable_from(&thread, &object_array_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_is_assignable_from_string_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let string_array_class = thread.class("[Ljava/lang/String;").await?;
        assert!(
            object_array_class
                .is_assignable_from(&thread, &string_array_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_string_array_not_is_assignable_from_object_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let string_array_class = thread.class("[Ljava/lang/String;").await?;
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        assert!(
            !string_array_class
                .is_assignable_from(&thread, &object_array_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_object_array_is_assignable_from_multiple_dimension_object_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let two_dimension_object_array_class = thread.class("[[Ljava/lang/Object;").await?;
        assert!(
            object_array_class
                .is_assignable_from(&thread, &two_dimension_object_array_class)
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_dimension_object_array_not_is_assignable_from_object_array() -> Result<()>
    {
        let (_vm, thread) = crate::test::thread().await?;
        let two_dimension_object_array_class = thread.class("[[Ljava/lang/Object;").await?;
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        assert!(
            !two_dimension_object_array_class
                .is_assignable_from(&thread, &object_array_class)
                .await?
        );
        Ok(())
    }
}
