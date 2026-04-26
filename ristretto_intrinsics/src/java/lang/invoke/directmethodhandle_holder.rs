use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getReference([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_reference<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.getReference([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeInterface([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn invoke_interface<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.invokeInterface([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeSpecial([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn invoke_special<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.invokeSpecial([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeStatic([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn invoke_static<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.invokeStatic([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeVirtual([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn invoke_virtual<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.invokeVirtual([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.getReference([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_reference_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.getReference([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeInterface([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn invoke_interface_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.invokeInterface([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeSpecial([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn invoke_special_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.invokeSpecial([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeStatic([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn invoke_static_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.invokeStatic([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[intrinsic_method(
    "java/lang/invoke/DirectMethodHandle$Holder.invokeVirtual([Ljava/lang/Object;)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn invoke_virtual_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java/lang/invoke/DirectMethodHandle$Holder.invokeVirtual([Ljava/lang/Object;)Ljava/lang/Object;".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_reference() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_reference(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.getReference([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_invoke_interface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoke_interface(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.invokeInterface([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_invoke_special() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoke_special(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.invokeSpecial([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_invoke_static() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoke_static(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.invokeStatic([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_invoke_virtual() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoke_virtual(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.invokeVirtual([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_reference_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_reference_ge_v11(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.getReference([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_invoke_interface_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            invoke_interface_ge_v11(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.invokeInterface([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_invoke_special_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            invoke_special_ge_v11(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.invokeSpecial([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_invoke_static_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = invoke_static_ge_v11(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.invokeStatic([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_invoke_virtual_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            invoke_virtual_ge_v11(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "java/lang/invoke/DirectMethodHandle$Holder.invokeVirtual([Ljava/lang/Object;)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }
}
