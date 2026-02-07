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

                    let (Some(self_component), Some(other_component)) =
                        (this.component_type(), other.component_type())
                    else {
                        return Ok(false);
                    };

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
