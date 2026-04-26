use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/loops/GraphicsPrimitiveMgr.initIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
    Any
)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _s_hints = parameters.pop_reference()?;
    let _path2_d_float = parameters.pop_reference()?;
    let _path2_d = parameters.pop_reference()?;
    let _alpha_comp = parameters.pop_reference()?;
    let _xor_comp = parameters.pop_reference()?;
    let _at = parameters.pop_reference()?;
    let _color = parameters.pop_reference()?;
    let _sg2_d = parameters.pop_reference()?;
    let _ct = parameters.pop_reference()?;
    let _st = parameters.pop_reference()?;
    let _gp = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.loops.GraphicsPrimitiveMgr.initIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V".to_string()).into())
}

#[intrinsic_method("sun/java2d/loops/GraphicsPrimitiveMgr.registerNativeLoops()V", Any)]
#[async_method]
pub async fn register_native_loops<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.loops.GraphicsPrimitiveMgr.registerNativeLoops()V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.loops.GraphicsPrimitiveMgr.initIDs(Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;Ljava/lang/Class;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_register_native_loops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = register_native_loops(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.loops.GraphicsPrimitiveMgr.registerNativeLoops()V",
            result.unwrap_err().to_string()
        );
    }
}
