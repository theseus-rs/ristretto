use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

async fn create_byte_buffer<T: Thread + 'static>(
    thread: &Arc<T>,
    bytes: Vec<i8>,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let byte_array = Value::new_object(vm.garbage_collector(), Reference::from(bytes));
    let result = thread
        .try_invoke(
            "java.nio.ByteBuffer",
            "wrap([B)Ljava/nio/ByteBuffer;",
            &[byte_array],
        )
        .await?;
    Ok(Some(result))
}

#[intrinsic_method(
    "sun/misc/Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn attach<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _mode = parameters.pop_int()?;
    let _lvmid = parameters.pop_int()?;
    let _user = parameters.pop()?;
    let _this = parameters.pop()?;
    create_byte_buffer(&thread, Vec::new()).await
}

#[intrinsic_method(
    "sun/misc/Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_byte_array<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _max_length = parameters.pop_int()?;
    let value = parameters.pop_reference()?;
    let _units = parameters.pop_int()?;
    let _variability = parameters.pop_int()?;
    let _name = parameters.pop()?;
    let _this = parameters.pop()?;
    let bytes = value.map_or_else(Vec::new, |reference| {
        let guard = reference.read();
        if let Reference::ByteArray(bytes) = &*guard {
            bytes.to_vec()
        } else {
            Vec::new()
        }
    });
    create_byte_buffer(&thread, bytes).await
}

#[intrinsic_method(
    "sun/misc/Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_long<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let value = parameters.pop_long()?;
    let _units = parameters.pop_int()?;
    let _variability = parameters.pop_int()?;
    let _name = parameters.pop()?;
    let _this = parameters.pop()?;
    #[expect(clippy::cast_possible_wrap)]
    let bytes = value.to_be_bytes().iter().map(|&byte| byte as i8).collect();
    create_byte_buffer(&thread, bytes).await
}

#[intrinsic_method(
    "sun/misc/Perf.detach(Ljava/nio/ByteBuffer;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn detach<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _buffer = parameters.pop()?;
    let _this = parameters.pop()?;
    Ok(None)
}

#[intrinsic_method("sun/misc/Perf.highResCounter()J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn high_res_counter<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| InternalError(error.to_string()))?;
    Ok(Some(Value::Long(i64::try_from(duration.as_nanos())?)))
}

#[intrinsic_method("sun/misc/Perf.highResFrequency()J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn high_res_frequency<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(1_000_000_000)))
}

#[intrinsic_method("sun/misc/Perf.registerNatives()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn register_natives<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_attach() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = attach(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await
        .expect("attach");
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_create_byte_array() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_byte_array(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await
        .expect("create byte array");
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_create_long() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_long(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await
        .expect("create long");
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_detach() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = detach(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await
        .expect("detach");
        assert_eq!(None, result);
    }

    #[tokio::test]
    async fn test_high_res_counter() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = high_res_counter(thread, Parameters::default())
            .await
            .expect("counter");
        assert!(matches!(result, Some(Value::Long(value)) if value > 0));
    }

    #[tokio::test]
    async fn test_high_res_frequency() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = high_res_frequency(thread, Parameters::default())
            .await
            .expect("frequency");
        assert_eq!(Some(Value::Long(1_000_000_000)), result);
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
