use ristretto_classloader::Method;
use std::sync::Arc;

/// Represents a handle to a member in the Java Virtual Machine (JVM). This is used to dynamically
/// invoke methods or access fields in a class.
#[derive(Debug)]
pub(crate) struct MemberHandle {
    pub(crate) method: Option<Arc<Method>>,
    pub(crate) field: Option<usize>,
}

impl From<Arc<Method>> for MemberHandle {
    fn from(method: Arc<Method>) -> Self {
        MemberHandle {
            method: Some(method),
            field: None,
        }
    }
}

impl From<usize> for MemberHandle {
    fn from(field: usize) -> Self {
        MemberHandle {
            method: None,
            field: Some(field),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[tokio::test]
    async fn test_member_handle_from_method() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java.lang.Object").await?;
        let method = class.try_get_method("hashCode", "()I")?;
        let member_handle: MemberHandle = method.into();
        assert_eq!(member_handle.method.expect("method").name(), "hashCode");
        assert!(member_handle.field.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_member_handle_from_field() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let class = thread.class("java.lang.Integer").await?;
        let field = class.field_offset("serialVersionUID")?;
        let method_handle: MemberHandle = field.into();
        assert!(method_handle.method.is_none());
        assert_eq!(method_handle.field.expect("name"), field);
        Ok(())
    }
}
