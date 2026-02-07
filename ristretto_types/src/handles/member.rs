use ristretto_classloader::Method;
use std::sync::Arc;

/// Represents a handle to a member in the Java Virtual Machine (JVM). This is used to dynamically
/// invoke methods or access fields in a class.
#[derive(Debug)]
pub struct MemberHandle {
    pub method: Option<Arc<Method>>,
    pub field: Option<usize>,
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
    use ristretto_classfile::MethodAccessFlags;

    fn test_method() -> Arc<Method> {
        let definition = ristretto_classfile::Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 0,
            descriptor_index: 0,
            attributes: Vec::new(),
        };
        Arc::new(Method::new_synthetic(
            definition,
            "testMethod".to_string(),
            "()V".to_string(),
            Vec::new(),
            None,
        ))
    }

    #[test]
    fn test_member_handle_from_method() {
        let method = test_method();
        let handle: MemberHandle = method.into();
        assert!(handle.method.is_some());
        assert_eq!(handle.method.as_ref().unwrap().name(), "testMethod");
        assert!(handle.field.is_none());
    }

    #[test]
    fn test_member_handle_from_field() {
        let handle: MemberHandle = 42usize.into();
        assert!(handle.method.is_none());
        assert_eq!(handle.field, Some(42));
    }

    #[test]
    fn test_member_handle_from_field_zero() {
        let handle: MemberHandle = 0usize.into();
        assert!(handle.method.is_none());
        assert_eq!(handle.field, Some(0));
    }

    #[test]
    fn test_member_handle_debug() {
        let handle: MemberHandle = 1usize.into();
        let debug = format!("{handle:?}");
        assert!(debug.contains("MemberHandle"));
    }
}
