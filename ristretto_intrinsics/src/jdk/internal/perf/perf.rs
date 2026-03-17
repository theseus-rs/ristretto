use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Helper function to create a `java.nio.ByteBuffer` wrapping a byte array by invoking
/// `ByteBuffer.wrap(byte[])`.
async fn create_byte_buffer<T: Thread + 'static>(
    thread: &Arc<T>,
    bytes: Vec<i8>,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    let collector = vm.garbage_collector();
    let byte_array = Value::new_object(collector, Reference::from(bytes));
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
    "jdk/internal/perf/Perf.attach(Ljava/lang/String;II)Ljava/nio/ByteBuffer;",
    Between(JAVA_11, JAVA_17)
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
    "jdk/internal/perf/Perf.attach0(I)Ljava/nio/ByteBuffer;",
    GreaterThan(JAVA_17)
)]
#[async_method]
pub async fn attach_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _lvmid = parameters.pop_int()?;
    let _this = parameters.pop()?;
    create_byte_buffer(&thread, Vec::new()).await
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.createByteArray(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;",
    GreaterThanOrEqual(JAVA_11)
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

    let bytes = match value {
        Some(reference) => {
            let guard = reference.read();
            match &*guard {
                Reference::ByteArray(array) => array.to_vec(),
                _ => Vec::new(),
            }
        }
        None => Vec::new(),
    };

    create_byte_buffer(&thread, bytes).await
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.createLong(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",
    GreaterThanOrEqual(JAVA_11)
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
    let bytes: Vec<i8> = value.to_be_bytes().iter().map(|&b| b as i8).collect();
    create_byte_buffer(&thread, bytes).await
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.detach(Ljava/nio/ByteBuffer;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn detach<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.highResCounter()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn high_res_counter<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let now = SystemTime::now();
    let duration = now
        .duration_since(UNIX_EPOCH)
        .map_err(|error| InternalError(error.to_string()))?;
    let time = i64::try_from(duration.as_nanos())?;
    Ok(Some(Value::Long(time)))
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.highResFrequency()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn high_res_frequency<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Long(1_000_000_000)))
}

#[intrinsic_method(
    "jdk/internal/perf/Perf.registerNatives()V",
    GreaterThanOrEqual(JAVA_11)
)]
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
    use ristretto_types::VM;

    #[tokio::test]
    async fn test_attach() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None)); // this
        parameters.push(Value::Object(None)); // user
        parameters.push_int(0); // lvmid
        parameters.push_int(0); // mode
        let result = attach(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_attach_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None)); // this
        parameters.push_int(0); // lvmid
        let result = attach_0(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_byte_array() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let vm = thread.vm()?;
        let collector = vm.garbage_collector();
        let bytes: Vec<i8> = vec![1, 2, 3];
        let byte_array = Value::new_object(collector, Reference::from(bytes));
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None)); // this
        parameters.push(Value::Object(None)); // name
        parameters.push_int(0); // variability
        parameters.push_int(0); // units
        parameters.push(byte_array); // value
        parameters.push_int(10); // maxLength
        let result = create_byte_array(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_long() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        parameters.push(Value::Object(None)); // this
        parameters.push(Value::Object(None)); // name
        parameters.push_int(0); // variability
        parameters.push_int(0); // units
        parameters.push_long(42); // value
        let result = create_long(thread, parameters).await?;
        assert!(result.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_detach() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = detach(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_high_res_counter() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = high_res_counter(thread, Parameters::default()).await?;
        assert!(matches!(result, Some(Value::Long(v)) if v > 0));
        Ok(())
    }

    #[tokio::test]
    async fn test_high_res_frequency() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = high_res_frequency(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::Long(1_000_000_000)));
        Ok(())
    }

    #[tokio::test]
    async fn test_register_natives() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = register_natives(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
