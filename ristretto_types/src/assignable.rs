use crate::Result;
use crate::Thread;
use ristretto_classloader::Class;
use std::sync::Arc;

/// Trait for checking type assignability between Java classes.
pub trait Assignable<T: Thread> {
    /// Returns true if the value can be assigned to the given type.
    ///
    /// # Errors
    /// Returns an error if the assignability check fails.
    fn is_assignable_from<'a>(
        &'a self,
        thread: &'a T,
        other: &'a Arc<Class>,
    ) -> crate::BoxFuture<'a, Result<bool>>;
}

impl<T: Thread + 'static> Assignable<T> for Arc<Class> {
    fn is_assignable_from<'a>(
        &'a self,
        thread: &'a T,
        other: &'a Arc<Class>,
    ) -> crate::BoxFuture<'a, Result<bool>> {
        let this = self.clone();
        let other = other.clone();
        Box::pin(async move {
            if Arc::ptr_eq(&this, &other) {
                return Ok(true);
            }

            let self_name = this.name();
            if self_name == "java/lang/Object" {
                return Ok(true);
            }

            // Handle array types
            if other.is_array() && this.is_array() {
                let self_dimensions = this.array_dimensions();
                let other_dimensions = other.array_dimensions();

                if self_dimensions == other_dimensions {
                    let other_name = other.name();
                    let self_is_primitive = !self_name.ends_with(';');
                    let other_is_primitive = !other_name.ends_with(';');

                    if self_is_primitive && other_is_primitive {
                        return Ok(self_name == other_name);
                    }

                    if self_is_primitive != other_is_primitive {
                        return Ok(false);
                    }

                    let self_component = this.component_type().unwrap_or_default();
                    let other_component = other.component_type().unwrap_or_default();
                    let self_component_class = thread.class(self_component).await?;
                    let other_component_class = thread.class(other_component).await?;
                    return self_component_class
                        .is_assignable_from(thread, &other_component_class)
                        .await;
                } else if self_dimensions < other_dimensions && self_name == "[Ljava/lang/Object;" {
                    return Ok(true);
                }

                return Ok(false);
            }

            // Check inheritance hierarchy
            if let Some(parent) = other.parent()?
                && this.is_assignable_from(thread, &parent).await?
            {
                return Ok(true);
            }

            // Check interfaces
            for interface in other.interfaces()? {
                if this.is_assignable_from(thread, &interface).await? {
                    return Ok(true);
                }
            }

            Ok(false)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;
    use ristretto_classfile::JAVA_17;

    #[tokio::test]
    async fn test_assignable_identity_object_arrays_parents_and_interfaces() -> Result<()> {
        let vm = test_utils::MockVm::new(JAVA_17);
        let thread = test_utils::MockThread::new(vm);

        let object = thread.class("java/lang/Object").await?;
        let string = thread.class("java/lang/String").await?;
        assert!(string.is_assignable_from(&*thread, &string).await?);
        assert!(object.is_assignable_from(&*thread, &string).await?);

        let int_array = test_utils::class("[I", &[])?;
        let other_int_array = test_utils::class("[I", &[])?;
        let long_array = test_utils::class("[J", &[])?;
        assert!(
            int_array
                .is_assignable_from(&*thread, &other_int_array)
                .await?
        );
        assert!(!int_array.is_assignable_from(&*thread, &long_array).await?);

        let string_array = test_utils::class("[Ljava/lang/String;", &[])?;
        let object_array = test_utils::class("[Ljava/lang/Object;", &[])?;
        assert!(
            object_array
                .is_assignable_from(&*thread, &string_array)
                .await?
        );
        assert!(
            !int_array
                .is_assignable_from(&*thread, &string_array)
                .await?
        );

        let two_dimensional_string_array = test_utils::class("[[Ljava/lang/String;", &[])?;
        assert!(
            object_array
                .is_assignable_from(&*thread, &two_dimensional_string_array)
                .await?
        );
        assert!(
            !string_array
                .is_assignable_from(&*thread, &two_dimensional_string_array)
                .await?
        );

        let parent = test_utils::class("example/Parent", &[])?;
        let child = test_utils::class("example/Child", &[])?;
        child.set_parent(Some(parent.clone()))?;
        assert!(parent.is_assignable_from(&*thread, &child).await?);

        let interface = test_utils::class("example/Interface", &[])?;
        let implementation = test_utils::class("example/Implementation", &[])?;
        implementation.set_interfaces(vec![interface.clone()])?;
        assert!(
            interface
                .is_assignable_from(&*thread, &implementation)
                .await?
        );
        let unrelated_interface = test_utils::class("example/UnrelatedInterface", &[])?;
        let unrelated_implementation = test_utils::class("example/UnrelatedImplementation", &[])?;
        unrelated_implementation.set_interfaces(vec![unrelated_interface])?;
        assert!(
            !interface
                .is_assignable_from(&*thread, &unrelated_implementation)
                .await?
        );
        assert!(
            !implementation
                .is_assignable_from(&*thread, &interface)
                .await?
        );
        Ok(())
    }
}
