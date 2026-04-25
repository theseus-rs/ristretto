use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/pipe/BufferedRenderPipe.fillSpans(Lsun/java2d/pipe/RenderQueue;JIILsun/java2d/pipe/SpanIterator;JII)I",
    Any
)]
#[async_method]
pub async fn fill_spans<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _transy = parameters.pop_int()?;
    let _transx = parameters.pop_int()?;
    let _iterator = parameters.pop_long()?;
    let _si = parameters.pop_reference()?;
    let _limit = parameters.pop_int()?;
    let _pos = parameters.pop_int()?;
    let _buf = parameters.pop_long()?;
    let _rq = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.pipe.BufferedRenderPipe.fillSpans(Lsun/java2d/pipe/RenderQueue;JIILsun/java2d/pipe/SpanIterator;JII)I".to_string()).into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fill_spans() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fill_spans(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.pipe.BufferedRenderPipe.fillSpans(Lsun/java2d/pipe/RenderQueue;JIILsun/java2d/pipe/SpanIterator;JII)I",
            result.unwrap_err().to_string()
        );
    }
}
