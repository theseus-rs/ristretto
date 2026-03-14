use ristretto_classloader::{Reference, Value};
use ristretto_types::Error::InternalError;

/// Reads the integer `fd` field from a `FileDescriptor` Java object.
pub(crate) fn get_fd(fd_value: &Value) -> ristretto_types::Result<i32> {
    let guard = fd_value.as_reference()?;
    let Reference::Object(object) = &*guard else {
        return Err(InternalError("not a FileDescriptor object".to_string()));
    };
    Ok(object.value("fd")?.as_i32()?)
}

/// Writes the integer `fd` field on a `FileDescriptor` Java object.
pub(crate) fn set_fd(fd_value: &Value, fd: i32) -> ristretto_types::Result<()> {
    let mut guard = fd_value.as_reference_mut()?;
    let Reference::Object(object) = &mut *guard else {
        return Err(InternalError("not a FileDescriptor object".to_string()));
    };
    object.set_value("fd", Value::Int(fd))?;
    Ok(())
}

/// Reads the `fd` from a socket implementation object (navigates `this.fd.fd`).
pub(crate) fn get_impl_fd(this: &Value) -> ristretto_types::Result<i32> {
    let fd_value = {
        let this_ref = this.as_object_ref()?;
        this_ref.value("fd")?
    };
    let fd_ref = fd_value.as_object_ref()?;
    Ok(fd_ref.value("fd")?.as_i32()?)
}

/// Writes the `fd` on a socket implementation object (navigates `this.fd.fd`).
pub(crate) fn set_impl_fd(this: &Value, fd: i32) -> ristretto_types::Result<()> {
    let fd_value = {
        let this_ref = this.as_object_ref()?;
        this_ref.value("fd")?
    };
    let mut fd_ref = fd_value.as_object_mut()?;
    fd_ref.set_value("fd", Value::Int(fd))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Object;
    use ristretto_types::VM;

    /// Helper: create a `FileDescriptor` object with the given `fd` value.
    async fn create_file_descriptor(
        thread: &std::sync::Arc<ristretto_vm::Thread>,
        fd_val: i32,
    ) -> ristretto_types::Result<Value> {
        let vm = thread.vm()?;
        let class = thread.class("java/io/FileDescriptor").await?;
        let mut object = Object::new(class)?;
        object.set_value("fd", Value::Int(fd_val))?;
        let value = Value::new_object(vm.garbage_collector(), Reference::Object(object));
        Ok(value)
    }

    /// Helper: create a socket-impl-like object whose `fd` field is a `FileDescriptor`.
    async fn create_socket_impl(
        thread: &std::sync::Arc<ristretto_vm::Thread>,
        fd_val: i32,
    ) -> ristretto_types::Result<Value> {
        let vm = thread.vm()?;
        let fd_value = create_file_descriptor(thread, fd_val).await?;
        let impl_class = thread.class("java/net/SocketImpl").await?;
        let mut impl_object = Object::new(impl_class)?;
        impl_object.set_value("fd", fd_value)?;
        let value = Value::new_object(vm.garbage_collector(), Reference::Object(impl_object));
        Ok(value)
    }

    #[tokio::test]
    async fn test_get_fd() -> ristretto_types::Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let fd_value = create_file_descriptor(&thread, 42).await?;
        let result = get_fd(&fd_value)?;
        assert_eq!(42, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_fd_not_object() {
        let result = get_fd(&Value::Int(1));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_fd_null() {
        let result = get_fd(&Value::Object(None));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_fd_array_reference() {
        let gc = ristretto_gc::GarbageCollector::new();
        let array_ref = Reference::from(vec![1i32, 2i32]);
        let value = Value::new_object(&gc, array_ref);
        let result = get_fd(&value);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_fd() -> ristretto_types::Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let fd_value = create_file_descriptor(&thread, 0).await?;
        set_fd(&fd_value, 99)?;
        let result = get_fd(&fd_value)?;
        assert_eq!(99, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_fd_not_object() {
        let result = set_fd(&Value::Int(1), 42);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_fd_null() {
        let result = set_fd(&Value::Object(None), 42);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_fd_array_reference() {
        let gc = ristretto_gc::GarbageCollector::new();
        let array_ref = Reference::from(vec![1i32, 2i32]);
        let value = Value::new_object(&gc, array_ref);
        let result = set_fd(&value, 42);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_impl_fd() -> ristretto_types::Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let impl_value = create_socket_impl(&thread, 7).await?;
        let result = get_impl_fd(&impl_value)?;
        assert_eq!(7, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_impl_fd_not_object() {
        let result = get_impl_fd(&Value::Int(1));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_impl_fd_null() {
        let result = get_impl_fd(&Value::Object(None));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_impl_fd_array_reference() {
        let gc = ristretto_gc::GarbageCollector::new();
        let array_ref = Reference::from(vec![1i32]);
        let value = Value::new_object(&gc, array_ref);
        let result = get_impl_fd(&value);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_impl_fd_null_fd_field() -> ristretto_types::Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let vm = thread.vm()?;
        let impl_class = thread.class("java/net/SocketImpl").await?;
        let mut impl_object = Object::new(impl_class)?;
        impl_object.set_value("fd", Value::Object(None))?;
        let value = Value::new_object(vm.garbage_collector(), Reference::Object(impl_object));
        let result = get_impl_fd(&value);
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_set_impl_fd() -> ristretto_types::Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let impl_value = create_socket_impl(&thread, 0).await?;
        set_impl_fd(&impl_value, 55)?;
        let result = get_impl_fd(&impl_value)?;
        assert_eq!(55, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_impl_fd_not_object() {
        let result = set_impl_fd(&Value::Int(1), 42);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_impl_fd_null() {
        let result = set_impl_fd(&Value::Object(None), 42);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_impl_fd_array_reference() {
        let gc = ristretto_gc::GarbageCollector::new();
        let array_ref = Reference::from(vec![1i32]);
        let value = Value::new_object(&gc, array_ref);
        let result = set_impl_fd(&value, 42);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_impl_fd_null_fd_field() -> ristretto_types::Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let vm = thread.vm()?;
        let impl_class = thread.class("java/net/SocketImpl").await?;
        let mut impl_object = Object::new(impl_class)?;
        impl_object.set_value("fd", Value::Object(None))?;
        let value = Value::new_object(vm.garbage_collector(), Reference::Object(impl_object));
        let result = set_impl_fd(&value, 42);
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_fd_roundtrip() -> ristretto_types::Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let fd_value = create_file_descriptor(&thread, -1).await?;
        assert_eq!(-1, get_fd(&fd_value)?);
        set_fd(&fd_value, 123)?;
        assert_eq!(123, get_fd(&fd_value)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_impl_fd_roundtrip() -> ristretto_types::Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let impl_value = create_socket_impl(&thread, -1).await?;
        assert_eq!(-1, get_impl_fd(&impl_value)?);
        set_impl_fd(&impl_value, 200)?;
        assert_eq!(200, get_impl_fd(&impl_value)?);
        Ok(())
    }
}
